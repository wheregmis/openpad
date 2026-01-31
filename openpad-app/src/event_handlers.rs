use crate::actions::AppAction;
use crate::components::message_list::MessageListWidgetRefExt;
use crate::components::projects_panel::ProjectsPanelWidgetRefExt;
use crate::constants::*;
use crate::network;
use crate::ui_state;
use makepad_widgets::*;
use openpad_protocol::{Event as OcEvent, MessageWithParts, Project, Session};

/// Data structure holding application state for event handling
#[derive(Default)]
pub struct AppState {
    pub messages_data: Vec<MessageWithParts>,
    pub projects: Vec<Project>,
    pub sessions: Vec<Session>,
    pub current_project: Option<Project>,
    pub selected_session_id: Option<String>,
    pub current_session_id: Option<String>,
    pub connected: bool,
    pub health_ok: Option<bool>,
    pub error_message: Option<String>,
}

impl AppState {
    /// Determines the session title to display
    pub fn get_session_title(&self) -> (String, bool) {
        if let Some(sid) = &self.current_session_id {
            let title = if let Some(session) = self.sessions.iter().find(|s| &s.id == sid) {
                network::get_session_title(session)
            } else {
                SESSION_TITLE_NEW.to_string()
            };
            (title, true)
        } else {
            (SESSION_TITLE_DEFAULT.to_string(), false)
        }
    }

    /// Updates UI to reflect current session title
    pub fn update_session_title_ui(&self, ui: &WidgetRef, cx: &mut Cx) {
        let (title, is_active) = self.get_session_title();
        ui_state::update_session_title_ui(ui, cx, &title, is_active);
    }

    /// Updates projects panel with current data
    pub fn update_projects_panel(&self, ui: &WidgetRef, cx: &mut Cx) {
        ui.projects_panel(id!(projects_panel)).set_data(
            cx,
            self.projects.clone(),
            self.sessions.clone(),
            self.selected_session_id.clone(),
        );
    }

    /// Clears all messages and updates the UI
    pub fn clear_messages(&mut self, ui: &WidgetRef, cx: &mut Cx) {
        self.messages_data.clear();
        ui.message_list(id!(message_list))
            .set_messages(cx, &self.messages_data);
    }
}

/// Handles AppAction events
pub fn handle_app_action(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    action: &AppAction,
) {
    match action {
        AppAction::Connected => {
            state.connected = true;
            state.error_message = None;
            ui_state::set_status_connected(ui, cx);
            cx.redraw_all();
        }
        AppAction::ConnectionFailed(err) => {
            state.error_message = Some(err.clone());
            ui_state::set_status_error(ui, cx, err);
            cx.redraw_all();
        }
        AppAction::HealthUpdated(health) => {
            state.health_ok = Some(health.healthy);
            if health.healthy {
                state.connected = true;
                state.error_message = None;
                ui_state::set_status_connected(ui, cx);
            } else {
                state.connected = false;
                ui_state::set_status_disconnected(ui, cx);
            }
            cx.redraw_all();
        }
        AppAction::ProjectsLoaded(projects) => {
            state.projects = projects.clone();
            state.update_projects_panel(ui, cx);
        }
        AppAction::CurrentProjectLoaded(project) => {
            state.current_project = Some(project.clone());
        }
        AppAction::SessionsLoaded(sessions) => {
            state.sessions = sessions.clone();
            state.update_projects_panel(ui, cx);
        }
        AppAction::SessionCreated(session) => {
            state.current_session_id = Some(session.id.clone());
            state.clear_messages(ui, cx);
            state.update_session_title_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::MessagesLoaded(messages) => {
            state.messages_data = messages.clone();
            ui.message_list(id!(message_list))
                .set_messages(cx, &state.messages_data);
        }
        AppAction::SendMessageFailed(err) => {
            state.error_message = Some(err.clone());
            cx.redraw_all();
        }
        _ => {}
    }
}

/// Handles OpenCode SSE events
pub fn handle_opencode_event(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    event: &OcEvent,
) {
    match event {
        OcEvent::SessionCreated(session) => {
            if state.current_session_id.is_none() {
                state.current_session_id = Some(session.id.clone());
                // Clear messages when starting a new session
                state.clear_messages(ui, cx);
            }
            state.sessions.push(session.clone());
            state.update_projects_panel(ui, cx);
            state.update_session_title_ui(ui, cx);
        }
        OcEvent::SessionUpdated(session) => {
            if let Some(existing) = state.sessions.iter_mut().find(|s| s.id == session.id) {
                *existing = session.clone();
            }
            state.update_projects_panel(ui, cx);
            state.update_session_title_ui(ui, cx);
        }
        OcEvent::MessageUpdated(message) => {
            handle_message_updated(state, ui, cx, message);
        }
        OcEvent::PartUpdated { part, .. } => {
            handle_part_updated(state, ui, cx, part);
        }
        _ => {}
    }
}

/// Handles message update events
fn handle_message_updated(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    message: &openpad_protocol::Message,
) {
    // If we don't have a current session yet (race during creation),
    // accept the message and set the session
    if state.current_session_id.is_none() {
        state.current_session_id = Some(message.session_id().to_string());
    }
    
    // Only process messages for the current session
    let current_sid = state.current_session_id.as_deref().unwrap_or("");
    if message.session_id() != current_sid {
        return;
    }
    
    // Find existing or add new MessageWithParts entry
    if let Some(existing) = state
        .messages_data
        .iter_mut()
        .find(|m| m.info.id() == message.id())
    {
        existing.info = message.clone();
    } else {
        state.messages_data.push(MessageWithParts {
            info: message.clone(),
            parts: Vec::new(),
        });
    }
    
    ui.message_list(id!(message_list))
        .set_messages(cx, &state.messages_data);
}

/// Handles part update events
fn handle_part_updated(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    part: &openpad_protocol::Part,
) {
    if let (Some(_), Some(msg_id)) = (part.text_content(), part.message_id()) {
        if let Some(mwp) = state
            .messages_data
            .iter_mut()
            .find(|m| m.info.id() == msg_id)
        {
            let part_id = match &part {
                openpad_protocol::Part::Text { id, .. } => id.as_str(),
                _ => "",
            };
            
            if !part_id.is_empty() {
                if let Some(existing) = mwp.parts.iter_mut().find(|p| {
                    matches!(p, openpad_protocol::Part::Text { id, .. } if id == part_id)
                }) {
                    *existing = part.clone();
                } else {
                    mwp.parts.push(part.clone());
                }
            } else {
                mwp.parts.push(part.clone());
            }
            
            ui.message_list(id!(message_list))
                .set_messages(cx, &state.messages_data);
        }
    }
}
