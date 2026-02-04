use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub HeaderBar = <View> {
        width: Fill, height: Fit
        flow: Right,
        spacing: 8,
        padding: 10,
        align: { y: 0.5 }
        show_bg: true
        draw_bg: {
            color: #22262c
            uniform border_color: #2c323a
            uniform border_radius: 8.0
            uniform border_size: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                sdf.fill_keep(self.color);
                // Draw only bottom border
                sdf.move_to(0.0, self.rect_size.y - 1.0);
                sdf.line_to(self.rect_size.x, self.rect_size.y - 1.0);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }
    }
}
