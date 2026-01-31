use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub AssistantBubble = <RoundedView> {
        width: Fit, height: Fit
        flow: Down,
        padding: { top: 12, bottom: 12, left: 16, right: 16 }
        draw_bg: {
            color: #252526
            border_radius: 12.0
            border_size: 1.0
            border_color: #333
        }
    }
}
