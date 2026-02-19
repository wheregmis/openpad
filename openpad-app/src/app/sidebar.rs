use super::*;

/// Shared resize hit-testing helper. Processes a drag-resize interaction for a sidebar handle.
/// Returns `Some(new_width)` when the width should be updated, `None` otherwise.
/// `delta_sign`: +1.0 for left sidebar (drag right = wider), -1.0 for right sidebar (drag left = wider).
fn perform_sidebar_resize(
    cx: &mut Cx,
    event: &Event,
    handle_area: Area,
    drag_start: &mut Option<(f64, f32)>,
    current_width: f32,
    delta_sign: f32,
) -> Option<f32> {
    let hit = event.hits_with_options(
        cx,
        handle_area,
        HitOptions::new().with_margin(Inset {
            left: 4.0,
            right: 4.0,
            top: 0.0,
            bottom: 0.0,
        }),
    );
    match hit {
        Hit::FingerHoverIn(_) => {
            cx.set_cursor(MouseCursor::ColResize);
            None
        }
        Hit::FingerDown(f) => {
            cx.set_cursor(MouseCursor::ColResize);
            *drag_start = Some((f.abs.x, current_width));
            None
        }
        Hit::FingerMove(f) => drag_start
            .map(|(start_x, start_width)| start_width + delta_sign * (f.abs.x - start_x) as f32),
        Hit::FingerUp(_) => {
            *drag_start = None;
            None
        }
        _ => None,
    }
}

impl App {
    pub(super) fn set_sidebar_width(&mut self, cx: &mut Cx, width: f32) {
        let clamped = width.clamp(SIDEBAR_MIN_WIDTH, SIDEBAR_MAX_WIDTH);
        self.sidebar_width = clamped;
        self.ui
            .side_panel(cx, &[id!(side_panel)])
            .set_open_size(cx, clamped);
        self.ui.redraw(cx);
    }

    pub(super) fn set_right_sidebar_width(&mut self, cx: &mut Cx, width: f32) {
        let clamped = width.clamp(RIGHT_SIDEBAR_MIN_WIDTH, RIGHT_SIDEBAR_MAX_WIDTH);
        self.right_sidebar_width = clamped;
        self.ui
            .side_panel(cx, &[id!(right_side_panel)])
            .set_open_size(cx, clamped);
        self.ui.redraw(cx);
    }

    pub(super) fn update_sidebar_handle_visibility(&self, cx: &mut Cx) {
        self.ui
            .view(cx, &[id!(sidebar_resize_handle)])
            .set_visible(cx, self.sidebar_open);
        self.ui
            .view(cx, &[id!(right_sidebar_resize_handle)])
            .set_visible(cx, self.right_sidebar_open);
    }

    pub(super) fn update_sidebar_panel_visibility(&mut self, cx: &mut Cx) {
        let show_files = self.sidebar_mode == SidebarMode::Files;
        let show_settings = self.sidebar_mode == SidebarMode::Settings;

        // Use widget() for custom widgets (FilesPanel, SettingsDialog).
        self.ui
            .widget(cx, &[id!(side_panel), id!(files_panel)])
            .set_visible(cx, show_files);
        self.ui
            .widget(cx, &[id!(side_panel), id!(settings_panel)])
            .set_visible(cx, show_settings);

        // Force redraw of the side panel to ensure visibility changes take effect
        self.ui.view(cx, &[id!(side_panel)]).redraw(cx);
        self.ui.redraw(cx);
    }

    pub(super) fn toggle_sidebar(&mut self, cx: &mut Cx) {
        self.sidebar_open = !self.sidebar_open;

        if self.sidebar_open && self.sidebar_width <= 0.0 {
            self.sidebar_width = SIDEBAR_DEFAULT_WIDTH;
        }
        if self.sidebar_open {
            self.set_sidebar_width(cx, self.sidebar_width);
        }
        self.ui
            .side_panel(cx, &[id!(side_panel)])
            .set_open(cx, self.sidebar_open);
        self.ui
            .side_panel(cx, &[id!(traffic_light_spacer)])
            .set_open(cx, !self.sidebar_open);
        self.update_sidebar_handle_visibility(cx);
        self.update_sidebar_panel_visibility(cx);

        if self.sidebar_open {
            self.ui
                .view(cx, &[id!(hamburger_button)])
                .animator_play(cx, &[id!(open), id!(on)]);
        } else {
            self.ui
                .view(cx, &[id!(hamburger_button)])
                .animator_play(cx, &[id!(open), id!(off)]);
        }
    }

    pub(super) fn toggle_terminal(&mut self, cx: &mut Cx) {
        self.terminal_open = !self.terminal_open;
        self.ui
            .terminal_panel(cx, &[id!(terminal_panel_wrap)])
            .set_open(cx, self.terminal_open);
    }

    pub(super) fn toggle_right_sidebar(&mut self, cx: &mut Cx) {
        self.right_sidebar_open = !self.right_sidebar_open;

        if self.right_sidebar_open && self.right_sidebar_width <= 0.0 {
            self.right_sidebar_width = RIGHT_SIDEBAR_DEFAULT_WIDTH;
        }
        if self.right_sidebar_open {
            self.set_right_sidebar_width(cx, self.right_sidebar_width);
        }
        self.ui
            .side_panel(cx, &[id!(right_side_panel)])
            .set_open(cx, self.right_sidebar_open);
        self.update_sidebar_handle_visibility(cx);
    }

    pub(super) fn handle_sidebar_resize(&mut self, cx: &mut Cx, event: &Event) {
        if !self.sidebar_open {
            self.sidebar_drag_start = None;
            return;
        }
        let handle_area = self.ui.view(cx, &[id!(sidebar_resize_handle)]).area();
        let drag_start = self.sidebar_drag_start;
        let current_width = self.sidebar_width;
        let mut drag_start_mut = drag_start;
        if let Some(new_width) = perform_sidebar_resize(
            cx,
            event,
            handle_area,
            &mut drag_start_mut,
            current_width,
            1.0,
        ) {
            self.sidebar_drag_start = drag_start_mut;
            self.set_sidebar_width(cx, new_width);
        } else {
            self.sidebar_drag_start = drag_start_mut;
        }
    }

    pub(super) fn handle_right_sidebar_resize(&mut self, cx: &mut Cx, event: &Event) {
        if !self.right_sidebar_open {
            self.right_sidebar_drag_start = None;
            return;
        }
        let handle_area = self.ui.view(cx, &[id!(right_sidebar_resize_handle)]).area();
        let drag_start = self.right_sidebar_drag_start;
        let current_width = self.right_sidebar_width;
        let mut drag_start_mut = drag_start;
        if let Some(new_width) = perform_sidebar_resize(
            cx,
            event,
            handle_area,
            &mut drag_start_mut,
            current_width,
            -1.0,
        ) {
            self.right_sidebar_drag_start = drag_start_mut;
            self.set_right_sidebar_width(cx, new_width);
        } else {
            self.right_sidebar_drag_start = drag_start_mut;
        }
    }
}
