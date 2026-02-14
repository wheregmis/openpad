//! UI Components module
//!
//! This module contains all the UI widget definitions and implementations.

pub mod projects_panel;
pub mod sidebar_header;

// Re-export the widget types that have Rust implementations
pub use projects_panel::{PanelItemKind, ProjectsPanel, ProjectsPanelRef};
pub use sidebar_header::SidebarHeader;
