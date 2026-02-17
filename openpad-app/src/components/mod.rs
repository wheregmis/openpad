//! UI Components module
//!
//! This module contains all the UI widget definitions and implementations.

pub mod editor_panel;
pub mod files_panel;
pub mod projects_panel;
pub mod session_context_menu;
pub mod session_options_popup;
pub mod sessions_panel;
pub mod sidebar_header;

// Re-export the widget types that have Rust implementations
pub use editor_panel::{EditorPanel, EditorPanelAction, EditorPanelRef};
pub use files_panel::{FilesPanel, FilesPanelRef};
pub use projects_panel::{PanelItemKind, ProjectsPanel, ProjectsPanelRef};
pub use session_context_menu::SessionContextMenu;
pub use session_options_popup::{SessionOptionsPopup, SessionOptionsPopupRef};
pub use sessions_panel::{SessionsPanel, SessionsPanelRef};
pub use sidebar_header::SidebarHeader;
