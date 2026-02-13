use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.InputField = TextInput{
        width: Fill
        height: Fit
        padding: Inset{left: 14 right: 14 top: 8 bottom: 8}
        empty_text: "Ask anything..."
        draw_bg +: {
            focus: instance(0.0)
            pixel: fn() {
                return #0000
            }
        }
        draw_text +: {
            color: #d9dde6
            text_style: theme.font_regular{
                font_size: 10.5
                line_spacing: 1.4
            }
            get_color: fn() {
                return mix(#99a1b2 #d9dde6 self.focus)
            }
        }
        text: ""
    }
}
