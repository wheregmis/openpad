//! UI Components module
//!
//! This module contains all the UI widget definitions and implementations.

pub mod app_bg;
pub mod assistant_bubble;
pub mod message_list;
pub mod permission_dialog;
pub mod projects_panel;
pub mod simple_dialog;
pub mod user_bubble;

// Re-export the widget types that have Rust implementations
pub use message_list::{MessageList, MessageListRef};
pub use permission_dialog::{PermissionDialog, PermissionDialogRef};
pub use projects_panel::{PanelItemKind, ProjectsPanel, ProjectsPanelRef};
pub use simple_dialog::{SimpleDialog, SimpleDialogRef};
