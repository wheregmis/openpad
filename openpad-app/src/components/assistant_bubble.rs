use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub AssistantBubble = <RoundedView> {
        width: Fit, height: Fit
        flow: Down,
        padding: { top: 8, bottom: 8, left: 14, right: 14 }
        draw_bg: {
            color: #252526
            border_radius: 12.0
            border_size: 1.0
            border_color: #333
        }
    }
}
