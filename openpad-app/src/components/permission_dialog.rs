use crate::actions::AppAction;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;

    pub PermissionDialog = {{PermissionDialog}} {
        visible: false
        width: Fill, height: Fill

        modal = <RoundedView> {
            width: 400, height: Fit
            align: { x: 0.5, y: 0.5 }
            padding: 20,
            spacing: 16,

            draw_bg: {
                color: #1c2026
                border_color: #2b3138
                border_radius: 12.0
                border_size: 1.0
            }

            title = <Label> {
                text: "Permission Request"
                draw_text: {
                    color: #e6e9ee
                    text_style: { font_size: 14, font_weight: 600 }
                }
            }

            description = <Label> {
                width: Fill, height: Fit
                wrap: Word
                text: ""
                draw_text: {
                    color: #aab3bd
                    text_style: { font_size: 11 }
                }
            }

            details_view = <View> {
                width: Fill, height: Fit
                padding: 12,
                show_bg: true
                draw_bg: {
                    color: #14161a
                    border_radius: 8.0
                }

                permission_type = <Label> {
                    width: Fill, height: Fit
                    text: ""
                    draw_text: {
                        color: #e6e9ee
                        text_style: { font_size: 11 }
                    }
                }

                pattern = <Label> {
                    width: Fill, height: Fit
                    wrap: Word
                    text: ""
                    draw_text: {
                        color: #6b7b8c
                        text_style: { font_size: 10, font_family: "Mono" }
                    }
                }
            }

            buttons_row = <View> {
                width: Fill, height: Fit
                flow: Right,
                spacing: 12,
                align: { x: 1.0 }

                reject_button = <Button> {
                    width: 100, height: 36
                    text: "Reject"
                    draw_bg: {
                        color: #2a2f36
                        color_hover: #313843
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text: { color: #e6e9ee }
                }

                accept_button = <Button> {
                    width: 100, height: 36
                    text: "Accept"
                    draw_bg: {
                        color: #3b82f6
                        color_hover: #1d4ed8
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text: { color: #ffffff }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct PermissionDialog {
    #[deref]
    view: View,

    #[rust]
    session_id: Option<String>,
    #[rust]
    permission_id: Option<String>,
}

impl Widget for PermissionDialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(id!(accept_button)).clicked(&actions) {
            cx.action(AppAction::PermissionResponded(true));
            self.hide(cx);
        }

        if self.view.button(id!(reject_button)).clicked(&actions) {
            cx.action(AppAction::PermissionResponded(false));
            self.hide(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl PermissionDialog {
    pub fn show_permission_request(
        &mut self,
        cx: &mut Cx,
        session_id: String,
        permission_id: String,
        permission: String,
        pattern: String,
    ) {
        self.session_id = Some(session_id);
        self.permission_id = Some(permission_id);

        let description = format!(
            "The session is requesting {} permission for:",
            permission.to_uppercase()
        );

        self.view.label(id!(description)).set_text(cx, &description);
        self.view
            .label(id!(permission_type))
            .set_text(cx, &format!("Type: {}", permission));
        self.view.label(id!(pattern)).set_text(cx, &pattern);

        self.view.set_visible(cx, true);
        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.session_id = None;
        self.permission_id = None;
        self.view.set_visible(cx, false);
        self.redraw(cx);
    }

    pub fn get_permission_info(&self) -> Option<(String, String)> {
        match (&self.session_id, &self.permission_id) {
            (Some(sid), Some(pid)) => Some((sid.clone(), pid.clone())),
            _ => None,
        }
    }
}

impl PermissionDialogRef {
    pub fn show_permission_request(
        &self,
        cx: &mut Cx,
        session_id: String,
        permission_id: String,
        permission: String,
        pattern: String,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_permission_request(cx, session_id, permission_id, permission, pattern);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    pub fn get_permission_info(&self) -> Option<(String, String)> {
        self.borrow().and_then(|inner| inner.get_permission_info())
    }
}
