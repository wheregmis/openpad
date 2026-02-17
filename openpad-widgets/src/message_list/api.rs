use super::*;

impl MessageListRef {
    pub fn set_messages(
        &self,
        cx: &mut Cx,
        messages_with_parts: &[openpad_protocol::MessageWithParts],
        revert_message_id: Option<String>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            let had_messages = !inner.messages.is_empty();
            let last_had_running_steps = inner
                .messages
                .last()
                .map(|m| {
                    m.role == "assistant"
                        && m.text.is_empty()
                        && m.steps.iter().any(|s| s.has_running)
                })
                .unwrap_or(false);
            inner.messages = MessageProcessor::rebuild_from_parts(messages_with_parts);
            inner.revert_message_id = revert_message_id;
            for msg in inner.messages.iter_mut() {
                if msg.role == "assistant" && msg.text.is_empty() && !msg.steps.is_empty() {
                    msg.show_steps = true;
                }
            }
            if let Some(last) = inner.messages.last_mut() {
                if last.role == "assistant"
                    && last.text.is_empty()
                    && !last.steps.is_empty()
                    && (last.steps.iter().any(|s| s.has_running) || last_had_running_steps)
                {
                    last.show_steps = true;
                }
            }
            inner.tail_mode = TailMode::after_set_messages(inner.tail_mode, had_messages);
            let msg_count = inner.messages.len();
            if inner.should_follow_tail() && msg_count > 0 {
                inner.tail_to_end(cx);
            }
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }

    pub fn append_text_for_message(&self, cx: &mut Cx, role: &str, message_id: &str, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            if let Some(last) = inner.messages.last_mut() {
                if last.role == role {
                    let was_empty = last.text.is_empty();
                    last.text.push_str(text);
                    if role == "assistant" && was_empty && !last.steps.is_empty() {
                        last.show_steps = false;
                    }
                    MessageProcessor::refresh_message_caches(last);
                    if inner.should_follow_tail() {
                        inner.tail_to_end(cx);
                    }
                    inner.update_cached_indices();
                    inner.redraw(cx);
                    return;
                }
            }
            let mut msg = DisplayMessage {
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
                steps: Vec::new(),
                show_steps: false,
                duration_ms: None,
                cached_steps_summary: String::new(),
                cached_grouped_summary: String::new(),
                cached_tool_groups: Vec::new(),
                cached_needs_markdown: false,
                cached_thinking_activity: String::new(),
                cached_running_tools: Vec::new(),
                cached_timestamp: String::new(),
                cached_token_usage: String::new(),
                cached_cost: String::new(),
                cached_full_diff: String::new(),
                cached_diff_files: String::new(),
                cached_diff_add: String::new(),
                cached_diff_del: String::new(),
            };
            MessageProcessor::refresh_message_caches(&mut msg);
            inner.messages.push(msg);
            if inner.should_follow_tail() {
                inner.tail_to_end(cx);
            }
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }

    pub fn add_user_message(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            let mut msg = DisplayMessage {
                role: "user".to_string(),
                text: text.to_string(),
                message_id: None,
                timestamp: Some(now),
                model_id: None,
                tokens: None,
                cost: None,
                error_text: None,
                is_error: false,
                diffs: Vec::new(),
                show_diffs: false,
                steps: Vec::new(),
                show_steps: false,
                duration_ms: None,
                cached_steps_summary: String::new(),
                cached_grouped_summary: String::new(),
                cached_tool_groups: Vec::new(),
                cached_needs_markdown: false,
                cached_thinking_activity: String::new(),
                cached_running_tools: Vec::new(),
                cached_timestamp: String::new(),
                cached_token_usage: String::new(),
                cached_cost: String::new(),
                cached_full_diff: String::new(),
                cached_diff_files: String::new(),
                cached_diff_add: String::new(),
                cached_diff_del: String::new(),
            };
            MessageProcessor::refresh_message_caches(&mut msg);
            inner.messages.push(msg);
            if inner.should_follow_tail() {
                inner.tail_to_end(cx);
            }
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }

    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.messages.clear();
            inner.tail_mode = TailMode::Follow;
            inner.streaming_anim_item = None;
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }

    pub fn set_working(&self, cx: &mut Cx, working: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_working = working;
            if working && inner.working_since.is_none() {
                inner.working_since = Some(std::time::Instant::now());
                inner.thinking_frame = 0;
                inner.frame_count = 0;
                inner.last_timer_secs = 0;
                inner.cached_timer_text = String::new();
                if inner.should_follow_tail() {
                    inner.tail_to_end(cx);
                }
            } else if !working {
                inner.working_since = None;
                inner.streaming_anim_item = None;
            }
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }

    pub fn set_pending_permissions(&self, cx: &mut Cx, permissions: &[PendingPermissionDisplay]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.pending_permissions = permissions.to_vec();
            if inner.should_follow_tail() {
                inner.tail_to_end(cx);
            }
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }

    pub fn remove_permission(&self, cx: &mut Cx, request_id: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .pending_permissions
                .retain(|p| p.request_id != request_id);
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }

    pub fn set_session_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]) {
        if let Some(mut inner) = self.borrow_mut() {
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
            inner.update_cached_indices();
            inner.redraw(cx);
        }
    }
}
