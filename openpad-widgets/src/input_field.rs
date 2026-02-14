use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.InputField = TextInput{
        width: Fill
        height: 44
        padding: Inset{left: 14 right: 14 top: 8 bottom: 8}
        empty_text: "Ask anything..."
        draw_bg +: {
            color: #1a1a1a
            color_focus: #252525
            color_empty: #1a1a1a
            border_radius: 8.0
            border_size: 1.0
            border_color: #333
        }
        draw_text +: {
            color: #d9dde6
            text_style: theme.font_regular{
                font_size: 10.5
                line_spacing: 1.4
            }
        }
    }
}
