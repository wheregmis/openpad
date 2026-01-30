use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub AppBg = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #14161a
            uniform color_2: #0f1114
            fn pixel(self) -> vec4 {
                return mix(self.color, self.color_2, self.pos.y);
            }
        }
    }
}
