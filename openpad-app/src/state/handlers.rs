use super::{AppState, ModelDropdownEntry};
use crate::state::actions::AppAction;
use crate::ui::state_updates;
use makepad_widgets::*;
use openpad_protocol::{Event as OcEvent, Provider};
use openpad_widgets::settings_dialog::SettingsDialogWidgetRefExt;
use openpad_widgets::UpDropDownWidgetRefExt;

#[allow(dead_code)]
fn build_model_entries(providers: &[Provider]) -> Vec<ModelDropdownEntry> {
    /// Helper to get display label, preferring name over id
    fn display_label(name: Option<&str>, id: &str) -> String {
        name.unwrap_or(id).to_string()
    }

    let mut entries = vec![ModelDropdownEntry::default_option()];
    for provider in providers {
        let models = &provider.models;
        if models.is_empty() {
            continue;
        }
        let provider_label = display_label(Some(provider.name.as_str()), &provider.id);
        let mut model_items: Vec<_> = models.iter().collect();
        model_items.sort_by(|a, b| {
            let a_label = display_label(Some(a.1.name.as_str()), &a.1.id);
            let b_label = display_label(Some(b.1.name.as_str()), &b.1.id);
            a_label.cmp(&b_label)
        });
        entries.push(ModelDropdownEntry::provider_header(provider_label.clone()));
        for (_key, model) in model_items {
            let model_label = format!("  {}", display_label(Some(model.name.as_str()), &model.id));
            entries.push(ModelDropdownEntry::model_option(
                provider.id.clone(),
                model.id.clone(),
                model_label,
            ));
        }
    }
    entries
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
                .set_config(cx, config);
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
            handle_app_action(
                state,
                ui,
                cx,
                &AppAction::SessionDeleted(session.id.clone()),
            );
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
