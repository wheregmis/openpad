use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.StatusDot = #(StatusDot::register_widget(vm)) {
        width: 8.0
        height: 8.0
        draw_bg: {
            color: #6b7b8c
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let cx = self.rect_size.x * 0.5
                let cy = self.rect_size.y * 0.5
                let r = min(cx, cy) - 1.0
                sdf.circle(cx, cy, r)
                sdf.fill(self.color)
                return sdf.result
            }
        }
    }
}

#[derive(Script, ScriptHook)]
#[repr(C)]
pub struct DrawStatusDot {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
}

#[derive(Script, ScriptHook, Widget)]
pub struct StatusDot {
    #[source]
    source: ScriptObjectRef,
    #[redraw]
    #[live]
    draw_bg: DrawStatusDot,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

impl Widget for StatusDot {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_bg.end(cx);
        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl StatusDotRef {
    pub fn set_color(&self, cx: &mut Cx, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.draw_bg.color = color;
            inner.draw_bg.redraw(cx);
        }
    }
}
