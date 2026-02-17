use makepad_code_editor::code_editor::CodeEditorAction;
use makepad_code_editor::decoration::DecorationSet;
use makepad_code_editor::{CodeDocument, CodeEditor, CodeSession};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.EditorPanelBase = #(EditorPanel::register_widget(vm))
    mod.widgets.EditorPanel = mod.widgets.EditorPanelBase {
        width: Fill
        height: Fill
        editor := CodeEditor {
            width: Fill
            height: Fill
            show_gutter: true
            word_wrap: false
            draw_bg +: {
                color: #12161d
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum EditorPanelAction {
    TextDidChange,
    #[default]
    None,
}

#[derive(Script, ScriptHook, WidgetRef, WidgetSet, WidgetRegister)]
pub struct EditorPanel {
    #[uid]
    uid: WidgetUid,

    #[live]
    pub editor: CodeEditor,

    #[rust]
    session: Option<CodeSession>,

    #[rust]
    last_text: String,

    #[rust]
    read_only: bool,
}

impl EditorPanel {
    fn ensure_session(&mut self) {
        if self.session.is_none() {
            let doc = CodeDocument::new(self.last_text.as_str().into(), DecorationSet::new());
            self.session = Some(CodeSession::new(doc));
        }
    }

    fn set_text_inner(&mut self, cx: &mut Cx, text: &str) {
        self.last_text = text.to_string();
        let doc = CodeDocument::new(text.into(), DecorationSet::new());
        self.session = Some(CodeSession::new(doc));
        self.editor.redraw(cx);
    }

    fn get_text_inner(&self) -> String {
        self.session
            .as_ref()
            .map(|s| s.document().as_text().to_string())
            .unwrap_or_default()
    }

    fn set_read_only_inner(&mut self, _cx: &mut Cx, read_only: bool) {
        self.read_only = read_only;
    }

    fn focus_editor_inner(&self, cx: &mut Cx) {
        cx.set_key_focus(self.editor.area());
    }
}

impl WidgetNode for EditorPanel {
    fn widget_uid(&self) -> WidgetUid {
        self.uid
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.editor.walk(cx)
    }

    fn area(&self) -> Area {
        self.editor.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.editor.redraw(cx)
    }

    fn find_widgets_from_point(&self, cx: &Cx, point: DVec2, found: &mut dyn FnMut(&WidgetRef)) {
        self.editor.find_widgets_from_point(cx, point, found)
    }

    fn visible(&self) -> bool {
        self.editor.visible()
    }

    fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.editor.set_visible(cx, visible)
    }
}

impl Widget for EditorPanel {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.ensure_session();
        if let Some(session) = self.session.as_mut() {
            self.editor.draw_walk_editor(cx, session, walk);
        } else {
            self.editor.draw_empty_editor(cx, walk);
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.ensure_session();
        let uid = self.widget_uid();
        let Some(session) = self.session.as_mut() else {
            return;
        };

        for action in self.editor.handle_event(cx, event, &mut Scope::empty(), session) {
            match action {
                CodeEditorAction::TextDidChange => {
                    if !self.read_only {
                        cx.widget_action(uid, EditorPanelAction::TextDidChange);
                    }
                }
                CodeEditorAction::UnhandledKeyDown(_) | CodeEditorAction::None => {}
            }
        }
        session.handle_changes();
    }
}

impl EditorPanelRef {
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text_inner(cx, text);
        }
    }

    pub fn get_text(&self) -> String {
        if let Some(inner) = self.borrow() {
            return inner.get_text_inner();
        }
        String::new()
    }

    pub fn set_read_only(&self, cx: &mut Cx, read_only: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_read_only_inner(cx, read_only);
        }
    }

    pub fn focus_editor(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow() {
            inner.focus_editor_inner(cx);
        }
    }
}
