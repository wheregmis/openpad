use makepad_widgets::*;
use openpad_widgets::SidePanelWidgetRefExt;
use openpad_protocol::{
    Event as OcEvent, HealthResponse, Message, OpenCodeClient, Project, Session,
};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Local, Utc};

/// Format millisecond timestamp to relative time string
#[allow(dead_code)]
fn format_relative_time(timestamp_ms: i64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(timestamp_ms / 1000, 0)
        .unwrap_or_else(|| Utc::now());
    let local: DateTime<Local> = datetime.into();
    let now = Local::now();

    // Check if it's the same calendar day
    let is_today = local.date_naive() == now.date_naive();
    let is_yesterday = local.date_naive() == now.date_naive() - chrono::Days::new(1);

    let duration = now.signed_duration_since(local);

    if duration.num_minutes() < 1 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 && is_today {
        format!("{}h ago", duration.num_hours())
    } else if is_today {
        format!("Today at {}", local.format("%I:%M%p").to_string().to_lowercase())
    } else if is_yesterday {
        format!("Yesterday at {}", local.format("%I:%M%p").to_string().to_lowercase())
    } else {
        format!("{}", local.format("%b %d at %I:%M%p").to_string().to_lowercase())
    }
}

/// Generate smart session title
#[allow(dead_code)]
fn generate_session_title(session: &Session) -> String {
    if !session.title.is_empty() {
        session.title.clone()
    } else if !session.slug.is_empty() {
        session.slug.clone()
    } else {
        let datetime = DateTime::<Utc>::from_timestamp(session.time.created / 1000, 0)
            .unwrap_or_else(|| Utc::now());
        let local: DateTime<Local> = datetime.into();
        format!("New session - {}", local.format("%b %d, %I:%M %p"))
    }
}

app_main!(App);

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;

    AppBg = <View> {
        show_bg: true
        draw_bg: {
            color: #14161a
            uniform color_2: #0f1114
            fn pixel(self) -> vec4 {
                return mix(self.color, self.color_2, self.pos.y);
            }
        }
    }

    UserBubble = <View> {
        width: Fit, height: Fit
        flow: Down,
        padding: 12,
        show_bg: true
        draw_bg: {
            color: #2a4a6a
            uniform border_color: #3a5f84
            uniform border_radius: 8.0
            uniform border_size: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }
    }

    AssistantBubble = <View> {
        width: Fit, height: Fit
        flow: Down,
        padding: 12,
        show_bg: true
        draw_bg: {
            color: #2b2f35
            uniform border_color: #3a414a
            uniform border_radius: 8.0
            uniform border_size: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }
    }

    ProjectsPanel = {{ProjectsPanel}} {
        width: Fill, height: Fill
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}

            ProjectHeader = <View> {
                width: Fill, height: Fit
                flow: Down,
                padding: { top: 6, bottom: 6 }
                project_name = <Label> {
                    draw_text: { color: #e6e9ee, text_style: { font_size: 12 } }
                }
                project_path = <Label> {
                    draw_text: { color: #aab3bd, text_style: { font_size: 10 } }
                }
            }

            NewSessionRow = <View> {
                width: Fill, height: Fit
                padding: { top: 6, bottom: 8 }
                new_session_button = <Button> {
                    width: Fill, height: 36
                    text: "+  New session"
                    draw_bg: {
                        color: #232830
                        color_hover: #2a313b
                        border_radius: 8.0
                        border_size: 1.0
                        border_color_1: #313842
                        border_color_2: #2a3039
                    }
                    draw_text: { color: #e6e9ee, text_style: { font_size: 11 } }
                }
            }

            SessionRow = <View> {
                width: Fill, height: Fit
                padding: 0

                session_row_bg = <View> {
                    width: Fill, height: 52
                    flow: Right
                    show_bg: true
                    draw_bg: {
                        color: #1f2329
                        uniform border_color: #4a90e2
                        uniform border_size: 3.0

                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            // Left status bar
                            sdf.rect(0.0, 0.0, self.border_size, self.rect_size.y);
                            sdf.fill(self.border_color);
                            // Background
                            sdf.rect(self.border_size, 0.0, self.rect_size.x - self.border_size, self.rect_size.y);
                            sdf.fill(self.color);
                            return sdf.result;
                        }
                    }

                    // Main content area
                    <View> {
                        width: Fill, height: Fill
                        flow: Down
                        padding: { left: 12, right: 12, top: 8, bottom: 8 }
                        spacing: 4

                        // Line 1: Title + icons
                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 6
                            align: { y: 0.5 }

                            session_title = <Label> {
                                width: Fill
                                draw_text: {
                                    color: #e6e9ee
                                    text_style: { font_size: 11 }
                                }
                            }
                        }

                        // Line 2: Metadata
                        session_metadata = <Label> {
                            width: Fill
                            draw_text: {
                                color: #7a8591
                                text_style: { font_size: 10 }
                            }
                        }
                    }
                }
            }

            Spacer = <View> { width: Fill, height: 12 }
        }
    }

    App = {{App}} {
        ui: <Window> {
            window: { inner_size: vec2(1200, 800) }
            pass: { clear_color: #1a1a1a }

            body = <AppBg> {
                flow: Down,
                spacing: 12,
                padding: 12,

                // Status bar at top
                <HeaderBar> {
                    hamburger_button = <HamburgerButton> {}
                    <View> { width: Fill }
                    app_title = <Label> {
                        text: "Openpad"
                        draw_text: { color: #e6e9ee, text_style: { font_size: 12 } }
                    }
                    <View> { width: Fill }
                    status_row = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 8
                        align: { y: 0.5 }
                        status_dot = <StatusDot> {}
                        status_label = <Label> {
                            text: "Connecting..."
                            draw_text: { color: #aab3bd, text_style: { font_size: 11 } }
                        }
                    }
                }

                <View> {
                    width: Fill, height: Fill
                    flow: Right,
                    spacing: 12,

                    side_panel = <SidePanel> {
                        projects_panel = <ProjectsPanel> {}
                    }

                    <View> {
                        width: Fill, height: Fill
                        flow: Down,
                        spacing: 12,

                        // Messages area (scrollable)
                        <ScrollYView> {
                            width: Fill, height: Fill

                            message_list = <PortalList> {
                                UserMsg = <View> {
                                    width: Fill, height: Fit
                                    flow: Right,
                                    padding: 8,
                                    align: { x: 1.0 }

                                    <UserBubble> {
                                        width: Fit, height: Fit
                                        margin: { left: 100 }
                                        flow: Down,

                                        msg_text = <Label> {
                                            draw_text: { color: #eef3f7, text_style: { font_size: 11 } }
                                        }
                                    }
                                }

                                AssistantMsg = <View> {
                                    width: Fill, height: Fit
                                    flow: Down,
                                    padding: 8,

                                    <AssistantBubble> {
                                        width: Fit, height: Fit
                                        margin: { right: 100 }
                                        flow: Down,

                                        msg_text = <Label> {
                                            draw_text: { color: #e6e9ee, text_style: { font_size: 11 } }
                                        }
                                    }
                                }
                            }
                        }

                        // Input area (fixed at bottom)
                        <InputBar> {
                            input_box = <InputField> {}
                            send_button = <SendButton> {}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum AppAction {
    None,
    Connected,
    ConnectionFailed(String),
    HealthUpdated(HealthResponse),
    ProjectsLoaded(Vec<Project>),
    CurrentProjectLoaded(Project),
    SessionsLoaded(Vec<Session>),
    SessionCreated(Session),
    OpenCodeEvent(OcEvent),
    SendMessageFailed(String),
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ProjectsPanelAction {
    None,
    SelectSession(String),
    CreateSession(Option<String>),
}

#[derive(Clone, Debug)]
enum PanelItemKind {
    ProjectHeader {
        project_id: Option<String>,
        name: String,
        path: String,
    },
    NewSession {
        project_id: Option<String>,
    },
    SessionRow {
        session_id: String,
        title: String,
        timestamp: String,
        is_archived: bool,
        is_shared: bool,
        is_forked: bool,
        file_changes: Option<(i64, i64, i64)>, // (additions, deletions, files)
        message_count: usize,
    },
    Spacer,
}

#[derive(Live, LiveHook, Widget)]
pub struct ProjectsPanel {
    #[deref]
    view: View,
    #[rust]
    projects: Vec<Project>,
    #[rust]
    sessions: Vec<Session>,
    #[rust]
    selected_session_id: Option<String>,
    #[rust]
    items: Vec<PanelItemKind>,
    #[rust]
    visible_items: Vec<(PanelItemKind, WidgetRef)>,
    #[rust]
    dirty: bool,
    #[rust]
    message_counts: HashMap<String, usize>,
}

impl ProjectsPanel {
    fn rebuild_items(&mut self, message_counts: &HashMap<String, usize>) {
        let mut grouped: HashMap<Option<String>, Vec<Session>> = HashMap::new();
        for session in &self.sessions {
            grouped
                .entry(Some(session.project_id.clone()))
                .or_default()
                .push(session.clone());
        }

        let mut items = Vec::new();
        for project in &self.projects {
            let project_id = Some(project.id.clone());
            let name = project
                .name
                .clone()
                .unwrap_or_else(|| project.id.clone());
            let path = project.path.clone().unwrap_or_default();

            items.push(PanelItemKind::ProjectHeader {
                project_id: project_id.clone(),
                name,
                path,
            });
            items.push(PanelItemKind::NewSession {
                project_id: project_id.clone(),
            });

            if let Some(mut sessions) = grouped.get(&project_id).cloned() {
                // Sort by updated time, most recent first
                sessions.sort_by(|a, b| b.time.updated.cmp(&a.time.updated));

                for session in sessions {
                    let title = generate_session_title(&session);
                    let timestamp = format_relative_time(session.time.updated);
                    let is_archived = session.time.archived.is_some();
                    let is_shared = session.share.is_some();
                    let is_forked = session.parent_id.is_some();
                    let file_changes = session.summary.as_ref().map(|s| {
                        (s.additions, s.deletions, s.files)
                    });
                    let message_count = *message_counts.get(&session.id).unwrap_or(&0);

                    items.push(PanelItemKind::SessionRow {
                        session_id: session.id.clone(),
                        title,
                        timestamp,
                        is_archived,
                        is_shared,
                        is_forked,
                        file_changes,
                        message_count,
                    });
                }
            }
            items.push(PanelItemKind::Spacer);
        }

        if let Some(mut sessions) = grouped.get(&None).cloned() {
            if !sessions.is_empty() {
                sessions.sort_by(|a, b| b.time.updated.cmp(&a.time.updated));

                items.push(PanelItemKind::ProjectHeader {
                    project_id: None,
                    name: "Other".to_string(),
                    path: "".to_string(),
                });
                items.push(PanelItemKind::NewSession { project_id: None });
                for session in sessions {
                    let title = generate_session_title(&session);
                    let timestamp = format_relative_time(session.time.updated);
                    let is_archived = session.time.archived.is_some();
                    let is_shared = session.share.is_some();
                    let is_forked = session.parent_id.is_some();
                    let file_changes = session.summary.as_ref().map(|s| {
                        (s.additions, s.deletions, s.files)
                    });
                    let message_count = *message_counts.get(&session.id).unwrap_or(&0);

                    items.push(PanelItemKind::SessionRow {
                        session_id: session.id.clone(),
                        title,
                        timestamp,
                        is_archived,
                        is_shared,
                        is_forked,
                        file_changes,
                        message_count,
                    });
                }
            }
        }

        self.items = items;
        self.dirty = false;
    }
}

impl Widget for ProjectsPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        for (item, widget) in &self.visible_items {
            match item {
                PanelItemKind::NewSession { project_id } => {
                    if widget.button(id!(new_session_button)).clicked(&actions) {
                        cx.action(ProjectsPanelAction::CreateSession(project_id.clone()));
                    }
                }
                PanelItemKind::SessionRow { session_id, .. } => {
                    if widget.button(id!(session_button)).clicked(&actions) {
                        cx.action(ProjectsPanelAction::SelectSession(session_id.clone()));
                    }
                }
                _ => {}
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.dirty {
            let message_counts = self.message_counts.clone();
            self.rebuild_items(&message_counts);
        }

        self.visible_items.clear();

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if self.items.is_empty() {
                    list.set_item_range(cx, 0, 0);
                    continue;
                } else {
                    // PortalList range end is inclusive
                    list.set_item_range(cx, 0, self.items.len().saturating_sub(1));
                }
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id >= self.items.len() {
                        continue;
                    }
                    let panel_item = self.items[item_id].clone();
                    let template = match panel_item {
                        PanelItemKind::ProjectHeader { .. } => live_id!(ProjectHeader),
                        PanelItemKind::NewSession { .. } => live_id!(NewSessionRow),
                        PanelItemKind::SessionRow { .. } => live_id!(SessionRow),
                        PanelItemKind::Spacer => live_id!(Spacer),
                    };
                    let item_widget = list.item(cx, item_id, template);

                    match &panel_item {
                        PanelItemKind::ProjectHeader { name, path, .. } => {
                            item_widget.label(id!(project_name)).set_text(cx, name);
                            item_widget.label(id!(project_path)).set_text(cx, path);
                        }
                        PanelItemKind::SessionRow {
                            session_id,
                            title,
                            timestamp,
                            is_archived,
                            file_changes,
                            message_count,
                            ..
                        } => {
                            // Set title
                            item_widget.label(id!(session_title)).set_text(cx, title);

                            // Build metadata string
                            let mut metadata_parts = vec![timestamp.clone()];

                            if let Some((additions, deletions, files)) = file_changes {
                                metadata_parts.push(format!("+{} -{}", additions, deletions));
                                metadata_parts.push(format!("{} files", files));
                            }

                            if *message_count > 0 {
                                metadata_parts.push(format!("{} messages", message_count));
                            }

                            let metadata = metadata_parts.join(" • ");
                            item_widget.label(id!(session_metadata)).set_text(cx, &metadata);

                            // Set background color based on selection
                            let selected = self
                                .selected_session_id
                                .as_ref()
                                .map(|id| id == session_id)
                                .unwrap_or(false);
                            let bg_color = if selected {
                                vec4(0.14, 0.16, 0.20, 1.0) // #242a32
                            } else {
                                vec4(0.12, 0.14, 0.16, 1.0) // #1f2329
                            };

                            // Determine status color
                            let status_color = if *is_archived {
                                vec4(0.42, 0.48, 0.55, 1.0) // gray #6b7b8c
                            } else {
                                // Check if updated in last 24h (we'll refine this)
                                vec4(0.29, 0.56, 0.87, 1.0) // blue #4a90e2
                            };

                            item_widget.view(id!(session_row_bg)).apply_over(cx, live! {
                                draw_bg: {
                                    color: (bg_color),
                                    border_color: (status_color)
                                }
                            });
                        }
                        _ => {}
                    }

                    item_widget.draw_all(cx, scope);
                    self.visible_items.push((panel_item, item_widget));
                }
            }
        }
        DrawStep::done()
    }
}

impl ProjectsPanelRef {
    pub fn set_data(
        &self,
        cx: &mut Cx,
        projects: Vec<Project>,
        sessions: Vec<Session>,
        selected_session_id: Option<String>,
        message_counts: &HashMap<String, usize>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.projects = projects;
            inner.sessions = sessions;
            inner.selected_session_id = selected_session_id;
            inner.message_counts = message_counts.clone();
            inner.dirty = true;
            inner.redraw(cx);
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    messages: Vec<Message>,
    #[rust]
    projects: Vec<Project>,
    #[rust]
    sessions: Vec<Session>,
    #[rust]
    current_project: Option<Project>,
    #[rust]
    selected_session_id: Option<String>,
    #[rust]
    current_session_id: Option<String>,
    #[rust]
    connected: bool,
    #[rust]
    health_ok: Option<bool>,
    #[rust]
    error_message: Option<String>,
    #[rust]
    sidebar_open: bool,
    #[rust]
    client: Option<Arc<OpenCodeClient>>,
    #[rust]
    _runtime: Option<tokio::runtime::Runtime>,
    #[rust]
    message_counts: HashMap<String, usize>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        openpad_widgets::live_design(cx);
    }
}

impl App {
    fn connect_to_opencode(&mut self, _cx: &mut Cx) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = Arc::new(OpenCodeClient::new("http://localhost:4096"));
        let client_clone = client.clone();
        let client_health = client.clone();
        let client_load = client.clone();

        runtime.spawn(async move {
            // Try to connect by listing sessions
            match client_clone.list_sessions().await {
                Ok(sessions) => {
                    Cx::post_action(AppAction::Connected);
                    Cx::post_action(AppAction::SessionsLoaded(sessions));

                    // Subscribe to SSE
                    if let Ok(mut rx) = client_clone.subscribe().await {
                        while let Ok(event) = rx.recv().await {
                            Cx::post_action(AppAction::OpenCodeEvent(event));
                        }
                    }
                }
                Err(e) => {
                    Cx::post_action(AppAction::ConnectionFailed(e.to_string()));
                }
            }
        });

        runtime.spawn(async move {
            use tokio::time::{sleep, Duration};
            loop {
                match client_health.health().await {
                    Ok(health) => Cx::post_action(AppAction::HealthUpdated(health)),
                    Err(_) => Cx::post_action(AppAction::HealthUpdated(HealthResponse {
                        healthy: false,
                        version: "unknown".to_string(),
                    })),
                }
                sleep(Duration::from_secs(10)).await;
            }
        });

        runtime.spawn(async move {
            if let Ok(projects) = client_load.list_projects().await {
                Cx::post_action(AppAction::ProjectsLoaded(projects));
            }
            if let Ok(current) = client_load.current_project().await {
                Cx::post_action(AppAction::CurrentProjectLoaded(current));
            }
        });

        self.client = Some(client);
        self._runtime = Some(runtime);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        for action in actions {
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                match app_action {
                    AppAction::Connected => {
                        self.connected = true;
                        self.error_message = None;
                        self.ui.label(id!(status_label)).set_text(cx, "Connected");
                        self.ui.view(id!(status_dot)).apply_over(
                            cx,
                            live! {
                                draw_bg: { color: (vec4(0.231, 0.824, 0.435, 1.0)) }
                            },
                        );
                        cx.redraw_all();
                    }
                    AppAction::ConnectionFailed(err) => {
                        self.error_message = Some(err.clone());
                        self.ui
                            .label(id!(status_label))
                            .set_text(cx, &format!("Error: {}", err));
                        self.ui.view(id!(status_dot)).apply_over(
                            cx,
                            live! {
                                draw_bg: { color: (vec4(0.886, 0.333, 0.353, 1.0)) }
                            },
                        );
                        cx.redraw_all();
                    }
                    AppAction::HealthUpdated(health) => {
                        self.health_ok = Some(health.healthy);
                        if health.healthy || self.connected {
                            self.ui.label(id!(status_label)).set_text(cx, "Connected");
                            self.ui.view(id!(status_dot)).apply_over(
                                cx,
                                live! {
                                    draw_bg: { color: (vec4(0.231, 0.824, 0.435, 1.0)) }
                                },
                            );
                        } else {
                            self.ui.label(id!(status_label)).set_text(cx, "Disconnected");
                            self.ui.view(id!(status_dot)).apply_over(
                                cx,
                                live! {
                                    draw_bg: { color: (vec4(0.55, 0.57, 0.60, 1.0)) }
                                },
                            );
                        }
                        cx.redraw_all();
                    }
                    AppAction::ProjectsLoaded(projects) => {
                        self.projects = projects.clone();
                        self.ui.projects_panel(id!(projects_panel)).set_data(
                            cx,
                            self.projects.clone(),
                            self.sessions.clone(),
                            self.selected_session_id.clone(),
                            &self.message_counts,
                        );
                    }
                    AppAction::CurrentProjectLoaded(project) => {
                        self.current_project = Some(project.clone());
                    }
                    AppAction::SessionsLoaded(sessions) => {
                        self.sessions = sessions.clone();
                        self.ui.projects_panel(id!(projects_panel)).set_data(
                            cx,
                            self.projects.clone(),
                            self.sessions.clone(),
                            self.selected_session_id.clone(),
                            &self.message_counts,
                        );
                    }
                    AppAction::SessionCreated(session) => {
                        self.current_session_id = Some(session.id.clone());
                        cx.redraw_all();
                    }
                    AppAction::OpenCodeEvent(oc_event) => {
                        self.handle_opencode_event(cx, oc_event);
                    }
                    AppAction::SendMessageFailed(err) => {
                        self.error_message = Some(err.clone());
                        cx.redraw_all();
                    }
                    _ => {}
                }
            }
            if let Some(panel_action) = action.downcast_ref::<ProjectsPanelAction>() {
                match panel_action {
                    ProjectsPanelAction::SelectSession(session_id) => {
                        self.selected_session_id = Some(session_id.clone());
                        self.ui.projects_panel(id!(projects_panel)).set_data(
                            cx,
                            self.projects.clone(),
                            self.sessions.clone(),
                            self.selected_session_id.clone(),
                            &self.message_counts,
                        );
                    }
                    ProjectsPanelAction::CreateSession(_project_id) => {
                        self.create_session(cx);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_opencode_event(&mut self, cx: &mut Cx, event: &OcEvent) {
        match event {
            OcEvent::SessionCreated(session) => {
                if self.current_session_id.is_none() {
                    self.current_session_id = Some(session.id.clone());
                }
                self.sessions.push(session.clone());
                self.ui.projects_panel(id!(projects_panel)).set_data(
                    cx,
                    self.projects.clone(),
                    self.sessions.clone(),
                    self.selected_session_id.clone(),
                    &self.message_counts,
                );
            }
            OcEvent::MessageUpdated(message) => {
                // Find existing message or add new
                if let Some(existing) = self.messages.iter_mut().find(|m| m.id() == message.id()) {
                    *existing = message.clone();
                } else {
                    self.messages.push(message.clone());
                }

                cx.redraw_all();
            }
            OcEvent::PartUpdated { .. } => {
                // Current protocol does not include message id; ignore for now.
            }
            _ => {}
        }
    }

    fn send_message(&mut self, _cx: &mut Cx, text: String) {
        let Some(client) = self.client.clone() else {
            self.error_message = Some("Not connected".to_string());
            return;
        };

        let session_id = self.current_session_id.clone();

        self._runtime.as_ref().unwrap().spawn(async move {
            // Create session if needed
            let sid = if let Some(id) = session_id {
                id
            } else {
                match client.create_session().await {
                    Ok(session) => {
                        Cx::post_action(AppAction::SessionCreated(session.clone()));
                        session.id
                    }
                    Err(e) => {
                        Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
                        return;
                    }
                }
            };

            // Send prompt
            if let Err(e) = client.send_prompt(&sid, &text).await {
                Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
            }
        });
    }

    fn create_session(&mut self, _cx: &mut Cx) {
        let Some(client) = self.client.clone() else {
            self.error_message = Some("Not connected".to_string());
            return;
        };

        self._runtime.as_ref().unwrap().spawn(async move {
            match client.create_session().await {
                Ok(session) => {
                    Cx::post_action(AppAction::SessionCreated(session));
                    if let Ok(sessions) = client.list_sessions().await {
                        Cx::post_action(AppAction::SessionsLoaded(sessions));
                    }
                }
                Err(e) => {
                    Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
                }
            }
        });
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.connect_to_opencode(cx);
            }
            Event::Actions(actions) => {
                self.handle_actions(cx, actions);
            }
            _ => {}
        }

        // Handle UI events and capture actions
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });

        // Check for text input return
        if let Some((text, _modifiers)) = self.ui.text_input(id!(input_box)).returned(&actions) {
            if !text.is_empty() {
                self.send_message(cx, text.clone());
                self.ui.text_input(id!(input_box)).set_text(cx, "");
            }
        }

        if self.ui.button(id!(hamburger_button)).clicked(&actions) {
            self.sidebar_open = !self.sidebar_open;
            self.ui
                .side_panel(id!(side_panel))
                .set_open(cx, self.sidebar_open);
            if self.sidebar_open {
                self.ui
                    .view(id!(hamburger_button))
                    .animator_play(cx, id!(open.on));
            } else {
                self.ui
                    .view(id!(hamburger_button))
                    .animator_play(cx, id!(open.off));
            }
        }

        if self.ui.button(id!(send_button)).clicked(&actions) {
            let text = self.ui.text_input(id!(input_box)).text();
            if !text.is_empty() {
                self.send_message(cx, text.clone());
                self.ui.text_input(id!(input_box)).set_text(cx, "");
            }
        }
    }
}
