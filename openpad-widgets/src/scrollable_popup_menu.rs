use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.ScrollablePopupMenuItemBase = #(ScrollablePopupMenuItem::script_component(vm))
    mod.widgets.ScrollablePopupMenuBase = #(ScrollablePopupMenu::script_component(vm))

    mod.widgets.ScrollablePopupMenuItem = set_type_default() do mod.widgets.ScrollablePopupMenuItemBase{
        width: Fill
        height: 22.0
        align: Align{y: 0.5}
        padding: theme.mspace_1{left: 15.}

        draw_text +: {
            active: instance(0.0)
            hover: instance(0.0)
            disabled: instance(0.0)

            color: theme.color_label_inner
            color_hover: uniform(theme.color_label_inner_hover)
            color_active: uniform(theme.color_label_inner_active)
            color_disabled: uniform(theme.color_label_inner_disabled)

            text_style: theme.font_regular{
                font_size: theme.font_size_p
            }

            get_color: fn() {
                self.color
                    .mix(self.color_active, self.active)
                    .mix(self.color_hover, self.hover)
                    .mix(self.color_disabled, self.disabled)
            }
        }

        draw_bg +: {
            active: instance(0.0)
            hover: instance(0.0)
            disabled: instance(0.0)

            gradient_border_horizontal: uniform(0.0)
            gradient_fill_horizontal: uniform(0.0)

            color_dither: uniform(1.0)
            border_size: uniform(theme.beveling)
            border_radius: uniform(theme.corner_radius)

            color: uniform(theme.color_u_hidden)
            color_hover: uniform(theme.color_outset_hover)
            color_active: uniform(theme.color_outset_active)
            color_disabled: uniform(theme.color_outset_disabled)

            color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            color_2_hover: uniform(theme.color_outset_2_hover)
            color_2_active: uniform(theme.color_outset_2_active)
            color_2_disabled: uniform(theme.color_outset_2_disabled)

            border_color: uniform(theme.color_u_hidden)
            border_color_hover: uniform(theme.color_u_hidden)
            border_color_active: uniform(theme.color_u_hidden)
            border_color_disabled: uniform(theme.color_u_hidden)

            border_color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            border_color_2_hover: uniform(theme.color_u_hidden)
            border_color_2_active: uniform(theme.color_u_hidden)
            border_color_2_disabled: uniform(theme.color_u_hidden)

            mark_color: uniform(theme.color_u_hidden)
            mark_color_active: uniform(theme.color_mark_active)
            mark_color_disabled: uniform(theme.color_mark_disabled)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                let border_sz_uv = vec2(
                    self.border_size / self.rect_size.x
                    self.border_size / self.rect_size.y
                )

                let sz_inner_px = vec2(
                    self.rect_size.x - self.border_size * 2.
                    self.rect_size.y - self.border_size * 2.
                )

                let scale_factor_fill = vec2(
                    self.rect_size.x / sz_inner_px.x
                    self.rect_size.y / sz_inner_px.y
                )

                sdf.box(
                    self.border_size
                    self.border_size
                    self.rect_size.x - self.border_size * 2.
                    self.rect_size.y - self.border_size * 2.
                    self.border_radius
                )

                let mut color_fill = self.color
                let mut color_fill_hover = self.color_hover
                let mut color_fill_active = self.color_active
                let mut color_fill_disabled = self.color_disabled

                if self.color_2.x > -0.5 {
                    let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                    let gradient_fill = vec2(
                        self.pos.x * scale_factor_fill.x - border_sz_uv.x * 2. + dither
                        self.pos.y * scale_factor_fill.y - border_sz_uv.y * 2. + dither
                    )
                    let dir = if self.gradient_fill_horizontal > 0.5 gradient_fill.x else gradient_fill.y
                    color_fill = mix(self.color, self.color_2, dir)
                    color_fill_hover = mix(self.color_hover, self.color_2_hover, dir)
                    color_fill_active = mix(self.color_active, self.color_2_active, dir)
                    color_fill_disabled = mix(self.color_disabled, self.color_2_disabled, dir)
                }

                let mut color_stroke = self.border_color
                let mut color_stroke_hover = self.border_color_hover
                let mut color_stroke_active = self.border_color_active
                let mut color_stroke_disabled = self.border_color_disabled

                if self.border_color_2.x > -0.5 {
                    let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither
                    let gradient_border = vec2(
                        self.pos.x + dither
                        self.pos.y + dither
                    )
                    let dir = if self.gradient_border_horizontal > 0.5 gradient_border.x else gradient_border.y
                    color_stroke = mix(self.border_color, self.border_color_2, dir)
                    color_stroke_hover = mix(self.border_color_hover, self.border_color_2_hover, dir)
                    color_stroke_active = mix(self.border_color_active, self.border_color_2_active, dir)
                    color_stroke_disabled = mix(self.border_color_disabled, self.border_color_2_disabled, dir)
                }

                let fill = color_fill
                    .mix(color_fill_active, self.active)
                    .mix(color_fill_hover, self.hover)
                    .mix(color_fill_disabled, self.disabled)

                let stroke = color_stroke
                    .mix(color_stroke_active, self.active)
                    .mix(color_stroke_hover, self.hover)
                    .mix(color_stroke_disabled, self.disabled)

                sdf.fill_keep(fill)
                sdf.stroke(stroke, self.border_size)

                return sdf.result
            }
        }

        animator: Animator{
            disabled: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.}}
                    apply: {
                        draw_bg: {disabled: 0.0}
                        draw_text: {disabled: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {disabled: 1.0}
                        draw_text: {disabled: 1.0}
                    }
                }
            }
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Snap}
                    apply: {
                        draw_bg: {hover: 0.0}
                        draw_text: {hover: 0.0}
                    }
                }
                on: AnimatorState{
                    cursor: MouseCursor.Hand
                    from: {all: Snap}
                    apply: {
                        draw_bg: {hover: 1.0}
                        draw_text: {hover: 1.0}
                    }
                }
            }

            active: {
                default: @off
                off: AnimatorState{
                    from: {all: Snap}
                    apply: {
                        draw_bg: {active: 0.0}
                        draw_text: {active: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Snap}
                    apply: {
                        draw_bg: {active: 1.0}
                        draw_text: {active: 1.0}
                    }
                }
            }
        }
        indent_width: 10.0
    }

    mod.widgets.ScrollablePopupMenuFlat = set_type_default() do mod.widgets.ScrollablePopupMenuBase{
        width: 150.0
        height: Fit
        flow: Flow.Down
        padding: theme.mspace_1

        menu_item: mod.widgets.ScrollablePopupMenuItem{}

        draw_bg +: {
            border_size: uniform(theme.beveling)
            gradient_border_horizontal: uniform(0.0)
            gradient_fill_horizontal: uniform(0.0)
            border_radius: uniform(theme.corner_radius)

            color: uniform(theme.color_fg_app)
            color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            border_color: uniform(theme.color_bevel)
            border_color_2: uniform(vec4(-1.0, -1.0, -1.0, -1.0))
            color_dither: uniform(1.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let dither = Math.random_2d(self.pos.xy) * 0.04 * self.color_dither

                let color_2 = if self.color_2.x > -0.5 self.color_2 else self.color
                let border_color_2 = if self.border_color_2.x > -0.5 self.border_color_2 else self.border_color

                let border_sz_uv = vec2(
                    self.border_size / self.rect_size.x
                    self.border_size / self.rect_size.y
                )

                let gradient_border = vec2(
                    self.pos.x + dither
                    self.pos.y + dither
                )

                let gradient_border_dir = if self.gradient_border_horizontal > 0.5 gradient_border.x else gradient_border.y

                let sz_inner_px = vec2(
                    self.rect_size.x - self.border_size * 2.
                    self.rect_size.y - self.border_size * 2.
                )

                let scale_factor_fill = vec2(
                    self.rect_size.x / sz_inner_px.x
                    self.rect_size.y / sz_inner_px.y
                )

                let gradient_fill = vec2(
                    self.pos.x * scale_factor_fill.x - border_sz_uv.x * 2. + dither
                    self.pos.y * scale_factor_fill.y - border_sz_uv.y * 2. + dither
                )

                let gradient_fill_dir = if self.gradient_fill_horizontal > 0.5 gradient_fill.x else gradient_fill.y

                sdf.box(
                    self.border_size
                    self.border_size
                    self.rect_size.x - self.border_size * 2.
                    self.rect_size.y - self.border_size * 2.
                    self.border_radius
                )

                sdf.fill_keep(mix(self.color, color_2, gradient_fill_dir))

                if self.border_size > 0.0 {
                    sdf.stroke(
                        mix(self.border_color, border_color_2, gradient_border_dir)
                        self.border_size
                    )
                }

                return sdf.result
            }
        }

        draw_scrollbar +: {
            color: uniform(#5a5f68)
            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 2.0)
                sdf.fill(self.color)
                return sdf.result
            }
        }

        scrollbar_width: 3.0
        scrollbar_inset: 2.0
        scrollbar_min_height: 16.0
    }

    mod.widgets.ScrollablePopupMenu = mod.widgets.ScrollablePopupMenuFlat{
        menu_item: mod.widgets.ScrollablePopupMenuItem{}
        draw_bg +: {
            border_color: theme.color_bevel_outset_1
            border_color_2: theme.color_bevel_outset_2
        }
    }
}

#[derive(Script, ScriptHook, Animator)]
pub struct ScrollablePopupMenuItem {
    #[source]
    source: ScriptObjectRef,

    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_text: DrawText,

    #[layout]
    layout: Layout,
    #[apply_default]
    animator: Animator,
    #[walk]
    walk: Walk,

    #[live]
    indent_width: f32,
    #[live]
    icon_walk: Walk,

    #[live]
    opened: f32,
    #[live]
    hover: f32,
    #[live]
    active: f32,
}

#[derive(Script)]
pub struct ScrollablePopupMenu {
    #[source]
    source: ScriptObjectRef,

    #[live]
    draw_list: DrawList2d,
    #[live]
    menu_item: ScriptValue,

    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_scrollbar: DrawQuad,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live]
    items: Vec<String>,
    #[rust]
    first_tap: bool,
    #[rust]
    menu_items: ComponentMap<ScrollablePopupMenuItemId, ScrollablePopupMenuItem>,
    #[rust]
    init_select_item: Option<ScrollablePopupMenuItemId>,

    #[rust]
    count: usize,
    #[rust]
    max_height: f64,
    #[rust]
    menu_scroll: f64,
    #[rust]
    content_height: f64,

    #[live]
    scrollbar_width: f64,
    #[live]
    scrollbar_inset: f64,
    #[live]
    scrollbar_min_height: f64,
}

impl ScriptHook for ScrollablePopupMenu {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        apply: &Apply,
        scope: &mut Scope,
        _value: ScriptValue,
    ) {
        if !self.menu_item.is_nil() {
            for (_, node) in self.menu_items.iter_mut() {
                node.script_apply(vm, apply, scope, self.menu_item);
            }
        }
        self.draw_list.redraw(vm.cx_mut());
    }
}

pub enum ScrollablePopupMenuItemAction {
    WasSweeped,
    WasSelected,
    MightBeSelected,
    None,
}

#[derive(Clone, Default)]
pub enum ScrollablePopupMenuAction {
    WasSweeped(ScrollablePopupMenuItemId),
    WasSelected(ScrollablePopupMenuItemId),
    #[default]
    None,
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ScrollablePopupMenuItemId(pub LiveId);

impl ScrollablePopupMenuItem {
    pub fn draw_item(&mut self, cx: &mut Cx2d, label: &str) {
        self.draw_bg.begin(cx, self.walk, self.layout);
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), label);
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

    pub fn set_content_height(&mut self, height: f64) {
        self.content_height = height;
    }

    pub fn begin(&mut self, cx: &mut Cx2d) {
        self.draw_list.begin_overlay_reuse(cx);

        let size = cx.current_pass_size();
        cx.begin_root_turtle(size, Layout::flow_down());

        self.walk.height = if self.max_height > 0.0 {
            Size::Fixed(self.max_height)
        } else {
            Size::fit()
        };
        self.layout.clip_y = true;
        self.layout.scroll = dvec2(0.0, self.menu_scroll);

        self.draw_bg.begin(cx, self.walk, self.layout);
        self.count = 0;
    }

    pub fn end(&mut self, cx: &mut Cx2d, shift_area: Area, shift: Vec2d) {
        self.draw_bg.end(cx);
        self.draw_scrollbar_if_needed(cx);
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
            cx.with_vm(|vm| ScrollablePopupMenuItem::script_from_value(vm, menu_item))
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
            node.handle_event_with(cx, event, sweep_area, &mut |_, e| {
                actions.push((*item_id, e))
            });
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

    fn draw_scrollbar_if_needed(&mut self, cx: &mut Cx2d) {
        let rect = self.draw_bg.area().rect(cx);
        let visible_height = if self.max_height > 0.0 {
            self.max_height.min(self.content_height.max(0.0))
        } else {
            self.content_height.max(0.0)
        };
        if self.content_height <= visible_height || visible_height <= 0.0 {
            return;
        }

        let inset = self.scrollbar_inset.max(0.0);
        let width = self.scrollbar_width.max(1.0);
        let track_height = (visible_height - inset * 2.0).max(0.0);
        if track_height <= 0.0 {
            return;
        }

        let mut handle_height = visible_height * (visible_height / self.content_height);
        if handle_height < self.scrollbar_min_height {
            handle_height = self.scrollbar_min_height;
        }
        if handle_height > track_height {
            handle_height = track_height;
        }

        let max_scroll = (self.content_height - visible_height).max(0.0);
        let scroll_ratio = if max_scroll > 0.0 {
            (self.menu_scroll / max_scroll).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let max_handle_offset = (track_height - handle_height).max(0.0);
        let handle_offset = max_handle_offset * scroll_ratio;

        let pos = dvec2(
            rect.pos.x + rect.size.x - inset - width,
            rect.pos.y + inset + handle_offset,
        );
        let size = dvec2(width, handle_height);
        self.draw_scrollbar.draw_abs(cx, Rect { pos, size });
    }
}
