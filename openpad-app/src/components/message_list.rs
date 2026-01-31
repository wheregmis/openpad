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

                    msg_text = <Markdown> {
                        width: Fit, height: Fit
                        font_size: 11
                        font_color: #e6e9ee
                        paragraph_spacing: 8
                        pre_code_spacing: 6
                        use_code_block_widget: true

                        draw_normal: {
                            text_style: <THEME_FONT_REGULAR> { font_size: 11 }
                            color: #e6e9ee
                        }
                        draw_italic: {
                            text_style: <THEME_FONT_ITALIC> { font_size: 11 }
                            color: #e6e9ee
                        }
                        draw_bold: {
                            text_style: <THEME_FONT_BOLD> { font_size: 11 }
                            color: #e6e9ee
                        }
                        draw_bold_italic: {
                            text_style: <THEME_FONT_BOLD_ITALIC> { font_size: 11 }
                            color: #e6e9ee
                        }
                        draw_fixed: {
                            text_style: <THEME_FONT_CODE> { font_size: 10 }
                            color: #d7dce2
                        }

                        code_block = <View> {
                            width: Fill, height: Fit
                            margin: { top: 6, bottom: 6 }
                            padding: { left: 8, right: 8, top: 6, bottom: 6 }
                            show_bg: true
                            draw_bg: {
                                color: #1f2329
                                border_radius: 4.0
                            }

                            code_view = <TextInput> {
                                width: Fill, height: Fit
                                is_read_only: true
                                padding: { left: 0, right: 0, top: 0, bottom: 0 }
                                margin: { left: 0, right: 0, top: 0, bottom: 0 }

                                draw_text: {
                                    color: #d7dce2,
                                    text_style: <THEME_FONT_CODE> { font_size: 10 }
                                }
                                draw_bg: {
                                    color: #0000
                                    border_radius: 0.0
                                    border_size: 0.0
                                    color_hover: #0000
                                    color_focus: #0000
                                    color_down: #0000
                                    color_empty: #0000
                                    color_disabled: #0000
                                    border_color_1: #0000
                                    border_color_2: #0000
                                    border_color_1_hover: #0000
                                    border_color_2_hover: #0000
                                    border_color_1_focus: #0000
                                    border_color_2_focus: #0000
                                    border_color_1_down: #0000
                                    border_color_2_down: #0000
                                    border_color_1_empty: #0000
                                    border_color_2_empty: #0000
                                    border_color_1_disabled: #0000
                                    border_color_2_disabled: #0000
                                }
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

                list.set_item_range(cx, 0, self.messages.len());

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
                    item_widget.widget(id!(msg_text)).set_text(cx, &msg.text);
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
