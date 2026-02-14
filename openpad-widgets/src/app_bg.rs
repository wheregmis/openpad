use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.AppBg = View{
        width: Fill
        height: Fill
        show_bg: true
        draw_bg.color: #1e1e1e
    }
}
