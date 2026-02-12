use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub InputField = <TextInput> {
        width: Fill, height: Fit
        padding: { left: 14, right: 14, top: 8, bottom: 8 }
        empty_text: "Ask anything...", aria_label: "Chat input"
        draw_bg: {
            instance focus: 0.0
            fn pixel(self) -> vec4 {
                return #0000;
            }
        }
        draw_text: {
            color: #d9dde6,
            text_style: <THEME_FONT_REGULAR> { font_size: 10.5, line_spacing: 1.4 }
            fn get_color(self) -> vec4 {
                return mix(#99a1b2, #d9dde6, self.focus);
            }
        }
        text: ""
    }
}
