use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.InputBar = View {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8
        padding: Inset{left: 16 right: 12 top: 12 bottom: 12}
        show_bg: true
        draw_bg +: {
            color: instance(#1a1a1a)
            border_color: uniform(#333333)
            border_radius: uniform(18.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let color = mix(self.color, self.color * 1.1, self.pos.y)
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius)
                sdf.fill_keep(color)

                let border_color = mix(self.border_color * 1.2, self.border_color * 0.8, self.pos.y)
                sdf.stroke(border_color, 1.0)

                return sdf.result
            }
        }
    }

    mod.widgets.InputBarToolbar = View {
        width: Fill
        height: 32
        flow: Right
        spacing: 12
        padding: Inset{left: 10 right: 8 top: 4 bottom: 4}
        align: Align{y: 0.5}
        clip_y: false
        show_bg: false
    }

    mod.widgets.InputBarDropDown = mod.widgets.UpDropDown {
        width: Fit
        height: 28
        padding: Inset{left: 12 right: 26 top: 5 bottom: 5}
        animator: Animator {
            disabled: {
                default: @off
                off: {apply: {draw_bg: {disabled: 0.0}}}
                on: {apply: {draw_bg: {disabled: 1.0}}}
            }
            hover: {
                default: @off
                off: {
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {hover: 0.0}}
                }
                on: {
                    from: {all: Forward {duration: 0.15}}
                    apply: {draw_bg: {hover: 1.0}}
                }
            }
            focus: {
                default: @off
                off: {apply: {draw_bg: {focus: 0.0}}}
                on: {apply: {draw_bg: {focus: 1.0}}}
            }
        }
        draw_text +: {
            text_style: theme.font_regular {font_size: 9.0}
            get_color: fn() {
                return mix(#99a1b2, #d6dbe4, self.hover)
            }
        }
        draw_bg +: {
            hover: instance(0.0)
            focus: instance(0.0)
            active: instance(0.0)
            disabled: instance(0.0)
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 8.0)

                let bg_color = mix(#222222, #2a2a2a, self.hover)
                sdf.fill_keep(bg_color)

                let stroke_color = mix(#333333, #444444, self.hover)
                sdf.stroke(stroke_color, 1.0)

                let arrow_x = self.rect_size.x - 17.0
                let arrow_y = self.rect_size.y * 0.5 - 1.0
                sdf.move_to(arrow_x, arrow_y)
                sdf.line_to(arrow_x + 4.5, arrow_y + 4.5)
                sdf.line_to(arrow_x + 9.0, arrow_y)
                sdf.stroke(#8e95a6, 1.2)

                return sdf.result
            }
        }
        popup_menu: mod.widgets.ScrollablePopupMenu {
            draw_bg +: {color: #1a1a1a}
        }
        labels: ["Default"]
        values: []
    }
}
