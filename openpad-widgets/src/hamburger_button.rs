use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    pub HamburgerButton = <Button> {
        width: 32, height: 32
        padding: { left: 6, right: 6, top: 6, bottom: 6 }
        text: "", aria_label: "Toggle sidebar"
        draw_text: { color: #0000 }
        draw_bg: {
            instance open: 0.0
            instance hover: 0.0
            instance down: 0.0
            uniform color: #cbd3dc
            uniform color_hover: #ffffff
            uniform color_down: #aeb7c2
            uniform line_thickness: 1.6
            uniform line_gap: 5.0
            uniform bg_color: #0000

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let w = self.rect_size.x * 0.28;
                let h = self.line_thickness;
                let gap = self.line_gap;
                let t = self.open;
                let ang = t * 0.785398; // 45deg

                let base = self.color;
                let hover = self.color_hover;
                let down = self.color_down;
                let color = mix(mix(base, hover, self.hover), down, self.down);

                sdf.clear(self.bg_color);

                sdf.rotate(ang, c.x, c.y);
                sdf.rect(c.x - w, c.y - gap - h * 0.5 * (1.0 - t), w * 2.0, h);
                sdf.fill_keep(color);
                sdf.rotate(-ang, c.x, c.y);

                sdf.rect(c.x - w, c.y - h * 0.5, w * 2.0, h);
                sdf.fill_keep(color * (1.0 - t));

                sdf.rotate(-ang, c.x, c.y);
                sdf.rect(c.x - w, c.y + gap - h * 0.5 * (1.0 - t), w * 2.0, h);
                sdf.fill_keep(color);
                sdf.rotate(ang, c.x, c.y);

                return sdf.result;
            }
        }
        animator: {
            open = {
                default: off
                off = {
                    from: { all: Forward { duration: 1.0 } }
                    apply: { draw_bg: { open: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 1.0 } }
                    apply: { draw_bg: { open: 1.0 } }
                }
            }
        }
    }
}
