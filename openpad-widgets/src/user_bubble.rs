use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.UserBubble = RoundedView{
        width: Fit
        height: Fit
        flow: Down
        padding: Inset{top: 8 bottom: 8 left: 14 right: 14}
        draw_bg.color: #2d2d30
        draw_bg.border_radius: 12.0
        draw_bg.border_size: 1.0
        draw_bg.border_color: #444
    }
}
