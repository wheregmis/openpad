use makepad_widgets::*;

pub mod hamburger_button;
pub mod header_bar;
pub mod input_bar;
pub mod input_field;
pub mod send_button;
pub mod side_panel;
pub mod status_dot;

// Re-export types from side_panel
pub use side_panel::{SidePanel, SidePanelRef, SidePanelWidgetRefExt};

pub fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    crate::openpad::live_design(cx);
}

pub mod openpad {
    use crate::SidePanel;
    use makepad_widgets::*;

    live_design! {
        use link::theme::*;
        use link::shaders::*;
        use link::widgets::*;

        // Widget DSL definitions are inline here for proper live_design registration
        pub HeaderBar = <View> {
            width: Fill, height: Fit
            flow: Right,
            spacing: 8,
            padding: 10,
            align: { y: 0.5 }
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

        pub HamburgerButton = <Button> {
            width: 32, height: 32
            padding: { left: 6, right: 6, top: 6, bottom: 6 }
            text: ""
            draw_text: { color: #0000 }
            draw_bg: {
                instance open: 0.0
                instance hover: 0.0
                instance down: 0.0
                uniform color: #cbd3dc
                uniform color_hover: #ffffff
                uniform color_down: #aeb7c2
                uniform line_thickness: 1.6
                uniform line_gap: 5.0
                uniform bg_color: #0000

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let w = self.rect_size.x * 0.28;
                    let h = self.line_thickness;
                    let gap = self.line_gap;
                    let t = self.open;
                    let ang = t * 0.785398; // 45deg

                    let base = self.color;
                    let hover = self.color_hover;
                    let down = self.color_down;
                    let color = mix(mix(base, hover, self.hover), down, self.down);

                    sdf.clear(self.bg_color);

                    sdf.rotate(ang, c.x, c.y);
                    sdf.rect(c.x - w, c.y - gap - h * 0.5 * (1.0 - t), w * 2.0, h);
                    sdf.fill_keep(color);
                    sdf.rotate(-ang, c.x, c.y);

                    sdf.rect(c.x - w, c.y - h * 0.5, w * 2.0, h);
                    sdf.fill_keep(color * (1.0 - t));

                    sdf.rotate(-ang, c.x, c.y);
                    sdf.rect(c.x - w, c.y + gap - h * 0.5 * (1.0 - t), w * 2.0, h);
                    sdf.fill_keep(color);
                    sdf.rotate(ang, c.x, c.y);

                    return sdf.result;
                }
            }
            animator: {
                open = {
                    default: off
                    off = {
                        from: { all: Forward { duration: 1.0 } }
                        apply: { draw_bg: { open: 0.0 } }
                    }
                    on = {
                        from: { all: Forward { duration: 1.0 } }
                        apply: { draw_bg: { open: 1.0 } }
                    }
                }
            }
        }

        pub SidePanelBase = {{SidePanel}} {}
        pub SidePanel = <SidePanelBase> {
            flow: Down,
            padding: 16,
            spacing: 12,
            clip_x: true
            show_bg: true
            open_size: 280.0
            close_size: 0.0
            draw_bg: {
                color: #1c2026
                uniform border_color: #2b3138
                uniform border_size: 1.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.rect(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0);
                    sdf.fill_keep(self.color);
                    sdf.stroke(self.border_color, self.border_size);
                    return sdf.result;
                }
            }
            animator: {
                open = {
                    default: off,
                    off = {
                        redraw: true
                        from: {all: Forward {duration: 1.0}}
                        ease: ExpDecay {d1: 0.80, d2: 0.97}
                        apply: {animator_panel_progress: 0.0}
                    }
                    on = {
                        redraw: true
                        from: {all: Forward {duration: 1.0}}
                        ease: ExpDecay {d1: 0.80, d2: 0.97}
                        apply: {animator_panel_progress: 1.0}
                    }
                }
            }
        }

        pub InputBar = <RoundedView> {
            width: Fill, height: 48
            flow: Right,
            spacing: 8,
            padding: { left: 14, right: 10, top: 6, bottom: 6 }
            align: { y: 0.5 }
            draw_bg: {
                color: #1f2329
                border_color: #2e343c
                border_radius: 18.0
                border_size: 1.0
            }
        }

        pub InputField = <TextInput> {
            width: Fill, height: 36
            empty_text: "Ask anything..."
            draw_bg: {
                color: #0000
                color_hover: #0000
                color_focus: #0000
                color_down: #0000
                color_empty: #0000
                color_disabled: #0000
                border_radius: 0.0
                border_size: 0.0
                border_color_1: #0000
                border_color_2: #0000
                border_color_1_hover: #0000
                border_color_2_hover: #0000
                border_color_1_focus: #0000
                border_color_2_focus: #0000
                border_color_1_down: #0000
                border_color_2_down: #0000
                border_color_1_empty: #0000
                border_color_2_empty: #0000
                border_color_1_disabled: #0000
                border_color_2_disabled: #0000
            }
            draw_text: { color: #e6e9ee, text_style: { font_size: 12 } }
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
