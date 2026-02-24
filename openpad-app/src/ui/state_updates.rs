use crate::constants::*;
use crate::utils::path_utils::normalize_worktree;
use makepad_widgets::*;
use openpad_protocol::{Project, SessionSummary};
use openpad_widgets::status_dot::StatusDotWidgetRefExt;
use std::path::Path;

/// Updates the status indicator UI (dot color and label text)
pub fn update_status_indicator(ui: &WidgetRef, cx: &mut Cx, status_text: &str, color: Vec4) {
    ui.label(cx, &[id!(status_label)]).set_text(cx, status_text);
    // Update the status dot color using the new setter
    ui.status_dot(cx, &[id!(status_dot)]).set_color(cx, color);
}

/// Sets status to connected
pub fn set_status_connected(ui: &WidgetRef, cx: &mut Cx) {
    update_status_indicator(ui, cx, STATUS_CONNECTED, COLOR_STATUS_CONNECTED);
}

/// Sets status to disconnected
pub fn set_status_disconnected(ui: &WidgetRef, cx: &mut Cx) {
    update_status_indicator(ui, cx, STATUS_DISCONNECTED, COLOR_STATUS_DISCONNECTED);
}

/// Sets status to error with message
pub fn set_status_error(ui: &WidgetRef, cx: &mut Cx, error: &str) {
    let msg = format!("{}{}", STATUS_ERROR_PREFIX, error);
    update_status_indicator(ui, cx, &msg, COLOR_STATUS_ERROR);
}

pub fn update_work_indicator(ui: &WidgetRef, cx: &mut Cx, working: bool) {
    ui.view(cx, &[id!(work_indicator)]).set_visible(cx, working);
    if !working {
        ui.label(cx, &[id!(work_label)]).set_text(cx, "Working...");
    }
}

/// Updates the session title label with appropriate styling
pub fn update_session_title_ui(ui: &WidgetRef, cx: &mut Cx, title: &str, is_active: bool) {
    let _color = if is_active {
        COLOR_TEXT_TITLE_ACTIVE
    } else {
        COLOR_TEXT_TITLE_INACTIVE
    };
    let marker = if is_active { "●" } else { "○" };
    ui.label(cx, &[id!(session_title)])
        .set_text(cx, &format!("{marker} {title}"));
}

/// Updates the revert indicator visibility based on session revert state
pub fn update_revert_indicator(ui: &WidgetRef, cx: &mut Cx, is_reverted: bool) {
    ui.view(cx, &[id!(revert_indicator)])
        .set_visible(cx, is_reverted);
    ui.view(cx, &[id!(unrevert_wrap)])
        .set_visible(cx, is_reverted);
}

pub fn update_share_ui(ui: &WidgetRef, cx: &mut Cx, share_url: Option<&str>) {
    let has_url = share_url.is_some();
    ui.label(cx, &[id!(share_url_label)])
        .set_text(cx, share_url.unwrap_or(""));
    ui.button(cx, &[id!(share_button)])
        .set_visible(cx, !has_url);
    ui.button(cx, &[id!(unshare_button)])
        .set_visible(cx, has_url);
    ui.button(cx, &[id!(copy_share_button)])
        .set_visible(cx, has_url);
    ui.widget(cx, &[id!(share_url_label)])
        .set_visible(cx, has_url);
}

pub fn update_summary_ui(ui: &WidgetRef, cx: &mut Cx, summary: Option<&SessionSummary>) {
    let _ = (ui, cx, summary);
}

#[allow(dead_code)]
fn build_summary_markdown(summary: &SessionSummary) -> String {
    if summary.diffs.is_empty() {
        return "_No diff details available._".to_string();
    }
    let mut out = String::new();
    for diff in &summary.diffs {
        out.push_str(&format!("#### {}\n\n", diff.file));
        out.push_str("**Before**\n");
        out.push_str("```text\n");
        out.push_str(&diff.before);
        if !diff.before.ends_with('\n') {
            out.push('\n');
        }
        out.push_str("```\n\n");
        out.push_str("**After**\n");
        out.push_str("```text\n");
        out.push_str(&diff.after);
        if !diff.after.ends_with('\n') {
            out.push('\n');
        }
        out.push_str("```\n\n");
    }
    out
}

fn project_display_name_and_path(project: &Project) -> (String, String) {
    let normalized_path = normalize_worktree(&project.worktree);
    let display_name = project
        .name
        .as_ref()
        .and_then(|name| {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .unwrap_or_else(|| {
            Path::new(&normalized_path)
                .file_name()
                .and_then(|segment| segment.to_str())
                .filter(|segment| !segment.is_empty())
                .map(|segment| segment.to_string())
                .unwrap_or_else(|| normalized_path.clone())
        });
    (display_name, normalized_path)
}

/// Updates the project context strip (badge + path) shown above the session title
pub fn update_project_context_ui(ui: &WidgetRef, cx: &mut Cx, project: Option<&Project>) {
    let (badge_text, path_text, _badge_color, _badge_text_color, path_visible) =
        if let Some(project) = project {
            let (name, path) = project_display_name_and_path(project);
            (
                name,
                path.clone(),
                COLOR_PROJECT_BADGE_ACTIVE,
                COLOR_PROJECT_BADGE_TEXT_ACTIVE,
                true,
            )
        } else {
            (
                PROJECT_CONTEXT_NO_PROJECT.to_string(),
                String::new(),
                COLOR_PROJECT_BADGE_DEFAULT,
                COLOR_PROJECT_BADGE_TEXT_INACTIVE,
                false,
            )
        };

    let badge_prefix = if project.is_some() { "📁" } else { "◌" };
    ui.label(cx, &[id!(project_badge_label)])
        .set_text(cx, &format!("{badge_prefix} {badge_text}"));

    let display_path = if path_text.len() > 30 {
        format!("...{}", &path_text[path_text.len() - 27..])
    } else {
        path_text.clone()
    };

    ui.label(cx, &[id!(project_path_label)])
        .set_text(cx, &format!("↳ {display_path}"));
    ui.view(cx, &[id!(project_path_wrap)])
        .set_visible(cx, path_visible);
}
