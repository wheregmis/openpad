use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;

    pub RunCommandDialog = {{RunCommandDialog}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false

        overlay = <View> {
            width: Fill, height: Fill
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0.0, 0.0, 0.0, 0.5);
                }
            }
        }

        <View> {
            width: Fill, height: Fill
            align: { x: 0.5, y: 0.5 }

            dialog_box = <RoundedView> {
                width: 460, height: Fit
                flow: Down
                padding: { left: 16, right: 16, top: 14, bottom: 14 }
                spacing: 10
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

                title_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: { y: 0.5 }

                    title_label = <Label> {
                        text: "Run"
                        draw_text: {
                            color: (THEME_COLOR_TEXT_PRIMARY)
                            text_style: <THEME_FONT_BOLD> { font_size: 12 }
                        }
                    }

                    <View> { width: Fill }

                    close_button = <Button> {
                        width: 24, height: 24
                        text: "X"
                        draw_bg: {
                            color: (THEME_COLOR_TRANSPARENT)
                            color_hover: (THEME_COLOR_HOVER_MEDIUM)
                            border_radius: 4.0
                            border_size: 0.0
                        }
                        draw_text: {
                            color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                            text_style: <THEME_FONT_BOLD> { font_size: 12 }
                        }
                    }
                }

                subtitle_label = <Label> {
                    text: "Tell Openpad how to start your app."
                    draw_text: {
                        color: (THEME_COLOR_TEXT_DIM)
                        text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                    }
                }

                input_label = <Label> {
                    text: "COMMAND TO RUN"
                    draw_text: {
                        color: (THEME_COLOR_TEXT_MUTED_DARKER)
                        text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                    }
                }

                input_box = <RoundedView> {
                    width: Fill, height: 140
                    show_bg: true
                    padding: { left: 8, right: 8, top: 6, bottom: 6 }
                    draw_bg: {
                        color: (THEME_COLOR_BG_INPUT)
                        uniform border_radius: 8.0
                        uniform border_size: 1.0
                        uniform border_color: (THEME_COLOR_BORDER_MEDIUM)

                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                            sdf.fill_keep(self.color);
                            sdf.stroke(self.border_color, self.border_size);
                            return sdf.result;
                        }
                    }

                    input_field = <TextInput> {
                        width: Fill, height: Fill
                        is_read_only: false
                        draw_bg: {
                            color: (THEME_COLOR_TRANSPARENT)
                            color_focus: (THEME_COLOR_TRANSPARENT)
                            color_empty: (THEME_COLOR_TRANSPARENT)
                            border_radius: 0.0
                            border_size: 0.0
                        }
                        draw_text: {
                            color: (THEME_COLOR_TEXT_BRIGHT)
                            text_style: <THEME_FONT_CODE> { font_size: 10 }
                        }
                        empty_text: "eg:\n  npm install\n  npm run dev"
                    }
                }

                error_row = <View> {
                    visible: false
                    error_label = <Label> {
                        text: ""
                        draw_text: {
                            color: (THEME_COLOR_ACCENT_AMBER)
                            text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                        }
                    }
                }

                buttons_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 10
                    align: { x: 1.0 }

                    env_button = <Button> {
                        width: Fit, height: 28
                        text: "Environment settings"
                        draw_bg: {
                            color: (THEME_COLOR_TRANSPARENT)
                            color_hover: (THEME_COLOR_HOVER_MEDIUM)
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: {
                            color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                            text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                        }
                    }

                    <View> { width: Fill }

                    cancel_button = <Button> {
                        width: 90, height: 28
                        text: "Cancel"
                        draw_bg: {
                            color: (THEME_COLOR_SHADE_3)
                            color_hover: (THEME_COLOR_SHADE_5)
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: { color: (THEME_COLOR_TEXT_PRIMARY), text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                    }

                    confirm_button = <Button> {
                        width: 120, height: 28
                        text: "Save and Run"
                        draw_bg: {
                            color: (THEME_COLOR_ACCENT_BLUE)
                            color_hover: (THEME_COLOR_ACCENT_BLUE_DARK)
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: { color: (THEME_COLOR_TEXT_BRIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum RunCommandDialogAction {
    Confirmed { command: String },
    Cancelled,
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct RunCommandDialog {
    #[deref]
    view: View,
}

impl Widget for RunCommandDialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(&[id!(close_button)]).clicked(&actions)
            || self.view.button(&[id!(cancel_button)]).clicked(&actions)
        {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                RunCommandDialogAction::Cancelled,
            );
            self.view.set_visible(cx, false);
            self.view.redraw(cx);
        }

        if self.view.button(&[id!(confirm_button)]).clicked(&actions) {
            let command = self.view.text_input(&[id!(input_field)]).text();
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                RunCommandDialogAction::Confirmed { command },
            );
            self.view.set_visible(cx, false);
            self.view.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl RunCommandDialog {
    pub fn show(&mut self, cx: &mut Cx, default_value: &str) {
        self.view
            .text_input(&[id!(input_field)])
            .set_text(cx, default_value);
        self.view.label(&[id!(error_label)]).set_text(cx, "");
        self.view.view(&[id!(error_row)]).set_visible(cx, false);
        self.view.set_visible(cx, true);
        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, false);
        self.redraw(cx);
    }

    pub fn show_error(&mut self, cx: &mut Cx, message: &str) {
        self.view.label(&[id!(error_label)]).set_text(cx, message);
        self.view.view(&[id!(error_row)]).set_visible(cx, true);
        self.view.set_visible(cx, true);
        self.redraw(cx);
    }
}

impl RunCommandDialogRef {
    pub fn show(&self, cx: &mut Cx, default_value: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx, default_value);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    pub fn show_error(&self, cx: &mut Cx, message: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_error(cx, message);
        }
    }
}
