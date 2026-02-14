use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.SendButton = Button {
        width: 32
        height: 32
        margin: Inset{left: 6}
        padding: Inset{left: 8 right: 8 top: 8 bottom: 8}
        text: ""
        icon_walk: Walk{width: 14 height: 14}
        draw_icon +: {
            svg: crate_resource("self://resources/icons/send.svg")
            color: #8e95a6
            color_hover: #ffffff
            color_down: #b2b9c4
        }
        draw_bg +: {
            hover: instance(0.0)
            down: instance(0.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0)

                let bg_color = mix(#222222, #2a2a2a, self.hover)
                let bg_color_final = mix(bg_color, #1a1a1a, self.down)
                sdf.fill_keep(bg_color_final)

                let stroke_color = mix(#333333, #444444, self.hover)
                sdf.stroke(stroke_color, 1.0)

                if self.hover > 0.0 {
                    let glow_sdf = Sdf2d.viewport(self.pos * self.rect_size)
                    glow_sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0)
                    let glow_color = vec4(0.2, 0.4, 1.0, 0.1 * self.hover)
                    glow_sdf.stroke(glow_color, 2.0)
                    return mix(sdf.result, glow_sdf.result, glow_sdf.result.a)
                }

                return sdf.result
            }
        }
    }
}
