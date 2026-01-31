//! SidePanel widget - Animated slide-out panel with open/close functionality.
//!
//! This module contains the Rust implementation for the SidePanel widget.
//! The DSL definition is in lib.rs.

use makepad_widgets::*;

#[derive(Live, Widget)]
pub struct SidePanel {
    #[deref]
    view: View,

    #[live]
    animator_panel_progress: f32,

    #[live(280.0)]
    open_size: f32,

    #[live(0.0)]
    close_size: f32,

    #[animator]
    animator: Animator,
}

impl LiveHook for SidePanel {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        if self.is_open(cx) {
            self.animator_panel_progress = 1.0;
        } else {
            self.animator_panel_progress = 0.0;
        }
    }
}

impl Widget for SidePanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let mut walk = walk;
        let size_range = self.open_size - self.close_size;
        let size = self.close_size + size_range * self.animator_panel_progress;
        walk.width = Size::Fixed(size.into());
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SidePanel {
    pub fn is_open(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, &[id!(open), id!(on)])
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if open {
            self.animator_play(cx, &[id!(open), id!(on)]);
        } else {
            self.animator_play(cx, &[id!(open), id!(off)]);
        }
    }
}

impl SidePanelRef {
    pub fn is_open(&self, cx: &Cx) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_open(cx)
        } else {
            false
        }
    }

    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }
}
