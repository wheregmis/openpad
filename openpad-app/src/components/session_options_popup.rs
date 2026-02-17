//! Small options popup for session/project context actions.

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

        overlay := SolidView {
            width: Fill, height: Fill
            draw_bg +: {
                color: #00000088
            }
        }

        View {
            width: Fill, height: Fill
            align: Align{ x: 0.5, y: 0.5 }

            box := RoundedView {
                width: 260, height: Fit
                flow: Down
                padding: Inset{ left: 10, right: 10, top: 10, bottom: 10 }
                spacing: 6
                new_batch: true
                draw_bg +: {
                    color: #2a313d
                    border_color: #4b5563
                    border_radius: 10.0
                    border_size: 1.0
                }

                title_label := Label {
                    text: "Session options"
                    margin: Inset{ left: 4, right: 4, top: 2, bottom: 6 }
                    draw_text +: {
                        color: #e6e9ee
                        text_style: theme.font_bold { font_size: 12 }
                    }
                }

                primary_btn := Button {
                    width: Fill, height: 30
                    new_batch: true
                    text: "Rename"
                    draw_bg +: {
                        color: #364154
                        color_hover: #50607c
                        color_active: #50607c
                        border_radius: 7.0
                        border_size: 0.0
                    }
                    draw_text +: {
                        color: #f8fafc
                        text_style: theme.font_regular { font_size: 11 }
                    }
                }

                secondary_btn := Button {
                    width: Fill, height: 30
                    new_batch: true
                    text: "Branch"
                    draw_bg +: {
                        color: #364154
                        color_hover: #50607c
                        color_active: #50607c
                        border_radius: 7.0
                        border_size: 0.0
                    }
                    draw_text +: {
                        color: #f8fafc
                        text_style: theme.font_regular { font_size: 11 }
                    }
                }

                abort_btn := Button {
                    width: Fill, height: 30
                    new_batch: true
                    text: "Abort"
                    draw_bg +: {
                        color: #473446
                        color_hover: #ef4444
                        color_active: #ef4444
                        border_radius: 7.0
                        border_size: 0.0
                    }
                    draw_text +: {
                        color: #ffe4e6
                        text_style: theme.font_regular { font_size: 11 }
                    }
                }

                danger_row := RoundedView {
                    width: Fill, height: 30
                    new_batch: true
                    flow: Overlay
                    draw_bg +: {
                        color: #473446
                        border_radius: 7.0
                        border_size: 0.0
                    }

                    View {
                        width: Fill, height: Fill
                        align: Align { x: 0.5, y: 0.5 }
                        danger_label := Label {
                            text: "Delete"
                            draw_text +: {
                                color: #fff3f3
                                text_style: theme.font_regular { font_size: 11 }
                            }
                        }
                    }

                    danger_btn := Button {
                        width: Fill, height: Fill
                        text: ""
                        draw_bg +: {
                            color: #0000
                            color_hover: #ef444433
                            color_active: #ef444466
                            border_radius: 7.0
                            border_size: 0.0
                        }
                        draw_text +: { color: #0000 }
                    }
                }

                close_row := RoundedView {
                    width: Fill, height: 30
                    new_batch: true
                    visible: false
                    flow: Overlay
                    draw_bg +: {
                        color: #473446
                        border_radius: 7.0
                        border_size: 0.0
                    }

                    View {
                        width: Fill, height: Fill
                        align: Align { x: 0.5, y: 0.5 }
                        close_label := Label {
                            text: "Close"
                            draw_text +: {
                                color: #fff3f3
                                text_style: theme.font_regular { font_size: 11 }
                            }
                        }
                    }

                    close_btn := Button {
                        width: Fill, height: Fill
                        text: ""
                        draw_bg +: {
                            color: #0000
                            color_hover: #ef444433
                            color_active: #ef444466
                            border_radius: 7.0
                            border_size: 0.0
                        }
                        draw_text +: { color: #0000 }
                    }
                }

                cancel_row := RoundedView {
                    width: Fill, height: 30
                    new_batch: true
                    flow: Overlay
                    draw_bg +: {
                        color: #3f4756
                        border_radius: 7.0
                        border_size: 0.0
                    }

                    View {
                        width: Fill, height: Fill
                        align: Align { x: 0.5, y: 0.5 }
                        cancel_label := Label {
                            text: "Cancel"
                            draw_text +: {
                                color: #f8fafc
                                text_style: theme.font_regular { font_size: 11 }
                            }
                        }
                    }

                    cancel_btn := Button {
                        width: Fill, height: Fill
                        text: ""
                        draw_bg +: {
                            color: #0000
                            color_hover: #4b556344
                            color_active: #4b556366
                            border_radius: 7.0
                            border_size: 0.0
                        }
                        draw_text +: { color: #0000 }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PopupMode {
    Session,
    Project,
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
    project_id: Option<String>,
    #[rust]
    working: bool,
    #[rust]
    mode: PopupMode,
}

impl Default for PopupMode {
    fn default() -> Self {
        Self::Session
    }
}

impl Widget for SessionOptionsPopup {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.session_id.is_none() && self.project_id.is_none() {
            return;
        }

        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(cx, &[id!(cancel_btn)]).clicked(&actions) {
            cx.action(ProjectsPanelAction::CloseSessionContextMenu);
            return;
        }

        match self.mode {
            PopupMode::Session => {
                let Some(session_id) = self.session_id.clone() else {
                    return;
                };
                if self.view.button(cx, &[id!(primary_btn)]).clicked(&actions) {
                    cx.action(ProjectsPanelAction::RenameSession(session_id));
                    cx.action(ProjectsPanelAction::CloseSessionContextMenu);
                } else if self
                    .view
                    .button(cx, &[id!(secondary_btn)])
                    .clicked(&actions)
                {
                    cx.action(ProjectsPanelAction::BranchSession(session_id));
                    cx.action(ProjectsPanelAction::CloseSessionContextMenu);
                } else if self.view.button(cx, &[id!(abort_btn)]).clicked(&actions) {
                    cx.action(ProjectsPanelAction::AbortSession(session_id));
                    cx.action(ProjectsPanelAction::CloseSessionContextMenu);
                } else if self.view.button(cx, &[id!(danger_btn)]).clicked(&actions) {
                    cx.action(ProjectsPanelAction::DeleteSession(session_id));
                    cx.action(ProjectsPanelAction::CloseSessionContextMenu);
                }
            }
            PopupMode::Project => {
                if self.view.button(cx, &[id!(primary_btn)]).clicked(&actions) {
                    cx.action(ProjectsPanelAction::CreateSession(self.project_id.clone()));
                    cx.action(ProjectsPanelAction::CloseSessionContextMenu);
                } else if self.view.button(cx, &[id!(close_btn)]).clicked(&actions) {
                    cx.action(ProjectsPanelAction::CloseSessionContextMenu);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SessionOptionsPopupRef {
    pub fn show(&self, cx: &mut Cx, session_id: String, working: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.mode = PopupMode::Session;
            inner.session_id = Some(session_id);
            inner.project_id = None;
            inner.working = working;

            inner
                .view
                .label(cx, &[id!(title_label)])
                .set_text(cx, "Session options");
            inner
                .view
                .button(cx, &[id!(primary_btn)])
                .set_text(cx, "Rename");
            inner
                .view
                .button(cx, &[id!(secondary_btn)])
                .set_text(cx, "Branch");
            inner
                .view
                .button(cx, &[id!(secondary_btn)])
                .set_visible(cx, true);
            inner
                .view
                .button(cx, &[id!(abort_btn)])
                .set_visible(cx, working);
            inner
                .view
                .view(cx, &[id!(danger_row)])
                .set_visible(cx, true);
            inner
                .view
                .label(cx, &[id!(danger_label)])
                .set_text(cx, "Delete");
            inner
                .view
                .view(cx, &[id!(close_row)])
                .set_visible(cx, false);
            inner
                .view
                .view(cx, &[id!(cancel_row)])
                .set_visible(cx, true);
            inner
                .view
                .label(cx, &[id!(cancel_label)])
                .set_text(cx, "Cancel");

            inner.view.set_visible(cx, true);
            inner.redraw(cx);
        }
    }

    pub fn show_project(&self, cx: &mut Cx, project_id: Option<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.mode = PopupMode::Project;
            inner.session_id = None;
            inner.project_id = project_id;
            inner.working = false;

            inner
                .view
                .label(cx, &[id!(title_label)])
                .set_text(cx, "Project options");
            inner
                .view
                .button(cx, &[id!(primary_btn)])
                .set_text(cx, "New Session");
            inner
                .view
                .button(cx, &[id!(secondary_btn)])
                .set_visible(cx, false);
            inner
                .view
                .button(cx, &[id!(abort_btn)])
                .set_visible(cx, false);
            inner
                .view
                .view(cx, &[id!(danger_row)])
                .set_visible(cx, false);
            inner.view.view(cx, &[id!(close_row)]).set_visible(cx, true);
            inner
                .view
                .label(cx, &[id!(close_label)])
                .set_text(cx, "Close");
            inner
                .view
                .view(cx, &[id!(cancel_row)])
                .set_visible(cx, true);
            inner
                .view
                .label(cx, &[id!(cancel_label)])
                .set_text(cx, "Cancel");

            inner.view.set_visible(cx, true);
            inner.redraw(cx);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.session_id = None;
            inner.project_id = None;
            inner.view.set_visible(cx, false);
            inner.redraw(cx);
        }
    }
}
