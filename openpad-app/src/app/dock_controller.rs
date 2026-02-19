use super::*;

impl App {
    pub(super) fn center_dock(&self, cx: &mut Cx) -> DockRef {
        self.ui.dock(cx, &[id!(center_dock)])
    }

    pub(super) fn has_unsaved_file_tab_changes(&self, cx: &mut Cx, tab_id: LiveId) -> bool {
        let Some(CenterTabKind::File { open_file }) = self.state.center_tabs_by_id.get(&tab_id)
        else {
            return false;
        };
        let item = self.center_dock(cx).item(tab_id);
        let current_text = item.editor_panel(cx, &[id!(editor_panel)]).get_text();
        current_text != open_file.text_cache
    }

    pub(super) fn save_file_tab(&mut self, cx: &mut Cx, tab_id: LiveId) -> bool {
        let Some(CenterTabKind::File { open_file }) =
            self.state.center_tabs_by_id.get(&tab_id).cloned()
        else {
            return false;
        };
        let item = self.center_dock(cx).item(tab_id);
        let text = item.editor_panel(cx, &[id!(editor_panel)]).get_text();
        if text == open_file.text_cache {
            return true;
        }
        if let Err(err) = std::fs::write(&open_file.absolute_path, text.as_bytes()) {
            self.state.error_message = Some(format!(
                "Failed to save {}: {}",
                open_file.absolute_path, err
            ));
            crate::ui::state_updates::set_status_error(
                &self.ui,
                cx,
                &format!("save failed: {}", err),
            );
            return false;
        }
        if let Some(CenterTabKind::File { open_file }) =
            self.state.center_tabs_by_id.get_mut(&tab_id)
        {
            open_file.text_cache = text;
            open_file.last_saved_revision = open_file.last_saved_revision.saturating_add(1);
        }
        self.update_editor_header_ui_for_tab(cx, tab_id);
        true
    }

    pub(super) fn discard_file_tab_changes(&mut self, cx: &mut Cx, tab_id: LiveId) {
        let Some(CenterTabKind::File { open_file }) =
            self.state.center_tabs_by_id.get(&tab_id).cloned()
        else {
            return;
        };
        let item = self.center_dock(cx).item(tab_id);
        item.editor_panel(cx, &[id!(editor_panel)])
            .set_text(cx, &open_file.text_cache);
        self.update_editor_header_ui_for_tab(cx, tab_id);
    }

    pub(super) fn show_unsaved_editor_dialog(&self, cx: &mut Cx) {
        self.ui
            .simple_dialog(cx, &[id!(simple_dialog)])
            .show_confirm_with_secondary(
                cx,
                "Unsaved changes",
                "You have unsaved changes in the open file.",
                "Save",
                "Discard",
                "Cancel",
                "unsaved_editor".to_string(),
            );
    }

    pub(super) fn queue_or_open_file(
        &mut self,
        cx: &mut Cx,
        project_id: String,
        absolute_path: String,
    ) {
        if let Some(tab_id) = self.current_active_file_tab_id() {
            if self.has_unsaved_file_tab_changes(cx, tab_id) {
                self.state.pending_center_intent = Some(PendingCenterIntent::OpenFile {
                    project_id,
                    absolute_path,
                });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.open_file_now(cx, project_id, absolute_path);
    }

    pub(super) fn queue_or_select_session(&mut self, cx: &mut Cx, session_id: String) {
        if let Some(tab_id) = self.current_active_file_tab_id() {
            if self.has_unsaved_file_tab_changes(cx, tab_id) {
                self.state.pending_center_intent =
                    Some(PendingCenterIntent::OpenSession { session_id });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.select_session_now(cx, session_id);
    }

    pub(super) fn queue_or_switch_tab(&mut self, cx: &mut Cx, tab_id: LiveId) {
        if let Some(active_file_tab_id) = self.current_active_file_tab_id() {
            if active_file_tab_id != tab_id
                && self.has_unsaved_file_tab_changes(cx, active_file_tab_id)
            {
                self.state.pending_center_intent = Some(PendingCenterIntent::SwitchTab { tab_id });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.activate_center_tab(cx, tab_id);
    }

    pub(super) fn queue_or_close_tab(&mut self, cx: &mut Cx, tab_id: LiveId) {
        if tab_id == live_id!(center_home_tab) {
            return;
        }
        if let Some(CenterTabKind::File { .. }) = self.state.center_tabs_by_id.get(&tab_id) {
            if self.has_unsaved_file_tab_changes(cx, tab_id) {
                self.state.pending_center_intent = Some(PendingCenterIntent::CloseTab { tab_id });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.close_tab_now(cx, tab_id);
    }

    pub(super) fn close_tab_now(&mut self, cx: &mut Cx, tab_id: LiveId) {
        let tab_bar_id = self
            .center_dock(cx)
            .find_tab_bar_of_tab(tab_id)
            .map(|(tab_bar, _)| tab_bar);

        let kind = self.state.center_tabs_by_id.remove(&tab_id);
        if let Some(kind) = kind {
            match kind {
                CenterTabKind::Chat { session_id } => {
                    self.state.tab_by_session.remove(&session_id);
                }
                CenterTabKind::File { open_file } => {
                    self.state.tab_by_file.remove(&open_file.absolute_path);
                }
                CenterTabKind::Home => {}
            }
        }
        self.center_dock(cx).close_tab(cx, tab_id);
        if self.state.active_center_tab == Some(tab_id) {
            let mut fallback = None;
            if let Some(tab_bar_id) = tab_bar_id {
                if let Some(dock_items) = self.center_dock(cx).clone_state() {
                    if let Some(DockItem::Tabs { tabs, selected, .. }) = dock_items.get(&tab_bar_id)
                    {
                        fallback = tabs.get(*selected).copied();
                    }
                }
            }
            if fallback.is_none() {
                fallback = Some(live_id!(center_home_tab));
            }
            if let Some(fallback_tab) = fallback {
                self.state.active_center_tab = Some(fallback_tab);
                self.center_dock(cx).select_tab(cx, fallback_tab);
            }
            self.sync_active_center_ui(cx);
        }
    }

    pub(super) fn activate_center_tab(&mut self, cx: &mut Cx, tab_id: LiveId) {
        self.center_dock(cx).select_tab(cx, tab_id);
        self.state.active_center_tab = Some(tab_id);
        self.sync_active_center_ui(cx);
        if let Some(kind) = self.state.center_tabs_by_id.get(&tab_id).cloned() {
            match kind {
                CenterTabKind::Chat { session_id } => self.render_chat_tab(cx, tab_id, &session_id),
                CenterTabKind::File { .. } => self.update_editor_header_ui_for_tab(cx, tab_id),
                CenterTabKind::Home => {}
            }
        }
    }

    pub(super) fn open_file_now(&mut self, cx: &mut Cx, project_id: String, absolute_path: String) {
        if let Some(existing_tab_id) = self.state.tab_by_file.get(&absolute_path).copied() {
            self.activate_center_tab(cx, existing_tab_id);
            return;
        }

        let path = Path::new(&absolute_path);
        if !path.is_file() {
            return;
        }

        let Ok(bytes) = std::fs::read(path) else {
            return;
        };
        if is_probably_binary(&bytes) {
            return;
        }

        let Ok(content) = String::from_utf8(bytes) else {
            return;
        };

        let display_name = std::path::Path::new(&absolute_path)
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or(&absolute_path)
            .to_string();
        let open_file = OpenFileState {
            project_id,
            absolute_path: absolute_path.clone(),
            display_name: display_name.clone(),
            text_cache: content.clone(),
            last_saved_revision: 0,
        };

        let dock = self.center_dock(cx);
        let (tab_bar, pos) = dock
            .find_tab_bar_of_tab(live_id!(center_home_tab))
            .unwrap_or((live_id!(root), 0));
        let tab_id = dock.unique_id(LiveId::from_str(&format!("file:{}", absolute_path)).0);
        let _ = dock.create_and_select_tab(
            cx,
            tab_bar,
            tab_id,
            live_id!(CenterCodeTab),
            display_name,
            live_id!(CloseableTab),
            Some(pos),
        );

        self.state.center_tabs_by_id.insert(
            tab_id,
            CenterTabKind::File {
                open_file: open_file.clone(),
            },
        );
        self.state.tab_by_file.insert(absolute_path, tab_id);

        let item = dock.item(tab_id);
        item.editor_panel(cx, &[id!(editor_panel)])
            .set_read_only(cx, false);
        item.editor_panel(cx, &[id!(editor_panel)])
            .set_text(cx, &content);
        item.editor_panel(cx, &[id!(editor_panel)]).focus_editor(cx);
        self.update_editor_header_ui_for_tab(cx, tab_id);
        self.activate_center_tab(cx, tab_id);
    }

    pub(super) fn select_session_now(&mut self, cx: &mut Cx, session_id: String) {
        self.state.selected_session_id = Some(session_id.clone());

        if let Some(tab_id) = self.state.tab_by_session.get(&session_id).copied() {
            self.activate_center_tab(cx, tab_id);
            self.load_pending_permissions();
            return;
        }

        let title = self
            .state
            .find_session(&session_id)
            .map(async_runtime::get_session_title)
            .unwrap_or_else(|| "Session".to_string());

        let dock = self.center_dock(cx);
        let (tab_bar, pos) = dock
            .find_tab_bar_of_tab(live_id!(center_home_tab))
            .unwrap_or((live_id!(root), 0));
        let tab_id = dock.unique_id(LiveId::from_str(&format!("chat:{}", session_id)).0);
        let _ = dock.create_and_select_tab(
            cx,
            tab_bar,
            tab_id,
            live_id!(CenterChatTab),
            title,
            live_id!(CloseableTab),
            Some(pos),
        );

        self.state.center_tabs_by_id.insert(
            tab_id,
            CenterTabKind::Chat {
                session_id: session_id.clone(),
            },
        );
        self.state.tab_by_session.insert(session_id.clone(), tab_id);
        self.state.active_center_tab = Some(tab_id);

        self.sync_active_center_ui(cx);
        self.render_chat_tab(cx, tab_id, &session_id);

        if !self.state.messages_by_session.contains_key(&session_id) {
            self.load_messages(session_id.clone());
        }
        self.load_pending_permissions();
    }

    pub(super) fn run_pending_center_intent(&mut self, cx: &mut Cx) {
        let Some(intent) = self.state.pending_center_intent.clone() else {
            return;
        };
        self.state.pending_center_intent = None;
        match intent {
            PendingCenterIntent::OpenFile {
                project_id,
                absolute_path,
            } => self.open_file_now(cx, project_id, absolute_path),
            PendingCenterIntent::OpenSession { session_id } => {
                self.select_session_now(cx, session_id)
            }
            PendingCenterIntent::SwitchTab { tab_id } => self.activate_center_tab(cx, tab_id),
            PendingCenterIntent::CloseTab { tab_id } => self.close_tab_now(cx, tab_id),
        }
    }
}
