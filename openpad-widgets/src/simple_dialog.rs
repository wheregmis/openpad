use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.SimpleDialog = #(SimpleDialog::register_widget(vm)) {
        width: Fill
        height: Fill
        flow: Overlay
        visible: false

        overlay = View {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg +: {
                pixel: fn() {
                    return vec4(0.0, 0.0, 0.0, 0.5)
                }
            }
        }

        View {
            width: Fill
            height: Fill
            align: {x: 0.5 y: 0.5}

            dialog_box = RoundedView {
                width: 400
                height: Fit
                flow: Down
                padding: {left: 14 right: 14 top: 12 bottom: 12}
                spacing: 10
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

                title_label = Label {
                    text: "Dialog"
                    draw_text: {
                        color: THEME_COLOR_TEXT_PRIMARY
                        text_style: theme.font_bold {font_size: 12}
                    }
                }

                message_label = Label {
                    width: Fill
                    height: Fit
                    text: ""
                    draw_text: {
                        color: THEME_COLOR_TEXT_DIM
                        text_style: theme.font_regular {font_size: 11}
                        wrap: Word
                    }
                }

                input_row = View {
                    width: Fill
                    height: Fit
                    visible: false

                    input_field = TextInput {
                        width: Fill
                        height: 32
                        padding: 8
                        draw_text: {
                            color: THEME_COLOR_TEXT_PRIMARY
                            text_style: theme.font_regular {font_size: 11}
                        }
                        draw_bg: {
                            color: THEME_COLOR_BG_INPUT
                            border_radius: 8.0
                            border_size: 0.0
                        }
                    }
                }

                buttons_row = View {
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 10
                    align: {x: 1.0}

                    cancel_button = Button {
                        width: 90
                        height: 32
                        text: "Cancel"
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

                    confirm_button = Button {
                        width: 90
                        height: 32
                        text: "OK"
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
    }
}

#[derive(Clone, Debug, Default)]
pub enum DialogType {
    #[default]
    Confirm,
    Input,
}

#[derive(Clone, Debug, Default)]
pub enum SimpleDialogAction {
    Confirmed { dialog_type: String, value: String },
    Cancelled,
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct SimpleDialog {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,
    #[rust]
    dialog_type: DialogType,
    #[rust]
    callback_data: String,
}

impl Widget for SimpleDialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(cx, ids!(cancel_button)).clicked(&actions) {
            cx.widget_action(self.widget_uid(), SimpleDialogAction::Cancelled);
            self.view.set_visible(cx, false);
            self.view.redraw(cx);
        }

        if self.view.button(cx, ids!(confirm_button)).clicked(&actions) {
            let value = if matches!(self.dialog_type, DialogType::Input) {
                self.view.text_input(cx, ids!(input_field)).text()
            } else {
                String::new()
            };

            cx.widget_action(
                self.widget_uid(),
                SimpleDialogAction::Confirmed {
                    dialog_type: self.callback_data.clone(),
                    value,
                },
            );

            self.view.set_visible(cx, false);
            self.view.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SimpleDialogRef {
    pub fn show_confirm(&self, cx: &mut Cx, title: &str, message: &str, callback_data: String) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.dialog_type = DialogType::Confirm;
            inner.callback_data = callback_data;

            inner.view.label(cx, ids!(title_label)).set_text(cx, title);
            inner
                .view
                .label(cx, ids!(message_label))
                .set_text(cx, message);
            inner.view.view(cx, ids!(input_row)).set_visible(cx, false);

            inner.view.set_visible(cx, true);
            inner.redraw(cx);
        }
    }

    pub fn show_input(
        &self,
        cx: &mut Cx,
        title: &str,
        message: &str,
        default_value: &str,
        callback_data: String,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.dialog_type = DialogType::Input;
            inner.callback_data = callback_data;

            inner.view.label(cx, ids!(title_label)).set_text(cx, title);
            inner
                .view
                .label(cx, ids!(message_label))
                .set_text(cx, message);
            inner
                .view
                .text_input(cx, ids!(input_field))
                .set_text(cx, default_value);
            inner.view.view(cx, ids!(input_row)).set_visible(cx, true);

            inner.view.set_visible(cx, true);
            inner.redraw(cx);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.set_visible(cx, false);
            inner.redraw(cx);
        }
    }
}
