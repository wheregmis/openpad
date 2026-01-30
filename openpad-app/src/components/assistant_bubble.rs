use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub AssistantBubble = <View> {
        width: Fit, height: Fit
        flow: Down,
        padding: 12,
        show_bg: true
        draw_bg: {
            color: #2b2f35
            uniform border_color: #3a414a
            uniform border_radius: 8.0
            uniform border_size: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }
    }
}
