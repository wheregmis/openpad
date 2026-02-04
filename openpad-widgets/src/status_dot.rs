use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub StatusDot = <View> {
        width: 10.0, height: 10.0
        show_bg: true
        draw_bg: {
            color: #6b7b8c
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y) - 1.0;
                sdf.circle(c.x, c.y, r);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
    }
}
