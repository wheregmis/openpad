use crate::upward_dropdown::UpDropDownWidgetExt;
use makepad_widgets::*;
use openpad_protocol::{Config, Provider};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.SettingsDialog = #(SettingsDialog::register_widget(vm)) {
        width: Fill
        height: Fill
        flow: Down
        show_bg: true

        draw_bg +: {
            color: #1e1e1e
        }

        ScrollYView {
            width: Fill
            height: Fill

            content := View {
                width: Fill
                height: Fit
                flow: Down
                spacing: 12
                padding: Inset{left: 16 right: 16 top: 16 bottom: 16}

                View {
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 5

                    Label {
                        text: "Select Provider"
                        draw_text +: {
                            color: #aab3bd
                            text_style: theme.font_regular {font_size: 10}
                        }
                    }

                    provider_dropdown := mod.widgets.UpDropDown {
                        width: Fill
                        height: 32
                        padding: Inset{left: 10 right: 10 top: 6 bottom: 6}

                        draw_text +: {
                            text_style: theme.font_regular {font_size: 11}
                            color: #e6e9ee
                        }

                        draw_bg +: {
                            color: #15181d
                            color_hover: #333
                            border_radius: 6.0
                            border_size: 1.0
                            border_color: #333
                        }

                    }
                }

                View {
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 5

                    Label {
                        text: "API Key"
                        draw_text +: {
                            color: #aab3bd
                            text_style: theme.font_regular {font_size: 10}
                        }
                    }

                    key_input := TextInput {
                        width: Fill
                        height: 32
                        is_password: true
                        empty_text: "Enter API Key"
                        draw_bg +: {
                            color: #15181d
                            color_focus: #15181d
                            border_radius: 6.0
                            border_size: 1.0
                            border_color: #333
                        }
                        draw_text +: {
                            color: #e6e9ee
                            text_style: theme.font_code {font_size: 10}
                        }
                    }
                }

                View {
                    width: Fill
                    height: Fit
                    align: Align{x: 1.0}

                    save_button := Button {
                        width: 100
                        height: 32
                        text: "Update Key"
                        draw_bg +: {
                            color: #2b2f35
                            color_hover: #353a40
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text +: {
                            color: #ffffff
                            text_style: theme.font_bold {font_size: 11}
                        }
                    }
                }

                View {height: 10}

                separator2 := View {
                    width: Fill
                    height: 1
                    show_bg: true
                    draw_bg +: {color: #333}
                }

                View {
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 5

                    Label {
                        text: "Current Configuration"
                        draw_text +: {
                            color: #aab3bd
                            text_style: theme.font_regular {font_size: 10}
                        }
                    }

                    config_display := Label {
                        width: Fill
                        height: Fit
                        text: "Loading..."
                        draw_text +: {
                            color: #aab3bd
                            text_style: theme.font_code {font_size: 9}}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum SettingsDialogAction {
    #[default]
    None,
    UpdateKey {
        provider_id: String,
        key: String,
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct SettingsDialog {
    #[source]
    source: ScriptObjectRef,

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

        if let Some(idx) = self
            .view
            .up_drop_down(cx, &[id!(content), id!(provider_dropdown)])
            .changed(&actions)
        {
            self.selected_provider_idx = Some(idx);
            self.view
                .text_input(cx, &[id!(content), id!(key_input)])
                .set_text(cx, "");
        }

        if self
            .view
            .button(cx, &[id!(content), id!(save_button)])
            .clicked(&actions)
        {
            if let Some(idx) = self.selected_provider_idx {
                if let Some(provider) = self.providers.get(idx) {
                    let key = self
                        .view
                        .text_input(cx, &[id!(content), id!(key_input)])
                        .text();
                    if !key.is_empty() {
                        cx.action(SettingsDialogAction::UpdateKey {
                            provider_id: provider.id.clone(),
                            key,
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

        let items: Vec<String> = self
            .providers
            .iter()
            .map(|p| p.name.clone().unwrap_or_default())
            .collect();
        self.view
            .up_drop_down(cx, &[id!(content), id!(provider_dropdown)])
            .set_labels(cx, items);
    }

    pub fn set_config(&mut self, cx: &mut Cx, config: &Config) {
        let display = format!("{:#?}", config);
        self.view
            .label(cx, &[id!(content), id!(config_display)])
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
