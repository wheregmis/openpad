use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.PermissionCard = #(PermissionCard::register_widget(vm)) {
        width: Fill
        height: Fit
        flow: Down
        padding: {left: 14 right: 14 top: 12 bottom: 12}
        spacing: 10
        show_bg: true

        draw_bg +: {
            color: uniform(THEME_COLOR_SHADE_1)
            border_color: uniform(THEME_COLOR_ACCENT_AMBER)
            border_radius: uniform(10.0)
            border_size: uniform(1.5)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius)
                sdf.fill_keep(self.color)
                sdf.stroke(self.border_color, self.border_size)
                return sdf.result
            }
        }

        header_label = Label {
            width: Fit
            height: Fit
            text: "Permission Request"
            draw_text: {
                color: THEME_COLOR_ACCENT_AMBER
                text_style: theme.font_bold {font_size: 12}
            }
        }

        permission_label = Label {
            width: Fill
            height: Fit
            text: ""
            draw_text: {
                color: THEME_COLOR_TEXT_PRIMARY
                text_style: theme.font_regular {font_size: 11}
                wrap: Word
            }
        }

        patterns_label = Label {
            width: Fill
            height: Fit
            text: ""
            draw_text: {
                color: THEME_COLOR_STATUS_DOT
                text_style: theme.font_code {font_size: 10}
                wrap: Word
            }
        }

        buttons_row = View {
            width: Fill
            height: Fit
            flow: Right
            spacing: 10
            align: {x: 1.0 y: 0.5}

            deny_button = Button {
                width: 80
                height: 32
                text: "Deny"
                draw_bg: {
                    color: THEME_COLOR_SHADE_3
                    color_hover: THEME_COLOR_SHADE_5
                    border_radius: 8.0
                    border_size: 1.0
                    border_color: THEME_COLOR_BORDER_LIGHT
                }
                draw_text: {
                    color: THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular {font_size: 11}
                }
            }

            always_button = Button {
                width: 100
                height: 32
                text: "Always"
                draw_bg: {
                    color: THEME_COLOR_BG_BUTTON
                    color_hover: THEME_COLOR_BG_BUTTON_HOVER
                    border_radius: 8.0
                    border_size: 0.0
                }
                draw_text: {
                    color: THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular {font_size: 11}
                }
            }

            approve_button = Button {
                width: 100
                height: 32
                text: "Approve"
                draw_bg: {
                    color: THEME_COLOR_ACCENT_BLUE
                    color_hover: THEME_COLOR_ACCENT_BLUE_HOVER
                    border_radius: 8.0
                    border_size: 0.0
                }
                draw_text: {
                    color: THEME_COLOR_TEXT_BRIGHT
                    text_style: theme.font_regular {font_size: 11}
                }
            }
        }

        status_label = Label {
            width: Fill
            height: Fit
            visible: false
            text: ""
            draw_text: {
                color: THEME_COLOR_TEXT_DIM
                text_style: theme.font_bold {font_size: 11}
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum PermissionCardAction {
    #[default]
    None,
    Approved {
        session_id: String,
        request_id: String,
    },
    AlwaysApproved {
        session_id: String,
        request_id: String,
    },
    Rejected {
        session_id: String,
        request_id: String,
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct PermissionCard {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[rust]
    session_id: String,
    #[rust]
    request_id: String,
    #[rust]
    resolved: bool,
}

impl Widget for PermissionCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.resolved {
            return;
        }

        if self.view.button(cx, &[id!(approve_button)]).clicked(&actions) {
            cx.action(PermissionCardAction::Approved {
                session_id: self.session_id.clone(),
                request_id: self.request_id.clone(),
            });
            self.mark_resolved_inner(cx, "Approved");
        }

        if self.view.button(cx, &[id!(always_button)]).clicked(&actions) {
            cx.action(PermissionCardAction::AlwaysApproved {
                session_id: self.session_id.clone(),
                request_id: self.request_id.clone(),
            });
            self.mark_resolved_inner(cx, "Always Approved");
        }

        if self.view.button(cx, &[id!(deny_button)]).clicked(&actions) {
            cx.action(PermissionCardAction::Rejected {
                session_id: self.session_id.clone(),
                request_id: self.request_id.clone(),
            });
            self.mark_resolved_inner(cx, "Denied");
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl PermissionCard {
    fn mark_resolved_inner(&mut self, cx: &mut Cx, status: &str) {
        self.resolved = true;
        self.view
            .view(cx, &[id!(buttons_row)])
            .set_visible(cx, false);
        self.view
            .label(cx, &[id!(status_label)])
            .set_text(cx, status);
        self.view
            .widget(cx, &[id!(status_label)])
            .set_visible(cx, true);
        self.redraw(cx);
    }
}

pub trait PermissionCardApi {
    fn set_permission(
        &self,
        cx: &mut Cx,
        session_id: String,
        request_id: String,
        permission: &str,
        patterns: &[String],
    );
    fn mark_resolved(&self, cx: &mut Cx, status: &str);
}

impl PermissionCardApi for PermissionCardRef {
    fn set_permission(
        &self,
        cx: &mut Cx,
        session_id: String,
        request_id: String,
        permission: &str,
        patterns: &[String],
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.session_id = session_id;
            inner.request_id = request_id;
            inner.resolved = false;

            inner
                .view
                .label(cx, &[id!(permission_label)])
                .set_text(cx, &format!("Permission: {}", permission.to_uppercase()));

            let patterns_text = if patterns.is_empty() {
                "Patterns: (none)".to_string()
            } else {
                format!("Patterns:\n{}", patterns.join("\n"))
            };
            inner
                .view
                .label(cx, &[id!(patterns_label)])
                .set_text(cx, &patterns_text);

            inner
                .view
                .view(cx, &[id!(buttons_row)])
                .set_visible(cx, true);
            inner
                .view
                .widget(cx, &[id!(status_label)])
                .set_visible(cx, false);

            inner.redraw(cx);
        }
    }

    fn mark_resolved(&self, cx: &mut Cx, status: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.mark_resolved_inner(cx, status);
        }
    }
}
