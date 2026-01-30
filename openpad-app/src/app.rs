use makepad_widgets::*;
use openpad_widgets::SidePanelWidgetRefExt;
use crate::actions::{AppAction, ProjectsPanelAction};
use crate::components::projects_panel::ProjectsPanelWidgetRefExt;
use openpad_protocol::{
    Event as OcEvent, HealthResponse, Message, OpenCodeClient, Project, Session,
};
use std::sync::Arc;

app_main!(App);

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;
    
    // Import component DSL definitions
    use crate::components::app_bg::AppBg;
    use crate::components::user_bubble::UserBubble;
    use crate::components::assistant_bubble::AssistantBubble;
    use crate::components::projects_panel::ProjectsPanel;

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
                            scroll_bars: true

                            message_list = <PortalList> {
                                width: Fill, height: Fill
                                
                                UserMsg = <View> {
                                    width: Fill, height: Fit
                                    flow: Right,
                                    padding: 8,
                                    align: { x: 1.0 }

                                    <UserBubble> {
                                        width: Fill, height: Fit
                                        max_width: 600.0
                                        margin: { left: 80 }
                                        flow: Down,

                                        msg_text = <Label> {
                                            width: Fill, height: Fit
                                            wrap: Word
                                            draw_text: { color: #eef3f7, text_style: { font_size: 11 } }
                                        }
                                    }
                                }

                                AssistantMsg = <View> {
                                    width: Fill, height: Fit
                                    flow: Down,
                                    padding: 8,

                                    <AssistantBubble> {
                                        width: Fill, height: Fit
                                        max_width: 600.0
                                        margin: { right: 80 }
                                        flow: Down,

                                        msg_text = <Label> {
                                            width: Fill, height: Fit
                                            wrap: Word
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
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        openpad_widgets::live_design(cx);
        crate::components::app_bg::live_design(cx);
        crate::components::user_bubble::live_design(cx);
        crate::components::assistant_bubble::live_design(cx);
        crate::components::projects_panel::live_design(cx);
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
