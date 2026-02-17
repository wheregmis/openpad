use super::*;

impl App {
    /// Update the attachments preview UI
    pub(super) fn update_attachments_ui(&self, cx: &mut Cx) {
        let has_attachments = !self.state.attached_files.is_empty();
        self.ui
            .view(cx, &[id!(attachments_preview)])
            .set_visible(cx, has_attachments);

        if has_attachments {
            let filenames: Vec<String> = self
                .state
                .attached_files
                .iter()
                .map(|f| f.filename.clone())
                .collect();
            let text = filenames.join(", ");
            self.ui
                .label(cx, &[id!(attachments_list)])
                .set_text(cx, &text);
        }
        self.ui.redraw(cx);
    }

    pub(super) fn update_skill_ui(&self, cx: &mut Cx) {
        let selected = self.state.selected_skill();
        let has_skill = selected.is_some();
        self.ui
            .view(cx, &[id!(skill_preview)])
            .set_visible(cx, has_skill);

        if let Some(skill) = selected {
            self.ui
                .label(cx, &[id!(skill_name_label)])
                .set_text(cx, &skill.name);
            self.ui
                .label(cx, &[id!(skill_desc_label)])
                .set_text(cx, &skill.description);
        }
        self.ui.redraw(cx);
    }

    pub(super) fn set_top_surfaces_for_active_kind(
        &self,
        cx: &mut Cx,
        kind: Option<&CenterTabKind>,
    ) {
        match kind {
            Some(CenterTabKind::Chat { .. }) => {
                self.ui.view(cx, &[id!(session_info)]).set_visible(cx, true);
                self.ui.view(cx, &[id!(editor_info)]).set_visible(cx, false);
                self.ui
                    .view(cx, &[id!(chat_composer)])
                    .set_visible(cx, true);
            }
            Some(CenterTabKind::File { .. }) => {
                self.ui
                    .view(cx, &[id!(session_info)])
                    .set_visible(cx, false);
                self.ui.view(cx, &[id!(editor_info)]).set_visible(cx, true);
                self.ui
                    .view(cx, &[id!(chat_composer)])
                    .set_visible(cx, false);
            }
            Some(CenterTabKind::Home) | None => {
                self.ui
                    .view(cx, &[id!(session_info)])
                    .set_visible(cx, false);
                self.ui.view(cx, &[id!(editor_info)]).set_visible(cx, false);
                self.ui
                    .view(cx, &[id!(chat_composer)])
                    .set_visible(cx, false);
            }
        }
    }

    pub(super) fn active_center_kind(&self) -> Option<&CenterTabKind> {
        self.state
            .active_center_tab
            .and_then(|tab_id| self.state.center_tabs_by_id.get(&tab_id))
    }

    pub(super) fn current_active_file_tab_id(&self) -> Option<LiveId> {
        let tab_id = self.state.active_center_tab?;
        match self.state.center_tabs_by_id.get(&tab_id) {
            Some(CenterTabKind::File { .. }) => Some(tab_id),
            _ => None,
        }
    }

    pub(super) fn update_editor_header_ui_for_tab(&self, cx: &mut Cx, tab_id: LiveId) {
        let Some(CenterTabKind::File { open_file }) = self.state.center_tabs_by_id.get(&tab_id)
        else {
            return;
        };
        let item = self.center_dock(cx).item(tab_id);
        self.ui
            .label(cx, &[id!(editor_file_label)])
            .set_text(cx, &open_file.absolute_path);
        let current_text = item.editor_panel(cx, &[id!(editor_panel)]).get_text();
        let is_dirty = current_text != open_file.text_cache;
        self.ui
            .label(cx, &[id!(editor_dirty_dot)])
            .set_text(cx, if is_dirty { "●" } else { "" });
    }

    pub(super) fn render_chat_tab(&mut self, cx: &mut Cx, tab_id: LiveId, session_id: &str) {
        let item = self.center_dock(cx).item(tab_id);
        let messages = self
            .state
            .messages_by_session
            .get(session_id)
            .cloned()
            .unwrap_or_default();
        let revert = self.state.current_revert_message_id_for_session(session_id);
        item.message_list(cx, &[id!(message_list)])
            .set_messages(cx, &messages, revert);
        let working = self
            .state
            .working_by_session
            .get(session_id)
            .copied()
            .unwrap_or(false);
        item.message_list(cx, &[id!(message_list)])
            .set_working(cx, working);

        let displays: Vec<openpad_widgets::message_list::PendingPermissionDisplay> = self
            .state
            .pending_permissions
            .iter()
            .filter(|p| p.session_id == session_id)
            .map(
                |p| openpad_widgets::message_list::PendingPermissionDisplay {
                    session_id: p.session_id.clone(),
                    request_id: p.id.clone(),
                    permission: p.permission.clone(),
                    patterns: p.patterns.clone(),
                },
            )
            .collect();
        item.message_list(cx, &[id!(message_list)])
            .set_pending_permissions(cx, &displays);

        if let Some(session) = self.state.find_session(session_id) {
            if let Some(summary) = &session.summary {
                item.view(cx, &[id!(session_summary)]).set_visible(cx, true);
                item.label(cx, &[id!(summary_stats_label)]).set_text(
                    cx,
                    &format!(
                        "{} files, +{}, -{}",
                        summary.files, summary.additions, summary.deletions
                    ),
                );
                item.message_list(cx, &[id!(message_list)])
                    .set_session_diffs(cx, &summary.diffs);
            } else {
                item.view(cx, &[id!(session_summary)])
                    .set_visible(cx, false);
                item.message_list(cx, &[id!(message_list)])
                    .set_session_diffs(cx, &[]);
            }
        } else {
            item.view(cx, &[id!(session_summary)])
                .set_visible(cx, false);
            item.message_list(cx, &[id!(message_list)])
                .set_session_diffs(cx, &[]);
        }
    }

    pub(super) fn refresh_open_center_tabs(&mut self, cx: &mut Cx) {
        let tabs: Vec<(LiveId, CenterTabKind)> = self
            .state
            .center_tabs_by_id
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
        for (tab_id, kind) in tabs {
            match kind {
                CenterTabKind::Chat { session_id } => self.render_chat_tab(cx, tab_id, &session_id),
                CenterTabKind::File { .. } => self.update_editor_header_ui_for_tab(cx, tab_id),
                CenterTabKind::Home => {}
            }
        }
    }

    pub(super) fn sync_active_center_ui(&mut self, cx: &mut Cx) {
        let active_kind = self.active_center_kind().cloned();
        self.set_top_surfaces_for_active_kind(cx, active_kind.as_ref());
        match active_kind {
            Some(CenterTabKind::Chat { session_id }) => {
                self.state.current_session_id = Some(session_id.clone());
                self.state.selected_session_id = Some(session_id.clone());
                self.state.messages_data = self
                    .state
                    .messages_by_session
                    .get(&session_id)
                    .cloned()
                    .unwrap_or_default();
                self.state.update_files_panel(&self.ui, cx);
                self.state.update_sessions_panel(&self.ui, cx);
                self.state.update_session_title_ui(&self.ui, cx);
                self.state.update_project_context_ui(&self.ui, cx);
                self.state.update_session_meta_ui(&self.ui, cx);
                let working = self
                    .state
                    .working_by_session
                    .get(&session_id)
                    .copied()
                    .unwrap_or(false);
                crate::ui::state_updates::update_work_indicator(&self.ui, cx, working);
            }
            Some(CenterTabKind::File { .. }) => {
                self.state.current_session_id = None;
                crate::ui::state_updates::update_work_indicator(&self.ui, cx, false);
                if let Some(tab_id) = self.state.active_center_tab {
                    self.update_editor_header_ui_for_tab(cx, tab_id);
                }
            }
            Some(CenterTabKind::Home) | None => {
                self.state.current_session_id = None;
                crate::ui::state_updates::update_work_indicator(&self.ui, cx, false);
            }
        }
    }
}
