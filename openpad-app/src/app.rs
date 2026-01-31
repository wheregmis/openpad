use crate::async_runtime;
use crate::components::message_list::MessageListWidgetRefExt;
use crate::components::permission_dialog::PermissionDialogWidgetRefExt;
use crate::components::simple_dialog::SimpleDialogWidgetRefExt;
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
                spacing: 12,
                padding: 12,

                // Status bar at top
                <HeaderBar> {
                    hamburger_button = <HamburgerButton> {}
                    <View> { width: Fill }
                    app_title = <Label> {
                        text: "Openpad"
                        draw_text: { color: #e6e9ee, text_style: <THEME_FONT_REGULAR> { font_size: 12 } }
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
                            draw_text: { color: #aab3bd, text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
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
                        spacing: 8,

                        // Session context bar with active project badge
                        session_info = <RoundedView> {
                            width: Fill, height: Fit
                            padding: { left: 12, right: 12, top: 10, bottom: 12 }
                            flow: Down,
                            spacing: 6,
                            align: { y: 0.5 }
                            draw_bg: {
                                color: #232830
                                border_radius: 8.0
                            }

                            project_row = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 2

                                project_name_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 8
                                    align: { y: 0.5 }

                                    project_badge = <RoundedView> {
                                        height: Fit
                                        padding: { left: 10, right: 10, top: 3, bottom: 3 }
                                        show_bg: true
                                        draw_bg: {
                                            color: #1f262f
                                            border_radius: 999.0
                                        }
                                        animator: {
                                            hover = {
                                                default: off
                                                off = {
                                                    from: { all: Forward { duration: 0.18 } }
                                                    apply: {
                                                        draw_bg: { color: #1f262f }
                                                    }
                                                }
                                                on = {
                                                    from: { all: Forward { duration: 0.18 } }
                                                    apply: {
                                                        draw_bg: { color: #29323c }
                                                    }
                                                }
                                            }
                                        }

                                        project_badge_label = <Label> {
                                            text: "No active project"
                                            draw_text: { color: #9aa4b2, text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
                                        }
                                    }

                                    <View> { width: Fill }
                                }

                                project_path_label = <Label> {
                                    text: ""
                                    draw_text: { color: #7a8794, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                                }
                            }

                            session_row = <View> {
                                width: Fill, height: Fit
                                flow: Right
                                spacing: 8
                                align: { y: 0.5 }

                                session_title = <Label> {
                                    text: "Select a session or start a new one"
                                    draw_text: { color: #6b7b8c, text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
                                }
                                <View> { width: Fill }
                                revert_indicator = <View> {
                                    visible: false
                                    width: Fit, height: Fit

                                    revert_indicator_label = <Label> {
                                        text: "⟲ Reverted"
                                        draw_text: { color: #f59e0b, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                                    }
                                }
                                unrevert_wrap = <View> {
                                    visible: false
                                    width: Fit, height: Fit

                                    unrevert_button = <Button> {
                                        width: Fit, height: 28
                                        text: "↻ Unrevert"
                                        draw_bg: {
                                            color: #3b82f6
                                            color_hover: #1d4fed
                                            border_radius: 6.0
                                            border_size: 0.0
                                        }
                                        draw_text: { color: #ffffff, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                                    }
                                }
                            }
                        }

                        // Messages area
                        message_list = <MessageList> { width: Fill, height: Fill }

                        // Inline permission prompt (shown only when needed)
                        permission_dialog = <PermissionDialog> { width: Fill }

                        // Input area (fixed at bottom)
                        input_row = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            align: { y: 0.5 }

                            <InputBar> {
                                width: Fill
                                input_box = <InputField> {}
                                send_button = <SendButton> {}
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
    }
}

impl App {
    fn connect_to_opencode(&mut self, _cx: &mut Cx) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = Arc::new(OpenCodeClient::new("http://localhost:4096"));

        // Spawn background tasks
        async_runtime::spawn_sse_subscriber(&runtime, client.clone());
        async_runtime::spawn_health_checker(&runtime, client.clone());
        async_runtime::spawn_project_loader(&runtime, client.clone());

        self.client = Some(client);
        self._runtime = Some(runtime);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        for action in actions {
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
                    _ => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                    }
                }
            }
        }
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

        async_runtime::spawn_message_loader(runtime, client, session_id);
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
        async_runtime::spawn_message_sender(runtime, client, session_id, text);
    }

    fn create_session(&mut self, _cx: &mut Cx) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_creator(runtime, client);
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

        async_runtime::spawn_session_brancher(runtime, client, parent_session_id);
    }

    fn revert_to_message(&mut self, _cx: &mut Cx, session_id: String, message_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_message_reverter(runtime, client, session_id, message_id);
    }

    fn unrevert_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_unreverter(runtime, client, session_id);
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
                    ProjectsPanelAction::CreateSession(_project_id) => {
                        self.create_session(cx);
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
