use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum TailMode {
    #[default]
    Follow,
    Detached,
}

impl TailMode {
    pub(crate) fn from_is_at_end(is_at_end: bool) -> Self {
        if is_at_end {
            Self::Follow
        } else {
            Self::Detached
        }
    }

    pub(crate) fn after_set_messages(current: Self, had_messages: bool) -> Self {
        if had_messages {
            current
        } else {
            Self::Follow
        }
    }
}

impl MessageList {
    pub(super) const MAX_STEP_ROWS: usize = 10;

    pub(super) fn thinking_icon(&self) -> &'static str {
        match self.thinking_frame {
            0 => "◐",
            1 => "◑",
            2 => "◒",
            3 => "◓",
            4 => "◔",
            _ => "◕",
        }
    }

    pub(super) fn steps_button_label(msg: &DisplayMessage) -> &'static str {
        if msg.show_steps {
            "▾ Details"
        } else {
            "▸ Details"
        }
    }

    fn total_items(&self) -> usize {
        self.messages.len() + self.pending_permissions.len() + if self.is_working { 1 } else { 0 }
    }

    pub(super) fn tail_to_end(&mut self, cx: &mut Cx) {
        let total = self.total_items();
        if total == 0 {
            return;
        }
        let list = self.view.portal_list(cx, &[id!(list)]);
        list.set_tail_range(true);
        list.set_first_id_and_scroll(total.saturating_sub(1), 0.0);
    }

    pub(super) fn update_cached_indices(&mut self) {
        self.cached_last_assistant_idx = self.messages.iter().rposition(|m| m.role == "assistant");

        self.cached_last_assistant_has_running = if let Some(idx) = self.cached_last_assistant_idx {
            self.messages[idx].steps.iter().any(|s| s.has_running)
        } else {
            false
        };
    }

    pub(crate) fn should_follow_tail(&self) -> bool {
        self.tail_mode == TailMode::Follow
    }
}

#[cfg(test)]
mod tests {
    use super::TailMode;

    #[test]
    fn tail_mode_transitions_from_end_state() {
        assert_eq!(TailMode::from_is_at_end(true), TailMode::Follow);
        assert_eq!(TailMode::from_is_at_end(false), TailMode::Detached);
    }

    #[test]
    fn detached_mode_is_preserved_on_message_replace_for_existing_list() {
        assert_eq!(
            TailMode::after_set_messages(TailMode::Detached, true),
            TailMode::Detached
        );
        assert_eq!(
            TailMode::after_set_messages(TailMode::Detached, false),
            TailMode::Follow
        );
    }
}
