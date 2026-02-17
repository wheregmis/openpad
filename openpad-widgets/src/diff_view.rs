use crate::colored_diff_text::{ColoredDiffTextApi, ColoredDiffTextWidgetExt};
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.DiffView = #(DiffView::register_widget(vm)) {
        width: Fill
        height: Fit
        flow: Down
        visible: false

        summary_header := RoundedView {
            visible: false
            width: Fill
            height: Fit
            new_batch: true
            padding: Inset{left: 12 right: 12 top: 8 bottom: 8}
            show_bg: true
            draw_bg +: {
                color: #1a1f2e
                border_radius: 8.0
            }

            summary_row := View {
                width: Fill
                height: Fit
                flow: Right
                spacing: 8
                align: Align{y: 0.5}

                summary_files_label := Label {
                    width: Fit
                    height: Fit
                    text: ""
                    draw_text +: {
                        color: #aab3bd
                        text_style: theme.font_bold {font_size: 11}
                    }
                }

                summary_add_label := Label {
                    width: Fit
                    height: Fit
                    text: ""
                    draw_text +: {
                        color: #4dca4d
                        text_style: theme.font_bold {font_size: 11}
                    }
                }

                summary_del_label := Label {
                    width: Fit
                    height: Fit
                    text: ""
                    draw_text +: {
                        color: #e06060
                        text_style: theme.font_bold {font_size: 11}
                    }
                }
            }
        }

        diff_content := RoundedView {
            width: Fill
            height: Fit
            visible: false
            new_batch: true
            padding: Inset{left: 12 right: 12 top: 8 bottom: 8}
            margin: Inset{top: 2}
            show_bg: true
            draw_bg +: {
                color: #1a1a1a
                border_radius: 0.0
            }

            ScrollYView {
                width: Fill
                height: Fit

                diff_text := mod.widgets.ColoredDiffText {
                    width: Fill
                    height: Fit
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct DiffView {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[rust]
    expanded: bool,
    #[rust]
    diff_text_content: String,
    #[rust]
    summary_text: String,
    #[rust]
    summary_header_clicked: bool,
}

impl Widget for DiffView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::MouseDown(mouse) = event {
            let header = self.view.view(cx, &[id!(summary_header)]);
            let header_rect = header.area().rect(cx);
            if header_rect.contains(mouse.abs) {
                self.summary_header_clicked = true;
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl DiffView {
    pub fn set_diff_text(
        &mut self,
        cx: &mut Cx,
        files: &str,
        additions: &str,
        deletions: &str,
        full_diff: &str,
    ) {
        if full_diff.is_empty() {
            self.clear_diffs(cx);
            return;
        }

        // Optimization: return early if the diff content hasn't changed.
        // This avoids redundant UI updates and string allocations in the draw loop.
        if self.diff_text_content == full_diff {
            return;
        }

        self.diff_text_content = full_diff.to_string();
        self.view
            .label(cx, &[id!(summary_files_label)])
            .set_text(cx, files);
        self.view
            .label(cx, &[id!(summary_add_label)])
            .set_text(cx, additions);
        self.view
            .label(cx, &[id!(summary_del_label)])
            .set_text(cx, deletions);

        self.view
            .colored_diff_text(cx, &[id!(diff_text)])
            .set_diff_text(cx, full_diff);

        self.expanded = false;
        self.view
            .view(cx, &[id!(diff_content)])
            .set_visible(cx, false);
        self.view
            .view(cx, &[id!(summary_header)])
            .set_visible(cx, true);
        self.view.set_visible(cx, true);
        self.redraw(cx);
    }

    pub fn set_expanded(&mut self, cx: &mut Cx, expanded: bool) {
        self.expanded = expanded;
        self.view
            .view(cx, &[id!(diff_content)])
            .set_visible(cx, expanded);
        if expanded {
            self.view.set_visible(cx, true);
        }
        self.redraw(cx);
    }

    pub fn clear_diffs(&mut self, cx: &mut Cx) {
        self.expanded = false;
        self.diff_text_content.clear();
        self.summary_text.clear();
        self.view
            .label(cx, &[id!(summary_files_label)])
            .set_text(cx, "");
        self.view
            .label(cx, &[id!(summary_add_label)])
            .set_text(cx, "");
        self.view
            .label(cx, &[id!(summary_del_label)])
            .set_text(cx, "");
        self.view
            .view(cx, &[id!(summary_header)])
            .set_visible(cx, false);
        self.view.set_visible(cx, false);
        self.redraw(cx);
    }

    pub fn summary_header_clicked(&mut self) -> bool {
        let clicked = self.summary_header_clicked;
        self.summary_header_clicked = false;
        clicked
    }
}

pub trait DiffViewApi {
    fn set_diff_text(
        &self,
        cx: &mut Cx,
        files: &str,
        additions: &str,
        deletions: &str,
        full_diff: &str,
    );
    fn clear_diffs(&self, cx: &mut Cx);
    fn set_expanded(&self, cx: &mut Cx, expanded: bool);
    fn summary_header_clicked(&self, cx: &mut Cx) -> bool;
}

impl DiffViewApi for DiffViewRef {
    fn set_diff_text(
        &self,
        cx: &mut Cx,
        files: &str,
        additions: &str,
        deletions: &str,
        full_diff: &str,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_diff_text(cx, files, additions, deletions, full_diff);
        }
    }

    fn clear_diffs(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_diffs(cx);
        }
    }

    fn set_expanded(&self, cx: &mut Cx, expanded: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_expanded(cx, expanded);
        }
    }

    fn summary_header_clicked(&self, _cx: &mut Cx) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.summary_header_clicked()
        } else {
            false
        }
    }
}
