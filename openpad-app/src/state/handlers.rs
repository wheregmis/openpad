use crate::async_runtime::tasks;
use crate::components::projects_panel::ProjectsPanelWidgetRefExt;
use crate::constants::*;
use crate::state::actions::AppAction;
use crate::ui::state_updates;
use makepad_widgets::*;
use openpad_protocol::{
    Agent, AssistantError, AssistantMessage, Event as OcEvent, Message, MessageTime,
    MessageWithParts, ModelSpec, Part, PermissionRequest, PermissionRuleset, Project, Provider,
    Session, Skill,
};
use openpad_widgets::message_list::MessageListWidgetRefExt;
use openpad_widgets::message_list::PendingPermissionDisplay;
use openpad_widgets::settings_dialog::SettingsDialogWidgetRefExt;
use openpad_widgets::UpDropDownWidgetRefExt;
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
    /// Selected provider index (0 = Default/All providers)
    pub selected_provider_idx: usize,
    /// Available provider labels for the provider dropdown
    pub provider_labels: Vec<String>,
    /// Currently filtered model labels based on selected provider
    pub model_labels: Vec<String>,
    /// Maps model dropdown index to (provider_id, model_id)
    pub model_entries: Vec<(String, String)>,
    pub selected_model_idx: usize,
    pub selected_agent_idx: Option<usize>,
    pub selected_skill_idx: Option<usize>,
    pub attached_files: Vec<AttachedFile>,
    pub config: Option<openpad_protocol::Config>,
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
            .get(self.selected_model_idx)
            .map(|(provider_id, model_id)| ModelSpec {
                provider_id: provider_id.clone(),
                model_id: model_id.clone(),
            })
    }

    /// Get models for the currently selected provider
    pub fn get_models_for_selected_provider(&self) -> Vec<(String, String, String)> {
        let mut models = Vec::new();

        if self.selected_provider_idx == 0 {
            // "Default" selected - show all models from all providers
            for provider in &self.providers {
                if let Some(provider_models) = provider.models.as_ref() {
                    for (_key, model) in provider_models {
                        let model_label = model.name.as_deref().unwrap_or(&model.id).to_string();
                        models.push((provider.id.clone(), model.id.clone(), model_label));
                    }
                }
            }
        } else if let Some(provider) = self.providers.get(self.selected_provider_idx - 1) {
            // Specific provider selected
            if let Some(provider_models) = provider.models.as_ref() {
                for (_key, model) in provider_models {
                    let model_label = model.name.as_deref().unwrap_or(&model.id).to_string();
                    models.push((provider.id.clone(), model.id.clone(), model_label));
                }
            }
        }

        // Sort by label
        models.sort_by(|a, b| a.2.cmp(&b.2));
        models
    }

    /// Update model list based on selected provider
    pub fn update_model_list_for_provider(&mut self) {
        let models = self.get_models_for_selected_provider();
        self.model_labels = models.iter().map(|(_, _, label)| label.clone()).collect();
        self.model_entries = models
            .iter()
            .map(|(pid, mid, _)| (pid.clone(), mid.clone()))
            .collect();

        // Reset model selection to first item (or 0 if empty)
        self.selected_model_idx = 0;
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

#[allow(dead_code)]
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
        // Keep both lookup paths while the sidebar tree is being stabilized.
        ui.projects_panel(cx, &[id!(projects_panel)]).set_data(
            cx,
            self.projects.clone(),
            self.sessions.clone(),
            self.selected_session_id.clone(),
            self.working_by_session.clone(),
        );
        ui.projects_panel(cx, &[id!(side_panel), id!(projects_panel)])
            .set_data(
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
        ui.message_list(cx, &[id!(message_list)])
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
            state.update_session_meta_ui(ui, cx);

            // Also update inline diffs in message list
            ui.message_list(cx, &[id!(message_list)])
                .set_session_diffs(cx, &diffs);

            cx.redraw_all();
        }
        AppAction::MessagesLoaded(messages) => {
            state.messages_data = messages.clone();
            ui.message_list(cx, &[id!(message_list)]).set_messages(
                cx,
                &state.messages_data,
                state.current_revert_message_id(),
            );
            // Request session diff so file changes show on the last assistant message
            if let Some(session_id) = state.current_session_id.clone() {
                let message_id = state
                    .messages_data
                    .iter()
                    .rev()
                    .find_map(|mwp| match &mwp.info {
                        openpad_protocol::Message::User(msg) => Some(msg.id.clone()),
                        _ => None,
                    });
                cx.action(AppAction::RequestSessionDiff {
                    session_id,
                    message_id,
                });
            }
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

            // Build provider dropdown labels (Default + provider names)
            let mut provider_labels = vec!["Default".to_string()];
            provider_labels.extend(
                state
                    .providers
                    .iter()
                    .map(|p| p.name.as_deref().unwrap_or(&p.id).to_string()),
            );
            state.provider_labels = provider_labels.clone();
            state.selected_provider_idx = 0;

            log!("Providers count: {}", state.providers.len());
            log!("Provider labels count: {}", provider_labels.len());
            log!("Provider labels: {:?}", provider_labels);

            // Initialize model list based on Default (all models)
            state.update_model_list_for_provider();

            // Set provider dropdown
            log!(
                "Setting provider dropdown with {} labels",
                state.provider_labels.len()
            );
            let provider_dd = ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(provider_dropdown)]);
            provider_dd.set_labels(cx, state.provider_labels.clone());
            provider_dd.set_selected_item(cx, 0);
            log!(
                "Provider dropdown labels set to: {:?}",
                state.provider_labels
            );

            // Set model dropdown
            log!(
                "Setting model dropdown with {} labels",
                state.model_labels.len()
            );
            let model_dd = ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(model_dropdown)]);
            model_dd.set_labels(cx, state.model_labels.clone());
            model_dd.set_selected_item(cx, 0);

            // Force redraw of the input bar area to ensure dropdowns are updated
            ui.redraw(cx);
            log!("Provider labels set: {:?}", state.provider_labels);

            ui.settings_dialog(cx, &[id!(side_panel), id!(settings_panel)])
                .set_providers(cx, state.providers.clone());

            cx.redraw_all();
        }
        AppAction::AgentsLoaded(agents) => {
            log!("AgentsLoaded: {} agents", agents.len());
            state.agents = agents.clone();
            let mut labels: Vec<String> = vec!["Default".to_string()];
            labels.extend(state.agents.iter().map(|a| a.name.clone()));
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(agent_dropdown)])
                .set_labels(cx, labels);
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(agent_dropdown)])
                .set_selected_item(cx, 0);
            state.selected_agent_idx = None;
            cx.redraw_all();
        }
        AppAction::SkillsLoaded(skills) => {
            log!("SkillsLoaded: {} skills", skills.len());
            state.skills = skills.clone();
            let mut labels: Vec<String> = vec!["Skill".to_string()];
            labels.extend(state.skills.iter().map(|s| s.name.clone()));
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(skill_dropdown)])
                .set_labels(cx, labels);
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(skill_dropdown)])
                .set_selected_item(cx, 0);
            state.selected_skill_idx = None;
            cx.redraw_all();
        }
        AppAction::ConfigLoaded(config) => {
            state.config = Some(config.clone());
            ui.settings_dialog(cx, &[id!(side_panel), id!(settings_panel)])
                .set_config(cx, &config);
            cx.redraw_all();
        }
        AppAction::AuthSet {
            provider_id: _,
            success,
        } => {
            if *success {
                // visual feedback handled by ProvidersLoaded which follows
            }
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

    ui.message_list(cx, &[id!(message_list)]).set_messages(
        cx,
        &state.messages_data,
        state.current_revert_message_id(),
    );
}

/// Handles part update events
fn handle_part_updated(
    state: &mut AppState,
    ui: &WidgetRef,
    cx: &mut Cx,
    part: &openpad_protocol::Part,
) {
    // Get message_id from the part - handle both text and non-text parts (tools, steps)
    let msg_id = part.message_id();
    if msg_id.is_none() {
        return;
    }
    let msg_id = msg_id.unwrap();

    // Only process parts for the current session
    let current_sid = state.current_session_id.as_deref().unwrap_or("");

    let mut should_update_work = false;
    let mut work_session_id: Option<String> = None;
    let mut did_mutate_parts = false;
    let mut requires_full_rebuild = false;
    let mut incremental_append: Option<(String, String)> = None;
    if let Some(mwp) = state
        .messages_data
        .iter_mut()
        .find(|m| m.info.id() == msg_id)
    {
        // Check if this part belongs to the current session
        if mwp.info.session_id() != current_sid {
            return;
        }

        if matches!(mwp.info, openpad_protocol::Message::Assistant(_)) {
            should_update_work = true;
            work_session_id = Some(mwp.info.session_id().to_string());
        }

        let role = match &mwp.info {
            openpad_protocol::Message::Assistant(_) => "assistant",
            openpad_protocol::Message::User(_) => "user",
        };

        match part {
            openpad_protocol::Part::Text { id, text, .. } => {
                if !id.is_empty() {
                    if let Some(existing) = mwp.parts.iter_mut().find(|p| {
                        matches!(p, openpad_protocol::Part::Text { id: existing_id, .. } if existing_id == id)
                    }) {
                        *existing = part.clone();
                        did_mutate_parts = true;
                        // Existing-id replacement may be non-delta content; keep correctness.
                        requires_full_rebuild = true;
                    } else {
                        mwp.parts.push(part.clone());
                        did_mutate_parts = true;
                        if !text.is_empty() {
                            incremental_append = Some((role.to_string(), text.clone()));
                        }
                    }
                } else {
                    // Text parts without IDs are treated as streaming deltas.
                    // Append part and incrementally update UI text.
                    mwp.parts.push(part.clone());
                    did_mutate_parts = true;
                    if !text.is_empty() {
                        incremental_append = Some((role.to_string(), text.clone()));
                    }
                }
            }
            _ => {
                // For non-text parts (StepStart, Tool, StepFinish), always append.
                mwp.parts.push(part.clone());
                did_mutate_parts = true;
                requires_full_rebuild = true;
            }
        };
    }

    if did_mutate_parts {
        if requires_full_rebuild {
            ui.message_list(cx, &[id!(message_list)]).set_messages(
                cx,
                &state.messages_data,
                state.current_revert_message_id(),
            );
        } else if let Some((role, text)) = incremental_append {
            ui.message_list(cx, &[id!(message_list)])
                .append_text_for_message(cx, &role, msg_id, &text);
        }
    }

    if should_update_work {
        update_work_indicator(state, ui, cx, true);
        if let Some(session_id) = work_session_id {
            set_session_working(state, ui, cx, &session_id, true);
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
        state
            .working_by_session
            .insert(session_id.to_string(), true);
    } else {
        state
            .working_by_session
            .insert(session_id.to_string(), false);
    }
    state.update_projects_panel(ui, cx);
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
    let Some(current_session_id) = &state.current_session_id else {
        // Clear permissions if no session
        ui.message_list(cx, &[id!(message_list)])
            .set_pending_permissions(cx, &[]);
        return;
    };

    let displays: Vec<PendingPermissionDisplay> = state
        .pending_permissions
        .iter()
        .filter(|p| &p.session_id == current_session_id)
        .map(|p| PendingPermissionDisplay {
            session_id: p.session_id.clone(),
            request_id: p.id.clone(),
            permission: p.permission.clone(),
            patterns: p.patterns.clone(),
        })
        .collect();

    ui.message_list(cx, &[id!(message_list)])
        .set_pending_permissions(cx, &displays);
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

    ui.message_list(cx, &[id!(message_list)]).set_messages(
        cx,
        &state.messages_data,
        state.current_revert_message_id(),
    );
}
