use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.TerminalPanelBase = #(TerminalPanel::register_widget(vm))
    mod.widgets.TerminalPanel = mod.widgets.TerminalPanelBase {
        width: Fill
        height: Fit
        flow: Down
        show_bg: true
        open_size: 250.0
        close_size: 0.0

        draw_bg +: {color: THEME_COLOR_BG_APP}

        View {
            width: Fill
            height: 1
            show_bg: true
            draw_bg +: {color: THEME_COLOR_BORDER_MEDIUM}
        }

        terminal_panel := mod.widgets.Terminal {
            width: Fill
            height: Fill
        }

        animator: Animator {
            open: {
                default: @off
                off: {
                    redraw: true
                    from: {all: Forward {duration: 0.4}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: {animator_panel_progress: 0.0}
                }
                on: {
                    redraw: true
                    from: {all: Forward {duration: 0.4}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: {animator_panel_progress: 1.0}
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct TerminalPanel {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[live]
    animator_panel_progress: f32,

    #[live(250.0)]
    open_size: f32,

    #[live(0.0)]
    close_size: f32,

    #[apply_default]
    animator: Animator,
}

impl Widget for TerminalPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let mut walk = walk;
        let size_range = self.open_size - self.close_size;
        let size = self.close_size + size_range * self.animator_panel_progress;
        walk.height = Size::Fixed(size.into());
        self.view.draw_walk(cx, scope, walk)
    }
}

impl TerminalPanel {
    pub fn is_open(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, &[id!(open), id!(on)])
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if open {
            self.animator_play(cx, &[id!(open), id!(on)]);
        } else {
            self.animator_play(cx, &[id!(open), id!(off)]);
        }
    }
}

impl TerminalPanelRef {
    pub fn is_open(&self, cx: &Cx) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_open(cx)
        } else {
            false
        }
    }

    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }
}
