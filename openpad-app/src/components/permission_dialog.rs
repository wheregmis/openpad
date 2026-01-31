use crate::actions::AppAction;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;

    pub PermissionDialog = {{PermissionDialog}} {
        width: 400, height: Fit
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
                text_style: <THEME_FONT_BOLD> { font_size: 14 }
            }
        }

        description = <Label> {
            width: Fill, height: Fit
            text: ""
            draw_text: {
                color: #aab3bd
                text_style: { font_size: 11 }
                wrap: Word
            }
        }

        details_view = <RoundedView> {
            width: Fill, height: Fit
            padding: 12,
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
                text: ""
                draw_text: {
                    color: #6b7b8c
                    text_style: { font_size: 10, font_family: "Mono" }
                    wrap: Word
                }
            }

            context = <Label> {
                width: Fill, height: Fit
                text: ""
                draw_text: {
                    color: #8fa0b3
                    text_style: { font_size: 10 }
                    wrap: Word
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

            always_button = <Button> {
                width: 130, height: 36
                text: "Always allow"
                draw_bg: {
                    color: #334155
                    color_hover: #475569
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
            if let Some(permission_id) = self.get_request_id() {
                cx.action(AppAction::PermissionResponded {
                    request_id: permission_id,
                    reply: openpad_protocol::PermissionReply::Once,
                });
            }
            self.hide(cx);
        }

        if self.view.button(id!(reject_button)).clicked(&actions) {
            if let Some(permission_id) = self.get_request_id() {
                cx.action(AppAction::PermissionResponded {
                    request_id: permission_id,
                    reply: openpad_protocol::PermissionReply::Reject,
                });
            }
            self.hide(cx);
        }

        if self.view.button(id!(always_button)).clicked(&actions) {
            if let Some(permission_id) = self.get_request_id() {
                cx.action(AppAction::PermissionResponded {
                    request_id: permission_id,
                    reply: openpad_protocol::PermissionReply::Always,
                });
            }
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
        patterns: Vec<String>,
        context: Option<String>,
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
        let patterns_text = if patterns.is_empty() {
            "Patterns: (none)".to_string()
        } else {
            format!("Patterns:\n{}", patterns.join("\n"))
        };
        self.view.label(id!(pattern)).set_text(cx, &patterns_text);
        if let Some(context_text) = context.filter(|text| !text.trim().is_empty()) {
            self.view
                .label(id!(context))
                .set_text(cx, &format!("Context:\n{}", context_text));
            self.view.widget(id!(context)).set_visible(cx, true);
        } else {
            self.view.label(id!(context)).set_text(cx, "");
            self.view.widget(id!(context)).set_visible(cx, false);
        }

        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.session_id = None;
        self.permission_id = None;
        self.redraw(cx);
    }

    pub fn get_request_id(&self) -> Option<String> {
        self.permission_id.clone()
    }
}

impl PermissionDialogRef {
    pub fn show_permission_request(
        &self,
        cx: &mut Cx,
        session_id: String,
        permission_id: String,
        permission: String,
        patterns: Vec<String>,
        context: Option<String>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_permission_request(
                cx,
                session_id,
                permission_id,
                permission,
                patterns,
                context,
            );
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    pub fn get_request_id(&self) -> Option<String> {
        self.borrow().and_then(|inner| inner.get_request_id())
    }
}
