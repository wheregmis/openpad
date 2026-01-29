use makepad_widgets::*;
use openpad_protocol::{OpenCodeClient, Session, Message, Event as OcEvent};
use std::sync::Arc;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    App = {{App}} {
        ui: <Window> {
            window: { inner_size: vec2(1200, 800) }
            pass: { clear_color: #1a1a1a }

            body = <View> {
                flow: Down,
                spacing: 0,

                // Status bar at top
                <View> {
                    walk: { width: Fill, height: Fit }
                    flow: Right,
                    spacing: 8,
                    padding: 8,
                    draw_bg: { color: #2a2a2a }

                    status_label = <Label> {
                        text: "Connecting..."
                        draw_text: { color: #888 }
                    }
                }

                // Messages area (scrollable)
                <ScrollYView> {
                    walk: { width: Fill, height: Fill }

                    message_list = <PortalList> {
                        UserMsg = <View> {
                            walk: { width: Fill, height: Fit }
                            flow: Right,
                            padding: 8,
                            align: { x: 1.0 }

                            <View> {
                                walk: { width: Fit, height: Fit, margin: { left: 100 } }
                                flow: Down,
                                padding: 12,
                                draw_bg: { color: #2a4a6a }

                                msg_text = <Label> {
                                    draw_text: { color: #fff, text_style: { font_size: 11 } }
                                }
                            }
                        }

                        AssistantMsg = <View> {
                            walk: { width: Fill, height: Fit }
                            flow: Down,
                            padding: 8,

                            <View> {
                                walk: { width: Fit, height: Fit, margin: { right: 100 } }
                                flow: Down,
                                padding: 12,
                                draw_bg: { color: #333 }

                                msg_text = <Label> {
                                    draw_text: { color: #fff, text_style: { font_size: 11 } }
                                }
                            }
                        }
                    }
                }

                // Input area (fixed at bottom)
                <View> {
                    walk: { width: Fill, height: Fit }
                    flow: Right,
                    spacing: 8,
                    padding: 16,
                    draw_bg: { color: #2a2a2a }

                    input_box = <TextInput> {
                        walk: { width: Fill, height: Fit }
                        draw_bg: { color: #333 }
                        draw_text: { color: #fff }
                        text: ""
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
    SessionCreated(Session),
    OpenCodeEvent(OcEvent),
    SendMessageFailed(String),
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,

    #[rust] messages: Vec<Message>,
    #[rust] current_session_id: Option<String>,
    #[rust] connected: bool,
    #[rust] error_message: Option<String>,
    #[rust] client: Option<Arc<OpenCodeClient>>,
    #[rust] _runtime: Option<tokio::runtime::Runtime>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl App {
    fn connect_to_opencode(&mut self, _cx: &mut Cx) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = Arc::new(OpenCodeClient::new("http://localhost:4096"));
        let client_clone = client.clone();

        runtime.spawn(async move {
            // Try to connect by listing sessions
            match client_clone.list_sessions().await {
                Ok(_) => {
                    Cx::post_action(AppAction::Connected);

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

        self.client = Some(client);
        self._runtime = Some(runtime);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        for action in actions {
            if let Some(app_action) = action.as_widget_action().cast() {
                match app_action {
                    AppAction::Connected => {
                        self.connected = true;
                        self.error_message = None;
                        self.ui.label(id!(status_label)).set_text(cx, "Connected");
                        cx.redraw_all();
                    }
                    AppAction::ConnectionFailed(err) => {
                        self.error_message = Some(err.clone());
                        self.ui.label(id!(status_label)).set_text(cx, &format!("Error: {}", err));
                        cx.redraw_all();
                    }
                    AppAction::SessionCreated(session) => {
                        self.current_session_id = Some(session.id.clone());
                        self.ui.label(id!(status_label)).set_text(cx, &format!("Session: {} | {} messages", session.id, self.messages.len()));
                        cx.redraw_all();
                    }
                    AppAction::OpenCodeEvent(oc_event) => {
                        self.handle_opencode_event(cx, &oc_event);
                    }
                    AppAction::SendMessageFailed(err) => {
                        self.error_message = Some(err.clone());
                        cx.redraw_all();
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
                    self.ui.label(id!(status_label)).set_text(cx, &format!("Session: {} | {} messages", session.id, self.messages.len()));
                }
            }
            OcEvent::MessageUpdated { message, .. } => {
                // Find existing message or add new
                if let Some(existing) = self.messages.iter_mut().find(|m| m.id() == message.id()) {
                    *existing = message.clone();
                } else {
                    self.messages.push(message.clone());
                }

                if let Some(session_id) = &self.current_session_id {
                    self.ui.label(id!(status_label)).set_text(
                        cx,
                        &format!("Session: {} | {} messages", session_id, self.messages.len())
                    );
                }
                cx.redraw_all();
            }
            OcEvent::PartUpdated { message_id, part_index, part, .. } => {
                if let Some(msg) = self.messages.iter_mut().find(|m| m.id() == message_id) {
                    let parts = msg.parts_mut();
                    if *part_index < parts.len() {
                        parts[*part_index] = part.clone();
                    } else {
                        parts.push(part.clone());
                    }
                    cx.redraw_all();
                }
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
    }
}

app_main!(App);

fn main() {
    app_main();
}
