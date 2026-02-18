use super::*;

impl MessageList {
    pub(crate) fn draw_walk_impl(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let is_empty =
            self.messages.is_empty() && self.pending_permissions.is_empty() && !self.is_working;
        self.view
            .view(cx, &[id!(empty_state)])
            .set_visible(cx, is_empty);

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
                    if item_id >= self.messages.len()
                        && item_id < self.messages.len() + self.pending_permissions.len()
                    {
                        let perm_idx = item_id - self.messages.len();
                        let perm = &self.pending_permissions[perm_idx];
                        let item_widget = list.item(cx, item_id, live_id!(PermissionMsg));
                        item_widget.permission_card(cx, &[]).set_permission(
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
                        let last_assistant_has_running = self.cached_last_assistant_has_running;
                        if last_assistant_has_running {
                            continue;
                        }
                        let elapsed = self
                            .working_since
                            .map(|t| t.elapsed().as_secs())
                            .unwrap_or(0);
                        let mins = elapsed / 60;
                        let secs = elapsed % 60;

                        let last_msg = self.messages.last();
                        let current_activity = last_msg
                            .map(|m| &m.cached_thinking_activity)
                            .map(|s| s.as_str())
                            .unwrap_or("Working...");
                        let running_tools = last_msg.map(|m| &m.cached_running_tools);

                        // Optimization: cache the timer text to avoid repeated formatting and allocations
                        // in the draw loop, which runs every frame during animations.
                        if elapsed != self.last_timer_secs {
                            self.last_timer_secs = elapsed;
                            self.cached_timer_text = if elapsed > 0 {
                                format!("· {}m, {}s", mins, secs)
                            } else {
                                String::new()
                            };
                        }

                        let item_widget = list.item(cx, item_id, live_id!(ThinkingMsg));
                        item_widget
                            .label(cx, &[id!(thinking_label)])
                            .set_text(cx, "Working");
                        item_widget
                            .label(cx, &[id!(thinking_icon)])
                            .set_text(cx, self.thinking_icon());
                        item_widget
                            .label(cx, &[id!(thinking_timer)])
                            .set_text(cx, &self.cached_timer_text);
                        item_widget
                            .label(cx, &[id!(thinking_activity)])
                            .set_text(cx, current_activity);

                        let has_tools = running_tools.map(|t| !t.is_empty()).unwrap_or(false);
                        item_widget
                            .view(cx, &[id!(thinking_tools)])
                            .set_visible(cx, has_tools);
                        if let Some(tools) = running_tools {
                            for (idx, (icon, name, input)) in tools.iter().take(5).enumerate() {
                                let Some(&(row_id, icon_id, name_id, input_id)) = Self::TOOL_ROW.get(idx) else {
                                    continue;
                                };
                                let tools_view = item_widget.view(cx, &[id!(thinking_tools)]);
                                tools_view.view(cx, &[row_id]).set_visible(cx, true);
                                tools_view.label(cx, &[icon_id]).set_text(cx, icon);
                                tools_view.label(cx, &[name_id]).set_text(cx, name);
                                tools_view.label(cx, &[input_id]).set_text(cx, input);
                            }
                            let shown = running_tools.map(|t| t.len()).unwrap_or(0);
                            for &(row_id, _, _, _) in Self::TOOL_ROW.iter().skip(shown) {
                                item_widget
                                    .view(cx, &[id!(thinking_tools)])
                                    .view(cx, &[row_id])
                                    .set_visible(cx, false);
                            }
                        }
                        item_widget.draw_all(cx, scope);
                    } else {
                        let msg = &self.messages[item_id];
                        let fallback_text = if msg.text.trim().is_empty() && !msg.steps.is_empty() {
                            msg.cached_steps_summary.as_str()
                        } else {
                            msg.text.as_str()
                        };
                        let last_assistant_idx = self.cached_last_assistant_idx;
                        let streaming_msg = self.is_working
                            && msg.role == "assistant"
                            && last_assistant_idx == Some(item_id);
                        let template = if msg.role == "user" {
                            live_id!(UserMsg)
                        } else {
                            live_id!(AssistantMsg)
                        };
                        let item_widget = list.item(cx, item_id, template);

                        if msg.role == "user" {
                            item_widget
                                .widget(cx, &[id!(msg_text)])
                                .set_text(cx, fallback_text);
                        } else {
                            let use_markdown = msg.cached_needs_markdown;
                            if use_markdown {
                                item_widget
                                    .view(cx, &[id!(label_view)])
                                    .set_visible(cx, false);
                                item_widget
                                    .view(cx, &[id!(markdown_view)])
                                    .set_visible(cx, true);
                                let mut markdown = item_widget.markdown(cx, &[id!(msg_text)]);
                                markdown.set_text(cx, fallback_text);
                                if streaming_msg {
                                    if self.streaming_anim_item != Some(item_id) {
                                        self.streaming_anim_item = Some(item_id);
                                        markdown.reset_all_streaming_animations();
                                    } else {
                                        markdown.start_streaming_animation();
                                    }
                                } else if self.streaming_anim_item == Some(item_id) {
                                    markdown.stop_streaming_animation();
                                    if markdown.is_streaming_animation_done() {
                                        self.streaming_anim_item = None;
                                    }
                                }
                            } else {
                                item_widget
                                    .view(cx, &[id!(markdown_view)])
                                    .set_visible(cx, false);
                                item_widget
                                    .view(cx, &[id!(label_view)])
                                    .set_visible(cx, true);
                                item_widget
                                    .widget(cx, &[id!(msg_label)])
                                    .set_text(cx, fallback_text);
                            }
                        }

                        let is_revert_point = msg
                            .message_id
                            .as_ref()
                            .and_then(|id| self.revert_message_id.as_ref().map(|rev| rev == id))
                            .unwrap_or(false);
                        let show_revert = is_revert_point
                            || (self.revert_message_id.is_none()
                                && last_assistant_idx == Some(item_id));
                        if msg.role == "assistant" {
                            item_widget
                                .button(cx, &[id!(copy_action_button)])
                                .set_visible(cx, true);
                            item_widget
                                .button(cx, &[id!(revert_action_button)])
                                .set_visible(cx, show_revert);
                        }

                        if msg.timestamp.is_some() {
                            item_widget
                                .label(cx, &[id!(timestamp_label)])
                                .set_text(cx, &msg.cached_timestamp);
                        }

                        if msg.role == "assistant" {
                            if let Some(ref model_id) = msg.model_id {
                                item_widget
                                    .label(cx, &[id!(model_label)])
                                    .set_text(cx, model_id);
                            }
                            if let Some(error_text) = &msg.error_text {
                                item_widget
                                    .label(cx, &[id!(error_label)])
                                    .set_text(cx, error_text);
                                item_widget
                                    .widget(cx, &[id!(error_label)])
                                    .set_visible(cx, true);
                            } else {
                                item_widget.label(cx, &[id!(error_label)]).set_text(cx, "");
                                item_widget
                                    .widget(cx, &[id!(error_label)])
                                    .set_visible(cx, false);
                            }

                            let mut show_stats = false;
                            if msg.tokens.is_some() {
                                item_widget
                                    .label(cx, &[id!(tokens_label)])
                                    .set_text(cx, &msg.cached_token_usage);
                                show_stats = true;
                            }
                            if msg.cost.is_some() {
                                item_widget
                                    .label(cx, &[id!(cost_label)])
                                    .set_text(cx, &msg.cached_cost);
                                show_stats = true;
                            }
                            if !msg.steps.is_empty() && msg.show_steps {
                                show_stats = false;
                            }
                            item_widget
                                .view(cx, &[id!(stats_row)])
                                .set_visible(cx, show_stats);

                            let has_steps = !msg.steps.is_empty();
                            item_widget
                                .view(cx, &[id!(steps_summary_row)])
                                .set_visible(cx, has_steps);
                            if has_steps {
                                item_widget
                                    .label(cx, &[id!(steps_summary_label)])
                                    .set_text(cx, &msg.cached_grouped_summary);
                                item_widget
                                    .button(cx, &[id!(steps_button)])
                                    .set_text(cx, Self::steps_button_label(msg));
                            }
                            item_widget
                                .view(cx, &[id!(steps_expanded)])
                                .set_visible(cx, has_steps && msg.show_steps);
                            if has_steps && msg.show_steps {
                                let steps_base = item_widget.view(
                                    cx,
                                    &[id!(steps_expanded), id!(steps_scroll), id!(content)],
                                );
                                for step_id in 0..Self::MAX_STEP_ROWS {
                                    let Some(&(row_id, header_id, body_id, content_id, dot_id, line_id)) = Self::STEP_ROW.get(step_id) else {
                                        continue;
                                    };
                                    if step_id < msg.steps.len() {
                                        let step = &msg.steps[step_id];
                                        let header = if step.expanded {
                                            &step.cached_header_expanded
                                        } else {
                                            &step.cached_header_collapsed
                                        };
                                        steps_base.view(cx, &[row_id]).set_visible(cx, true);
                                        let header_button =
                                            steps_base.view(cx, &[row_id]).button(cx, &[header_id]);
                                        header_button.set_text(cx, header);
                                        steps_base
                                            .view(cx, &[row_id])
                                            .view(cx, &[body_id])
                                            .set_visible(cx, step.expanded);
                                        steps_base
                                            .view(cx, &[row_id])
                                            .label(cx, &[content_id])
                                            .set_text(cx, &step.cached_body);
                                        let _ = dot_id;
                                        let show_line = step_id + 1 < msg.steps.len();
                                        let line_view =
                                            steps_base.view(cx, &[row_id]).view(cx, &[line_id]);
                                        line_view.set_visible(cx, show_line);
                                    } else {
                                        steps_base.view(cx, &[row_id]).set_visible(cx, false);
                                    }
                                }
                            }
                            item_widget
                                .button(cx, &[id!(copy_button)])
                                .set_visible(cx, false);
                            item_widget
                                .button(cx, &[id!(revert_button)])
                                .set_visible(cx, false);
                            item_widget
                                .view(cx, &[id!(msg_actions)])
                                .set_visible(cx, true);
                            let diff_view = item_widget.diff_view(cx, &[id!(diff_view)]);
                            if msg.cached_full_diff.is_empty() {
                                diff_view.clear_diffs(cx);
                            } else {
                                diff_view.set_diff_text(
                                    cx,
                                    &msg.cached_diff_files,
                                    &msg.cached_diff_add,
                                    &msg.cached_diff_del,
                                    &msg.cached_full_diff,
                                );
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
