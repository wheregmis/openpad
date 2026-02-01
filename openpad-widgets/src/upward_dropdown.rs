use {
    makepad_widgets::{
        makepad_derive_widget::*,
        makepad_draw::*,
        popup_menu::{PopupMenu, PopupMenuAction},
        widget::*,
    },
    std::cell::RefCell,
    std::rc::Rc,
};

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum PopupMenuPosition {
    #[pick]
    AboveInput,
    OnSelected,
    BelowInput,
}

live_design! {
    link widgets;
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub UpDrawLabelText = {{UpDrawLabelText}} {}
    pub UpDropDownBase = {{UpDropDown}} {}

    pub UpDropDown = <UpDropDownBase> {
        popup_menu: <PopupMenu> {}
    }
}

#[derive(Live, Widget)]
pub struct UpDropDown {
    #[animator]
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
    popup_menu: Option<LivePtr>,

    #[live]
    labels: Vec<String>,
    #[live]
    values: Vec<LiveValue>,

    #[live]
    popup_menu_position: PopupMenuPosition,

    #[rust]
    is_active: bool,

    #[live]
    selected_item: usize,

    #[layout]
    layout: Layout,
}

#[derive(Default, Clone)]
struct PopupMenuGlobal {
    map: Rc<RefCell<ComponentMap<LivePtr, PopupMenu>>>,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
struct UpDrawLabelText {
    #[deref]
    draw_super: DrawText,
    #[live]
    focus: f32,
    #[live]
    hover: f32,
}

impl LiveHook for UpDropDown {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if self.popup_menu.is_none() || !apply.from.is_from_doc() {
            return;
        }
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut map = global.map.borrow_mut();

        // when live styling clean up old style references
        map.retain(|k, _| cx.live_registry.borrow().generation_valid(*k));

        let list_box = self.popup_menu.unwrap();
        map.get_or_insert(cx, list_box, |cx| {
            PopupMenu::new_from_ptr(cx, Some(list_box))
        });
    }
}
#[derive(Clone, Debug, DefaultNone)]
pub enum UpDropDownAction {
    Select(usize, LiveValue),
    None,
}

impl UpDropDown {
    pub fn set_active(&mut self, cx: &mut Cx) {
        self.is_active = true;
        self.draw_bg.apply_over(cx, live! {active: 1.0});
        self.draw_bg.redraw(cx);
        let global = cx.global::<PopupMenuGlobal>().clone();
        let mut map = global.map.borrow_mut();
        let lb = map.get_mut(&self.popup_menu.unwrap()).unwrap();
        let node_id = LiveId(self.selected_item as u64).into();
        lb.init_select_item(node_id);
        cx.sweep_lock(self.draw_bg.area());
    }

    pub fn set_closed(&mut self, cx: &mut Cx) {
        self.is_active = false;
        self.draw_bg.apply_over(cx, live! {active: 0.0});
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
        //cx.clear_sweep_lock(self.draw_bg.area());
        // ok so what if. what do we have
        // we have actions
        // and we have applying states/values in response

        self.draw_bg.begin(cx, walk, self.layout);
        //let start_pos = cx.turtle().rect().pos;

        if let Some(val) = self.labels.get(self.selected_item) {
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), val);
        } else {
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), " ");
        }
        self.draw_bg.end(cx);

        cx.add_nav_stop(self.draw_bg.area(), NavRole::DropDown, Margin::default());

        if self.is_active && self.popup_menu.is_some() {
            //cx.set_sweep_lock(self.draw_bg.area());
            // ok so if self was not active, we need to
            // ok so how will we solve this one
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let popup_menu = map.get_mut(&self.popup_menu.unwrap()).unwrap();

            // we kinda need to draw it twice.
            popup_menu.begin(cx);

            match self.popup_menu_position {
                PopupMenuPosition::OnSelected => {
                    let mut item_pos = None;
                    for (i, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(i as u64).into();
                        if i == self.selected_item {
                            item_pos = Some(cx.turtle().pos());
                        }
                        popup_menu.draw_item(cx, node_id, &item);
                    }

                    // ok we shift the entire menu. however we shouldnt go outside the screen area
                    popup_menu.end(
                        cx,
                        self.draw_bg.area(),
                        -item_pos.unwrap_or(dvec2(0.0, 0.0)),
                    );
                }
                PopupMenuPosition::AboveInput => {
                    for (i, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(i as u64).into();
                        popup_menu.draw_item(cx, node_id, &item);
                    }

                    let menu_height = cx.turtle().used_height();
                    let area = self.draw_bg.area().rect(cx);
                    let mut shift_y = -menu_height;
                    if area.pos.y + shift_y < 0.0 {
                        shift_y = -area.pos.y * 0.8;
                    }
                    let shift = DVec2 { x: 0.0, y: shift_y };

                    popup_menu.end(cx, self.draw_bg.area(), shift);
                }
                PopupMenuPosition::BelowInput => {
                    for (i, item) in self.labels.iter().enumerate() {
                        let node_id = LiveId(i as u64).into();
                        popup_menu.draw_item(cx, node_id, &item);
                    }

                    let area = self.draw_bg.area().rect(cx);
                    let shift = DVec2 {
                        x: 0.0,
                        y: area.size.y,
                    };

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

    fn widget_to_data(
        &self,
        _cx: &mut Cx,
        actions: &Actions,
        nodes: &mut LiveNodeVec,
        path: &[LiveId],
    ) -> bool {
        match actions.find_widget_action_cast(self.widget_uid()) {
            UpDropDownAction::Select(_, value) => {
                nodes.write_field_value(path, value.clone());
                true
            }
            _ => false,
        }
    }

    fn data_to_widget(&mut self, cx: &mut Cx, nodes: &[LiveNode], path: &[LiveId]) {
        if let Some(value) = nodes.read_field_value(path) {
            if let Some(index) = self.values.iter().position(|v| v == value) {
                if self.selected_item != index {
                    self.selected_item = index;
                    self.redraw(cx);
                }
            } else {
                error!("Value not in values list {:?}", value);
            }
        }
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        let uid = self.widget_uid();

        if self.is_active && self.popup_menu.is_some() {
            // ok so how will we solve this one
            let global = cx.global::<PopupMenuGlobal>().clone();
            let mut map = global.map.borrow_mut();
            let menu = map.get_mut(&self.popup_menu.unwrap()).unwrap();
            let mut close = false;
            menu.handle_event_with(cx, event, self.draw_bg.area(), &mut |cx, action| {
                match action {
                    PopupMenuAction::WasSweeped(_node_id) => {
                        //dispatch_action(cx, PopupMenuAction::WasSweeped(node_id));
                    }
                    PopupMenuAction::WasSelected(node_id) => {
                        //dispatch_action(cx, PopupMenuAction::WasSelected(node_id));
                        self.selected_item = node_id.0 .0 as usize;
                        cx.widget_action(
                            uid,
                            &scope.path,
                            UpDropDownAction::Select(
                                self.selected_item,
                                self.values
                                    .get(self.selected_item)
                                    .cloned()
                                    .unwrap_or(LiveValue::None),
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

            // check if we clicked outside of the popup menu
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
                            &scope.path,
                            UpDropDownAction::Select(
                                self.selected_item,
                                self.values
                                    .get(self.selected_item)
                                    .cloned()
                                    .unwrap_or(LiveValue::None),
                            ),
                        );
                        self.set_closed(cx);
                        self.draw_bg.redraw(cx);
                    }
                }
                KeyCode::ArrowDown => {
                    if self.values.len() > 0 && self.selected_item < self.values.len() - 1 {
                        self.selected_item += 1;
                        cx.widget_action(
                            uid,
                            &scope.path,
                            UpDropDownAction::Select(
                                self.selected_item,
                                self.values
                                    .get(self.selected_item)
                                    .cloned()
                                    .unwrap_or(LiveValue::None),
                            ),
                        );
                        self.set_closed(cx);
                        self.draw_bg.redraw(cx);
                    }
                }
                _ => (),
            },
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                if self
                    .animator
                    .animator_in_state(cx, &[id!(disabled), id!(off)])
                {
                    cx.set_key_focus(self.draw_bg.area());
                    self.animator_play(cx, &[id!(hover), id!(on)]);
                    self.set_active(cx);
                }
                // self.animator_play(cx, id!(hover.down));
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
                if s.len() == 0 {
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

    //DEPRICATED
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
