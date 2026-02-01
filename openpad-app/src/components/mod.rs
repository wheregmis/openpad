//! UI Components module
//!
//! This module contains all the UI widget definitions and implementations.

pub mod assistant_bubble;
pub mod diff_view;
pub mod message_list;
pub mod permission_dialog;
pub mod projects_panel;
pub mod terminal;
pub mod user_bubble;

// Re-export the widget types that have Rust implementations
pub use diff_view::{DiffView, DiffViewApi};
pub use message_list::{MessageList, MessageListRef};
pub use permission_dialog::{PermissionDialog, PermissionDialogRef};
pub use projects_panel::{PanelItemKind, ProjectsPanel, ProjectsPanelRef};
pub use terminal::{Terminal, TerminalAction, TerminalWidgetRefExt};
