use crate::async_runtime::tasks;
use crate::components::files_panel::FilesPanelWidgetRefExt;
use crate::components::sessions_panel::SessionsPanelWidgetRefExt;
use crate::constants::*;
use crate::state::actions::AppAction;
use crate::ui::state_updates;
use makepad_widgets::*;
use openpad_protocol::{
    Agent, Event as OcEvent, MessageWithParts, ModelSpec, PermissionRequest, PermissionRuleset,
    Project, Provider, Session, Skill,
};
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

#[derive(Clone, Debug, Default)]
pub struct OpenFileState {
    pub project_id: String,
    pub absolute_path: String,
    pub display_name: String,
    pub text_cache: String,
    pub last_saved_revision: u64,
}

#[derive(Clone, Debug)]
pub enum CenterTabKind {
    Home,
    Chat { session_id: String },
    File { open_file: OpenFileState },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum CenterTabTarget {
    #[default]
    Home,
    ChatSession(String),
    FilePath(String),
}

#[derive(Clone, Debug)]
pub enum PendingCenterIntent {
    OpenFile {
        project_id: String,
        absolute_path: String,
    },
    OpenSession {
        session_id: String,
    },
    SwitchTab {
        tab_id: LiveId,
    },
    CloseTab {
        tab_id: LiveId,
    },
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
    pub messages_by_session: HashMap<String, Vec<MessageWithParts>>,
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
    pub center_tabs_by_id: HashMap<LiveId, CenterTabKind>,
    pub tab_by_session: HashMap<String, LiveId>,
    pub tab_by_file: HashMap<String, LiveId>,
    pub active_center_tab: Option<LiveId>,
    pub pending_center_intent: Option<PendingCenterIntent>,
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
        self.update_files_panel(ui, cx);
        self.update_sessions_panel(ui, cx);
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

    pub fn messages_for_session(&self, session_id: &str) -> &[MessageWithParts] {
        self.messages_by_session
            .get(session_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn set_messages_for_session(
        &mut self,
        session_id: String,
        messages: Vec<MessageWithParts>,
    ) {
        self.messages_by_session.insert(session_id, messages);
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
            .and_then(|sid| self.current_revert_message_id_for_session(sid))
    }

    pub fn current_revert_message_id_for_session(&self, session_id: &str) -> Option<String> {
        self.find_session(session_id)
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

    /// Updates left files panel (projects + file tree) with current data
    pub fn update_files_panel(&self, ui: &WidgetRef, cx: &mut Cx) {
        ui.files_panel(cx, &[id!(side_panel), id!(files_panel)])
            .set_data(cx, self.projects.clone());
    }

    /// Updates right sessions panel with current data
    pub fn update_sessions_panel(&self, ui: &WidgetRef, cx: &mut Cx) {
        ui.sessions_panel(cx, &[id!(right_side_panel), id!(sessions_panel)])
            .set_data(
                cx,
                self.projects.clone(),
                self.sessions.clone(),
                self.selected_session_id.clone(),
                self.working_by_session.clone(),
            );
    }

}

/// Handles AppAction events
pub fn handle_app_action(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx, action: &AppAction) {
    let effects = crate::state::reducer::reduce_app_state(state, action);

    match action {
        AppAction::Connected => {
            state_updates::set_status_connected(ui, cx);
            state_updates::update_work_indicator(ui, cx, false);
            cx.redraw_all();
        }
        AppAction::ConnectionFailed(err) => {
            state_updates::set_status_error(ui, cx, err);
            state_updates::update_work_indicator(ui, cx, false);
            cx.redraw_all();
        }
        AppAction::HealthUpdated(health) => {
            if health.healthy {
                state_updates::set_status_connected(ui, cx);
            } else {
                state_updates::set_status_disconnected(ui, cx);
                state_updates::update_work_indicator(ui, cx, false);
            }
            cx.redraw_all();
        }
        AppAction::ProjectsLoaded(_) => {
            state.update_files_panel(ui, cx);
            state.update_sessions_panel(ui, cx);
            state.update_project_context_ui(ui, cx);
        }
        AppAction::CurrentProjectLoaded(_) => {
            state.update_project_context_ui(ui, cx);
        }
        AppAction::SessionsLoaded(_) => {
            state.refresh_session_ui(ui, cx);
        }
        AppAction::SessionLoaded(_) => {
            state.refresh_session_ui(ui, cx);
        }
        AppAction::SessionCreated(_) => {
            state.refresh_session_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::SessionDeleted(_) => {
            state_updates::update_work_indicator(ui, cx, state.is_working);
            state.refresh_session_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::SessionUpdated(_) => {
            state.refresh_session_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::SessionDiffLoaded { .. } => {
            state.update_session_meta_ui(ui, cx);
            cx.redraw_all();
        }
        AppAction::MessagesLoaded {
            session_id: _,
            messages: _,
        } => {}
        AppAction::MessageReceived(message) => {
            if matches!(message, openpad_protocol::Message::Assistant(_)) {
                state.update_sessions_panel(ui, cx);
                if state.current_session_id.as_deref() == Some(message.session_id()) {
                    state_updates::update_work_indicator(ui, cx, state.is_working);
                }
            }
        }
        AppAction::PartReceived { .. } => {
            state.update_sessions_panel(ui, cx);
            state_updates::update_work_indicator(ui, cx, state.is_working);
        }
        AppAction::SendMessageFailed(err) => {
            state_updates::set_status_error(ui, cx, err);
            state_updates::update_work_indicator(ui, cx, false);
            cx.redraw_all();
        }
        AppAction::PendingPermissionsLoaded(_) => {
            show_next_pending_permission(state, ui, cx);
        }
        AppAction::PendingPermissionReceived(_) => {
            show_next_pending_permission(state, ui, cx);
        }
        AppAction::PermissionResponded { .. } => {
            show_next_pending_permission(state, ui, cx);
        }
        AppAction::PermissionDismissed { .. } => {
            show_next_pending_permission(state, ui, cx);
        }
        AppAction::SessionErrorReceived { .. } => {
            state_updates::update_work_indicator(ui, cx, false);
        }
        AppAction::ProvidersLoaded(providers_response) => {
            log!(
                "ProvidersLoaded: {} providers",
                providers_response.providers.len()
            );

            log!("Providers count: {}", state.providers.len());
            log!("Provider labels count: {}", state.provider_labels.len());
            log!("Provider labels: {:?}", state.provider_labels);

            // Set provider dropdown
            log!(
                "Setting provider dropdown with {} labels",
                state.provider_labels.len()
            );
            let provider_dd =
                ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(provider_dropdown)]);
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
            let mut labels: Vec<String> = vec!["Default".to_string()];
            labels.extend(state.agents.iter().map(|a| a.name.clone()));
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(agent_dropdown)])
                .set_labels(cx, labels);
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(agent_dropdown)])
                .set_selected_item(cx, 0);
            cx.redraw_all();
        }
        AppAction::SkillsLoaded(skills) => {
            log!("SkillsLoaded: {} skills", skills.len());
            let mut labels: Vec<String> = vec!["Skill".to_string()];
            labels.extend(state.skills.iter().map(|s| s.name.clone()));
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(skill_dropdown)])
                .set_labels(cx, labels);
            ui.up_drop_down(cx, &[id!(input_bar_toolbar), id!(skill_dropdown)])
                .set_selected_item(cx, 0);
            cx.redraw_all();
        }
        AppAction::ConfigLoaded(config) => {
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

    crate::state::effect_executor::execute_state_effects(cx, effects);
}

/// Handles OpenCode SSE events
pub fn handle_opencode_event(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx, event: &OcEvent) {
    match event {
        OcEvent::SessionCreated(session) => {
            handle_app_action(state, ui, cx, &AppAction::SessionLoaded(session.clone()));
        }
        OcEvent::SessionUpdated(session) => {
            handle_app_action(state, ui, cx, &AppAction::SessionUpdated(session.clone()));
        }
        OcEvent::SessionDeleted(session) => {
            handle_app_action(state, ui, cx, &AppAction::SessionDeleted(session.id.clone()));
        }
        OcEvent::MessageUpdated(message) => {
            handle_app_action(state, ui, cx, &AppAction::MessageReceived(message.clone()));
        }
        OcEvent::PartUpdated { part, delta } => {
            handle_app_action(
                state,
                ui,
                cx,
                &AppAction::PartReceived {
                    part: part.clone(),
                    delta: delta.clone(),
                },
            );
        }
        OcEvent::PermissionAsked(request) => {
            handle_app_action(
                state,
                ui,
                cx,
                &AppAction::PendingPermissionReceived(request.clone()),
            );
        }
        OcEvent::PermissionReplied {
            session_id,
            request_id,
            ..
        } => {
            handle_app_action(
                state,
                ui,
                cx,
                &AppAction::PermissionDismissed {
                    session_id: session_id.clone(),
                    request_id: request_id.clone(),
                },
            );
        }
        OcEvent::SessionError { session_id, error } => {
            handle_app_action(
                state,
                ui,
                cx,
                &AppAction::SessionErrorReceived {
                    session_id: session_id.clone(),
                    error: error.clone(),
                },
            );
        }
        _ => {}
    }
}

fn show_next_pending_permission(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx) {
    let _ = (state, ui);
    cx.redraw_all();
}
