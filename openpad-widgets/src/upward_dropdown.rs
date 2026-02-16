use {
    crate::scrollable_popup_menu::{ScrollablePopupMenu, ScrollablePopupMenuAction},
    makepad_widgets::*,
    std::cell::RefCell,
    std::rc::Rc,
};

#[derive(Script, ScriptHook, Clone, Copy, Debug)]
#[repr(C)]
pub enum PopupMenuPosition {
    #[pick]
    AboveInput,
    OnSelected,
    BelowInput,
}

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.PopupMenuPosition = #(PopupMenuPosition::script_component(vm))
    mod.widgets.UpDrawLabelTextBase = #(UpDrawLabelText::script_component(vm))
    mod.widgets.UpDropDownBase = #(UpDropDown::register_widget(vm))

    set_type_default() do #(UpDrawLabelText::script_shader(vm)){
        ..mod.draw.DrawText
    }

    mod.widgets.UpDropDown = mod.widgets.UpDropDownBase {
        popup_menu: mod.widgets.ScrollablePopupMenu {}
    }
}

#[derive(Script, Widget, Animator)]
pub struct UpDropDown {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_text: UpDrawLabelText,

    #[walk]
    walk: Walk,

    #[live]
    bind: String,
    #[live]
    bind_enum: String,

    #[live]
    popup_menu: ScriptValue,

    #[live]
    labels: Vec<String>,
    #[live]
    values: Vec<ScriptValue>,

    #[live]
    popup_menu_position: PopupMenuPosition,

    #[rust]
    is_active: bool,

    #[live]
    selected_item: usize,

    #[layout]
    layout: Layout,

    #[rust]
    menu_scroll: f64,
    #[rust]
    menu_scroll_max: f64,
    #[rust]
    menu_visible_height: f64,
    #[rust]
    popup_defer_frames: u8,
}

#[derive(Default, Clone)]
struct PopupMenuGlobal {
    map: Rc<RefCell<ComponentMap<ScriptValue, ScrollablePopupMenu>>>,
}

#[derive(Script, ScriptHook)]
#[repr(C)]
struct UpDrawLabelText {
    #[deref]
    draw_super: DrawText,
    #[live]
    focus: f32,
    #[live]
    hover: f32,
}

impl ScriptHook for UpDropDown {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _obj: ScriptValue,
    ) {
        if self.popup_menu.is_nil() {
            return;
        }
        vm.with_cx_mut(|cx| {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let Ok(mut map) = global.map.try_borrow_mut() else {
                return;
            };

            let popup_menu_val = self.popup_menu;
            map.get_or_insert(cx, popup_menu_val, |cx| {
                cx.with_vm(|vm| ScrollablePopupMenu::script_from_value(vm, popup_menu_val))
            });
        });
    }
}

#[derive(Clone, Debug, Default)]
pub enum UpDropDownAction {
    Select(usize, ScriptValue),
    #[default]
    None,
}

impl UpDropDown {
    pub fn set_active(&mut self, cx: &mut Cx) {
        self.is_active = true;
        self.menu_scroll = 0.0;
        self.menu_scroll_max = 0.0;
        self.menu_visible_height = 0.0;
        self.popup_defer_frames = 1;
        self.draw_bg.redraw(cx);
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut map = global.map.borrow_mut();
        let menu = map.get_mut(&self.popup_menu).unwrap();
        let node_id = LiveId(self.selected_item as u64).into();
        menu.init_select_item(node_id);
        cx.sweep_lock(self.draw_bg.area());
    }

    pub fn set_closed(&mut self, cx: &mut Cx) {
        self.is_active = false;
        self.menu_scroll = 0.0;
        self.menu_scroll_max = 0.0;
        self.menu_visible_height = 0.0;
        self.popup_defer_frames = 0;
        self.draw_bg.redraw(cx);
        cx.sweep_unlock(self.draw_bg.area());
    }

    pub fn draw_text(&mut self, cx: &mut Cx2d, label: &str) {
        self.draw_bg.begin(cx, self.walk, self.layout);
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), label);
        self.draw_bg.end(cx);
    }

    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        self.draw_bg.begin(cx, walk, self.layout);

        if let Some(val) = self.labels.get(self.selected_item) {
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), val);
        } else {
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), " ");
        }
        self.draw_bg.end(cx);

        cx.add_nav_stop(self.draw_bg.area(), NavRole::DropDown, Inset::default());

        if self.is_active && !self.popup_menu.is_nil() {
            if self.popup_defer_frames > 0 {
                self.popup_defer_frames -= 1;
                cx.new_next_frame();
                return;
            }
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup_menu).unwrap();
            let window_size = cx.current_pass_size();
            let anchor = self.draw_bg.area().rect(cx);

            // Guard against invalid geometry on the first frame after open.
            if window_size.y <= 1.0 || anchor.size.y <= 1.0 {
                cx.new_next_frame();
                return;
            }

            let padding = 4.0;
            let window_top = padding;
            let window_bottom = (window_size.y - padding).max(padding);
            let available_above = (anchor.pos.y - padding).max(0.0);
            let available_below =
                (window_size.y - (anchor.pos.y + anchor.size.y) - padding).max(0.0);
            let max_height = match self.popup_menu_position {
                PopupMenuPosition::AboveInput => {
                    if available_above > 0.0 {
                        available_above
                    } else {
                        available_below
                    }
                }
                PopupMenuPosition::BelowInput => {
                    if available_below > 0.0 {
                        available_below
                    } else {
                        available_above
                    }
                }
                PopupMenuPosition::OnSelected => available_above.max(available_below),
            }
            .max(1.0);

            popup_menu.set_max_height(max_height);
            popup_menu.set_scroll(self.menu_scroll);
            popup_menu.begin(cx);

            match self.popup_menu_position {
                PopupMenuPosition::OnSelected => {
                    let mut item_pos = None;
                    for (i, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(i as u64).into();
                        if i == self.selected_item {
                            item_pos = Some(cx.turtle().pos());
                        }
                        popup_menu.draw_item(cx, node_id, item);
                    }

                    let menu_height = cx.turtle().used_height();
                    popup_menu.set_content_height(menu_height);

                    let mut desired_scroll_max = 0.0;
                    if menu_height > max_height {
                        desired_scroll_max = menu_height - max_height;
                    }

                    let mut needs_redraw = false;
                    if (self.menu_scroll_max - desired_scroll_max).abs() > 0.5 {
                        self.menu_visible_height = if desired_scroll_max > 0.0 {
                            menu_height.min(max_height)
                        } else {
                            0.0
                        };
                        self.menu_scroll_max = desired_scroll_max;
                        self.menu_scroll = self.menu_scroll.clamp(0.0, self.menu_scroll_max);
                        needs_redraw = true;
                    } else if self.menu_scroll > self.menu_scroll_max {
                        self.menu_scroll = self.menu_scroll_max;
                        needs_redraw = true;
                    }

                    if needs_redraw {
                        popup_menu.redraw(cx);
                    }

                    let drawn_height = menu_height.min(max_height);
                    let mut shift = -item_pos.unwrap_or(dvec2(0.0, 0.0));
                    let mut shift_y = shift.y;
                    let mut top = anchor.pos.y + shift_y;
                    if top < window_top {
                        shift_y += window_top - top;
                        top = window_top;
                    }
                    let bottom = top + drawn_height;
                    if bottom > window_bottom {
                        shift_y -= bottom - window_bottom;
                    }
                    shift.y = shift_y;
                    popup_menu.end(cx, self.draw_bg.area(), shift);
                }
                PopupMenuPosition::AboveInput => {
                    for (i, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(i as u64).into();
                        popup_menu.draw_item(cx, node_id, item);
                    }

                    let menu_height = cx.turtle().used_height();
                    popup_menu.set_content_height(menu_height);

                    let mut desired_scroll_max = 0.0;
                    if menu_height > max_height {
                        desired_scroll_max = menu_height - max_height;
                    }

                    let mut needs_redraw = false;
                    if (self.menu_scroll_max - desired_scroll_max).abs() > 0.5 {
                        self.menu_visible_height = if desired_scroll_max > 0.0 {
                            menu_height.min(max_height)
                        } else {
                            0.0
                        };
                        self.menu_scroll_max = desired_scroll_max;
                        self.menu_scroll = self.menu_scroll.clamp(0.0, self.menu_scroll_max);
                        needs_redraw = true;
                    } else if self.menu_scroll > self.menu_scroll_max {
                        self.menu_scroll = self.menu_scroll_max;
                        needs_redraw = true;
                    }

                    if needs_redraw {
                        popup_menu.redraw(cx);
                    }

                    let drawn_height = menu_height.min(max_height);

                    let mut shift_y = -drawn_height;
                    let mut top = anchor.pos.y + shift_y;
                    if top < window_top {
                        shift_y += window_top - top;
                        top = window_top;
                    }
                    let bottom = top + drawn_height;
                    if bottom > window_bottom {
                        shift_y -= bottom - window_bottom;
                    }

                    let shift = Vec2d { x: 0.0, y: shift_y };
                    popup_menu.end(cx, self.draw_bg.area(), shift);
                }
                PopupMenuPosition::BelowInput => {
                    for (i, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(i as u64).into();
                        popup_menu.draw_item(cx, node_id, item);
                    }

                    let menu_height = cx.turtle().used_height();
                    popup_menu.set_content_height(menu_height);

                    let mut desired_scroll_max = 0.0;
                    if menu_height > max_height {
                        desired_scroll_max = menu_height - max_height;
                    }

                    let mut needs_redraw = false;
                    if (self.menu_scroll_max - desired_scroll_max).abs() > 0.5 {
                        self.menu_visible_height = if desired_scroll_max > 0.0 {
                            menu_height.min(max_height)
                        } else {
                            0.0
                        };
                        self.menu_scroll_max = desired_scroll_max;
                        self.menu_scroll = self.menu_scroll.clamp(0.0, self.menu_scroll_max);
                        needs_redraw = true;
                    } else if self.menu_scroll > self.menu_scroll_max {
                        self.menu_scroll = self.menu_scroll_max;
                        needs_redraw = true;
                    }

                    if needs_redraw {
                        popup_menu.redraw(cx);
                    }

                    let drawn_height = menu_height.min(max_height);

                    let mut shift_y = anchor.size.y;
                    let mut top = anchor.pos.y + shift_y;
                    if top < window_top {
                        shift_y += window_top - top;
                        top = window_top;
                    }
                    let bottom = top + drawn_height;
                    if bottom > window_bottom {
                        shift_y -= bottom - window_bottom;
                    }

                    let shift = Vec2d { x: 0.0, y: shift_y };
                    popup_menu.end(cx, self.draw_bg.area(), shift);
                }
            }
        }
    }
}

impl Widget for UpDropDown {
    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.animator_toggle(
            cx,
            disabled,
            Animate::Yes,
            &[id!(disabled), id!(on)],
            &[id!(disabled), id!(off)],
        );
    }

    fn disabled(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, &[id!(disabled), id!(on)])
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        let uid = self.widget_uid();

        if self.is_active && !self.popup_menu.is_nil() {
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let menu = map.get_mut(&self.popup_menu).unwrap();
            let mut close = false;

            if self.menu_scroll_max > 0.0 {
                if let Event::Scroll(e) = event {
                    if menu.menu_contains_pos(cx, e.abs) && !e.handled_y.get() {
                        let next_scroll =
                            (self.menu_scroll + e.scroll.y).clamp(0.0, self.menu_scroll_max);
                        if (next_scroll - self.menu_scroll).abs() > 0.1 {
                            self.menu_scroll = next_scroll;
                            e.handled_y.set(true);
                            menu.redraw(cx);
                        }
                    }
                }
            }

            menu.handle_event_with(cx, event, self.draw_bg.area(), &mut |cx, action| {
                match action {
                    ScrollablePopupMenuAction::WasSweeped(_node_id) => {}
                    ScrollablePopupMenuAction::WasSelected(node_id) => {
                        self.selected_item = node_id.0 .0 as usize;
                        cx.widget_action(
                            uid,
                            UpDropDownAction::Select(
                                self.selected_item,
                                self.values
                                    .get(self.selected_item)
                                    .copied()
                                    .unwrap_or(ScriptValue::NIL),
                            ),
                        );
                        self.draw_bg.redraw(cx);
                        close = true;
                    }
                    _ => (),
                }
            });
            if close {
                self.set_closed(cx);
            }

            if let Event::MouseDown(e) = event {
                if !menu.menu_contains_pos(cx, e.abs) {
                    self.set_closed(cx);
                    self.animator_play(cx, &[id!(hover), id!(off)]);
                    return;
                }
            }
        }

        match event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area()) {
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, &[id!(focus), id!(off)]);
                self.set_closed(cx);
                self.animator_play(cx, &[id!(hover), id!(off)]);
                self.draw_bg.redraw(cx);
            }
            Hit::KeyFocus(_) => {
                self.animator_play(cx, &[id!(focus), id!(on)]);
            }
            Hit::KeyDown(ke) => match ke.key_code {
                KeyCode::ArrowUp => {
                    if self.selected_item > 0 {
                        self.selected_item -= 1;
                        cx.widget_action(
                            uid,
                            UpDropDownAction::Select(
                                self.selected_item,
                                self.values
                                    .get(self.selected_item)
                                    .copied()
                                    .unwrap_or(ScriptValue::NIL),
                            ),
                        );
                        self.set_closed(cx);
                        self.draw_bg.redraw(cx);
                    }
                }
                KeyCode::ArrowDown => {
                    if !self.labels.is_empty() && self.selected_item < self.labels.len() - 1 {
                        self.selected_item += 1;
                        cx.widget_action(
                            uid,
                            UpDropDownAction::Select(
                                self.selected_item,
                                self.values
                                    .get(self.selected_item)
                                    .copied()
                                    .unwrap_or(ScriptValue::NIL),
                            ),
                        );
                        self.set_closed(cx);
                        self.draw_bg.redraw(cx);
                    }
                }
                _ => (),
            },
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                if self.animator_in_state(cx, &[id!(disabled), id!(off)]) {
                    cx.set_key_focus(self.draw_bg.area());
                    self.animator_play(cx, &[id!(hover), id!(on)]);
                    self.set_active(cx);
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, &[id!(hover), id!(on)]);
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, &[id!(hover), id!(off)]);
            }
            Hit::FingerUp(fe) if fe.is_primary_hit() => {
                if fe.is_over {
                    if fe.device.has_hovers() {
                        self.animator_play(cx, &[id!(hover), id!(on)]);
                    }
                } else {
                    self.animator_play(cx, &[id!(hover), id!(off)]);
                }
            }
            _ => (),
        };
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_walk(cx, walk);
        DrawStep::done()
    }
}

impl UpDropDownRef {
    pub fn set_labels_with<F: FnMut(&mut String)>(&self, cx: &mut Cx, mut f: F) {
        if let Some(mut inner) = self.borrow_mut() {
            let mut i = 0;
            loop {
                if i >= inner.labels.len() {
                    inner.labels.push(String::new());
                }
                let s = &mut inner.labels[i];
                s.clear();
                f(s);
                if s.is_empty() {
                    break;
                }
                i += 1;
            }
            inner.labels.truncate(i);
            inner.draw_bg.redraw(cx);
        }
    }

    pub fn set_labels(&self, cx: &mut Cx, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.labels = labels;
            inner.draw_bg.redraw(cx);
        }
    }

    pub fn selected(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let UpDropDownAction::Select(id, _) = item.cast() {
                return Some(id);
            }
        }
        None
    }

    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let UpDropDownAction::Select(id, _) = item.cast() {
                return Some(id);
            }
        }
        None
    }

    pub fn changed_label(&self, actions: &Actions) -> Option<String> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let UpDropDownAction::Select(id, _) = item.cast() {
                if let Some(inner) = self.borrow() {
                    return Some(inner.labels[id].clone());
                }
            }
        }
        None
    }

    pub fn set_selected_item(&self, cx: &mut Cx, item: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            let new_selected = item.min(inner.labels.len().max(1) - 1);
            if new_selected != inner.selected_item {
                inner.selected_item = new_selected;
                inner.draw_bg.redraw(cx);
            }
        }
    }

    pub fn selected_item(&self) -> usize {
        if let Some(inner) = self.borrow() {
            return inner.selected_item;
        }
        0
    }

    pub fn selected_label(&self) -> String {
        if let Some(inner) = self.borrow() {
            return inner.labels[inner.selected_item].clone();
        }
        "".to_string()
    }

    pub fn set_selected_by_label(&self, label: &str, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            if let Some(index) = inner.labels.iter().position(|v| v == label) {
                if inner.selected_item != index {
                    inner.selected_item = index;
                    inner.draw_bg.redraw(cx);
                }
            }
        }
    }
}
