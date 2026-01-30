//! UI Components module
//!
//! This module contains all the UI widget definitions and implementations.

pub mod app_bg;
pub mod assistant_bubble;
pub mod projects_panel;
pub mod user_bubble;

// Re-export the widget types that have Rust implementations
pub use projects_panel::{PanelItemKind, ProjectsPanel, ProjectsPanelRef};
