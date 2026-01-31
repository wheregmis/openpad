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
            color: #1e1e1e
            fn pixel(self) -> vec4 {
                return self.color;
            }
        }
    }
}
