use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub SendButton = <Button> {
        width: 32, height: 32
        margin: { left: 6 }
        padding: { left: 8, right: 8, top: 8, bottom: 8 }
        text: "Send Message"
        draw_text: { color: #0000 }
        icon_walk: { width: 14, height: Fit }
        draw_icon: {
            svg_file: dep("crate://self/resources/icons/send.svg")
            color: #8e95a6
            color_hover: #ffffff
            color_down: #b2b9c4
        }
        draw_bg: {
            instance hover: 0.0
            instance down: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0);

                let bg_color = mix(#222222, #2a2a2a, self.hover);
                let bg_color_final = mix(bg_color, #1a1a1a, self.down);
                sdf.fill_keep(bg_color_final);

                let stroke_color = mix(#333333, #444444, self.hover);
                sdf.stroke(stroke_color, 1.0);

                // Subtle glow effect on hover
                if self.hover > 0.0 {
                    let glow_sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    glow_sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0);
                    let glow_color = vec4(0.2, 0.4, 1.0, 0.1 * self.hover);
                    glow_sdf.stroke(glow_color, 2.0);
                    return mix(sdf.result, glow_sdf.result, glow_sdf.result.a);
                }

                return sdf.result;
            }
        }
    }
}
