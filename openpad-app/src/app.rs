use crate::async_runtime;
use crate::components::message_list::MessageListWidgetRefExt;
use crate::components::permission_dialog::PermissionDialogWidgetRefExt;
use crate::components::simple_dialog::SimpleDialogWidgetRefExt;
use crate::components::terminal::{TerminalAction, TerminalWidgetRefExt};
use crate::constants::OPENCODE_SERVER_URL;
use crate::state::{self, AppAction, AppState, ProjectsPanelAction};
use makepad_widgets::*;
use openpad_protocol::OpenCodeClient;
use openpad_widgets::SidePanelWidgetRefExt;
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
    use crate::components::permission_dialog::PermissionDialog;
    use crate::components::message_list::MessageList;
    use crate::components::simple_dialog::SimpleDialog;
    use crate::components::terminal::Terminal;

    App = {{App}} {
        ui: <Window> {
            window: { inner_size: vec2(1200, 800) }
            pass: { clear_color: #1a1a1a }

            body = <View> {
                width: Fill, height: Fill
                flow: Overlay

                <AppBg> {
                width: Fill, height: Fill
                flow: Down,
                spacing: 0,
                padding: 0,

                <View> {
                    width: Fill, height: Fill
                    flow: Right,
                    spacing: 0,

                    side_panel = <SidePanel> {
                        width: 260.0, height: Fill
                        open_size: 260.0

                        <HeaderBar> {
                            height: 36
                            width: 260
                            padding: { left: 80, right: 10 }
                            draw_bg: {
                                color: #1e1e1e
                                border_color: #333
                                border_radius: 0.0
                                border_size: 1.0
                            }
                        }

                        projects_panel = <ProjectsPanel> {}
                    }



                    <View> {
                        width: Fill, height: Fill
                        flow: Down,
                        spacing: 0,

                        // Main Header
                        main_header = <HeaderBar> {
                            height: 36
                            padding: { left: 16, right: 10 }
                            draw_bg: {
                                color: #1e1e1e
                                border_color: #333
                                border_radius: 0.0
                            }

                            // This spacer expands when the sidebar closes to keep traffic lights clear
                            traffic_light_spacer = <SidePanel> {
                                width: 0.0, height: Fill
                                open_size: 80.0
                                close_size: 0.0
                                draw_bg: { color: #0000, border_size: 0.0 } // Transparent!
                            }

                            hamburger_button = <HamburgerButton> {
                                width: 32, height: 32
                            }
                            <View> { width: 4 }
                            app_title = <Label> {
                                text: "Openpad"
                                draw_text: { color: #888, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                            }
                            <View> { width: Fill }
                            status_row = <View> {
                                width: Fit, height: Fit
                                flow: Right
                                spacing: 8
                                align: { y: 0.5 }
                                status_dot = <StatusDot> {}
                                status_label = <Label> {
                                    text: "Connected"
                                    draw_text: { color: #555, text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }
                            }
                        }

                        // Slim Breadcrumbs Bar
                        session_info = <View> {
                            width: Fill, height: 32
                            padding: { left: 16, right: 16 }
                            flow: Right,
                            spacing: 8,
                            align: { y: 0.5 }
                            show_bg: true
                            draw_bg: { color: #1e1e1e }

                            project_row = <View> {
                                width: Fit, height: Fit
                                flow: Right, spacing: 4, align: {y: 0.5}
                                project_badge = <View> {
                                    width: Fit, height: Fit
                                    project_badge_label = <Label> {
                                        text: "No project"
                                        draw_text: { color: #888, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                                    }
                                }
                            }

                            <Label> { text: "/", draw_text: { color: #444, text_style: <THEME_FONT_REGULAR> { font_size: 10 } } }

                            session_row = <View> {
                                width: Fit, height: Fit
                                session_title = <Label> {
                                    text: "New Session"
                                    draw_text: { color: #aaa, text_style: <THEME_FONT_BOLD> { font_size: 10 } }
                                }
                            }

                            project_path_wrap = <View> {
                                visible: false
                                project_path_label = <Label> { text: "" }
                            }

                            <View> { width: Fill }

                            revert_indicator = <View> {
                                visible: false
                                revert_indicator_label = <Label> {
                                    text: "⟲ Reverted"
                                    draw_text: { color: #f59e0b, text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }
                            }
                            unrevert_wrap = <View> {
                                visible: false
                                unrevert_button = <Button> {
                                    width: Fit, height: 20
                                    text: "Unrevert"
                                    draw_text: { color: #3b82f6, text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }
                            }
                        }
                        <View> { width: Fill, height: 1, show_bg: true, draw_bg: { color: #2a2a2a } }

                        // Chat area - Unified
                        <View> {
                            width: Fill, height: Fill
                            flow: Down
                            spacing: 0
                            show_bg: true
                            draw_bg: { color: #1e1e1e }

                            <View> {
                                width: Fill, height: Fill
                                message_list = <MessageList> { width: Fill, height: Fill }
                            }

                            permission_dialog = <PermissionDialog> { width: Fill }

                            input_row = <View> {
                                width: Fill, height: Fit
                                padding: { left: 32, right: 32, top: 12, bottom: 20 }
                                <InputBar> {
                                    width: Fill
                                    input_box = <InputField> {}
                                    <InputBarToolbar> {
                                        agent_dropdown = <InputBarDropDown> {
                                            labels: ["Agent"]
                                        }
                                        model_dropdown = <InputBarDropDown> {
                                            width: 150
                                            labels: ["Model"]
                                        }
                                        <View> { width: Fill }
                                        send_button = <SendButton> {
                                            margin: { left: 0 }
                                            width: 32, height: 32
                                        }
                                    }
                                }
                            }
                        }

                        // Integrated Terminal
                        <View> {
                            width: Fill, height: 250
                            flow: Down
                            show_bg: true
                            draw_bg: { color: #1e1e1e }

                            // Separator line
                            <View> { width: Fill, height: 1, show_bg: true, draw_bg: { color: #333 } }

                            terminal_panel = <Terminal> {
                                width: Fill
                                height: Fill
                            }
                        }
                    }
                }
                }

                // Simple dialog for confirmations and inputs (shown as overlay)
                simple_dialog = <SimpleDialog> {}
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    state: AppState,
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
        crate::components::message_list::live_design(cx);
        crate::components::permission_dialog::live_design(cx);
        crate::components::simple_dialog::live_design(cx);
        crate::components::terminal::live_design(cx);
    }
}

impl App {
    fn normalize_project_directory(worktree: &str) -> String {
        if worktree == "." {
            if let Ok(current_dir) = std::env::current_dir() {
                return current_dir.to_string_lossy().to_string();
            }
        }

        match std::fs::canonicalize(worktree) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_) => worktree.to_string(),
        }
    }

    fn connect_to_opencode(&mut self, _cx: &mut Cx) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = Arc::new(OpenCodeClient::new(OPENCODE_SERVER_URL));

        // Spawn background tasks
        async_runtime::spawn_sse_subscriber(&runtime, client.clone());
        async_runtime::spawn_health_checker(&runtime, client.clone());
        async_runtime::spawn_project_loader(&runtime, client.clone());

        self.client = Some(client);
        self._runtime = Some(runtime);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        for action in actions {
            // Handle TerminalAction from background thread
            if let Some(terminal_action) = action.downcast_ref::<TerminalAction>() {
                self.ui
                    .terminal(id!(terminal_panel))
                    .handle_action(cx, terminal_action);
            }

            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                match app_action {
                    AppAction::OpenCodeEvent(oc_event) => {
                        state::handle_opencode_event(&mut self.state, &self.ui, cx, oc_event);
                    }
                    AppAction::PermissionResponded { request_id, reply } => {
                        state::handle_permission_responded(
                            &mut self.state,
                            &self.ui,
                            cx,
                            request_id,
                        );
                        self.respond_to_permission(cx, request_id.clone(), reply.clone());
                        self.ui.permission_dialog(id!(permission_dialog)).hide(cx);
                    }
                    AppAction::RevertToMessage {
                        session_id,
                        message_id,
                    } => {
                        self.revert_to_message(cx, session_id.clone(), message_id.clone());
                    }
                    AppAction::UnrevertSession(session_id) => {
                        self.unrevert_session(cx, session_id.clone());
                    }
                    AppAction::DialogConfirmed { dialog_type, value } => {
                        self.handle_dialog_confirmed(cx, dialog_type.clone(), value.clone());
                    }
                    AppAction::Connected => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.load_providers_and_agents();
                    }
                    _ => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                    }
                }
            }
        }
    }

    fn load_providers_and_agents(&mut self) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };
        async_runtime::spawn_providers_loader(runtime, client.clone());
        async_runtime::spawn_agents_loader(runtime, client);
    }

    fn load_pending_permissions(&mut self) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_pending_permissions_loader(runtime, client);
    }

    fn load_messages(&mut self, session_id: String) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the session to get its directory
        let directory = self
            .state
            .sessions
            .iter()
            .find(|s| s.id == session_id)
            .map(|s| s.directory.clone());

        async_runtime::spawn_message_loader(runtime, client, session_id, directory);
    }

    fn send_message(&mut self, _cx: &mut Cx, text: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        let session_id = self.state.current_session_id.clone();
        let directory = session_id
            .as_ref()
            .and_then(|sid| {
                self.state
                    .sessions
                    .iter()
                    .find(|session| &session.id == sid)
                    .map(|session| session.directory.clone())
            })
            .or_else(|| {
                self.state
                    .current_project
                    .as_ref()
                    .map(|project| Self::normalize_project_directory(&project.worktree))
            });
        let model_spec = self.state.selected_model_spec();
        async_runtime::spawn_message_sender(
            runtime, client, session_id, text, model_spec, directory,
        );
    }

    fn create_session(&mut self, _cx: &mut Cx, project_id: Option<String>) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Look up project directory if project_id is provided
        let project_directory = project_id.as_ref().and_then(|pid| {
            self.state
                .projects
                .iter()
                .find(|p| &p.id == pid)
                .map(|p| {
                    let normalized = Self::normalize_project_directory(&p.worktree);
                    log!(
                        "Create session: project_id={:?} worktree={} directory={}",
                        pid,
                        p.worktree,
                        normalized
                    );
                    normalized
                })
        });

        async_runtime::spawn_session_creator(runtime, client, project_directory);
    }

    fn respond_to_permission(
        &mut self,
        _cx: &mut Cx,
        request_id: String,
        reply: openpad_protocol::PermissionReply,
    ) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_permission_reply(runtime, client, request_id, reply);
    }

    fn delete_session(&mut self, cx: &mut Cx, session_id: String) {
        // Show confirmation dialog
        self.ui.simple_dialog(id!(simple_dialog)).show_confirm(
            cx,
            "Delete Session",
            "Are you sure you want to delete this session? This action cannot be undone.",
            format!("delete_session:{}", session_id),
        );
    }

    fn rename_session(&mut self, cx: &mut Cx, session_id: String) {
        // Get current title
        let current_title = self
            .state
            .sessions
            .iter()
            .find(|s| s.id == session_id)
            .map(|s| async_runtime::get_session_title(s))
            .unwrap_or_else(|| "Session".to_string());

        // Show input dialog
        self.ui.simple_dialog(id!(simple_dialog)).show_input(
            cx,
            "Rename Session",
            "Enter a new name for this session:",
            &current_title,
            format!("rename_session:{}", session_id),
        );
    }

    fn abort_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_aborter(runtime, client, session_id);
    }

    fn branch_session(&mut self, _cx: &mut Cx, parent_session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the parent session to get its directory for the new branched session
        let directory = self
            .state
            .sessions
            .iter()
            .find(|s| s.id == parent_session_id)
            .map(|s| s.directory.clone());

        async_runtime::spawn_session_brancher(runtime, client, parent_session_id, directory);
    }

    fn revert_to_message(&mut self, _cx: &mut Cx, session_id: String, message_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the session to get its directory
        let directory = self
            .state
            .sessions
            .iter()
            .find(|s| s.id == session_id)
            .map(|s| s.directory.clone());

        async_runtime::spawn_message_reverter(runtime, client, session_id, message_id, directory);
    }

    fn unrevert_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the session to get its directory
        let directory = self
            .state
            .sessions
            .iter()
            .find(|s| s.id == session_id)
            .map(|s| s.directory.clone());

        async_runtime::spawn_session_unreverter(runtime, client, session_id, directory);
    }

    fn handle_dialog_confirmed(&mut self, _cx: &mut Cx, dialog_type: String, value: String) {
        // Parse the dialog_type which is in format "action:data"
        let Some((action, data)) = dialog_type.split_once(':') else {
            return;
        };

        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        match action {
            "delete_session" => {
                async_runtime::spawn_session_deleter(runtime, client, data.to_string());
            }
            "rename_session" => {
                if !value.is_empty() {
                    async_runtime::spawn_session_updater(runtime, client, data.to_string(), value);
                }
            }
            _ => {}
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.connect_to_opencode(cx);
                // Initialize terminal
                self.ui.terminal(id!(terminal_panel)).init_pty(cx);

                // Initialize sidebar to open
                self.sidebar_open = true;
                self.ui.side_panel(id!(side_panel)).set_open(cx, true);
                self.ui
                    .side_panel(id!(traffic_light_spacer))
                    .set_open(cx, false);
                self.ui
                    .view(id!(hamburger_button))
                    .animator_play(cx, id!(open.on));
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

        // Process widget actions (e.g. ProjectsPanelAction from sidebar clicks)
        for action in &actions {
            if let Some(panel_action) = action.downcast_ref::<ProjectsPanelAction>() {
                match panel_action {
                    ProjectsPanelAction::SelectSession(session_id) => {
                        self.state.selected_session_id = Some(session_id.clone());
                        self.state.current_session_id = Some(session_id.clone());
                        self.ui.permission_dialog(id!(permission_dialog)).hide(cx);
                        self.state.messages_data.clear();
                        self.ui
                            .message_list(id!(message_list))
                            .set_messages(cx, &self.state.messages_data);
                        self.state.update_projects_panel(&self.ui, cx);
                        self.state.update_session_title_ui(&self.ui, cx);
                        self.state.update_project_context_ui(&self.ui, cx);
                        self.load_messages(session_id.clone());
                        self.load_pending_permissions();
                    }
                    ProjectsPanelAction::CreateSession(project_id) => {
                        log!(
                            "UI action: create session button clicked (project_id={:?})",
                            project_id
                        );
                        self.create_session(cx, project_id.clone());
                    }
                    ProjectsPanelAction::DeleteSession(session_id) => {
                        self.delete_session(cx, session_id.clone());
                    }
                    ProjectsPanelAction::RenameSession(session_id) => {
                        self.rename_session(cx, session_id.clone());
                    }
                    ProjectsPanelAction::AbortSession(session_id) => {
                        self.abort_session(cx, session_id.clone());
                    }
                    ProjectsPanelAction::BranchSession(session_id) => {
                        self.branch_session(cx, session_id.clone());
                    }
                    _ => {}
                }
            }

            // Handle MessageListAction
            if let Some(msg_action) =
                action.downcast_ref::<crate::state::actions::MessageListAction>()
            {
                use crate::state::actions::MessageListAction;
                match msg_action {
                    MessageListAction::RevertToMessage(message_id) => {
                        if let Some(session_id) = &self.state.current_session_id {
                            self.revert_to_message(cx, session_id.clone(), message_id.clone());
                        }
                    }
                    _ => {}
                }
            }

            // Handle TerminalAction
            if let Some(terminal_action) = action.downcast_ref::<TerminalAction>() {
                self.ui
                    .terminal(id!(terminal_panel))
                    .handle_action(cx, terminal_action);
            }

            // Handle AppAction from captured UI actions (e.g. DialogConfirmed, PermissionResponded)
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                match app_action {
                    AppAction::DialogConfirmed { dialog_type, value } => {
                        self.handle_dialog_confirmed(cx, dialog_type.clone(), value.clone());
                    }
                    AppAction::PermissionResponded { request_id, reply } => {
                        state::handle_permission_responded(
                            &mut self.state,
                            &self.ui,
                            cx,
                            request_id,
                        );
                        self.respond_to_permission(cx, request_id.clone(), reply.clone());
                        self.ui.permission_dialog(id!(permission_dialog)).hide(cx);
                    }
                    _ => {}
                }
            }
        }

        // Check for text input return
        if let Some((text, _modifiers)) = self.ui.text_input(id!(input_box)).returned(&actions) {
            if !text.is_empty() {
                self.send_message(cx, text.clone());
                self.ui.text_input(id!(input_box)).set_text(cx, "");
            }
        }

        // Handle unrevert button
        if self.ui.button(id!(unrevert_button)).clicked(&actions) {
            if let Some(session_id) = &self.state.current_session_id {
                self.unrevert_session(cx, session_id.clone());
            }
        }

        if self.ui.button(id!(hamburger_button)).clicked(&actions) {
            self.sidebar_open = !self.sidebar_open;

            // Toggle sidebar and synchronized spacer
            self.ui
                .side_panel(id!(side_panel))
                .set_open(cx, self.sidebar_open);
            self.ui
                .side_panel(id!(traffic_light_spacer))
                .set_open(cx, !self.sidebar_open);

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

        // Handle dropdown selections
        if let Some(idx) = self.ui.drop_down(id!(model_dropdown)).changed(&actions) {
            if let Some(entry) = self.state.model_entries.get(idx) {
                if entry.selectable {
                    self.state.selected_model_entry = idx;
                } else if self.state.model_entries.len() > self.state.selected_model_entry {
                    self.ui
                        .drop_down(id!(model_dropdown))
                        .set_selected_item(cx, self.state.selected_model_entry);
                }
            }
        }
        if let Some(idx) = self.ui.drop_down(id!(agent_dropdown)).changed(&actions) {
            self.state.selected_agent_idx = if idx > 0 { Some(idx - 1) } else { None };
        }
    }
}
