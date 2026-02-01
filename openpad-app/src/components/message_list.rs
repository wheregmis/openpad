use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::*;
    use crate::components::user_bubble::UserBubble;
    use crate::components::assistant_bubble::AssistantBubble;

    pub MessageList = {{MessageList}} {
        width: Fill, height: Fill
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}

            UserMsg = <View> {
                width: Fill, height: Fit
                flow: Down,
                padding: { top: 4, bottom: 4, left: 100, right: 24 }
                align: { x: 1.0 }

                <UserBubble> {
                    width: Fill, height: Fit
                    flow: Down,
                    align: { x: 1.0 }

                    // Metadata row
                    <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: { bottom: 4 }
                        align: { y: 0.5 }

                        timestamp_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARKER),
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 },
                            }
                            text: "..."
                        }

                        <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARK),
                                text_style: <THEME_FONT_BOLD> { font_size: 8 },
                            }
                            text: "YOU"
                        }

                    }

                    msg_text = <Label> {
                        width: Fill, height: Fit
                        draw_text: {
                            color: (THEME_COLOR_TEXT_LIGHT),
                            text_style: <THEME_FONT_REGULAR> { font_size: 10, line_spacing: 1.4 },
                            word: Wrap,
                        }
                    }
                }
            }

            AssistantMsg = <View> {
                width: Fill, height: Fit
                flow: Down,
                padding: { top: 4, bottom: 4, left: 24, right: 100 }

                <AssistantBubble> {
                    width: Fill, height: Fit
                    flow: Down,

                    // Metadata row
                    <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: { bottom: 4 }
                        align: { y: 0.5 }

                        model_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARK),
                                text_style: <THEME_FONT_BOLD> { font_size: 8 },
                            }
                            text: "ASSISTANT"
                        }

                        timestamp_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARKER),
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 },
                            }
                            text: "..."
                        }
                    }

                    msg_text = <Markdown> {
                        width: Fill, height: Fit
                        font_size: 10
                        font_color: (THEME_COLOR_TEXT_NORMAL)
                        paragraph_spacing: 8
                        pre_code_spacing: 6
                        use_code_block_widget: false

                        draw_normal: {
                            text_style: <THEME_FONT_REGULAR> { font_size: 10, line_spacing: 1.4 }
                            color: (THEME_COLOR_TEXT_NORMAL)
                        }
                        draw_italic: {
                            text_style: <THEME_FONT_ITALIC> { font_size: 10 }
                            color: (THEME_COLOR_TEXT_NORMAL)
                        }
                        draw_bold: {
                            text_style: <THEME_FONT_BOLD> { font_size: 10 }
                            color: (THEME_COLOR_TEXT_BOLD)
                        }
                        draw_fixed: {
                            text_style: <THEME_FONT_CODE> { font_size: 9 }
                            color: (THEME_COLOR_TEXT_CODE)
                        }
                    }

                    msg_actions = <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 6,
                        margin: { top: 8 }

                        revert_button = <Button> {
                            width: Fit, height: 20
                            text: "Revert"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                color_hover: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DisplayMessage {
    pub role: String,
    pub text: String,
    pub message_id: Option<String>,
    pub timestamp: Option<i64>,   // Unix timestamp in milliseconds
    pub model_id: Option<String>, // Model ID (for assistant messages)
}

#[derive(Live, LiveHook, Widget)]
pub struct MessageList {
    #[deref]
    view: View,
    #[rust]
    messages: Vec<DisplayMessage>,
    #[rust]
    is_working: bool,
}

impl MessageList {
    fn rebuild_from_parts(
        messages_with_parts: &[openpad_protocol::MessageWithParts],
    ) -> Vec<DisplayMessage> {
        let mut display = Vec::new();
        for mwp in messages_with_parts {
            let (role, timestamp, model_id) = match &mwp.info {
                openpad_protocol::Message::User(msg) => ("user", Some(msg.time.created), None),
                openpad_protocol::Message::Assistant(msg) => {
                    let model = if !msg.model_id.is_empty() {
                        Some(msg.model_id.clone())
                    } else {
                        None
                    };
                    ("assistant", Some(msg.time.created), model)
                }
            };

            let message_id = mwp.info.id().to_string();

            let mut text_parts: Vec<String> = Vec::new();
            for p in &mwp.parts {
                if let Some(text) = p.text_content() {
                    text_parts.push(text.to_string());
                } else if let Some((_mime, filename, _url)) = p.file_info() {
                    let name = filename.unwrap_or("attachment");
                    text_parts.push(format!("[Attachment: {}]", name));
                }
            }
            let text = text_parts.join("\n");

            if text.is_empty() {
                continue;
            }

            display.push(DisplayMessage {
                role: role.to_string(),
                text,
                message_id: Some(message_id),
                timestamp,
                model_id,
            });
        }
        display
    }
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        use crate::state::actions::MessageListAction;

        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        let list = self.view.portal_list(&[id!(list)]);
        for (item_id, widget) in list.items_with_actions(&actions) {
            if item_id >= self.messages.len() {
                continue;
            }

            if widget.button(&[id!(revert_button)]).clicked(&actions) {
                if let Some(message_id) = &self.messages[item_id].message_id {
                    cx.action(MessageListAction::RevertToMessage(message_id.clone()));
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let total_items = self.messages.len() + if self.is_working { 1 } else { 0 };
                if total_items == 0 {
                    list.set_item_range(cx, 0, 0);
                    continue;
                }

                list.set_item_range(cx, 0, total_items);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id >= self.messages.len() {
                        if !self.is_working {
                            continue;
                        }
                        let item_widget = list.item(cx, item_id, live_id!(AssistantMsg));
                        item_widget
                            .widget(&[id!(msg_text)])
                            .set_text(cx, "Thinking...");
                        item_widget.label(&[id!(timestamp_label)]).set_text(cx, "");
                        item_widget
                            .label(&[id!(model_label)])
                            .set_text(cx, "ASSISTANT");
                        item_widget.view(&[id!(msg_actions)]).set_visible(cx, false);
                        item_widget.draw_all(cx, scope);
                    } else {
                        let msg = &self.messages[item_id];
                        let template = if msg.role == "user" {
                            live_id!(UserMsg)
                        } else {
                            live_id!(AssistantMsg)
                        };

                        let item_widget = list.item(cx, item_id, template);
                        item_widget.widget(&[id!(msg_text)]).set_text(cx, &msg.text);

                        // Set timestamp if available
                        if let Some(timestamp) = msg.timestamp {
                            let formatted = crate::ui::formatters::format_timestamp(timestamp);
                            item_widget
                                .label(&[id!(timestamp_label)])
                                .set_text(cx, &formatted);
                        }

                        // Set model ID for assistant messages
                        if msg.role == "assistant" {
                            if let Some(ref model_id) = msg.model_id {
                                item_widget
                                    .label(&[id!(model_label)])
                                    .set_text(cx, model_id);
                            }
                            item_widget.view(&[id!(msg_actions)]).set_visible(cx, true);
                        }

                        item_widget.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl MessageListRef {
    pub fn set_messages(
        &self,
        cx: &mut Cx,
        messages_with_parts: &[openpad_protocol::MessageWithParts],
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            let was_empty = inner.messages.is_empty();
            inner.messages = MessageList::rebuild_from_parts(messages_with_parts);
            // Reset scroll position when loading a new set of messages
            // to avoid stale first_id from a previous longer list
            if was_empty || messages_with_parts.is_empty() {
                inner.view.portal_list(&[id!(list)]).set_first_id(0);
            }
            inner.redraw(cx);
        }
    }

    pub fn append_text_for_message(&self, cx: &mut Cx, role: &str, message_id: &str, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            // Try to find an existing message to append to (by checking last message)
            // SSE parts arrive in order, so the last message of the matching role is the target
            if let Some(last) = inner.messages.last_mut() {
                if last.role == role {
                    last.text.push_str(text);
                    inner.redraw(cx);
                    return;
                }
            }
            // New message (no timestamp/model during streaming; will be updated later)
            inner.messages.push(DisplayMessage {
                role: role.to_string(),
                text: text.to_string(),
                message_id: Some(message_id.to_string()),
                timestamp: None,
                model_id: None,
            });
            inner.redraw(cx);
        }
    }

    pub fn add_user_message(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            // Use current time for user messages
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;

            inner.messages.push(DisplayMessage {
                role: "user".to_string(),
                text: text.to_string(),
                message_id: None, // User messages don't have IDs yet
                timestamp: Some(now),
                model_id: None,
            });
            inner.redraw(cx);
        }
    }

    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.messages.clear();
            inner.redraw(cx);
        }
    }

    pub fn set_working(&self, cx: &mut Cx, working: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            if inner.is_working == working {
                return;
            }
            inner.is_working = working;
            inner.redraw(cx);
        }
    }
}
