use makepad_widgets::*;

pub mod app_bg;
pub mod hamburger_button;
pub mod header_bar;
pub mod input_bar;
pub mod input_field;
pub mod openpad;
pub mod scrollable_popup_menu;
pub mod send_button;
pub mod side_panel;
pub mod simple_dialog;
pub mod status_dot;
pub mod theme;
pub mod upward_dropdown;

// Generic components moved from openpad-app
pub mod assistant_bubble;
pub mod colored_diff_text;
pub mod diff_view;
pub mod message_list;
pub mod message_logic;
pub mod permission_card;
pub mod permission_dialog;
pub mod settings_dialog;
pub mod terminal;
pub mod terminal_panel;
pub mod user_bubble;
pub mod utils;

// Re-export types
pub use message_list::{MessageList, MessageListAction, MessageListRef};
pub use permission_dialog::{PermissionDialog, PermissionDialogAction, PermissionDialogRef};
pub use settings_dialog::{SettingsDialog, SettingsDialogAction, SettingsDialogRef};
pub use side_panel::{SidePanel, SidePanelRef, SidePanelWidgetRefExt};
pub use simple_dialog::{SimpleDialog, SimpleDialogAction, SimpleDialogRef};
pub use terminal::{Terminal, TerminalRef};
pub use terminal_panel::{TerminalPanel, TerminalPanelRef};
pub use upward_dropdown::{UpDropDown, UpDropDownRef, UpDropDownWidgetRefExt};

pub fn script_mod(vm: &mut ScriptVm) {
    makepad_widgets::script_mod(vm);
    makepad_studio::script_mod(vm);

    crate::theme::script_mod(vm);

    crate::app_bg::script_mod(vm);
    crate::hamburger_button::script_mod(vm);
    crate::header_bar::script_mod(vm);
    crate::input_field::script_mod(vm);
    crate::send_button::script_mod(vm);
    crate::side_panel::script_mod(vm);
    crate::simple_dialog::script_mod(vm);
    crate::scrollable_popup_menu::script_mod(vm);
    crate::status_dot::script_mod(vm);
    crate::upward_dropdown::script_mod(vm);
    crate::input_bar::script_mod(vm);

    crate::user_bubble::script_mod(vm);
    crate::assistant_bubble::script_mod(vm);
    crate::permission_card::script_mod(vm);
    crate::colored_diff_text::script_mod(vm);
    crate::diff_view::script_mod(vm);
    crate::terminal::script_mod(vm);
    crate::terminal_panel::script_mod(vm);
    crate::message_list::script_mod(vm);
    crate::permission_dialog::script_mod(vm);
    crate::settings_dialog::script_mod(vm);
}
