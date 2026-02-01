use crate::async_runtime::tasks;
use crate::components::message_list::MessageListWidgetRefExt;
use crate::components::permission_dialog::PermissionDialogWidgetRefExt;
use crate::components::projects_panel::ProjectsPanelWidgetRefExt;
use crate::constants::*;
use crate::state::actions::AppAction;
use crate::ui::state_updates;
use makepad_widgets::*;
use openpad_widgets::UpDropDownWidgetRefExt;
use openpad_protocol::{
    Agent, AssistantMessage, AssistantError, Event as OcEvent, Message, MessageTime, MessageWithParts,
    ModelSpec, Part, PermissionRequest, PermissionRuleset, Project, Provider, Session, Skill,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ModelDropdownEntry {
    pub label: String,
    pub provider_id: Option<String>,
    pub model_id: Option<String>,
    pub selectable: bool,
}

impl ModelDropdownEntry {
    pub fn default_option() -> Self {
        Self {
            label: "Default".to_string(),
            provider_id: None,
            model_id: None,
            selectable: true,
        }
    }

    pub fn provider_header(label: String) -> Self {
        Self {
            label,
            provider_id: None,
            model_id: None,
            selectable: false,
        }
    }

    pub fn model_option(provider_id: String, model_id: String, label: String) -> Self {
        Self {
            label,
            provider_id: Some(provider_id),
            model_id: Some(model_id),
            selectable: true,
        }
    }
}

/// Information about an attached file ready to be sent
#[derive(Clone, Debug)]
pub struct AttachedFile {
    pub filename: String,
    pub mime_type: String,
    pub data_url: String,
    /// For text/plain attachments, store the raw text to send as a PartInput::Text
    /// instead of a file attachment. This avoids server-side file processing artifacts.
    pub raw_text: Option<String>,
}

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
    pub is_working: bool,
    pub working_by_session: HashMap<String, bool>,
    pub pending_permissions: Vec<PermissionRequest>,
    pub providers: Vec<Provider>,
    pub agents: Vec<Agent>,
    pub skills: Vec<Skill>,
    pub model_entries: Vec<ModelDropdownEntry>,
    pub selected_model_entry: usize,
    pub selected_agent_idx: Option<usize>,
    pub selected_skill_idx: Option<usize>,
    pub attached_files: Vec<AttachedFile>,
}

impl AppState {
    /// Helper method to find a session by ID
    pub fn find_session(&self, session_id: &str) -> Option<&Session> {
        self.sessions.iter().find(|s| s.id == session_id)
    }

    /// Helper method to find a mutable session by ID
    pub fn find_session_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.iter_mut().find(|s| s.id == session_id)
    }

    /// Refresh all session-related UI components
    pub fn refresh_session_ui(&self, ui: &WidgetRef, cx: &mut Cx) {
        self.update_projects_panel(ui, cx);
        self.update_session_title_ui(ui, cx);
        self.update_project_context_ui(ui, cx);
        self.update_session_meta_ui(ui, cx);
    }

    /// Get the currently selected ModelSpec, if any
    pub fn selected_model_spec(&self) -> Option<ModelSpec> {
        self.model_entries
            .get(self.selected_model_entry)
            .and_then(|entry| {
                if !entry.selectable {
                    return None;
                }
                entry.provider_id.as_ref().and_then(|provider_id| {
                    entry.model_id.as_ref().map(|model_id| ModelSpec {
                        provider_id: provider_id.clone(),
                        model_id: model_id.clone(),
                    })
                })
            })
    }

    pub fn selected_agent_name(&self) -> Option<String> {
        self.selected_agent_idx
            .and_then(|idx| self.agents.get(idx))
            .map(|agent| agent.name.clone())
    }

    pub fn selected_agent_permission(&self) -> Option<PermissionRuleset> {
        self.selected_agent_idx
            .and_then(|idx| self.agents.get(idx))
            .and_then(|agent| agent.permission.clone())
    }

    pub fn selected_skill(&self) -> Option<&Skill> {
        self.selected_skill_idx.and_then(|idx| self.skills.get(idx))
    }

    pub fn selected_skill_prompt(&self) -> Option<String> {
        self.selected_skill()
            .map(|skill| format!("Use skill: {}", skill.name))
    }
}

fn build_model_entries(providers: &[Provider]) -> Vec<ModelDropdownEntry> {
    /// Helper to get display label, preferring name over id
    fn display_label(name: Option<&str>, id: &str) -> String {
        name.unwrap_or(id).to_string()
    }

    let mut entries = vec![ModelDropdownEntry::default_option()];
    for provider in providers {
        if let Some(models) = provider.models.as_ref() {
            if models.is_empty() {
                continue;
            }
            let provider_label = display_label(provider.name.as_deref(), &provider.id);
            let mut model_items: Vec<_> = models.iter().collect();
            model_items.sort_by(|a, b| {
                let a_label = display_label(a.1.name.as_deref(), &a.1.id);
                let b_label = display_label(b.1.name.as_deref(), &b.1.id);
                a_label.cmp(&b_label)
            });
            entries.push(ModelDropdownEntry::provider_header(provider_label.clone()));
            for (_key, model) in model_items {
                let model_label = format!("  {}", display_label(model.name.as_deref(), &model.id));
                entries.push(ModelDropdownEntry::model_option(
                    provider.id.clone(),
                    model.id.clone(),
                    model_label,
                ));
            }
        }
    }
    entries
}

impl AppState {
    /// Determines the session title to display
    pub fn get_session_title(&self) -> (String, bool) {
        if let Some(sid) = &self.current_session_id {
            let title = if let Some(session) = self.find_session(sid) {
                tasks::get_session_title(session)
            } else {
                SESSION_TITLE_NEW.to_string()
            };
            (title, true)
        } else {
            (SESSION_TITLE_DEFAULT.to_string(), false)
        }
    }

    /// Check if the current session is reverted
    pub fn is_current_session_reverted(&self) -> bool {
        if let Some(sid) = &self.current_session_id {
            if let Some(session) = self.find_session(sid) {
                return session.revert.is_some();
            }
        }
        false
    }

    /// Updates UI to reflect current session title
    pub fn update_session_title_ui(&self, ui: &WidgetRef, cx: &mut Cx) {
        let (title, is_active) = self.get_session_title();
        state_updates::update_session_title_ui(ui, cx, &title, is_active);

        // Update revert indicator
        let is_reverted = self.is_current_session_reverted();
        state_updates::update_revert_indicator(ui, cx, is_reverted);
    }

    pub fn current_revert_message_id(&self) -> Option<String> {
        self.current_session_id
            .as_ref()
            .and_then(|sid| self.find_session(sid))
            .and_then(|session| session.revert.as_ref())
            .map(|revert| revert.message_id.clone())
    }

    pub fn update_session_meta_ui(&self, ui: &WidgetRef, cx: &mut Cx) {
        let session = self
            .current_session_id
            .as_ref()
            .and_then(|sid| self.find_session(sid));
        let share_url = session.and_then(|s| s.share.as_ref().map(|share| share.url.as_str()));
        let summary = session.and_then(|s| s.summary.as_ref());
        state_updates::update_share_ui(ui, cx, share_url);
        state_updates::update_summary_ui(ui, cx, summary);
    }

    pub fn current_share_url(&self) -> Option<String> {
        self.current_session_id.as_ref().and_then(|sid| {
            self.find_session(sid)
                .and_then(|session| session.share.as_ref().map(|share| share.url.clone()))
        })
    }

    fn project_for_current_session(&self) -> Option<&Project> {
        let session_project_id = self.current_session_id.as_ref().and_then(|session_id| {
            self.find_session(session_id)
                .map(|session| &session.project_id)
        });
        session_project_id.and_then(|project_id| {
            self.projects
                .iter()
                .find(|project| &project.id == project_id)
        })
    }

    pub fn update_project_context_ui(&self, ui: &WidgetRef, cx: &mut Cx) {
        let project_for_session = self.project_for_current_session();
        let project = project_for_session.or_else(|| self.current_project.as_ref());
        state_updates::update_project_context_ui(ui, cx, project);
    }

    /// Updates projects panel with current data
    pub fn update_projects_panel(&self, ui: &WidgetRef, cx: &mut Cx) {
        ui.projects_panel(&[id!(projects_panel)]).set_data(
            cx,
            self.projects.clone(),
            self.sessions.clone(),
            self.selected_session_id.clone(),
            self.working_by_session.clone(),
        );
    }

    /// Clears all messages and updates the UI
    pub fn clear_messages(&mut self, ui: &WidgetRef, cx: &mut Cx) {
        self.messages_data.clear();
        ui.message_list(&[id!(message_list)])
            .set_messages(cx, &self.messages_data, None);
    }

    /// Handles session deletion, clearing relevant state and updating UI
    pub fn handle_session_deletion(&mut self, ui: &WidgetRef, cx: &mut Cx, session_id: &str) {
        // If the deleted session is currently selected, clear it
        if self.current_session_id.as_deref() == Some(session_id) {
            self.current_session_id = None;
            self.selected_session_id = None;
            self.is_working = false;
            self.clear_messages(ui, cx);
            state_updates::update_work_indicator(ui, cx, false);
        } else if self.selected_session_id.as_deref() == Some(session_id) {
            self.selected_session_id = None;
        }
        // Remove from sessions list
        self.sessions.retain(|s| s.id != session_id);
    }
}

/// Handles AppAction events
pub fn handle_app_action(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx, action: &AppAction) {
    match action {
        AppAction::Connected => {
            state.connected = true;
            state.error_message = None;
            state.is_working = false;
            state_updates::set_status_connected(ui, cx);
            state_updates::update_work_indicator(ui, cx, false);
            cx.redraw_all();
        }
        AppAction::ConnectionFailed(err) => {
            state.error_message = Some(err.clone());
            state.is_working = false;
            state_updates::set_status_error(ui, cx, err);
            state_updates::update_work_indicator(ui, cx, false);
            cx.redraw_all();
        }
        AppAction::HealthUpdated(health) => {
            state.health_ok = Some(health.healthy);
            if health.healthy {
                state.connected = true;
                state.error_message = None;
                state_updates::set_status_connected(ui, cx);
            } else {
                state.connected = false;
                state.is_working = false;
                state_updates::set_status_disconnected(ui, cx);
                state_updates::update_work_indicator(ui, cx, false);
            }
            cx.redraw_all();
        }
        AppAction::ProjectsLoaded(projects) => {
            state.projects = projects.clone();
            state.update_projects_panel(ui, cx);
            state.update_project_context_ui(ui, cx);
        }
        AppAction::CurrentProjectLoaded(project) => {
            state.current_project = Some(project.clone());
            state.update_project_context_ui(ui, cx);
        }
        AppAction::SessionsLoaded(sessions) => {
            state.sessions = sessions.clone();
            state.refresh_session_ui(ui, cx);
        }
        AppAction::SessionCreated(session) => {
            state.current_session_id = Some(session.id.clone());
            state.clear_messages(ui, cx);

            // Add the session to the sessions list immediately (don't wait for SSE)
            // Check if it's not already there to avoid duplicates
            if !state.sessions.iter().any(|s| s.id == session.id) {
                state.sessions.push(session.clone());
            }

            // Update current_project to match the session's project
            if let Some(project) = state.projects.iter().find(|p| p.id == session.project_id) {
                state.current_project = Some(project.clone());
            }

            state.refresh_session_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::SessionDeleted(session_id) => {
            state.handle_session_deletion(ui, cx, session_id);
            state.refresh_session_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::SessionUpdated(session) => {
            // Update the session in the list
            if let Some(existing) = state.find_session_mut(&session.id) {
                *existing = session.clone();
            }
            state.refresh_session_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::SessionDiffLoaded { session_id, diffs } => {
            if let Some(existing) = state.find_session_mut(session_id) {
                let summary = existing.summary.get_or_insert_with(|| {
                    openpad_protocol::SessionSummary {
                        additions: 0,
                        deletions: 0,
                        files: diffs.len() as i64,
                        diffs: Vec::new(),
                    }
                });
                summary.diffs = diffs.clone();
            }
            state.update_session_meta_ui(ui, cx);

            // Also update inline diffs in message list
            ui.message_list(&[id!(message_list)])
                .set_session_diffs(cx, &diffs);

            cx.redraw_all();
        }
        AppAction::MessagesLoaded(messages) => {
            state.messages_data = messages.clone();
            ui.message_list(&[id!(message_list)])
                .set_messages(cx, &state.messages_data, state.current_revert_message_id());
        }
        AppAction::SendMessageFailed(err) => {
            state.error_message = Some(err.clone());
            state.is_working = false;
            state_updates::set_status_error(ui, cx, err);
            state_updates::update_work_indicator(ui, cx, false);
            cx.redraw_all();
        }
        AppAction::PendingPermissionsLoaded(permissions) => {
            state.pending_permissions = permissions.clone();
            show_next_pending_permission(state, ui, cx);
        }
        AppAction::ProvidersLoaded(providers_response) => {
            log!(
                "ProvidersLoaded: {} providers",
                providers_response.providers.len()
            );
            state.providers = providers_response.providers.clone();
            state.model_entries = build_model_entries(&state.providers);
            state.selected_model_entry = 0;
            let labels: Vec<String> = state
                .model_entries
                .iter()
                .map(|entry| entry.label.clone())
                .collect();
            ui.up_drop_down(&[id!(model_dropdown)]).set_labels(cx, labels);
            ui.up_drop_down(&[id!(model_dropdown)])
                .set_selected_item(cx, 0);
            cx.redraw_all();
        }
        AppAction::AgentsLoaded(agents) => {
            log!("AgentsLoaded: {} agents", agents.len());
            state.agents = agents.clone();
            let mut labels: Vec<String> = vec!["Default".to_string()];
            labels.extend(state.agents.iter().map(|a| a.name.clone()));
            ui.up_drop_down(&[id!(agent_dropdown)]).set_labels(cx, labels);
            ui.up_drop_down(&[id!(agent_dropdown)])
                .set_selected_item(cx, 0);
            state.selected_agent_idx = None;
            cx.redraw_all();
        }
        AppAction::SkillsLoaded(skills) => {
            log!("SkillsLoaded: {} skills", skills.len());
            state.skills = skills.clone();
            let mut labels: Vec<String> = vec!["Skill".to_string()];
            labels.extend(state.skills.iter().map(|s| s.name.clone()));
            ui.up_drop_down(&[id!(skill_dropdown)]).set_labels(cx, labels);
            ui.up_drop_down(&[id!(skill_dropdown)])
                .set_selected_item(cx, 0);
            state.selected_skill_idx = None;
            cx.redraw_all();
        }
        _ => {}
    }
}

/// Handles OpenCode SSE events
pub fn handle_opencode_event(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx, event: &OcEvent) {
    match event {
        OcEvent::SessionCreated(session) => {
            if state.current_session_id.is_none() {
                state.current_session_id = Some(session.id.clone());
                state.clear_messages(ui, cx);
            }
            // Only add the session if it's not already in the list (avoid duplicates from AppAction::SessionCreated)
            if !state.sessions.iter().any(|s| s.id == session.id) {
                state.sessions.push(session.clone());
            }
            state.refresh_session_ui(ui, cx);
        }
        OcEvent::SessionUpdated(session) => {
            if let Some(existing) = state.find_session_mut(&session.id) {
                *existing = session.clone();
            }
            state.refresh_session_ui(ui, cx);
        }
        OcEvent::SessionDeleted(session) => {
            state.handle_session_deletion(ui, cx, &session.id);
            state.refresh_session_ui(ui, cx);
        }
        OcEvent::MessageUpdated(message) => {
            handle_message_updated(state, ui, cx, message);
        }
        OcEvent::PartUpdated { part, .. } => {
            handle_part_updated(state, ui, cx, part);
        }
        OcEvent::PermissionAsked(request) => {
            enqueue_pending_permission(state, request);
            show_next_pending_permission(state, ui, cx);
        }
        OcEvent::PermissionReplied { request_id, .. } => {
            // If the current dialog is for this request, hide it
            // (another client or auto-reply may have responded)
            let dialog_request_id = ui
                .permission_dialog(&[id!(permission_dialog)])
                .get_request_id();
            if dialog_request_id.as_deref() == Some(request_id.as_str()) {
                ui.permission_dialog(&[id!(permission_dialog)]).hide(cx);
            }
            remove_pending_permission(state, request_id);
            show_next_pending_permission(state, ui, cx);
        }
        OcEvent::SessionError { session_id, error } => {
            update_work_indicator(state, ui, cx, false);
            if state.current_session_id.as_deref() == Some(session_id.as_str()) {
                push_session_error_message(state, ui, cx, session_id, error);
            }
        }
        _ => {}
    }
}

pub fn handle_permission_responded(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    request_id: &str,
) {
    remove_pending_permission(state, request_id);
    show_next_pending_permission(state, ui, cx);
}

/// Handles message update events
fn handle_message_updated(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    message: &openpad_protocol::Message,
) {
    let session_id = message.session_id().to_string();

    if let openpad_protocol::Message::Assistant(msg) = message {
        let working = msg.time.completed.is_none() && msg.error.is_none();
        set_session_working(state, ui, cx, &session_id, working);
    }

    // If we don't have a current session yet (race during creation),
    // accept the message and set the session
    if state.current_session_id.is_none() {
        state.current_session_id = Some(session_id.clone());
    }

    // Only process messages for the current session
    let current_sid = state.current_session_id.as_deref().unwrap_or("");
    if session_id != current_sid {
        return;
    }

    if let openpad_protocol::Message::Assistant(msg) = message {
        let working = msg.time.completed.is_none() && msg.error.is_none();
        update_work_indicator(state, ui, cx, working);
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

    ui.message_list(&[id!(message_list)])
        .set_messages(cx, &state.messages_data, state.current_revert_message_id());
}

/// Handles part update events
fn handle_part_updated(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    part: &openpad_protocol::Part,
) {
    if let (Some(_), Some(msg_id)) = (part.text_content(), part.message_id()) {
        let mut should_update_work = false;
        let mut work_session_id: Option<String> = None;
        if let Some(mwp) = state
            .messages_data
            .iter_mut()
            .find(|m| m.info.id() == msg_id)
        {
            if matches!(mwp.info, openpad_protocol::Message::Assistant(_)) {
                should_update_work = true;
                work_session_id = Some(mwp.info.session_id().to_string());
            }
            let part_id = match &part {
                openpad_protocol::Part::Text { id, .. } => id.as_str(),
                _ => "",
            };

            if !part_id.is_empty() {
                if let Some(existing) = mwp
                    .parts
                    .iter_mut()
                    .find(|p| matches!(p, openpad_protocol::Part::Text { id, .. } if id == part_id))
                {
                    *existing = part.clone();
                } else {
                    mwp.parts.push(part.clone());
                }
            } else {
                mwp.parts.push(part.clone());
            }

            ui.message_list(&[id!(message_list)])
                .set_messages(cx, &state.messages_data, state.current_revert_message_id());
        }

        if should_update_work {
            update_work_indicator(state, ui, cx, true);
            if let Some(session_id) = work_session_id {
                set_session_working(state, ui, cx, &session_id, true);
            }
        }
    }
}

fn update_work_indicator(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx, working: bool) {
    if state.is_working == working {
        return;
    }
    state.is_working = working;
    state_updates::update_work_indicator(ui, cx, working);
}

fn set_session_working(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    session_id: &str,
    working: bool,
) {
    let previous = state.working_by_session.get(session_id).copied();
    if previous == Some(working) {
        return;
    }
    if working {
        state.working_by_session.insert(session_id.to_string(), true);
    } else {
        state.working_by_session.insert(session_id.to_string(), false);
    }
    state.update_projects_panel(ui, cx);
}

fn format_permission_context(request: &PermissionRequest) -> Option<String> {
    let mut lines = Vec::new();

    lines.push(format!("Session: {}", request.session_id));

    if let Some(tool) = &request.tool {
        lines.push(format!("Tool message: {}", tool.message_id));
        lines.push(format!("Tool call: {}", tool.call_id));
    }

    if !request.always.is_empty() {
        lines.push(format!("Always: {}", request.always.join(", ")));
    }

    if !request.metadata.is_empty() {
        let mut entries: Vec<_> = request.metadata.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        for (key, value) in entries {
            lines.push(format!("{}: {}", key, value));
        }
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}

fn enqueue_pending_permission(state: &mut AppState, request: &PermissionRequest) {
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

fn show_next_pending_permission(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx) {
    if ui
        .permission_dialog(&[id!(permission_dialog)])
        .get_request_id()
        .is_some()
    {
        return;
    }

    let Some(current_session_id) = &state.current_session_id else {
        return;
    };

    let Some(request) = state
        .pending_permissions
        .iter()
        .find(|permission| &permission.session_id == current_session_id)
    else {
        return;
    };

    let context = format_permission_context(request);
    ui.permission_dialog(&[id!(permission_dialog)])
        .show_permission_request(
            cx,
            request.session_id.clone(),
            request.id.clone(),
            request.permission.clone(),
            request.patterns.clone(),
            context,
        );
}

fn push_session_error_message(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    session_id: &str,
    error: &AssistantError,
) {
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
        finish: None,
    };

    let part = Part::Text {
        id: format!("part_{}", message_id),
        session_id: session_id.to_string(),
        message_id: message_id.clone(),
        text: "Session error".to_string(),
    };

    state.messages_data.push(MessageWithParts {
        info: Message::Assistant(assistant),
        parts: vec![part],
    });

    ui.message_list(&[id!(message_list)])
        .set_messages(cx, &state.messages_data, state.current_revert_message_id());
}
