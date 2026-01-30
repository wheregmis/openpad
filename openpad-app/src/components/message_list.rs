use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::user_bubble::UserBubble;
    use crate::components::assistant_bubble::AssistantBubble;

    pub MessageList = {{MessageList}} {
        width: Fill, height: Fill
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}

            UserMsg = <View> {
                width: Fill, height: Fit
                flow: Right,
                padding: 8,
                align: { x: 1.0 }

                <UserBubble> {
                    width: Fit, height: Fit
                    margin: { left: 80 }
                    flow: Down,

                    msg_text = <Label> {
                        width: Fit, height: Fit
                        draw_text: {
                            color: #eef3f7,
                            text_style: { font_size: 11 },
                            word: Wrap
                        }
                    }
                }
            }

            AssistantMsg = <View> {
                width: Fill, height: Fit
                flow: Down,
                padding: 8,

                <AssistantBubble> {
                    width: Fit, height: Fit
                    margin: { right: 80 }
                    flow: Down,

                    msg_text = <Label> {
                        width: Fit, height: Fit
                        draw_text: {
                            color: #e6e9ee,
                            text_style: { font_size: 11 },
                            word: Wrap
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
}

#[derive(Live, LiveHook, Widget)]
pub struct MessageList {
    #[deref]
    view: View,
    #[rust]
    messages: Vec<DisplayMessage>,
}

impl MessageList {
    fn rebuild_from_parts(
        messages_with_parts: &[openpad_protocol::MessageWithParts],
    ) -> Vec<DisplayMessage> {
        let mut display = Vec::new();
        for mwp in messages_with_parts {
            let role = match &mwp.info {
                openpad_protocol::Message::User(_) => "user",
                openpad_protocol::Message::Assistant(_) => "assistant",
            };

            let text: String = mwp
                .parts
                .iter()
                .filter_map(|p| p.text_content())
                .collect::<Vec<_>>()
                .join("\n");

            if text.is_empty() {
                continue;
            }

            display.push(DisplayMessage {
                role: role.to_string(),
                text,
            });
        }
        display
    }
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if self.messages.is_empty() {
                    list.set_item_range(cx, 0, 0);
                    continue;
                }

                list.set_item_range(cx, 0, self.messages.len().saturating_sub(1));

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id >= self.messages.len() {
                        continue;
                    }

                    let msg = &self.messages[item_id];
                    let template = if msg.role == "user" {
                        live_id!(UserMsg)
                    } else {
                        live_id!(AssistantMsg)
                    };

                    let item_widget = list.item(cx, item_id, template);
                    item_widget.label(id!(msg_text)).set_text(cx, &msg.text);
                    item_widget.draw_all(cx, scope);
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
            inner.messages = MessageList::rebuild_from_parts(messages_with_parts);
            inner.redraw(cx);
        }
    }

    pub fn append_text_for_message(&self, cx: &mut Cx, role: &str, _message_id: &str, text: &str) {
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
            // New message
            inner.messages.push(DisplayMessage {
                role: role.to_string(),
                text: text.to_string(),
            });
            inner.redraw(cx);
        }
    }

    pub fn add_user_message(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.messages.push(DisplayMessage {
                role: "user".to_string(),
                text: text.to_string(),
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
}
