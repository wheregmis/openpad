use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::*;

    pub AppBg = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: (THEME_COLOR_BG_APP)
        }
    }
}
