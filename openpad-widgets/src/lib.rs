use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    crate::openpad::live_design(cx);
}

pub mod openpad {
    use makepad_widgets::*;

    live_design! {
        use link::theme::*;
        use link::shaders::*;
        use link::widgets::*;

        pub HeaderBar = <View> {
            width: Fill, height: Fit
            flow: Overlay,
            spacing: 8,
            padding: 10,
            show_bg: true
            draw_bg: {
                color: #22262c
                uniform border_color: #2c323a
                uniform border_radius: 8.0
                uniform border_size: 1.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                    sdf.fill_keep(self.color);
                    sdf.stroke(self.border_color, self.border_size);
                    return sdf.result;
                }
            }
        }

        pub StatusDot = <View> {
            width: 10.0, height: 10.0
            show_bg: true
            draw_bg: {
                color: #6b7b8c
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 1.0;
                    sdf.circle(c.x, c.y, r);
                    sdf.fill(self.color);
                    return sdf.result;
                }
            }
        }

        pub InputBar = <RoundedView> {
            width: Fill, height: Fit
            flow: Right,
            spacing: 8,
            padding: 12,
            align: { y: 0.5 }
            draw_bg: {
                color: #1f2329
                border_color: #2e343c
                border_radius: 18.0
                border_size: 1.0
            }
        }

        pub InputField = <TextInput> {
            width: Fill, height: Fit
            empty_text: "Ask anything..."
            draw_bg: {
                color: #0000
                color_hover: #0000
                color_focus: #0000
                color_down: #0000
                border_size: 0.0
            }
            draw_text: { color: #e6e9ee }
            text: ""
        }

        pub SendButton = <Button> {
            width: 36, height: 36
            margin: { left: 6 }
            padding: { left: 8, right: 8, top: 8, bottom: 8 }
            text: ""
            icon_walk: { width: 16, height: Fit }
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/send.svg")
                color: #cbd3dc
                color_hover: #ffffff
                color_down: #aeb7c2
            }
            draw_bg: {
                border_radius: 8.0
                border_size: 0.0
                color: #2a2f36
                color_hover: #313843
                color_down: #242a32
            }
        }
    }
}
