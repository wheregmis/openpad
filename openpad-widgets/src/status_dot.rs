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
