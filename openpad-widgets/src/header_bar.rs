use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.HeaderBar = View{
        width: Fill
        height: Fit
        flow: Right
        spacing: 8
        padding: Inset{left: 10 right: 10 top: 6 bottom: 6}
        align: Align{y: 0.5}
        show_bg: true
        draw_bg +: {
            color: instance(#1d2128)
            border_color: uniform(#2b313a)
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
