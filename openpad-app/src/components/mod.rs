//! UI Components module
//!
//! This module contains all the UI widget definitions and implementations.

pub mod assistant_bubble;
pub mod colored_diff_text;
pub mod diff_view;
pub mod message_list;
pub mod permission_card;
pub mod permission_dialog;
pub mod projects_panel;
pub mod settings_dialog;
pub mod terminal;
pub mod user_bubble;

// Re-export the widget types that have Rust implementations
pub use colored_diff_text::{ColoredDiffText, ColoredDiffTextApi, ColoredDiffTextWidgetRefExt};
pub use diff_view::{DiffView, DiffViewApi};
pub use message_list::{MessageList, MessageListRef};
pub use permission_card::{PermissionCard, PermissionCardAction, PermissionCardApi};
pub use permission_dialog::{PermissionDialog, PermissionDialogRef};
pub use projects_panel::{PanelItemKind, ProjectsPanel, ProjectsPanelRef};
pub use terminal::{Terminal, TerminalAction, TerminalWidgetRefExt};
pub use settings_dialog::{SettingsDialog, SettingsDialogRef};
