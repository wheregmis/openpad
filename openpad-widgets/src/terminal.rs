use makepad_studio::studio_terminal::{StudioTerminal, StudioTerminalRef};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    // Keep Openpad's terminal type name while reusing Makepad Studio's implementation.
    mod.widgets.TerminalBase = #(StudioTerminal::register_widget(vm))
    mod.widgets.Terminal = set_type_default() do mod.widgets.TerminalBase {
        width: Fill
        height: Fill
        font_size: 9.0
        cell_width_factor: 0.6
        cell_height_factor: 1.4
        pad_x: 4.0
        pad_y: 2.0
        text_y_offset: 0.0
        cursor_y_offset: 0.0
        bold_is_bright: true
        faint_factor: 0.75
        scroll_bars: ScrollBars {
            show_scroll_x: false
            show_scroll_y: true
        }
        draw_bg +: {
            color: uniform(#x1d1f21)
            pixel: fn() {
                return self.color
            }
        }
        draw_text +: {
            text_style: theme.font_code
        }
    }
}

pub type Terminal = StudioTerminal;
pub type TerminalRef = StudioTerminalRef;
