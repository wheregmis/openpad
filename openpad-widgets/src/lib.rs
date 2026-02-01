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
                    sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                    sdf.fill_keep(self.color);
                    // Draw only bottom border
                    sdf.move_to(0.0, self.rect_size.y - 1.0);
                    sdf.line_to(self.rect_size.x, self.rect_size.y - 1.0);
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
            width: 280.0, height: Fill
            flow: Down,
            padding: 0,
            spacing: 0,
            clip_x: true
            show_bg: true
            open_size: 280.0
            close_size: 0.0
            draw_bg: {
                color: #1e1e1e
                uniform border_color: #333
                uniform border_size: 1.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                    sdf.fill_keep(self.color);
                    // Draw only right border
                    sdf.move_to(self.rect_size.x - 1.0, 0.0);
                    sdf.line_to(self.rect_size.x - 1.0, self.rect_size.y);
                    sdf.stroke(self.border_color, self.border_size);
                    return sdf.result;
                }
            }
            animator: {
                open = {
                    default: off,
                    off = {
                        redraw: true
                        from: {all: Forward {duration: 0.4}}
                        ease: ExpDecay {d1: 0.80, d2: 0.97}
                        apply: {animator_panel_progress: 0.0}
                    }
                    on = {
                        redraw: true
                        from: {all: Forward {duration: 0.4}}
                        ease: ExpDecay {d1: 0.80, d2: 0.97}
                        apply: {animator_panel_progress: 1.0}
                    }
                }
            }
        }

        pub InputBar = <View> {
            width: Fill, height: Fit
            flow: Down,
            spacing: 8,
            padding: { left: 16, right: 12, top: 12, bottom: 12 }
            show_bg: true
            draw_bg: {
                color: #1a1a1a
                uniform border_color: #333333
                uniform border_radius: 18.0
                
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    
                    // Main background with subtle vertical gradient
                    let color = mix(self.color, self.color * 1.1, self.pos.y);
                    sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius);
                    sdf.fill_keep(color);
                    
                    // Border with "light source" effect
                    let border_color = mix(self.border_color * 1.2, self.border_color * 0.8, self.pos.y);
                    sdf.stroke(border_color, 1.0);
                    
                    return sdf.result;
                }
            }
        }

        pub InputBarToolbar = <View> {
            width: Fill, height: 32
            flow: Right
            spacing: 12
            padding: { left: 10, right: 8, top: 4, bottom: 4 }
            align: { y: 0.5 }
            show_bg: false
        }

        pub InputBarDropDown = <DropDown> {
            width: Fit, height: 28
            padding: { left: 12, right: 26, top: 5, bottom: 5 }
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                fn get_color(self) -> vec4 {
                    return mix(#99a1b2, #d6dbe4, self.hover);
                }
            }
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 8.0);
                    
                    let bg_color = mix(#222222, #2a2a2a, self.hover);
                    sdf.fill_keep(bg_color);
                    
                    let stroke_color = mix(#333333, #444444, self.hover);
                    sdf.stroke(stroke_color, 1.0);
                    
                    // Draw dropdown arrow
                    let arrow_x = self.rect_size.x - 17.0;
                    let arrow_y = self.rect_size.y * 0.5 - 1.0;
                    sdf.move_to(arrow_x, arrow_y);
                    sdf.line_to(arrow_x + 4.5, arrow_y + 4.5);
                    sdf.line_to(arrow_x + 9.0, arrow_y);
                    sdf.stroke(#8e95a6, 1.2);
                    
                    return sdf.result;
                }
            }
            animator: {
                hover = {
                    default: off
                    off = {from: {all: Forward {duration: 0.15}}, apply: {draw_bg: {hover: 0.0}}}
                    on = {from: {all: Forward {duration: 0.15}}, apply: {draw_bg: {hover: 1.0}}}
                }
            }
            popup_menu: {
                draw_bg: { color: #202020 }
            }
            labels: ["Default"]
            values: []
        }

        pub InputField = <TextInput> {
            width: Fill, height: Fit
            padding: { left: 14, right: 14, top: 8, bottom: 8 }
            empty_text: "Ask anything..."
            draw_bg: {
                instance focus: 0.0
                fn pixel(self) -> vec4 {
                    // Transparent background, borderless feel until focused
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

        pub SendButton = <Button> {
            width: 32, height: 32
            margin: { left: 6 }
            padding: { left: 8, right: 8, top: 8, bottom: 8 }
            text: ""
            icon_walk: { width: 14, height: Fit }
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/send.svg")
                color: #8e95a6
                color_hover: #ffffff
                color_down: #b2b9c4
            }
            draw_bg: {
                instance hover: 0.0
                instance down: 0.0
                
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0);
                    
                    let bg_color = mix(#222222, #2a2a2a, self.hover);
                    let bg_color_final = mix(bg_color, #1a1a1a, self.down);
                    sdf.fill_keep(bg_color_final);
                    
                    let stroke_color = mix(#333333, #444444, self.hover);
                    sdf.stroke(stroke_color, 1.0);
                    
                    // Subtle glow effect on hover
                    if self.hover > 0.0 {
                        let glow_sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        glow_sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0);
                        let glow_color = vec4(0.2, 0.4, 1.0, 0.1 * self.hover);
                        glow_sdf.stroke(glow_color, 2.0);
                        return mix(sdf.result, glow_sdf.result, glow_sdf.result.a);
                    }
                    
                    return sdf.result;
                }
            }
        }
    }
}
