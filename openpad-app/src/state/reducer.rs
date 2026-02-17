use crate::state::actions::AppAction;
use crate::state::effects::StateEffect;
use crate::state::{AppState, PendingCenterIntent};
use makepad_widgets::LiveId;
use std::collections::HashMap;

pub fn reduce_app_state(state: &mut AppState, action: &AppAction) -> Vec<StateEffect> {
    let mut effects = Vec::new();

    match action {
        AppAction::SessionCreated(session) => {
            state.current_session_id = Some(session.id.clone());
            state.messages_data.clear();
            state.messages_by_session.entry(session.id.clone()).or_default();

            if !state.sessions.iter().any(|s| s.id == session.id) {
                state.sessions.push(session.clone());
            }

            if let Some(project) = state.projects.iter().find(|p| p.id == session.project_id) {
                state.current_project = Some(project.clone());
            }
        }
        AppAction::SessionDeleted(session_id) => {
            if state.current_session_id.as_deref() == Some(session_id) {
                state.current_session_id = None;
                state.selected_session_id = None;
                state.is_working = false;
                state.messages_data.clear();
            } else if state.selected_session_id.as_deref() == Some(session_id) {
                state.selected_session_id = None;
            }

            state.messages_by_session.remove(session_id);
            state.working_by_session.remove(session_id);
            state.tab_by_session.remove(session_id);
            state.sessions.retain(|s| s.id != *session_id);
        }
        AppAction::SessionUpdated(session) => {
            if let Some(existing) = state.find_session_mut(&session.id) {
                *existing = session.clone();
            }
        }
        AppAction::SessionDiffLoaded { session_id, diffs } => {
            if let Some(existing) = state.find_session_mut(session_id) {
                let summary =
                    existing
                        .summary
                        .get_or_insert_with(|| openpad_protocol::SessionSummary {
                            additions: 0,
                            deletions: 0,
                            files: diffs.len() as i64,
                            diffs: Vec::new(),
                        });
                summary.diffs = diffs.clone();
            }

            if state.current_session_id.as_deref() == Some(session_id.as_str()) {
                state.messages_data = state.messages_for_session(session_id).to_vec();
            }
        }
        AppAction::MessagesLoaded {
            session_id,
            messages,
        } => {
            state.set_messages_for_session(session_id.clone(), messages.clone());
            if state.current_session_id.as_deref() == Some(session_id.as_str()) {
                state.messages_data = messages.clone();
            }

            let message_id = messages.iter().rev().find_map(|mwp| match &mwp.info {
                openpad_protocol::Message::User(msg) => Some(msg.id.clone()),
                _ => None,
            });
            effects.push(StateEffect::RequestSessionDiff {
                session_id: session_id.clone(),
                message_id,
            });
        }
        AppAction::PendingPermissionsLoaded(permissions) => {
            state.pending_permissions = permissions.clone();
        }
        _ => {}
    }

    effects
}

pub fn upsert_session_tab(
    tab_by_session: &mut HashMap<String, LiveId>,
    session_id: String,
    new_tab_id: LiveId,
) -> (LiveId, bool) {
    if let Some(existing) = tab_by_session.get(&session_id).copied() {
        (existing, false)
    } else {
        tab_by_session.insert(session_id, new_tab_id);
        (new_tab_id, true)
    }
}

pub fn upsert_file_tab(
    tab_by_file: &mut HashMap<String, LiveId>,
    absolute_path: String,
    new_tab_id: LiveId,
) -> (LiveId, bool) {
    if let Some(existing) = tab_by_file.get(&absolute_path).copied() {
        (existing, false)
    } else {
        tab_by_file.insert(absolute_path, new_tab_id);
        (new_tab_id, true)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnsavedDecision {
    Save,
    Discard,
    Cancel,
}

pub fn resolve_pending_center_intent(
    pending_intent: Option<PendingCenterIntent>,
    decision: UnsavedDecision,
) -> Option<PendingCenterIntent> {
    match decision {
        UnsavedDecision::Save | UnsavedDecision::Discard => pending_intent,
        UnsavedDecision::Cancel => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::handlers::{CenterTabKind, OpenFileState};
    use openpad_protocol::{Message, MessageTime, MessageWithParts, SessionTime, UserMessage};

    fn user_message(session_id: &str, id: &str) -> MessageWithParts {
        MessageWithParts {
            info: Message::User(UserMessage {
                id: id.to_string(),
                session_id: session_id.to_string(),
                time: MessageTime {
                    created: 1,
                    completed: Some(1),
                },
                summary: None,
                format: None,
                agent: String::new(),
                model: None,
                system: None,
                tools: None,
                variant: None,
            }),
            parts: vec![],
        }
    }

    #[test]
    fn messages_loaded_routes_to_target_session_cache() {
        let mut state = AppState::default();
        state.current_session_id = Some("s1".to_string());
        state
            .messages_by_session
            .insert("s1".to_string(), vec![user_message("s1", "m1")]);

        let action = AppAction::MessagesLoaded {
            session_id: "s2".to_string(),
            messages: vec![user_message("s2", "m2")],
        };

        let effects = reduce_app_state(&mut state, &action);

        assert_eq!(state.messages_for_session("s1").len(), 1);
        assert_eq!(state.messages_for_session("s2").len(), 1);
        assert_eq!(state.messages_data.len(), 0);
        assert_eq!(effects.len(), 1);
    }

    #[test]
    fn upsert_session_tab_dedupes_existing() {
        let mut map = HashMap::new();
        let existing = LiveId(11);
        map.insert("s1".to_string(), existing);

        let (tab, created) = upsert_session_tab(&mut map, "s1".to_string(), LiveId(12));

        assert_eq!(tab, existing);
        assert!(!created);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn upsert_file_tab_creates_when_missing() {
        let mut map = HashMap::new();

        let (tab, created) = upsert_file_tab(&mut map, "/tmp/a.rs".to_string(), LiveId(77));

        assert_eq!(tab, LiveId(77));
        assert!(created);
        assert_eq!(map.get("/tmp/a.rs").copied(), Some(LiveId(77)));
    }

    #[test]
    fn pending_intent_resolution_respects_cancel() {
        let intent = Some(PendingCenterIntent::CloseTab { tab_id: LiveId(9) });
        assert!(resolve_pending_center_intent(intent.clone(), UnsavedDecision::Cancel).is_none());
        assert!(matches!(
            resolve_pending_center_intent(intent, UnsavedDecision::Save),
            Some(PendingCenterIntent::CloseTab { tab_id: LiveId(9) })
        ));
    }

    #[test]
    fn session_deleted_cleans_session_maps() {
        let mut state = AppState::default();
        state.current_session_id = Some("s1".to_string());
        state.selected_session_id = Some("s1".to_string());
        state.messages_data = vec![user_message("s1", "m1")];
        state
            .messages_by_session
            .insert("s1".to_string(), vec![user_message("s1", "m1")]);
        state.working_by_session.insert("s1".to_string(), true);
        state.tab_by_session.insert("s1".to_string(), LiveId(2));
        state.sessions.push(openpad_protocol::Session {
            id: "s1".to_string(),
            slug: String::new(),
            project_id: "p".to_string(),
            directory: "/tmp".to_string(),
            parent_id: None,
            title: String::new(),
            version: String::new(),
            time: SessionTime {
                created: 1,
                updated: 1,
                compacting: None,
                archived: None,
            },
            summary: None,
            share: None,
            permission: None,
            revert: None,
        });
        state.center_tabs_by_id.insert(
            LiveId(100),
            CenterTabKind::File {
                open_file: OpenFileState {
                    project_id: "p".to_string(),
                    absolute_path: "/tmp/a.rs".to_string(),
                    display_name: "a.rs".to_string(),
                    text_cache: "".to_string(),
                    last_saved_revision: 0,
                },
            },
        );

        reduce_app_state(&mut state, &AppAction::SessionDeleted("s1".to_string()));

        assert_eq!(state.current_session_id, None);
        assert_eq!(state.selected_session_id, None);
        assert!(state.messages_data.is_empty());
        assert!(!state.messages_by_session.contains_key("s1"));
        assert!(!state.working_by_session.contains_key("s1"));
        assert!(!state.tab_by_session.contains_key("s1"));
        assert!(state.sessions.is_empty());
    }
}
