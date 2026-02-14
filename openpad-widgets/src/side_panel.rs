use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.SidePanelBase = #(SidePanel::register_widget(vm))
    mod.widgets.SidePanel = mod.widgets.SidePanelBase {
        width: 280.0
        height: Fill
        flow: Down
        padding: 0
        spacing: 0
        clip_x: true
        show_bg: true
        open_size: 280.0
        close_size: 0.0

        draw_bg +: {
            color: instance(#1e1e1e)
            border_color: uniform(#333)
            border_size: uniform(1.0)
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.rect(0.0 0.0 self.rect_size.x self.rect_size.y)
                sdf.fill_keep(self.color)
                sdf.move_to(self.rect_size.x - 1.0 0.0)
                sdf.line_to(self.rect_size.x - 1.0 self.rect_size.y)
                sdf.stroke(self.border_color self.border_size)
                return sdf.result
            }
        }

        animator: Animator {
            open = {
                default: @off
                off = {
                    redraw: true
                    from: {all: Forward {duration: 0.4}}
                    ease: ExpDecay {d1: 0.80 d2: 0.97}
                    apply: {animator_panel_progress: 0.0}
                }
                on = {
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
pub struct SidePanel {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[live]
    animator_panel_progress: f32,

    #[live(280.0)]
    open_size: f32,

    #[live(0.0)]
    close_size: f32,

    #[apply_default]
    animator: Animator,
}

impl Widget for SidePanel {
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
        walk.width = Size::Fixed(size.into());
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SidePanel {
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

impl SidePanelRef {
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
