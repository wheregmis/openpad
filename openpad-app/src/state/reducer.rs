use crate::state::actions::AppAction;
use crate::state::effects::StateEffect;
use crate::state::{AppState, PendingCenterIntent};
use makepad_widgets::LiveId;
use openpad_protocol::{
    AssistantError, AssistantMessage, Message, MessageTime, MessageWithParts, Part,
};
use std::collections::HashMap;

pub fn reduce_app_state(state: &mut AppState, action: &AppAction) -> Vec<StateEffect> {
    let mut effects = Vec::new();

    match action {
        AppAction::Connected => {
            state.connected = true;
            state.error_message = None;
            state.is_working = false;
        }
        AppAction::ConnectionFailed(err) => {
            state.error_message = Some(err.clone());
            state.is_working = false;
        }
        AppAction::HealthUpdated(health) => {
            state.health_ok = Some(health.healthy);
            if health.healthy {
                state.connected = true;
                state.error_message = None;
            } else {
                state.connected = false;
                state.is_working = false;
            }
        }
        AppAction::ProjectsLoaded(projects) => {
            state.projects = projects.clone();
        }
        AppAction::CurrentProjectLoaded(project) => {
            state.current_project = Some(project.clone());
        }
        AppAction::SessionsLoaded(sessions) => {
            state.sessions = sessions.clone();
        }
        AppAction::SessionLoaded(session) => {
            if state.current_session_id.is_none() {
                state.current_session_id = Some(session.id.clone());
            }

            if let Some(existing) = state.find_session_mut(&session.id) {
                *existing = session.clone();
            } else {
                state.sessions.push(session.clone());
            }
        }
        AppAction::SessionCreated(session) => {
            state.current_session_id = Some(session.id.clone());
            state
                .messages_by_session
                .entry(session.id.clone())
                .or_default();

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
        }
        AppAction::MessagesLoaded {
            session_id,
            messages,
        } => {
            state.set_messages_for_session(session_id.clone(), messages.clone());

            let message_id = messages.iter().rev().find_map(|mwp| match &mwp.info {
                openpad_protocol::Message::User(msg) => Some(msg.id.clone()),
                _ => None,
            });
            effects.push(StateEffect::RequestSessionDiff {
                session_id: session_id.clone(),
                message_id,
            });
        }
        AppAction::MessageReceived(message) => {
            reduce_message_received(state, message);
        }
        AppAction::PartReceived { part, delta: _ } => {
            reduce_part_received(state, part);
        }
        AppAction::PendingPermissionsLoaded(permissions) => {
            state.pending_permissions = permissions.clone();
        }
        AppAction::PendingPermissionReceived(request) => {
            enqueue_pending_permission(state, request);
        }
        AppAction::PermissionResponded {
            session_id: _,
            request_id,
            reply: _,
        } => {
            remove_pending_permission(state, request_id);
        }
        AppAction::PermissionDismissed {
            session_id: _,
            request_id,
        } => {
            remove_pending_permission(state, request_id);
        }
        AppAction::SessionErrorReceived { session_id, error } => {
            reduce_session_error(state, session_id, error);
        }
        AppAction::SendMessageFailed(err) => {
            state.error_message = Some(err.clone());
            state.is_working = false;
        }
        AppAction::ProvidersLoaded(providers_response) => {
            state.providers = providers_response.providers.clone();

            let mut provider_labels = vec!["Default".to_string()];
            provider_labels.extend(
                state
                    .providers
                    .iter()
                    .map(|p| p.name.clone()),
            );
            state.provider_labels = provider_labels;
            state.selected_provider_idx = 0;
            state.update_model_list_for_provider();
        }
        AppAction::AgentsLoaded(agents) => {
            state.agents = agents.clone();
            state.selected_agent_idx = None;
        }
        AppAction::SkillsLoaded(skills) => {
            state.skills = skills.clone();
            state.selected_skill_idx = None;
        }
        AppAction::ConfigLoaded(config) => {
            state.config = Some(config.clone());
        }
        _ => {}
    }

    effects
}

fn reduce_message_received(state: &mut AppState, message: &Message) {
    let session_id = message.session_id().to_string();

    if let Message::Assistant(msg) = message {
        let working = msg.time.completed.is_none() && msg.error.is_none();
        state.working_by_session.insert(session_id.clone(), working);
    }

    if state.current_session_id.is_none() {
        state.current_session_id = Some(session_id.clone());
    }

    {
        let session_messages = state
            .messages_by_session
            .entry(session_id.clone())
            .or_default();
        if let Some(existing) = session_messages
            .iter_mut()
            .find(|m| m.info.id() == message.id())
        {
            existing.info = message.clone();
        } else {
            session_messages.push(MessageWithParts {
                info: message.clone(),
                parts: Vec::new(),
            });
        }
    }

    let current_sid = state.current_session_id.as_deref().unwrap_or("");
    if session_id == current_sid {
        if let Message::Assistant(msg) = message {
            state.is_working = msg.time.completed.is_none() && msg.error.is_none();
        }
    }
}

fn reduce_part_received(state: &mut AppState, part: &Part) {
    let Some(msg_id) = part.message_id() else {
        return;
    };

    let mut should_update_work = false;
    let mut work_session_id: Option<String> = None;

    for (_sid, messages) in state.messages_by_session.iter_mut() {
        if let Some(mwp) = messages.iter_mut().find(|m| m.info.id() == msg_id) {
            if matches!(mwp.info, Message::Assistant(_)) {
                should_update_work = true;
                work_session_id = Some(mwp.info.session_id().to_string());
            }

            match part {
                Part::Text { id, .. } if !id.is_empty() => {
                    if let Some(existing) = mwp.parts.iter_mut().find(
                        |p| matches!(p, Part::Text { id: existing_id, .. } if existing_id == id),
                    ) {
                        *existing = part.clone();
                    } else {
                        mwp.parts.push(part.clone());
                    }
                }
                _ => {
                    mwp.parts.push(part.clone());
                }
            };
            break;
        }
    }


    if should_update_work {
        state.is_working = true;
        if let Some(session_id) = work_session_id {
            state.working_by_session.insert(session_id, true);
        }
    }
}

fn enqueue_pending_permission(state: &mut AppState, request: &openpad_protocol::PermissionRequest) {
    if state
        .pending_permissions
        .iter()
        .any(|pending| pending.id == request.id)
    {
        return;
    }
    state.pending_permissions.push(request.clone());
}

fn remove_pending_permission(state: &mut AppState, request_id: &str) {
    state
        .pending_permissions
        .retain(|permission| permission.id != request_id);
}

fn reduce_session_error(state: &mut AppState, session_id: &str, error: &AssistantError) {
    state.is_working = false;
    if state.current_session_id.as_deref() != Some(session_id) {
        return;
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let message_id = format!("err_{}_{}", session_id, now);

    let assistant = AssistantMessage {
        id: message_id.clone(),
        session_id: session_id.to_string(),
        time: MessageTime {
            created: now,
            completed: Some(now),
        },
        error: Some(error.clone()),
        parent_id: String::new(),
        model_id: String::new(),
        provider_id: String::new(),
        mode: String::new(),
        agent: String::new(),
        path: None,
        summary: None,
        cost: 0.0,
        tokens: None,
        structured: None,
        variant: None,
        finish: None,
    };

    let part = Part::Text {
        id: format!("part_{}", message_id),
        session_id: session_id.to_string(),
        message_id: message_id.clone(),
        text: "Session error".to_string(),
        synthetic: None,
        ignored: None,
        time: None,
        metadata: None,
    };

    state
        .messages_by_session
        .entry(session_id.to_string())
        .or_default()
        .push(MessageWithParts {
            info: Message::Assistant(assistant),
            parts: vec![part],
        });
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
    use crate::state::{CenterTabKind, OpenFileState};
    use openpad_protocol::{
        AssistantMessage, Message, MessageTime, MessageWithParts, PermissionRequest, SessionTime,
        UserMessage,
    };

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

    fn assistant_message(session_id: &str, id: &str, completed: Option<i64>) -> Message {
        Message::Assistant(AssistantMessage {
            id: id.to_string(),
            session_id: session_id.to_string(),
            time: MessageTime {
                created: 1,
                completed,
            },
            error: None,
            parent_id: String::new(),
            model_id: String::new(),
            provider_id: String::new(),
            mode: String::new(),
            agent: String::new(),
            path: None,
            summary: None,
            cost: 0.0,
            tokens: None,
            structured: None,
            variant: None,
            finish: None,
        })
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
        assert_eq!(effects.len(), 1);
    }

    #[test]
    fn message_received_updates_working_for_target_session() {
        let mut state = AppState::default();
        state.current_session_id = Some("s1".to_string());

        reduce_app_state(
            &mut state,
            &AppAction::MessageReceived(assistant_message("s1", "a1", None)),
        );

        assert_eq!(state.working_by_session.get("s1").copied(), Some(true));
        assert!(state.is_working);
        assert_eq!(state.messages_for_session("s1").len(), 1);
    }

    #[test]
    fn pending_permission_received_dedupes_by_id() {
        let mut state = AppState::default();
        let request = PermissionRequest {
            id: "perm-1".to_string(),
            session_id: "s1".to_string(),
            permission: "read".to_string(),
            patterns: vec!["*".to_string()],
            metadata: HashMap::new(),
            always: vec![],
            tool: None,
        };

        reduce_app_state(
            &mut state,
            &AppAction::PendingPermissionReceived(request.clone()),
        );
        reduce_app_state(&mut state, &AppAction::PendingPermissionReceived(request));

        assert_eq!(state.pending_permissions.len(), 1);
    }

    #[test]
    fn permission_responded_removes_pending_request() {
        let mut state = AppState::default();
        state.pending_permissions.push(PermissionRequest {
            id: "perm-2".to_string(),
            session_id: "s1".to_string(),
            permission: "edit".to_string(),
            patterns: vec!["*".to_string()],
            metadata: HashMap::new(),
            always: vec![],
            tool: None,
        });

        reduce_app_state(
            &mut state,
            &AppAction::PermissionResponded {
                session_id: "s1".to_string(),
                request_id: "perm-2".to_string(),
                reply: openpad_protocol::PermissionReply::Once,
            },
        );

        assert!(state.pending_permissions.is_empty());
    }

    #[test]
    fn session_loaded_preserves_active_when_already_set() {
        let mut state = AppState::default();
        state.current_session_id = Some("active".to_string());
        let loaded = openpad_protocol::Session {
            id: "loaded".to_string(),
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
        };

        reduce_app_state(&mut state, &AppAction::SessionLoaded(loaded.clone()));

        assert_eq!(state.current_session_id.as_deref(), Some("active"));
        assert!(state.sessions.iter().any(|s| s.id == loaded.id));
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
    fn upsert_session_tab_creates_when_missing() {
        let mut map = HashMap::new();

        let (tab, created) = upsert_session_tab(&mut map, "s2".to_string(), LiveId(31));

        assert_eq!(tab, LiveId(31));
        assert!(created);
        assert_eq!(map.get("s2").copied(), Some(LiveId(31)));
    }

    #[test]
    fn upsert_file_tab_dedupes_existing() {
        let mut map = HashMap::new();
        let existing = LiveId(90);
        map.insert("/tmp/a.rs".to_string(), existing);

        let (tab, created) = upsert_file_tab(&mut map, "/tmp/a.rs".to_string(), LiveId(91));

        assert_eq!(tab, existing);
        assert!(!created);
        assert_eq!(map.len(), 1);
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
        assert!(!state.messages_by_session.contains_key("s1"));
        assert!(!state.working_by_session.contains_key("s1"));
        assert!(!state.tab_by_session.contains_key("s1"));
        assert!(state.sessions.is_empty());
    }

    #[test]
    fn health_update_unhealthy_clears_connected_and_working() {
        let mut state = AppState::default();
        state.connected = true;
        state.is_working = true;

        reduce_app_state(
            &mut state,
            &AppAction::HealthUpdated(openpad_protocol::HealthResponse {
                healthy: false,
                version: String::new(),
            }),
        );

        assert_eq!(state.health_ok, Some(false));
        assert!(!state.connected);
        assert!(!state.is_working);
    }
}
