use crate::actions::{AppAction, ProjectsPanelAction};
use crate::components::message_list::MessageListWidgetRefExt;
use crate::event_handlers::{self, AppState};
use crate::network;
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
    use crate::components::message_list::MessageList;

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
                        spacing: 8,

                        // Session context bar
                        session_info = <RoundedView> {
                            width: Fill, height: Fit
                            padding: { left: 12, right: 12, top: 8, bottom: 8 }
                            flow: Right,
                            spacing: 8,
                            align: { y: 0.5 }
                            draw_bg: {
                                color: #232830
                                border_radius: 8.0
                            }
                            session_title = <Label> {
                                text: "Select a session or start a new one"
                                draw_text: { color: #6b7b8c, text_style: { font_size: 11 } }
                            }
                        }

                        // Messages area
                        message_list = <MessageList> {}

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
    }
}

impl App {
    fn connect_to_opencode(&mut self, _cx: &mut Cx) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = Arc::new(OpenCodeClient::new("http://localhost:4096"));

        // Spawn background tasks
        network::spawn_sse_subscriber(&runtime, client.clone());
        network::spawn_health_checker(&runtime, client.clone());
        network::spawn_project_loader(&runtime, client.clone());

        self.client = Some(client);
        self._runtime = Some(runtime);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        for action in actions {
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                if let AppAction::OpenCodeEvent(oc_event) = app_action {
                    event_handlers::handle_opencode_event(&mut self.state, &self.ui, cx, oc_event);
                } else {
                    event_handlers::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                }
            }
        }
    }

    fn load_messages(&mut self, session_id: String) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        network::spawn_message_loader(runtime, client, session_id);
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
        network::spawn_message_sender(runtime, client, session_id, text);
    }

    fn create_session(&mut self, _cx: &mut Cx) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        network::spawn_session_creator(runtime, client);
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
                        self.state.messages_data.clear();
                        self.ui
                            .message_list(id!(message_list))
                            .set_messages(cx, &self.state.messages_data);
                        self.state.update_projects_panel(&self.ui, cx);
                        self.state.update_session_title_ui(&self.ui, cx);
                        self.load_messages(session_id.clone());
                    }
                    ProjectsPanelAction::CreateSession(_project_id) => {
                        self.create_session(cx);
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
