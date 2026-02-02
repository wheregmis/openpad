use crate::components::diff_view::DiffViewWidgetRefExt;
use crate::components::permission_card::PermissionCardWidgetRefExt;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;
    use crate::components::user_bubble::UserBubble;
    use crate::components::assistant_bubble::AssistantBubble;
    use crate::components::diff_view::DiffView;
    use crate::components::permission_card::PermissionCard;

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

                        revert_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_ACCENT_AMBER),
                                text_style: <THEME_FONT_BOLD> { font_size: 8 },
                            }
                            text: "REVERT"
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
                                // radius: 4.0
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

            PermissionMsg = <PermissionCard> {}

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

                        revert_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_ACCENT_AMBER),
                                text_style: <THEME_FONT_BOLD> { font_size: 8 },
                            }
                            text: "REVERT"
                        }
                    }

                    markdown_view = <View> {
                        visible: false
                        width: Fill, height: Fit
                        msg_text = <Markdown> {
                            width: Fill, height: Fit
                            font_size: 10
                            font_color: (THEME_COLOR_TEXT_NORMAL)
                            paragraph_spacing: 8
                            pre_code_spacing: 6
                            use_code_block_widget: true

                            code_block = <RoundedView> {
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
                    }

                    label_view = <View> {
                        visible: false
                        width: Fill, height: Fit
                        msg_label = <Label> {
                            width: Fill, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_NORMAL)
                                text_style: <THEME_FONT_REGULAR> { font_size: 10, line_spacing: 1.4 }
                                word: Wrap
                            }
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

                    diff_view = <DiffView> {}

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
                                // radius: 4.0
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
                                // radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                                color_hover: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                        }

                        diff_button = <Button> {
                            width: Fit, height: 20
                            text: "Diff"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                // radius: 4.0
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
    pub diffs: Vec<openpad_protocol::FileDiff>,
    pub show_diffs: bool,
}

#[derive(Clone, Debug)]
pub struct PendingPermissionDisplay {
    pub session_id: String,
    pub request_id: String,
    pub permission: String,
    pub patterns: Vec<String>,
}

#[derive(Live, LiveHook, Widget)]
pub struct MessageList {
    #[deref]
    view: View,
    #[rust]
    messages: Vec<DisplayMessage>,
    #[rust]
    is_working: bool,
    #[rust]
    revert_message_id: Option<String>,
    #[rust]
    pending_permissions: Vec<PendingPermissionDisplay>,
    #[rust]
    working_since: Option<std::time::Instant>,
}

impl MessageList {
    fn diff_summary(diffs: &[openpad_protocol::FileDiff], expanded: bool) -> String {
        let file_count = diffs.len();
        let additions: i64 = diffs.iter().map(|d| d.additions).sum();
        let deletions: i64 = diffs.iter().map(|d| d.deletions).sum();
        let file_label = if file_count == 1 { "file" } else { "files" };
        let chevron = if expanded { "▾" } else { "▸" };
        format!(
            "{} {} · +{} -{} {}",
            file_count, file_label, additions, deletions, chevron
        )
    }

    fn rebuild_from_parts(
        messages_with_parts: &[openpad_protocol::MessageWithParts],
    ) -> Vec<DisplayMessage> {
        let mut display = Vec::new();
        let mut pending_diffs: Option<Vec<openpad_protocol::FileDiff>> = None;
        for mwp in messages_with_parts {
            let (role, timestamp, model_id, tokens, cost, error_text, is_error) = match &mwp.info {
                openpad_protocol::Message::User(msg) => (
                    "user",
                    Some(msg.time.created),
                    None,
                    None,
                    None,
                    None,
                    false,
                ),
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

            let mut diffs = Vec::new();
            match &mwp.info {
                openpad_protocol::Message::User(msg) => {
                    if let Some(summary) = &msg.summary {
                        if !summary.diffs.is_empty() {
                            pending_diffs = Some(summary.diffs.clone());
                        }
                    }
                }
                openpad_protocol::Message::Assistant(_) => {
                    if let Some(pending) = pending_diffs.take() {
                        diffs = pending;
                    }
                }
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
                diffs,
                show_diffs: false,
            });
        }
        display
    }
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        use crate::state::actions::MessageListAction;

        if self.is_working {
            if let Event::NextFrame(_) = event {
                // Only redraw on next frame if we are actually working/streaming
                // We rely on append_text_for_message to trigger redraws when content updates
                // But we still need this for the "working..." timer update
                self.redraw(cx);
            }
            // Throttle timer updates to 100ms instead of every frame to save CPU
            // The timer only updates seconds anyway
             cx.new_next_frame(); 
        }

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

            if widget.button(&[id!(diff_button)]).clicked(&actions) {
                if let Some(message) = self.messages.get_mut(item_id) {
                    message.show_diffs = !message.show_diffs;
                    self.redraw(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let total_items = self.messages.len()
                    + self.pending_permissions.len()
                    + if self.is_working { 1 } else { 0 };
                if total_items == 0 {
                    list.set_item_range(cx, 0, 0);
                    continue;
                }

                list.set_item_range(cx, 0, total_items);

                while let Some(item_id) = list.next_visible_item(cx) {
                    // After messages, render pending permissions
                    if item_id >= self.messages.len()
                        && item_id < self.messages.len() + self.pending_permissions.len()
                    {
                        let perm_idx = item_id - self.messages.len();
                        let perm = &self.pending_permissions[perm_idx];
                        let item_widget = list.item(cx, item_id, live_id!(PermissionMsg));
                        use crate::components::permission_card::PermissionCardApi;
                        item_widget.permission_card(&[]).set_permission(
                            cx,
                            perm.session_id.clone(),
                            perm.request_id.clone(),
                            &perm.permission,
                            &perm.patterns,
                        );
                        item_widget.draw_all(cx, scope);
                        continue;
                    }

                    if item_id >= self.messages.len() + self.pending_permissions.len() {
                        if !self.is_working {
                            continue;
                        }
                        let elapsed = self
                            .working_since
                            .map(|t| t.elapsed().as_secs())
                            .unwrap_or(0);
                        let mins = elapsed / 60;
                        let secs = elapsed % 60;
                        let status_text = if elapsed > 0 {
                            format!("Agent working... {}:{:02}", mins, secs)
                        } else {
                            "Agent working...".to_string()
                        };
                        let item_widget = list.item(cx, item_id, live_id!(AssistantMsg));
                        
                        // For status, always use label for better performance
                        item_widget.view(&[id!(markdown_view)]).set_visible(cx, false);
                        item_widget.view(&[id!(label_view)]).set_visible(cx, true);
                        item_widget
                            .widget(&[id!(msg_label)])
                            .set_text(cx, &status_text);
                        item_widget.label(&[id!(timestamp_label)]).set_text(cx, "");
                        item_widget
                            .label(&[id!(revert_label)])
                            .set_visible(cx, false);
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
                        
                        if msg.role == "user" {
                            item_widget.widget(&[id!(msg_text)]).set_text(cx, &msg.text);
                        } else {
                            // HEURISTIC: content triggers markdown if it has backticks, hash headers, or > quotes
                            // This is a simple check to avoid markdown widget cost for plain text
                            let needs_markdown = msg.text.contains("```") 
                                || msg.text.contains("`") 
                                || msg.text.contains("# ")
                                || msg.text.contains("> ");

                            if needs_markdown {
                                item_widget.view(&[id!(label_view)]).set_visible(cx, false);
                                item_widget.view(&[id!(markdown_view)]).set_visible(cx, true);
                                item_widget.widget(&[id!(msg_text)]).set_text(cx, &msg.text);
                            } else {
                                item_widget.view(&[id!(markdown_view)]).set_visible(cx, false);
                                item_widget.view(&[id!(label_view)]).set_visible(cx, true);
                                item_widget.widget(&[id!(msg_label)]).set_text(cx, &msg.text);
                            }
                        }

                        let is_revert_point = msg
                            .message_id
                            .as_ref()
                            .and_then(|id| self.revert_message_id.as_ref().map(|rev| rev == id))
                            .unwrap_or(false);
                        item_widget
                            .label(&[id!(revert_label)])
                            .set_visible(cx, is_revert_point);

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
                                let formatted = crate::ui::formatters::format_token_usage(tokens);
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
                            let show_diff_button = !msg.diffs.is_empty();
                            let diff_button = item_widget.button(&[id!(diff_button)]);
                            if show_diff_button {
                                let label = Self::diff_summary(&msg.diffs, msg.show_diffs);
                                diff_button.set_text(cx, &label);
                                diff_button.set_visible(cx, true);
                            } else {
                                diff_button.set_text(cx, "");
                                diff_button.set_visible(cx, false);
                            }
                            item_widget.view(&[id!(msg_actions)]).set_visible(cx, true);

                            // Set diff view data
                            use crate::components::diff_view::DiffViewApi;
                            let diff_view = item_widget.diff_view(&[id!(diff_view)]);
                            if msg.diffs.is_empty() {
                                diff_view.clear_diffs(cx);
                            } else {
                                diff_view.set_diffs(cx, &msg.diffs);
                            }
                            diff_view.set_expanded(cx, msg.show_diffs);
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
        revert_message_id: Option<String>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.messages = MessageList::rebuild_from_parts(messages_with_parts);
            inner.revert_message_id = revert_message_id;
            // Scroll to the last message so users see the most recent conversation
            let msg_count = inner.messages.len();
            if msg_count > 0 {
                inner
                    .view
                    .portal_list(&[id!(list)])
                    .set_first_id(msg_count.saturating_sub(1));
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
                    // Only redraw if content changed
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
                diffs: Vec::new(),
                show_diffs: false,
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
                diffs: Vec::new(),
                show_diffs: false,
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
            inner.is_working = working;
            if working && inner.working_since.is_none() {
                inner.working_since = Some(std::time::Instant::now());
            } else if !working {
                inner.working_since = None;
            }
            inner.redraw(cx);
        }
    }

    pub fn set_pending_permissions(&self, cx: &mut Cx, permissions: &[PendingPermissionDisplay]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.pending_permissions = permissions.to_vec();
            inner.redraw(cx);
        }
    }

    pub fn remove_permission(&self, cx: &mut Cx, request_id: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .pending_permissions
                .retain(|p| p.request_id != request_id);
            inner.redraw(cx);
        }
    }

    pub fn set_session_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]) {
        if let Some(mut inner) = self.borrow_mut() {
            // Apply diffs to the last assistant message
            if let Some(last_assistant) = inner
                .messages
                .iter_mut()
                .rev()
                .find(|m| m.role == "assistant")
            {
                last_assistant.diffs = diffs.to_vec();
                if last_assistant.diffs.is_empty() {
                    last_assistant.show_diffs = false;
                }
            }
            inner.redraw(cx);
        }
    }
}
