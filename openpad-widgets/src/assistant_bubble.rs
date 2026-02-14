use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.AssistantBubble = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        new_batch: true
        padding: Inset{top: 8 bottom: 8 left: 14 right: 14}
        draw_bg.color: #252526
        draw_bg.border_radius: 12.0
        draw_bg.border_size: 1.0
        draw_bg.border_color: #333
    }
}
