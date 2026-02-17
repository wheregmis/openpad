use crate::state::actions::{AppAction, SidebarMode};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.SidebarHeader = #(SidebarHeader::register_widget(vm)) {
        width: Fill, height: Fit
        flow: Down
        padding: Inset{ left: 8, right: 8, top: 6, bottom: 6 }

        tabs_row := View {
            width: Fill, height: Fit
            flow: Right
            spacing: 2

            projects_tab := Button {
                width: Fit, height: 28
                text: "Files"
                draw_bg +: {
                    color: theme.THEME_COLOR_TRANSPARENT
                    color_hover: theme.THEME_COLOR_HOVER_SUBTLE
                    color_active: theme.THEME_COLOR_HOVER_SUBTLE
                    border_radius: 6.0
                    border_size: 0.0
                }
                draw_text +: {
                    color: theme.THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular { font_size: 11 }
                }
            }

            settings_tab := Button {
                width: Fit, height: 28
                text: "Settings"
                draw_bg +: {
                    color: theme.THEME_COLOR_TRANSPARENT
                    color_hover: theme.THEME_COLOR_HOVER_SUBTLE
                    color_active: theme.THEME_COLOR_HOVER_SUBTLE
                    border_radius: 6.0
                    border_size: 0.0
                }
                draw_text +: {
                    color: theme.THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular { font_size: 11 }
                }
            }
        }

        divider := View {
            width: Fill, height: 1
            show_bg: true
            draw_bg +: {
                color: theme.THEME_COLOR_BORDER_MEDIUM
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct SidebarHeader {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,
}

impl Widget for SidebarHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(cx, &[id!(projects_tab)]).clicked(&actions) {
            cx.action(AppAction::SetSidebarMode(SidebarMode::Files));
        }
        if self.view.button(cx, &[id!(settings_tab)]).clicked(&actions) {
            cx.action(AppAction::SetSidebarMode(SidebarMode::Settings));
        }
    }
}
