//! Session context menu shown as an app-level popup to avoid redrawing the entire
//! projects list when opening the menu (which was causing a multi-second delay).

use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.SessionContextMenu = #(SessionContextMenu::register_widget(vm)) {
        width: Fill, height: Fill
        flow: Overlay
        visible: false

        backdrop := View {
            width: Fill, height: Fill
            show_bg: true
            draw_bg +: {
                color: #00000001
            }
        }

        menu_box := RoundedView {
            width: Fit, height: Fit
            flow: Down
            spacing: 0
            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
            show_bg: true
            draw_bg +: {
                color: #313843
                border_radius: 7.0
                border_size: 1.0
                border_color: #333
            }

            menu_collapse := Button {
                width: 22, height: 22
                text: "〉"
                align: Align{ x: 0.5, y: 0.5 }
                draw_bg +: {
                    color: #0000
                    color_hover: #333
                    border_radius: 4.0
                    border_size: 0.0
                }
                draw_text +: { color: #e6e9ee, text_style: theme.font_bold { font_size: 10 } }
            }

            menu_rename := Button {
                width: Fit, height: 22
                text: "Rename"
                draw_bg +: {
                    color: #0000
                    color_hover: #333
                    border_radius: 4.0
                    border_size: 0.0
                }
                draw_text +: { color: #e6e9ee, text_style: theme.font_regular { font_size: 10 } }
            }

            menu_branch := Button {
                width: Fit, height: 22
                text: "Branch"
                draw_bg +: {
                    color: #0000
                    color_hover: #333
                    border_radius: 4.0
                    border_size: 0.0
                }
                draw_text +: { color: #e6e9ee, text_style: theme.font_regular { font_size: 10 } }
            }

            menu_abort := Button {
                width: Fit, height: 22
                text: "Abort"
                visible: true
                draw_bg +: {
                    color: #0000
                    color_hover: #ef4444
                    border_radius: 4.0
                    border_size: 0.0
                }
                draw_text +: { color: #e6e9ee, text_style: theme.font_regular { font_size: 10 } }
            }

            menu_delete := Button {
                width: Fit, height: 22
                text: "Delete"
                draw_bg +: {
                    color: #0000
                    color_hover: #ef4444
                    border_radius: 4.0
                    border_size: 0.0
                }
                draw_text +: { color: #e6e9ee, text_style: theme.font_regular { font_size: 10 } }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct SessionContextMenu {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[rust]
    session_id: Option<String>,
    #[rust]
    working: bool,
}

impl Widget for SessionContextMenu {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.session_id.is_none() {
            return;
        }
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });
        let session_id = match &self.session_id {
            Some(s) => s.clone(),
            None => return,
        };

        if self.view.button(cx, &[id!(menu_collapse)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(menu_rename)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::RenameSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(menu_branch)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::BranchSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(menu_abort)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::AbortSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(menu_delete)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::DeleteSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        }
    }
}

impl SessionContextMenuRef {
    pub fn open(&self, cx: &mut Cx, session_id: String, _x: f32, _y: f32, working: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.session_id = Some(session_id);
            inner.working = working;
            inner
                .view
                .view(cx, &[id!(menu_box), id!(menu_abort)])
                .set_visible(cx, working);
            inner.redraw(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.session_id = None;
            inner.redraw(cx);
        }
    }
}
