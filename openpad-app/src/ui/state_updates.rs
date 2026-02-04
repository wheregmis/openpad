use crate::constants::*;
use crate::utils::path_utils::normalize_worktree;
use makepad_widgets::*;
use openpad_protocol::{Project, SessionSummary};
use openpad_widgets::message_list::MessageListWidgetRefExt;
use std::path::Path;

/// Updates the status indicator UI (dot color and label text)
pub fn update_status_indicator(ui: &WidgetRef, cx: &mut Cx, status_text: &str, color: Vec4) {
    ui.label(&[id!(status_label)]).set_text(cx, status_text);
    ui.view(&[id!(status_dot)]).apply_over(
        cx,
        live! {
            draw_bg: { color: (color) }
        },
    );
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
    ui.view(&[id!(work_indicator)]).set_visible(cx, working);
    ui.message_list(&[id!(message_list)])
        .set_working(cx, working);
}

/// Updates the session title label with appropriate styling
pub fn update_session_title_ui(ui: &WidgetRef, cx: &mut Cx, title: &str, is_active: bool) {
    let color = if is_active {
        COLOR_TEXT_TITLE_ACTIVE
    } else {
        COLOR_TEXT_TITLE_INACTIVE
    };

    ui.label(&[id!(session_title)]).set_text(cx, title);
    ui.label(&[id!(session_title)]).apply_over(
        cx,
        live! {
            draw_text: { color: (color) }
        },
    );
}

/// Updates the revert indicator visibility based on session revert state
pub fn update_revert_indicator(ui: &WidgetRef, cx: &mut Cx, is_reverted: bool) {
    ui.view(&[id!(revert_indicator)])
        .set_visible(cx, is_reverted);
    ui.view(&[id!(unrevert_wrap)]).set_visible(cx, is_reverted);
}

pub fn update_share_ui(ui: &WidgetRef, cx: &mut Cx, share_url: Option<&str>) {
    let has_url = share_url.is_some();
    ui.label(&[id!(share_url_label)])
        .set_text(cx, share_url.unwrap_or(""));
    ui.button(&[id!(share_button)]).set_visible(cx, !has_url);
    ui.button(&[id!(unshare_button)]).set_visible(cx, has_url);
    ui.button(&[id!(copy_share_button)])
        .set_visible(cx, has_url);
    ui.widget(&[id!(share_url_label)]).set_visible(cx, has_url);
}

pub fn update_summary_ui(ui: &WidgetRef, cx: &mut Cx, summary: Option<&SessionSummary>) {
    let _ = summary;
    ui.view(&[id!(session_summary)]).set_visible(cx, false);
    ui.label(&[id!(summary_stats_label)]).set_text(cx, "");
    ui.widget(&[id!(summary_diff)]).set_text(cx, "");
}

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
    let (badge_text, path_text, badge_color, badge_text_color, path_visible) =
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

    ui.label(&[id!(project_badge_label)])
        .set_text(cx, &badge_text);
    ui.label(&[id!(project_badge_label)]).apply_over(
        cx,
        live! {
            draw_text: { color: (badge_text_color) }
        },
    );

    let display_path = if path_text.len() > 30 {
        format!("...{}", &path_text[path_text.len() - 27..])
    } else {
        path_text.clone()
    };

    ui.label(&[id!(project_path_label)])
        .set_text(cx, &display_path);
    ui.view(&[id!(project_path_wrap)])
        .set_visible(cx, path_visible);
    ui.label(&[id!(project_path_label)]).apply_over(
        cx,
        live! {
            draw_text: { color: (COLOR_PROJECT_PATH_TEXT) }
        },
    );

    ui.view(&[id!(project_badge)]).apply_over(
        cx,
        live! {
            draw_bg: { color: (badge_color) }
        },
    );
}
