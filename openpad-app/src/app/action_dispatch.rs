use super::*;

impl App {
    pub(super) fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        let mut saw_app_action = false;
        for action in actions {
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                saw_app_action = true;
                match app_action {
                    AppAction::SessionCreated(session) => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.queue_or_select_session(cx, session.id.clone());
                    }
                    AppAction::SessionDeleted(session_id) => {
                        let tab_to_close = self.state.tab_by_session.get(session_id).copied();
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        if let Some(tab_id) = tab_to_close {
                            self.close_tab_now(cx, tab_id);
                        } else {
                            self.sync_active_center_ui(cx);
                        }
                    }
                    AppAction::OpenCodeEvent(oc_event) => {
                        let deleted_tab_id = match oc_event {
                            openpad_protocol::Event::SessionDeleted(session) => {
                                self.state.tab_by_session.get(&session.id).copied()
                            }
                            _ => None,
                        };
                        state::handle_opencode_event(&mut self.state, &self.ui, cx, oc_event);
                        if let Some(tab_id) = deleted_tab_id {
                            self.close_tab_now(cx, tab_id);
                        }
                    }
                    AppAction::PermissionResponded {
                        session_id,
                        request_id,
                        reply,
                    } => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.respond_to_permission(
                            cx,
                            session_id.clone(),
                            request_id.clone(),
                            reply.clone(),
                        );
                    }
                    AppAction::RevertToMessage {
                        session_id,
                        message_id,
                    } => {
                        self.revert_to_message(cx, session_id.clone(), message_id.clone());
                    }
                    AppAction::UnrevertSession(session_id) => {
                        self.unrevert_session(cx, session_id.clone());
                    }
                    AppAction::RequestSessionDiff {
                        session_id,
                        message_id,
                    } => {
                        self.load_session_diff(cx, session_id.clone(), message_id.clone());
                    }
                    AppAction::DialogConfirmed { dialog_type, value } => {
                        self.handle_dialog_confirmed(cx, dialog_type.clone(), value.clone());
                    }
                    AppAction::Connected => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.load_providers_and_agents();
                    }
                    AppAction::ProjectsLoaded(projects) => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.load_all_sessions(projects.clone());
                    }
                    _ => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                    }
                }
            }
        }
        if saw_app_action {
            self.refresh_open_center_tabs(cx);
            self.sync_active_center_ui(cx);
        }
    }

    pub(super) fn handle_dialog_confirmed(
        &mut self,
        cx: &mut Cx,
        dialog_type: String,
        value: String,
    ) {
        if dialog_type == "unsaved_editor" {
            let saved = if let Some(tab_id) = self.current_active_file_tab_id() {
                self.save_file_tab(cx, tab_id)
            } else {
                true
            };
            if saved {
                self.run_pending_center_intent(cx);
            } else {
                self.state.pending_center_intent = None;
            }
            return;
        }

        // Parse the dialog_type which is in format "action:data"
        let Some((action, data)) = dialog_type.split_once(':') else {
            return;
        };

        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        let directory = self.get_session_directory(data);

        match action {
            "delete_session" => {
                async_runtime::spawn_session_deleter(runtime, client, data.to_string(), directory);
            }
            "rename_session" => {
                if !value.is_empty() {
                    async_runtime::spawn_session_updater(
                        runtime,
                        client,
                        data.to_string(),
                        value,
                        directory,
                    );
                }
            }
            "set_auth" => {
                async_runtime::spawn_auth_setter(runtime, client, data.to_string(), value);
            }
            _ => {}
        }
    }

    pub(super) fn handle_dialog_secondary(&mut self, cx: &mut Cx, dialog_type: String) {
        if dialog_type == "unsaved_editor" {
            if let Some(tab_id) = self.current_active_file_tab_id() {
                self.discard_file_tab_changes(cx, tab_id);
            }
            self.run_pending_center_intent(cx);
        }
    }

    pub(super) fn handle_dialog_cancelled(&mut self, _cx: &mut Cx) {
        self.state.pending_center_intent = None;
    }
}
