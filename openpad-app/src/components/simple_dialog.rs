use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::*;

    pub SimpleDialog = {{SimpleDialog}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false

        // Semi-transparent background overlay
        overlay = <View> {
            width: Fill, height: Fill
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0.0, 0.0, 0.0, 0.5);
                }
            }
        }

        // Center the dialog
        <View> {
            width: Fill, height: Fill
            align: { x: 0.5, y: 0.5 }

            dialog_box = <RoundedView> {
                width: 400, height: Fit
                flow: Down,
                padding: { left: 14, right: 14, top: 12, bottom: 12 }
                spacing: 10,
                show_bg: true
                draw_bg: {
                    color: (THEME_COLOR_BG_DIALOG)
                    uniform border_color: (THEME_COLOR_BORDER_DIALOG)
                    uniform border_radius: 10.0
                    uniform border_size: 1.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                        sdf.fill_keep(self.color);
                        sdf.stroke(self.border_color, self.border_size);
                        return sdf.result;
                    }
                }

                title_label = <Label> {
                    text: "Dialog"
                    draw_text: {
                        color: (THEME_COLOR_TEXT_PRIMARY)
                        text_style: <THEME_FONT_BOLD> { font_size: 12 }
                    }
                }

                message_label = <Label> {
                    width: Fill, height: Fit
                    text: ""
                    draw_text: {
                        color: (THEME_COLOR_TEXT_DIM)
                        text_style: <THEME_FONT_REGULAR> { font_size: 11 }
                        wrap: Word
                    }
                }

                input_row = <View> {
                    width: Fill, height: Fit
                    visible: false

                    input_field = <TextInput> {
                        width: Fill, height: 32
                        padding: 8,
                        draw_text: {
                            color: (THEME_COLOR_TEXT_PRIMARY)
                            text_style: <THEME_FONT_REGULAR> { font_size: 11 }
                        }
                        draw_bg: {
                            color: (THEME_COLOR_BG_INPUT)
                            border_radius: 8.0
                            border_size: 0.0
                        }
                    }
                }

                buttons_row = <View> {
                    width: Fill, height: Fit
                    flow: Right,
                    spacing: 10,
                    align: { x: 1.0 }

                    cancel_button = <Button> {
                        width: 90, height: 32
                        text: "Cancel"
                        draw_bg: {
                            color: (THEME_COLOR_SHADE_3)
                            color_hover: (THEME_COLOR_SHADE_5)
                            border_radius: 8.0
                            border_size: 0.0
                        }
                        draw_text: { color: (THEME_COLOR_TEXT_PRIMARY), text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
                    }

                    confirm_button = <Button> {
                        width: 90, height: 32
                        text: "OK"
                        draw_bg: {
                            color: (THEME_COLOR_ACCENT_BLUE)
                            color_hover: (THEME_COLOR_ACCENT_BLUE_DARK)
                            border_radius: 8.0
                            border_size: 0.0
                        }
                        draw_text: { color: (THEME_COLOR_TEXT_BRIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
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

#[derive(Live, LiveHook, Widget)]
pub struct SimpleDialog {
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

        if self.view.button(&[id!(cancel_button)]).clicked(&actions) {
            self.view.set_visible(cx, false);
            self.view.redraw(cx);
        }

        if self.view.button(&[id!(confirm_button)]).clicked(&actions) {
            use crate::state::actions::AppAction;

            // Get the input value if it's an input dialog
            let value = if matches!(self.dialog_type, DialogType::Input) {
                self.view.text_input(&[id!(input_field)]).text()
            } else {
                String::new()
            };

            // Post action with callback_data (which identifies what to do) and the value
            cx.action(AppAction::DialogConfirmed {
                dialog_type: self.callback_data.clone(),
                value,
            });

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

            inner.view.label(&[id!(title_label)]).set_text(cx, title);
            inner
                .view
                .label(&[id!(message_label)])
                .set_text(cx, message);
            inner.view.view(&[id!(input_row)]).set_visible(cx, false);

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

            inner.view.label(&[id!(title_label)]).set_text(cx, title);
            inner
                .view
                .label(&[id!(message_label)])
                .set_text(cx, message);
            inner
                .view
                .text_input(&[id!(input_field)])
                .set_text(cx, default_value);
            inner.view.view(&[id!(input_row)]).set_visible(cx, true);

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
