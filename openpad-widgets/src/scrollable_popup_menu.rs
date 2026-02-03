use {
    makepad_widgets::{
        makepad_draw::*,
        widget::*,
    },
};

live_design! {
    link widgets;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    pub ScrollablePopupMenuItemBase = {{ScrollablePopupMenuItem}} {}
    pub ScrollablePopupMenuBase = {{ScrollablePopupMenu}} {}

    pub ScrollablePopupMenuItem = <ScrollablePopupMenuItemBase> {
        width: Fill, height: 22.0,
        align: { y: 0.5 }
        padding: <THEME_MSPACE_1> { left: 15. }

        draw_text: {
            instance active: 0.0
            instance hover: 0.0
            instance disabled: 0.0

            uniform color: (THEME_COLOR_LABEL_INNER)
            uniform color_hover: (THEME_COLOR_LABEL_INNER_HOVER)
            uniform color_active: (THEME_COLOR_LABEL_INNER_ACTIVE)
            uniform color_disabled: (THEME_COLOR_LABEL_INNER_DISABLED)

            text_style: <THEME_FONT_REGULAR> {
                font_size: (THEME_FONT_SIZE_P),
            }

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            self.color,
                            self.color_active,
                            self.active
                        ),
                        self.color_hover,
                        self.hover
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }

        draw_bg: {
            instance active: 0.0
            instance hover: 0.0
            instance disabled: 0.0

            uniform gradient_border_horizontal: 0.0
            uniform gradient_fill_horizontal: 0.0

            uniform color_dither: 1.0
            uniform border_size: (THEME_BEVELING)
            uniform border_radius: (THEME_CORNER_RADIUS)

            uniform color: (THEME_COLOR_U_HIDDEN)
            uniform color_hover: (THEME_COLOR_OUTSET_HOVER)
            uniform color_active: (THEME_COLOR_OUTSET_ACTIVE)
            uniform color_disabled: (THEME_COLOR_OUTSET_DISABLED)

            uniform color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform color_2_hover: (THEME_COLOR_OUTSET_2_HOVER)
            uniform color_2_active: (THEME_COLOR_OUTSET_2_ACTIVE)
            uniform color_2_disabled: (THEME_COLOR_OUTSET_2_DISABLED)

            uniform border_color: (THEME_COLOR_U_HIDDEN)
            uniform border_color_hover: (THEME_COLOR_U_HIDDEN)
            uniform border_color_active: (THEME_COLOR_U_HIDDEN)
            uniform border_color_disabled: (THEME_COLOR_U_HIDDEN)

            uniform border_color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform border_color_2_hover: (THEME_COLOR_U_HIDDEN)
            uniform border_color_2_active: (THEME_COLOR_U_HIDDEN)
            uniform border_color_2_disabled: (THEME_COLOR_U_HIDDEN)

            uniform mark_color: (THEME_COLOR_U_HIDDEN)
            uniform mark_color_active: (THEME_COLOR_MARK_ACTIVE)
            uniform mark_color_disabled: (THEME_COLOR_MARK_DISABLED)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let dither = Math::random_2d(self.pos.xy) * 0.04 * self.color_dither;

                let color_2 = self.color;
                let color_2_hover = self.color_hover;
                let color_2_active = self.color_active;
                let color_2_disabled = self.color_disabled;

                let border_color_2 = self.border_color;
                let border_color_2_hover = self.border_color_hover;
                let border_color_2_active = self.border_color_active;
                let border_color_2_disabled = self.border_color_disabled;

                if (self.color_2.x > -0.5) {
                    color_2 = self.color_2
                    color_2_hover = self.color_2_hover
                    color_2_active = self.color_2_active;
                    color_2_disabled = self.color_2_disabled;
                }

                if (self.border_color_2.x > -0.5) {
                    border_color_2 = self.border_color_2;
                    border_color_2_hover = self.border_color_2_hover;
                    border_color_2_active = self.border_color_2_active;
                    border_color_2_disabled = self.border_color_2_disabled;
                }

                let border_sz_uv = vec2(
                    self.border_size / self.rect_size.x,
                    self.border_size / self.rect_size.y
                )

                let gradient_border = vec2(
                    self.pos.x + dither,
                    self.pos.y + dither
                )

                let gradient_fill = vec2(
                    self.pos.x * 0.5 + dither,
                    self.pos.y * 0.5 + dither
                )

                let gradient_border_dir = gradient_border.y;
                if (self.gradient_border_horizontal > 0.5) {
                    gradient_border_dir = gradient_border.x;
                }

                let gradient_fill_dir = gradient_fill.y;
                if (self.gradient_fill_horizontal > 0.5) {
                    gradient_fill_dir = gradient_fill.x;
                }

                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.,
                    self.rect_size.y - self.border_size * 2.,
                    self.border_radius
                )

                sdf.fill_keep(mix(color_2, color_2_hover, self.hover) * (1.0 - self.active)
                    + mix(color_2_active, color_2_active, self.hover) * self.active);

                if self.border_size > 0.0 {
                    sdf.stroke(
                        mix(
                            mix(border_color_2, border_color_2_hover, self.hover),
                            border_color_2_active,
                            self.active
                        ),
                        self.border_size
                    );
                }

                return sdf.result;
            }
        }
    }

    pub ScrollablePopupMenuFlat = <ScrollablePopupMenuBase> {
        width: 150., height: Fit,
        flow: Down,
        padding: <THEME_MSPACE_1> {}

        menu_item: <ScrollablePopupMenuItem> {}

        draw_bg: {
            uniform border_size: (THEME_BEVELING)
            uniform gradient_border_horizontal: 0.0
            uniform gradient_fill_horizontal: 0.0
            uniform border_radius: (THEME_CORNER_RADIUS)

            uniform color: (THEME_COLOR_FG_APP)
            uniform color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform border_color: (THEME_COLOR_BEVEL)
            uniform border_color_2: vec4(-1.0, -1.0, -1.0, -1.0)
            uniform color_dither: 1.0

            uniform gradient_fill_dir: 0.0
            uniform gradient_border_dir: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                let dither = Math::random_2d(self.pos.xy) * 0.04 * self.color_dither;

                let color_2 = self.color;
                let border_color_2 = self.border_color;

                if (self.color_2.x > -0.5) {
                    color_2 = self.color_2
                }

                if (self.border_color_2.x > -0.5) {
                    border_color_2 = self.border_color_2;
                }

                let border_sz_uv = vec2(
                    self.border_size / self.rect_size.x,
                    self.border_size / self.rect_size.y
                )

                let gradient_border = vec2(
                    self.pos.x + dither,
                    self.pos.y + dither
                )

                let gradient_fill = vec2(
                    self.pos.x * 0.5 + dither,
                    self.pos.y * 0.5 + dither
                )

                let gradient_border_dir = gradient_border.y;
                if (self.gradient_border_horizontal > 0.5) {
                    gradient_border_dir = gradient_border.x;
                }

                let gradient_fill_dir = gradient_fill.y;
                if (self.gradient_fill_horizontal > 0.5) {
                    gradient_fill_dir = gradient_fill.x;
                }

                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.,
                    self.rect_size.y - self.border_size * 2.,
                    self.border_radius
                )

                sdf.fill_keep(mix(self.color, color_2, gradient_fill_dir));

                if self.border_size > 0.0 {
                    sdf.stroke(
                        mix(
                            self.border_color,
                            border_color_2,
                            gradient_border_dir
                        ), self.border_size
                    );
                }

                return sdf.result;
            }
        }
    }

    pub ScrollablePopupMenu = <ScrollablePopupMenuFlat> {
        menu_item: <ScrollablePopupMenuItem> {}
        draw_bg: {
            border_color: (THEME_COLOR_BEVEL_OUTSET_1)
            border_color_2: (THEME_COLOR_BEVEL_OUTSET_2)
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
pub struct ScrollablePopupMenuItem {
    #[live] draw_bg: DrawQuad,
    #[live] draw_text: DrawText,

    #[layout] layout: Layout,
    #[animator] animator: Animator,
    #[walk] walk: Walk,

    #[live] indent_width: f32,
    #[live] icon_walk: Walk,

    #[live] opened: f32,
    #[live] hover: f32,
    #[live] active: f32,
}

#[derive(Live, LiveRegister)]
pub struct ScrollablePopupMenu {
    #[live] draw_list: DrawList2d,
    #[live] menu_item: Option<LivePtr>,

    #[live] draw_bg: DrawQuad,
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    #[live] items: Vec<String>,
    #[rust] first_tap: bool,
    #[rust] menu_items: ComponentMap<ScrollablePopupMenuItemId, ScrollablePopupMenuItem>,
    #[rust] init_select_item: Option<ScrollablePopupMenuItemId>,

    #[rust] count: usize,
    #[rust] max_height: f64,
    #[rust] menu_scroll: f64,
}

impl LiveHook for ScrollablePopupMenu {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        if let Some(index) = nodes.child_by_name(index, live_id!(list_node).as_field()) {
            for (_, node) in self.menu_items.iter_mut() {
                node.apply(cx, apply, index, nodes);
            }
        }
        self.draw_list.redraw(cx);
    }
}

pub enum ScrollablePopupMenuItemAction {
    WasSweeped,
    WasSelected,
    MightBeSelected,
    None,
}

#[derive(Clone, DefaultNone)]
pub enum ScrollablePopupMenuAction {
    WasSweeped(ScrollablePopupMenuItemId),
    WasSelected(ScrollablePopupMenuItemId),
    None,
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ScrollablePopupMenuItemId(pub LiveId);

impl ScrollablePopupMenuItem {
    pub fn draw_item(&mut self, cx: &mut Cx2d, label: &str) {
        self.draw_bg.begin(cx, self.walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), label);
        self.draw_bg.end(cx);
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        dispatch_action: &mut dyn FnMut(&mut Cx, ScrollablePopupMenuItemAction),
    ) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_bg.area().redraw(cx);
        }

        match event.hits_with_options(
            cx,
            self.draw_bg.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        ) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                dispatch_action(cx, ScrollablePopupMenuItemAction::WasSweeped);
                self.animator_play(cx, ids!(hover.on));
                self.animator_play(cx, ids!(active.on));
            }
            Hit::FingerUp(se) if se.is_primary_hit() => {
                if !se.is_sweep {
                    dispatch_action(cx, ScrollablePopupMenuItemAction::WasSelected);
                } else {
                    self.animator_play(cx, ids!(hover.off));
                    self.animator_play(cx, ids!(active.off));
                }
            }
            _ => {}
        }
    }
}

impl ScrollablePopupMenu {
    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: Vec2d) -> bool {
        self.draw_bg.area().clipped_rect(cx).contains(pos)
    }

    pub fn set_max_height(&mut self, max_height: f64) {
        self.max_height = max_height;
    }

    pub fn set_scroll(&mut self, scroll: f64) {
        self.menu_scroll = scroll;
    }

    pub fn begin(&mut self, cx: &mut Cx2d) {
        self.draw_list.begin_overlay_reuse(cx);

        let size = cx.current_pass_size();
        cx.begin_root_turtle(size, Layout::flow_down());

        if self.max_height > 0.0 {
            self.walk.height = Size::Fixed(self.max_height);
        } else {
            self.walk.height = Size::fit();
        }
        self.layout.clip_y = true;
        self.layout.scroll = dvec2(0.0, self.menu_scroll);

        self.draw_bg.begin(cx, self.walk, self.layout);
        self.count = 0;
    }

    pub fn end(&mut self, cx: &mut Cx2d, shift_area: Area, shift: Vec2d) {
        self.draw_bg.end(cx);
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
        self.menu_items.retain_visible();
        if let Some(init_select_item) = self.init_select_item.take() {
            self.select_item_state(cx, init_select_item);
        }
    }

    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_list.redraw(cx);
    }

    pub fn draw_item(&mut self, cx: &mut Cx2d, item_id: ScrollablePopupMenuItemId, label: &str) {
        self.count += 1;

        let menu_item = self.menu_item;
        let menu_item = self.menu_items.get_or_insert(cx, item_id, |cx| {
            ScrollablePopupMenuItem::new_from_ptr(cx, menu_item)
        });
        menu_item.draw_item(cx, label);
    }

    pub fn init_select_item(&mut self, which_id: ScrollablePopupMenuItemId) {
        self.init_select_item = Some(which_id);
        self.first_tap = true;
    }

    fn select_item_state(&mut self, cx: &mut Cx, which_id: ScrollablePopupMenuItemId) {
        for (id, item) in &mut *self.menu_items {
            if *id == which_id {
                item.animator_cut(cx, ids!(active.on));
                item.animator_cut(cx, ids!(hover.on));
            } else {
                item.animator_cut(cx, ids!(active.off));
                item.animator_cut(cx, ids!(hover.off));
            }
        }
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        dispatch_action: &mut dyn FnMut(&mut Cx, ScrollablePopupMenuAction),
    ) {
        let mut actions = Vec::new();
        for (item_id, node) in self.menu_items.iter_mut() {
            node.handle_event_with(cx, event, sweep_area, &mut |_, e| actions.push((*item_id, e)));
        }

        for (node_id, action) in actions {
            match action {
                ScrollablePopupMenuItemAction::MightBeSelected => {
                    if self.first_tap {
                        self.first_tap = false;
                    } else {
                        self.select_item_state(cx, node_id);
                        dispatch_action(cx, ScrollablePopupMenuAction::WasSelected(node_id));
                    }
                }
                ScrollablePopupMenuItemAction::WasSweeped => {
                    self.select_item_state(cx, node_id);
                    dispatch_action(cx, ScrollablePopupMenuAction::WasSweeped(node_id));
                }
                ScrollablePopupMenuItemAction::WasSelected => {
                    self.select_item_state(cx, node_id);
                    dispatch_action(cx, ScrollablePopupMenuAction::WasSelected(node_id));
                }
                _ => (),
            }
        }
    }
}
