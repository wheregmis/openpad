use crate::constants::*;
use makepad_widgets::*;

/// Updates the status indicator UI (dot color and label text)
pub fn update_status_indicator(
    ui: &WidgetRef,
    cx: &mut Cx,
    status_text: &str,
    color: Vec4,
) {
    ui.label(id!(status_label)).set_text(cx, status_text);
    ui.view(id!(status_dot)).apply_over(
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

/// Updates the session title label with appropriate styling
pub fn update_session_title_ui(
    ui: &WidgetRef,
    cx: &mut Cx,
    title: &str,
    is_active: bool,
) {
    let color = if is_active {
        COLOR_TEXT_TITLE_ACTIVE
    } else {
        COLOR_TEXT_TITLE_INACTIVE
    };
    
    ui.label(id!(session_title)).set_text(cx, title);
    ui.label(id!(session_title)).apply_over(
        cx,
        live! {
            draw_text: { color: (color) }
        },
    );
}

/// Updates the revert indicator visibility based on session revert state
pub fn update_revert_indicator(
    ui: &WidgetRef,
    cx: &mut Cx,
    is_reverted: bool,
) {
    ui.view(id!(revert_indicator)).set_visible(cx, is_reverted);
    ui.view(id!(unrevert_wrap)).set_visible(cx, is_reverted);
}
