use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub AppBg = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #1e1e1e
        }
    }
}
