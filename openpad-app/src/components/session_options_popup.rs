//! Small session-options popup using the same overlay pattern as SimpleDialog
//! so it reliably appears and receives clicks.

use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.SessionOptionsPopup = #(SessionOptionsPopup::register_widget(vm)) {
        width: Fill, height: Fill
        flow: Overlay
        visible: false

        overlay := View {
            width: Fill, height: Fill
            show_bg: true
            draw_bg +: {
                color: #00000099
            }
        }

        View {
            width: Fill, height: Fill
            align: Align{ x: 0.5, y: 0.5 }

            box := View {
                width: 240, height: Fit
                flow: Down
                padding: Inset{ left: 16, right: 16, top: 14, bottom: 14 }
                spacing: 10
                show_bg: true
                draw_bg +: {
                    color: #252a33
                    border_color: #e6e9ee
                    border_radius: 10.0
                    border_size: 2.0
                }

                title_label := Label {
                    text: "Session options"
                    draw_text +: {
                        color: #ffffff
                        text_style: theme.font_bold { font_size: 14 }
                    }
                }

                rename_btn := Button {
                    width: Fill, height: 36
                    text: "Rename"
                    draw_bg +: {
                        color: #2a2f36
                        color_hover: #3b82f6
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text +: { color: #ffffff, text_style: theme.font_regular { font_size: 12 } }
                }

                branch_btn := Button {
                    width: Fill, height: 36
                    text: "Branch"
                    draw_bg +: {
                        color: #2a2f36
                        color_hover: #3b82f6
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text +: { color: #ffffff, text_style: theme.font_regular { font_size: 12 } }
                }

                abort_btn := Button {
                    width: Fill, height: 36
                    visible: true
                    text: "Abort"
                    draw_bg +: {
                        color: #2a2f36
                        color_hover: #ef4444
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text +: { color: #ffffff, text_style: theme.font_regular { font_size: 12 } }
                }

                delete_btn := Button {
                    width: Fill, height: 36
                    text: "Delete"
                    draw_bg +: {
                        color: #2a2f36
                        color_hover: #ef4444
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text +: { color: #ffffff, text_style: theme.font_regular { font_size: 12 } }
                }

                cancel_btn := Button {
                    width: Fill, height: 36
                    text: "Cancel"
                    draw_bg +: {
                        color: #444
                        color_hover: #555
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text +: { color: #ffffff, text_style: theme.font_regular { font_size: 12 } }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct SessionOptionsPopup {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[rust]
    session_id: Option<String>,
    #[rust]
    working: bool,
}

impl Widget for SessionOptionsPopup {
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

        if self.view.button(cx, &[id!(rename_btn)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::RenameSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(branch_btn)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::BranchSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(abort_btn)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::AbortSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(delete_btn)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::DeleteSession(session_id));
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        } else if self.view.button(cx, &[id!(cancel_btn)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
        }
    }
}

impl SessionOptionsPopupRef {
    pub fn show(&self, cx: &mut Cx, session_id: String, working: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.session_id = Some(session_id);
            inner.working = working;
            inner
                .view
                .view(cx, &[id!(box), id!(abort_btn)])
                .set_visible(cx, working);
            inner.view.set_visible(cx, true);
            inner.redraw(cx);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.session_id = None;
            inner.view.set_visible(cx, false);
            inner.redraw(cx);
        }
    }
}
