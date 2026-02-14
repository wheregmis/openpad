//! UI Components module
//!
//! This module contains all the UI widget definitions and implementations.

pub mod projects_panel;
pub mod session_context_menu;
pub mod session_options_popup;
pub mod sidebar_header;

// Re-export the widget types that have Rust implementations
pub use projects_panel::{PanelItemKind, ProjectsPanel, ProjectsPanelRef};
pub use session_context_menu::SessionContextMenu;
pub use session_options_popup::{SessionOptionsPopup, SessionOptionsPopupRef};
pub use sidebar_header::SidebarHeader;
