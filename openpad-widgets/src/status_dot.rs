use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.StatusDot = View{
        width: 10.0
        height: 10.0
        show_bg: true
        draw_bg +: {
            color: instance(#6b7b8c)
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let c = self.rect_size * 0.5
                let r = min(c.x c.y) - 1.0
                sdf.circle(c.x c.y r)
                sdf.fill(self.color)
                return sdf.result
            }
        }
    }
}
