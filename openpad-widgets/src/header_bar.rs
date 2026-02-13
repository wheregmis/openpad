use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.HeaderBar = View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 8
        padding: 10
        align: Align{y: 0.5}
        show_bg: true
        draw_bg +: {
            color: instance(#22262c)
            border_color: uniform(#2c323a)
            border_radius: uniform(8.0)
            border_size: uniform(1.0)
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.rect(0.0 0.0 self.rect_size.x self.rect_size.y)
                sdf.fill_keep(self.color)
                sdf.move_to(0.0 self.rect_size.y - 1.0)
                sdf.line_to(self.rect_size.x self.rect_size.y - 1.0)
                sdf.stroke(self.border_color self.border_size)
                return sdf.result
            }
        }
    }
}
