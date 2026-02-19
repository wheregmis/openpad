use makepad_widgets::*;

const DIFF_COLOR_ADD: Vec4 = vec4(0.301, 0.792, 0.301, 1.0);
const DIFF_COLOR_DEL: Vec4 = vec4(0.878, 0.376, 0.376, 1.0);
const DIFF_COLOR_CONTEXT: Vec4 = vec4(0.733, 0.757, 0.788, 1.0);
const DIFF_COLOR_HEADER: Vec4 = vec4(0.533, 0.690, 0.859, 1.0);

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.ColoredDiffText = #(ColoredDiffText::register_widget(vm)) {
        width: Fill
        height: Fit

        draw_text +: {
            text_style: theme.font_code {font_size: 10 line_spacing: 1.2}
            get_color: fn() {
                return self.color
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ColoredDiffText {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,

    #[live]
    draw_text: DrawText,

    #[rust]
    lines: Vec<DiffLine>,
    #[rust]
    cached_text: String,
    #[rust]
    normalized_text: String,
}

#[derive(Clone)]
struct DiffLine {
    start: usize,
    end: usize,
    line_type: DiffLineType,
}

#[derive(Clone, Copy, PartialEq)]
enum DiffLineType {
    Addition,
    Deletion,
    Context,
    Header,
}

impl Widget for ColoredDiffText {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(
            walk,
            Layout {
                flow: Flow::Down,
                ..Layout::default()
            },
        );

        for line in &self.lines {
            let text_color = match line.line_type {
                DiffLineType::Addition => DIFF_COLOR_ADD,
                DiffLineType::Deletion => DIFF_COLOR_DEL,
                DiffLineType::Context => DIFF_COLOR_CONTEXT,
                DiffLineType::Header => DIFF_COLOR_HEADER,
            };

            self.draw_text.color = text_color;
            // Optimization: use a string slice from the persistent normalized_text buffer.
            // This avoids creating new String objects per line during the draw loop.
            let text = &self.normalized_text[line.start..line.end];
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), text);
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

impl ColoredDiffText {
    pub fn set_diff_text(&mut self, cx: &mut Cx, text: &str) {
        if self.cached_text == text {
            return;
        }
        // Optimization: avoid repeated allocation by clearing and reusing persistent buffers.
        // We reuse cached_text, normalized_text, and lines Vec to reduce heap churn.
        self.cached_text.clear();
        self.cached_text.push_str(text);

        self.normalized_text.clear();
        self.lines.clear();

        for line in text.lines() {
            let line_type = if line.starts_with('+') {
                DiffLineType::Addition
            } else if line.starts_with('-') {
                DiffLineType::Deletion
            } else if line.starts_with("──") || line.starts_with("...") || line.starts_with("@@")
            {
                DiffLineType::Header
            } else {
                DiffLineType::Context
            };

            let start = self.normalized_text.len();
            if let Some(stripped) = line.strip_prefix(' ') {
                self.normalized_text.push('·');
                self.normalized_text.push_str(stripped);
            } else {
                self.normalized_text.push_str(line);
            };
            let end = self.normalized_text.len();

            self.lines.push(DiffLine {
                start,
                end,
                line_type,
            });
        }

        self.view.redraw(cx);
    }
}

pub trait ColoredDiffTextApi {
    fn set_diff_text(&self, cx: &mut Cx, text: &str);
}

impl ColoredDiffTextApi for ColoredDiffTextRef {
    fn set_diff_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_diff_text(cx, text);
        }
    }
}
