use crate::state::actions::AppAction;
use makepad_widgets::*;
use openpad_protocol::{Config, Provider};
use openpad_widgets::upward_dropdown::UpDropDownWidgetExt;
use openpad_widgets::UpDropDownWidgetRefExt;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;

    pub SettingsDialog = {{SettingsDialog}} {
        width: 400, height: Fit
        flow: Down
        padding: { left: 20, right: 20, top: 20, bottom: 20 }
        spacing: 15
        visible: false
        show_bg: true

        draw_bg: {
            color: (THEME_COLOR_BG_APP)
            uniform border_color: (THEME_COLOR_BORDER_MEDIUM)
            uniform border_radius: 12.0
            uniform border_size: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: { y: 0.5 }

            title = <Label> {
                text: "Settings"
                draw_text: {
                    color: (THEME_COLOR_TEXT_PRIMARY)
                    text_style: <THEME_FONT_BOLD> { font_size: 14 }
                }
            }

            <View> { width: Fill }

            close_button = <Button> {
                width: 24, height: 24
                text: "x"
                draw_bg: {
                    color: (THEME_COLOR_TRANSPARENT)
                    color_hover: (THEME_COLOR_HOVER_MEDIUM)
                    border_radius: 4.0
                    border_size: 0.0
                }
                draw_text: {
                    color: (THEME_COLOR_TEXT_DIM)
                    text_style: <THEME_FONT_REGULAR> { font_size: 12 }
                }
            }
        }

        separator = <View> {
            width: Fill, height: 1
            show_bg: true
            draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) }
        }

        // Provider Selection
        <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 5

            <Label> {
                text: "Select Provider"
                draw_text: {
                    color: (THEME_COLOR_TEXT_DIM)
                    text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                }
            }

            provider_dropdown = <UpDropDown> {
                width: Fill, height: 32
                padding: { left: 10, right: 10, top: 6, bottom: 6 }
                popup_menu_position: AboveInput

                animator: {
                    disabled = {
                        default: off
                        off = { apply: { draw_bg: { disabled: 0.0 } } }
                        on = { apply: { draw_bg: { disabled: 1.0 } } }
                    }
                    hover = {
                        default: off
                        off = {
                            from: {all: Forward {duration: 0.15}}
                            apply: { draw_bg: {hover: 0.0} }
                        }
                        on = {
                            from: {all: Forward {duration: 0.15}}
                            apply: { draw_bg: {hover: 1.0} }
                        }
                    }
                    focus = {
                        default: off
                        off = { apply: { draw_bg: { focus: 0.0 } } }
                        on = { apply: { draw_bg: { focus: 1.0 } } }
                    }
                }

                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 11 }
                    fn get_color(self) -> vec4 {
                        return mix((THEME_COLOR_TEXT_PRIMARY), (THEME_COLOR_TEXT_BRIGHT), self.hover);
                    }
                }

                draw_bg: {
                    instance hover: 0.0
                    instance focus: 0.0
                    instance active: 0.0
                    instance disabled: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 6.0);

                        let bg_color = mix((THEME_COLOR_BG_INPUT), (THEME_COLOR_HOVER_MEDIUM), self.hover);
                        sdf.fill_keep(bg_color);

                        let stroke_color = mix((THEME_COLOR_BORDER_MEDIUM), (THEME_COLOR_BORDER_LIGHT), self.hover);
                        sdf.stroke(stroke_color, 1.0);

                        return sdf.result;
                    }
                }

                popup_menu: {
                    draw_bg: {
                        color: (THEME_COLOR_BG_DARKER)
                        border_radius: 6.0
                        border_size: 1.0
                        border_color: (THEME_COLOR_BORDER_MEDIUM)
                    }
                }
            }
        }

        // API Key Input
        <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 5

            <Label> {
                text: "API Key"
                draw_text: {
                    color: (THEME_COLOR_TEXT_DIM)
                    text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                }
            }

            key_input = <TextInput> {
                width: Fill, height: 32
                empty_text: "Enter API Key"
                draw_bg: {
                    color: (THEME_COLOR_BG_INPUT)
                    color_focus: (THEME_COLOR_BG_INPUT)
                    border_radius: 6.0
                    border_size: 1.0
                    border_color: (THEME_COLOR_BORDER_MEDIUM)
                }
                draw_text: {
                    color: (THEME_COLOR_TEXT_PRIMARY)
                    text_style: <THEME_FONT_CODE> { font_size: 10 }
                }
            }
        }

        // Save Button
        <View> {
            width: Fill, height: Fit
            align: { x: 1.0 }

            save_button = <Button> {
                width: 100, height: 32
                text: "Update Key"
                draw_bg: {
                    color: (THEME_COLOR_BG_BUTTON)
                    color_hover: (THEME_COLOR_BG_BUTTON_HOVER)
                    border_radius: 6.0
                    border_size: 0.0
                }
                draw_text: {
                    color: (THEME_COLOR_TEXT_BRIGHT)
                    text_style: <THEME_FONT_BOLD> { font_size: 11 }
                }
            }
        }

        <View> { height: 10 }

        separator2 = <View> {
            width: Fill, height: 1
            show_bg: true
            draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) }
        }

        // Config Display
        <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 5

            <Label> {
                text: "Current Configuration"
                draw_text: {
                    color: (THEME_COLOR_TEXT_DIM)
                    text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                }
            }

            config_display = <Label> {
                width: Fill, height: Fit
                text: "Loading..."
                draw_text: {
                    color: (THEME_COLOR_TEXT_DIM)
                    text_style: <THEME_FONT_CODE> { font_size: 9 }
                    wrap: Word
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SettingsDialog {
    #[deref]
    view: View,

    #[rust]
    providers: Vec<Provider>,

    #[rust]
    selected_provider_idx: Option<usize>,
}

impl Widget for SettingsDialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(&[id!(close_button)]).clicked(&actions) {
            self.hide(cx);
        }

        if let Some(idx) = self
            .view
            .up_drop_down(&[id!(provider_dropdown)])
            .changed(&actions)
        {
            self.selected_provider_idx = Some(idx);
            self.view.text_input(&[id!(key_input)]).set_text(cx, "");
        }

        if self.view.button(&[id!(save_button)]).clicked(&actions) {
            if let Some(idx) = self.selected_provider_idx {
                if let Some(provider) = self.providers.get(idx) {
                    let key = self.view.text_input(&[id!(key_input)]).text();
                    if !key.is_empty() {
                        cx.action(AppAction::DialogConfirmed {
                            dialog_type: format!("set_auth:{}", provider.id),
                            value: key,
                        });
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SettingsDialog {
    pub fn show(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, true);
        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, false);
        self.redraw(cx);
    }

    pub fn set_providers(&mut self, cx: &mut Cx, providers: Vec<Provider>) {
        self.providers = providers;

        // Populate dropdown
        let items: Vec<String> = self
            .providers
            .iter()
            .map(|p| p.name.clone().unwrap_or_default())
            .collect();
        self.view
            .up_drop_down(&[id!(provider_dropdown)])
            .set_labels(cx, items);
    }

    pub fn set_config(&mut self, cx: &mut Cx, config: &Config) {
        let display = format!("{:#?}", config); // Simple debug print for now
        self.view
            .label(&[id!(config_display)])
            .set_text(cx, &display);
    }
}

impl SettingsDialogRef {
    pub fn show(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    pub fn set_providers(&self, cx: &mut Cx, providers: Vec<Provider>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_providers(cx, providers);
        }
    }

    pub fn set_config(&self, cx: &mut Cx, config: &Config) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_config(cx, config);
        }
    }
}
