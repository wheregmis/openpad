use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.HamburgerButton = Button {
        width: 32
        height: 32
        padding: Inset{left: 6 right: 6 top: 6 bottom: 6}
        text: ""
        draw_text +: {color: #0000}
        draw_bg +: {
            open: instance(0.0)
            hover: instance(0.0)
            down: instance(0.0)
            color: uniform(#cbd3dc)
            color_hover: uniform(#ffffff)
            color_down: uniform(#aeb7c2)
            line_thickness: uniform(1.6)
            line_gap: uniform(5.0)
            bg_color: uniform(#0000)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let cx = self.rect_size.x * 0.5
                let cy = self.rect_size.y * 0.5;
                let w = self.rect_size.x * 0.28
                let h = self.line_thickness
                let gap = self.line_gap
                let t = self.open
                let ang = t * 0.785398

                let base = self.color
                let hover = self.color_hover
                let down = self.color_down
                let color = mix(mix(base, hover, self.hover), down, self.down)

                sdf.clear(self.bg_color)

                sdf.rotate(ang, cx, cy)
                sdf.rect(cx - w, cy - gap - h * 0.5 * (1.0 - t), w * 2.0, h)
                sdf.fill_keep(color)
                sdf.rotate(-ang, cx, cy)

                sdf.rect(cx - w, cy - h * 0.5, w * 2.0, h)
                sdf.fill_keep(color * (1.0 - t))

                sdf.rotate(-ang, cx, cy)
                sdf.rect(cx - w, cy + gap - h * 0.5 * (1.0 - t), w * 2.0, h)
                sdf.fill_keep(color)
                sdf.rotate(ang, cx, cy)

                return sdf.result
            }
        }
        animator: Animator {
            open: {
                default: @off
                off: {
                    from: {all: Forward {duration: 1.0}}
                    apply: {draw_bg: {open: 0.0}}
                }
                on: {
                    from: {all: Forward {duration: 1.0}}
                    apply: {draw_bg: {open: 1.0}}
                }
            }
        }
    }
}
