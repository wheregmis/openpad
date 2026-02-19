pub mod actions;
pub mod effect_executor;
pub mod effects;
pub mod handlers;
pub mod reducer;

pub use actions::{AppAction, ProjectsPanelAction, SidebarMode};
pub use effect_executor::execute_state_effects;
pub use effects::StateEffect;
pub use handlers::{handle_app_action, handle_opencode_event};
pub use reducer::{
    reduce_app_state, resolve_pending_center_intent, upsert_file_tab, upsert_session_tab,
    UnsavedDecision,
};

use crate::async_runtime::tasks;
use crate::components::files_panel::FilesPanelWidgetRefExt;
use crate::components::sessions_panel::SessionsPanelWidgetRefExt;
use crate::constants::*;
use crate::ui::state_updates;
use makepad_widgets::*;
use openpad_protocol::{
    Agent, MessageWithParts, ModelSpec, PermissionRequest, PermissionRuleset, Project, Provider,
    Session, Skill,
};
use std::collections::HashMap;

// ── Types ────────────────────────────────────────────────────────────────────

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

// ── AppState ─────────────────────────────────────────────────────────────────

/// Data structure holding application state for event handling
#[derive(Default)]
pub struct AppState {
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
                    for model in provider_models.values() {
                        let model_label = model.name.clone();
                        models.push((provider.id.clone(), model.id.clone(), model_label));
                    }
                }
            }
        } else if let Some(provider) = self.providers.get(self.selected_provider_idx - 1) {
            // Specific provider selected
            if let Some(provider_models) = provider.models.as_ref() {
                for model in provider_models.values() {
                    let model_label = model.name.clone();
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
            .map(|agent| agent.permission.clone())
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
        let project = project_for_session.or(self.current_project.as_ref());
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
