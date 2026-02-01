use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;

    pub UserBubble = <RoundedView> {
        width: Fit, height: Fit
        flow: Down,
        padding: { top: 8, bottom: 8, left: 14, right: 14 }
        draw_bg: {
            color: (THEME_COLOR_BG_USER_BUBBLE)
            border_radius: 12.0
            border_size: 1.0
            border_color: (THEME_COLOR_BORDER_LIGHT)
        }
    }
}
