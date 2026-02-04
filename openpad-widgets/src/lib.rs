use makepad_widgets::*;

pub mod app_bg;
pub mod hamburger_button;
pub mod header_bar;
pub mod input_bar;
pub mod input_field;
pub mod send_button;
pub mod side_panel;
pub mod simple_dialog;
pub mod scrollable_popup_menu;
pub mod status_dot;
pub mod theme;
pub mod upward_dropdown;
pub mod openpad;

// Generic components moved from openpad-app
pub mod user_bubble;
pub mod assistant_bubble;
pub mod permission_card;
pub mod colored_diff_text;
pub mod diff_view;
pub mod terminal;
pub mod terminal_panel;
pub mod message_list;
pub mod message_logic;
pub mod permission_dialog;
pub mod settings_dialog;
pub mod utils;

// Re-export types
pub use side_panel::{SidePanel, SidePanelRef, SidePanelWidgetRefExt};
pub use upward_dropdown::{UpDropDown, UpDropDownRef, UpDropDownWidgetRefExt};
pub use simple_dialog::{SimpleDialog, SimpleDialogAction, SimpleDialogRef};
pub use terminal::{Terminal, TerminalRef, TerminalAction};
pub use terminal_panel::{TerminalPanel, TerminalPanelRef};
pub use message_list::{MessageList, MessageListRef, MessageListAction};
pub use permission_dialog::{PermissionDialog, PermissionDialogRef, PermissionDialogAction};
pub use settings_dialog::{SettingsDialog, SettingsDialogRef, SettingsDialogAction};

pub fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    crate::app_bg::live_design(cx);
    crate::theme::live_design(cx);
    crate::simple_dialog::live_design(cx);
    crate::scrollable_popup_menu::live_design(cx);
    crate::upward_dropdown::live_design(cx);
    crate::openpad::live_design(cx);

    crate::user_bubble::live_design(cx);
    crate::assistant_bubble::live_design(cx);
    crate::permission_card::live_design(cx);
    crate::colored_diff_text::live_design(cx);
    crate::diff_view::live_design(cx);
    crate::terminal::live_design(cx);
    crate::terminal_panel::live_design(cx);
    crate::message_list::live_design(cx);
    crate::permission_dialog::live_design(cx);
    crate::settings_dialog::live_design(cx);
}
