use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
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

                    msg_actions = <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 6,
                        margin: { top: 6 }

                        copy_button = <Button> {
                            width: Fit, height: 20
                            text: "Copy"
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
                        use_code_block_widget: true

                        code_block = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            padding: { left: 8, right: 8, top: 6, bottom: 6 }
                            margin: { top: 4, bottom: 4 }
                            draw_bg: {
                                color: (THEME_COLOR_BG_INPUT)
                                border_radius: 6.0
                            }

                            code_view = <CodeView> {
                                editor: {
                                    width: Fill
                                    height: Fit
                                    draw_bg: { color: (THEME_COLOR_BG_INPUT) }
                                    token_colors: {
                                        unknown: (THEME_COLOR_TEXT_NORMAL)
                                        branch_keyword: (THEME_COLOR_ACCENT_PURPLE)
                                        comment: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                        constant: (THEME_COLOR_ACCENT_AMBER)
                                        delimiter: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                        delimiter_highlight: (THEME_COLOR_TEXT_BRIGHT)
                                        identifier: (THEME_COLOR_TEXT_NORMAL)
                                        loop_keyword: (THEME_COLOR_ACCENT_PURPLE)
                                        number: (THEME_COLOR_ACCENT_AMBER)
                                        other_keyword: (THEME_COLOR_ACCENT_BLUE)
                                        function: (THEME_COLOR_ACCENT_BLUE)
                                        punctuator: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                        string: (THEME_COLOR_TEXT_CODE)
                                        typename: (THEME_COLOR_TEXT_BOLD)
                                        whitespace: (THEME_COLOR_TEXT_NORMAL)
                                        error_decoration: (THEME_COLOR_ACCENT_RED)
                                        warning_decoration: (THEME_COLOR_ACCENT_AMBER)
                                    }
                                }
                            }
                        }

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

                    error_label = <Label> {
                        width: Fill, height: Fit
                        text: ""
                        draw_text: {
                            color: (THEME_COLOR_ACCENT_RED)
                            text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.4 }
                            wrap: Word
                        }
                    }

                    stats_row = <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 8,
                        margin: { top: 6 }
                        align: { y: 0.5 }

                        tokens_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }

                        cost_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }
                    }

                    msg_actions = <View> {
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 6,
                        margin: { top: 8 }

                        copy_button = <Button> {
                            width: Fit, height: 20
                            text: "Copy"
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
    pub tokens: Option<openpad_protocol::TokenUsage>,
    pub cost: Option<f64>,
    pub error_text: Option<String>,
    pub is_error: bool,
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
            let (role, timestamp, model_id, tokens, cost, error_text, is_error) = match &mwp.info
            {
                openpad_protocol::Message::User(msg) => {
                    ("user", Some(msg.time.created), None, None, None, None, false)
                }
                openpad_protocol::Message::Assistant(msg) => {
                    let model = if !msg.model_id.is_empty() {
                        Some(msg.model_id.clone())
                    } else {
                        None
                    };
                    let error_text = msg
                        .error
                        .as_ref()
                        .map(crate::ui::formatters::format_assistant_error);
                    (
                        "assistant",
                        Some(msg.time.created),
                        model,
                        msg.tokens.clone(),
                        Some(msg.cost),
                        error_text,
                        msg.error.is_some(),
                    )
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
            let mut text = text_parts.join("\n");
            if text.is_empty() && error_text.is_some() {
                text = "Assistant error".to_string();
            }
            if text.is_empty() {
                continue;
            }

            display.push(DisplayMessage {
                role: role.to_string(),
                text,
                message_id: Some(message_id),
                timestamp,
                model_id,
                tokens,
                cost,
                error_text,
                is_error,
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

            if widget.button(&[id!(copy_button)]).clicked(&actions) {
                cx.copy_to_clipboard(&self.messages[item_id].text);
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
                        item_widget.label(&[id!(error_label)]).set_text(cx, "");
                        item_widget.view(&[id!(stats_row)]).set_visible(cx, false);
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
                            if let Some(error_text) = &msg.error_text {
                                item_widget
                                    .label(&[id!(error_label)])
                                    .set_text(cx, error_text);
                                item_widget
                                    .widget(&[id!(error_label)])
                                    .set_visible(cx, true);
                            } else {
                                item_widget.label(&[id!(error_label)]).set_text(cx, "");
                                item_widget
                                    .widget(&[id!(error_label)])
                                    .set_visible(cx, false);
                            }

                            let mut show_stats = false;
                            if let Some(tokens) = &msg.tokens {
                                let formatted =
                                    crate::ui::formatters::format_token_usage(tokens);
                                item_widget
                                    .label(&[id!(tokens_label)])
                                    .set_text(cx, &formatted);
                                show_stats = true;
                            } else {
                                item_widget.label(&[id!(tokens_label)]).set_text(cx, "");
                            }

                            if let Some(cost) = msg.cost {
                                let formatted = crate::ui::formatters::format_cost(cost);
                                item_widget
                                    .label(&[id!(cost_label)])
                                    .set_text(cx, &formatted);
                                show_stats = true;
                            } else {
                                item_widget.label(&[id!(cost_label)]).set_text(cx, "");
                            }

                            item_widget
                                .view(&[id!(stats_row)])
                                .set_visible(cx, show_stats);

                            let show_revert = msg.message_id.is_some() && !msg.is_error;
                            item_widget
                                .button(&[id!(revert_button)])
                                .set_visible(cx, show_revert);
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
                tokens: None,
                cost: None,
                error_text: None,
                is_error: false,
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
                tokens: None,
                cost: None,
                error_text: None,
                is_error: false,
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
