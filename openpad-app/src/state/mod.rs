pub mod actions;
pub mod effects;
pub mod handlers;
pub mod reducer;

pub use actions::{AppAction, ProjectsPanelAction, SidebarMode};
pub use effects::StateEffect;
pub use handlers::{
    handle_app_action, handle_opencode_event, handle_permission_responded, AppState, CenterTabKind,
    CenterTabTarget, OpenFileState, PendingCenterIntent,
};
pub use reducer::{
    reduce_app_state, resolve_pending_center_intent, upsert_file_tab, upsert_session_tab,
    UnsavedDecision,
};
