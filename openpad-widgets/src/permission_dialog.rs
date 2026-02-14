use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.PermissionDialog = #(PermissionDialog::register_widget(vm)) {
        width: Fill
        height: Fit
        flow: Down
        padding: {left: 14 right: 14 top: 12 bottom: 12}
        spacing: 10
        visible: false
        show_bg: true

        draw_bg +: {
            color: uniform(THEME_COLOR_BG_DIALOG)
            border_color: uniform(THEME_COLOR_BORDER_DIALOG)
            border_radius: uniform(10.0)
            border_size: uniform(1.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius)
                sdf.fill_keep(self.color)
                sdf.stroke(self.border_color, self.border_size)
                return sdf.result
            }
        }

        header_row = View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 8
            align: {y: 0.5}

            title = Label {
                text: "Permission required"
                draw_text: {
                    color: THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_bold {font_size: 12}
                }
            }

            View {width: Fill}

            permission_badge = RoundedView {
                width: Fit
                height: Fit
                padding: {left: 8 right: 8 top: 4 bottom: 4}
                draw_bg: {
                    color: THEME_COLOR_SHADE_1
                    border_radius: 10.0
                }

                permission_type = Label {
                    width: Fit
                    height: Fit
                    text: ""
                    draw_text: {
                        color: THEME_COLOR_SHADE_10
                        text_style: theme.font_regular {font_size: 10}
                    }
                }
            }
        }

        description = Label {
            width: Fill
            height: Fit
            text: ""
            draw_text: {
                color: THEME_COLOR_TEXT_DIM
                text_style: theme.font_regular {font_size: 11}
                wrap: Word
            }
        }

        details_view = RoundedView {
            width: Fill
            height: 200
            padding: 10
            draw_bg: {
                color: THEME_COLOR_BG_INPUT
                border_radius: 8.0
            }

            ScrollYView {
                width: Fill
                height: Fill

                View {
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 6

                    pattern = Label {
                        width: Fill
                        height: Fit
                        text: ""
                        draw_text: {
                            color: THEME_COLOR_STATUS_DOT
                            text_style: theme.font_code {font_size: 10}
                            wrap: Word
                        }
                    }

                    context = Label {
                        width: Fill
                        height: Fit
                        text: ""
                        draw_text: {
                            color: THEME_COLOR_SHADE_7
                            text_style: theme.font_regular {font_size: 10}
                            wrap: Word
                        }
                    }
                }
            }
        }

        buttons_row = View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            align: {x: 1.0 y: 0.5}

            reject_button = Button {
                width: 90
                height: 32
                text: "Reject"
                draw_bg: {
                    color: THEME_COLOR_SHADE_3
                    color_hover: THEME_COLOR_SHADE_5
                    border_radius: 8.0
                    border_size: 0.0
                }
                draw_text: {
                    color: THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular {font_size: 11}
                }
            }

            always_button = Button {
                width: 120
                height: 32
                text: "Always allow"
                draw_bg: {
                    color: THEME_COLOR_SHADE_4
                    color_hover: THEME_COLOR_SHADE_6
                    border_radius: 8.0
                    border_size: 0.0
                }
                draw_text: {
                    color: THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular {font_size: 11}
                }
            }

            accept_button = Button {
                width: 110
                height: 32
                text: "Allow once"
                draw_bg: {
                    color: THEME_COLOR_ACCENT_BLUE
                    color_hover: THEME_COLOR_ACCENT_BLUE_DARK
                    border_radius: 8.0
                    border_size: 0.0
                }
                draw_text: {
                    color: THEME_COLOR_TEXT_BRIGHT
                    text_style: theme.font_regular {font_size: 11}
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum PermissionDialogAction {
    #[default]
    None,
    Responded {
        session_id: String,
        request_id: String,
        reply: openpad_protocol::PermissionReply,
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct PermissionDialog {
    #[source]
    source: ScriptObjectRef,

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

        if self.view.button(cx, &[id!(accept_button)]).clicked(&actions) {
            if let (Some(session_id), Some(permission_id)) =
                (self.session_id.clone(), self.get_request_id())
            {
                cx.action(PermissionDialogAction::Responded {
                    session_id,
                    request_id: permission_id,
                    reply: openpad_protocol::PermissionReply::Once,
                });
            }
            self.hide(cx);
        }

        if self.view.button(cx, &[id!(reject_button)]).clicked(&actions) {
            if let (Some(session_id), Some(permission_id)) =
                (self.session_id.clone(), self.get_request_id())
            {
                cx.action(PermissionDialogAction::Responded {
                    session_id,
                    request_id: permission_id,
                    reply: openpad_protocol::PermissionReply::Reject,
                });
            }
            self.hide(cx);
        }

        if self.view.button(cx, &[id!(always_button)]).clicked(&actions) {
            if let (Some(session_id), Some(permission_id)) =
                (self.session_id.clone(), self.get_request_id())
            {
                cx.action(PermissionDialogAction::Responded {
                    session_id,
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
            "This session is requesting {} access:",
            permission.to_uppercase()
        );

        self.view
            .label(cx, &[id!(description)])
            .set_text(cx, &description);
        self.view
            .label(cx, &[id!(permission_type)])
            .set_text(cx, &permission.to_uppercase());

        let patterns_text = if patterns.is_empty() {
            "Patterns: (none)".to_string()
        } else {
            format!("Patterns:\n{}", patterns.join("\n"))
        };
        self.view
            .label(cx, &[id!(pattern)])
            .set_text(cx, &patterns_text);

        if let Some(context_text) = context.filter(|text| !text.trim().is_empty()) {
            self.view
                .label(cx, &[id!(context)])
                .set_text(cx, &format!("Context:\n{}", context_text));
            self.view
                .widget(cx, &[id!(context)])
                .set_visible(cx, true);
        } else {
            self.view.label(cx, &[id!(context)]).set_text(cx, "");
            self.view
                .widget(cx, &[id!(context)])
                .set_visible(cx, false);
        }

        self.view.set_visible(cx, true);
        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.session_id = None;
        self.permission_id = None;
        self.view.set_visible(cx, false);
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
