use makepad_widgets::*;
use openpad_protocol::{Event as OcEvent, Message, OpenCodeClient, Session};
use std::sync::Arc;

app_main!(App);

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

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

    HeaderBar = <View> {
        width: Fill, height: Fit
        flow: Overlay,
        spacing: 8,
        padding: 10,
        show_bg: true
        draw_bg: {
            color: #22262c
            uniform border_color: #2c323a
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

    StatusDot = <View> {
        width: 10.0, height: 10.0
        show_bg: true
        draw_bg: {
            color: #6b7b8c
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y) - 1.0;
                sdf.circle(c.x, c.y, r);
                sdf.fill(self.color);
                return sdf.result;
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

    InputBar = <RoundedView> {
        width: Fill, height: Fit
        flow: Right,
        spacing: 8,
        padding: 12,
        align: { y: 0.5 }
        draw_bg: {
            color: #1f2329
            border_color: #2e343c
            border_radius: 18.0
            border_size: 1.0
        }
    }

    InputField = <TextInput> {
        width: Fill, height: Fit
        empty_text: "Ask anything..."
        draw_bg: {
            color: #0000
            color_hover: #0000
            color_focus: #0000
            color_down: #0000
            border_size: 0.0
        }
        draw_text: { color: #e6e9ee }
        text: ""
    }

    SendButton = <Button> {
        width: 36, height: 36
        margin: { left: 6 }
        padding: { left: 8, right: 8, top: 8, bottom: 8 }
        text: ""
        icon_walk: { width: 16, height: Fit }
        draw_icon: {
            svg_file: dep("crate://self/resources/icons/send.svg")
            color: #cbd3dc
            color_hover: #ffffff
            color_down: #aeb7c2
        }
        draw_bg: {
            border_radius: 8.0
            border_size: 0.0
            color: #2a2f36
            color_hover: #313843
            color_down: #242a32
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
                    <View> {
                        width: Fill, height: Fit
                        align: { x: 0.5, y: 0.5 }
                        app_title = <Label> {
                            text: "Openpad"
                            draw_text: { color: #e6e9ee, text_style: { font_size: 12 } }
                        }
                    }
                    <View> {
                        width: Fill, height: Fit
                        flow: Right,
                        align: { x: 1.0, y: 0.5 }
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
                }

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
    #[live]
    ui: WidgetRef,

    #[rust]
    messages: Vec<Message>,
    #[rust]
    current_session_id: Option<String>,
    #[rust]
    connected: bool,
    #[rust]
    error_message: Option<String>,
    #[rust]
    client: Option<Arc<OpenCodeClient>>,
    #[rust]
    _runtime: Option<tokio::runtime::Runtime>,
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
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                match app_action {
                    AppAction::Connected => {
                        self.connected = true;
                        self.error_message = None;
                        self.ui.label(id!(status_label)).set_text(cx, "Connected");
                        cx.redraw_all();
                    }
                    AppAction::ConnectionFailed(err) => {
                        self.error_message = Some(err.clone());
                        self.ui
                            .label(id!(status_label))
                            .set_text(cx, &format!("Error: {}", err));
                        cx.redraw_all();
                    }
                    AppAction::SessionCreated(session) => {
                        self.current_session_id = Some(session.id.clone());
                        self.ui.label(id!(status_label)).set_text(
                            cx,
                            &format!("Session: {} | {} messages", session.id, self.messages.len()),
                        );
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
                    self.ui.label(id!(status_label)).set_text(
                        cx,
                        &format!("Session: {} | {} messages", session.id, self.messages.len()),
                    );
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
                        &format!("Session: {} | {} messages", session_id, self.messages.len()),
                    );
                }
                cx.redraw_all();
            }
            OcEvent::PartUpdated {
                message_id,
                part_index,
                part,
                ..
            } => {
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

        if self.ui.button(id!(send_button)).clicked(&actions) {
            let text = self.ui.text_input(id!(input_box)).text();
            if !text.is_empty() {
                self.send_message(cx, text.clone());
                self.ui.text_input(id!(input_box)).set_text(cx, "");
            }
        }
    }
}
