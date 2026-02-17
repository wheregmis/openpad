use super::*;

impl MessageList {
    pub(crate) fn handle_event_impl(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.is_working {
            if let Event::NextFrame(_) = event {
                // Optimization: throttle redraw frequency from ~60fps to ~10fps
                // This significantly reduces CPU usage during the "thinking" state
                self.frame_count += 1;
                if self.frame_count.is_multiple_of(6) {
                    self.thinking_frame = (self.thinking_frame + 1) % 6;
                    self.redraw(cx);
                }
            }
            cx.new_next_frame();
        }

        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        let list = self.view.portal_list(cx, &[id!(list)]);
        if list.scrolled(&actions) {
            let at_end = list.is_at_end();
            self.tail_mode = TailMode::from_is_at_end(at_end);
            list.set_tail_range(at_end);
        }
        for (item_id, widget) in list.items_with_actions(&actions) {
            if item_id >= self.messages.len() {
                continue;
            }

            if widget
                .button(cx, &[id!(copy_action_button)])
                .clicked(&actions)
                || widget.button(cx, &[id!(copy_button)]).clicked(&actions)
            {
                cx.copy_to_clipboard(&self.messages[item_id].text);
            }

            if widget
                .button(cx, &[id!(revert_action_button)])
                .clicked(&actions)
                || widget.button(cx, &[id!(revert_button)]).clicked(&actions)
            {
                if let Some(message_id) = &self.messages[item_id].message_id {
                    cx.action(MessageListAction::RevertToMessage(message_id.clone()));
                }
            }

            if widget.button(cx, &[id!(steps_button)]).clicked(&actions) {
                if let Some(message) = self.messages.get_mut(item_id) {
                    if !message.steps.is_empty() {
                        message.show_steps = !message.show_steps;
                        self.redraw(cx);
                    }
                }
            }

            if item_id < self.messages.len() {
                let msg = &self.messages[item_id];
                if !msg.diffs.is_empty()
                    && widget
                        .diff_view(cx, &[id!(diff_view)])
                        .summary_header_clicked(cx)
                {
                    if let Some(message) = self.messages.get_mut(item_id) {
                        message.show_diffs = !message.show_diffs;
                        self.redraw(cx);
                    }
                }
            }

            if item_id < self.messages.len() {
                let msg = &self.messages[item_id];
                if msg.role == "assistant" && msg.show_steps && !msg.steps.is_empty() {
                    let steps_base =
                        widget.view(cx, &[id!(steps_expanded), id!(steps_scroll), id!(content)]);
                    for step_id in 0..MessageList::MAX_STEP_ROWS.min(msg.steps.len()) {
                        let (row_id, header_id) = match step_id {
                            0 => (live_id!(step_row_0), live_id!(step_row_0_header)),
                            1 => (live_id!(step_row_1), live_id!(step_row_1_header)),
                            2 => (live_id!(step_row_2), live_id!(step_row_2_header)),
                            3 => (live_id!(step_row_3), live_id!(step_row_3_header)),
                            4 => (live_id!(step_row_4), live_id!(step_row_4_header)),
                            5 => (live_id!(step_row_5), live_id!(step_row_5_header)),
                            6 => (live_id!(step_row_6), live_id!(step_row_6_header)),
                            7 => (live_id!(step_row_7), live_id!(step_row_7_header)),
                            8 => (live_id!(step_row_8), live_id!(step_row_8_header)),
                            9 => (live_id!(step_row_9), live_id!(step_row_9_header)),
                            _ => continue,
                        };
                        if steps_base
                            .view(cx, &[row_id])
                            .button(cx, &[header_id])
                            .clicked(&actions)
                        {
                            if let Some(step) = self.messages[item_id].steps.get_mut(step_id) {
                                step.expanded = !step.expanded;
                                self.redraw(cx);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}
