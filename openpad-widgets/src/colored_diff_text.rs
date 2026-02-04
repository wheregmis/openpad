use makepad_widgets::*;

// Diff line colors - matching theme where possible, with custom colors for better readability
const DIFF_COLOR_ADD: Vec4 = vec4(0.301, 0.792, 0.301, 1.0); // #4dca4d - bright green
const DIFF_COLOR_DEL: Vec4 = vec4(0.878, 0.376, 0.376, 1.0); // #e06060 - soft red
const DIFF_COLOR_CONTEXT: Vec4 = vec4(0.733, 0.757, 0.788, 1.0); // #bbc1c9 - lighter gray for readability
const DIFF_COLOR_HEADER: Vec4 = vec4(0.533, 0.690, 0.859, 1.0); // #88b0db - soft blue

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::*;

    // ColoredDiffText widget: Renders diff text with per-line coloring
    // - Green for additions (+)
    // - Red for deletions (-)
    // - Gray for context lines
    // - Blue for headers
    pub ColoredDiffText = {{ColoredDiffText}} {
        width: Fill, height: Fit

        draw_text: {
            // Slightly tighter line spacing for compact diff display
            text_style: <THEME_FONT_CODE> { font_size: 10, line_spacing: 1.2 }

            fn get_color(self) -> vec4 {
                return self.color;
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ColoredDiffText {
    #[deref]
    view: View,

    #[live]
    draw_text: DrawText,

    /// Parsed diff lines with their associated types
    #[rust]
    lines: Vec<DiffLine>,
}

/// A single line in the diff with its type
#[derive(Clone)]
struct DiffLine {
    text: String,
    line_type: DiffLineType,
}

/// Type of diff line based on its prefix character
#[derive(Clone, Copy, PartialEq)]
enum DiffLineType {
    Addition, // Lines starting with '+'
    Deletion, // Lines starting with '-'
    Context,  // Normal lines (space prefix or no special prefix)
    Header,   // Separator lines (──) or ellipsis (...)
}

impl Widget for ColoredDiffText {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    /// Draws the diff text with each line colored according to its type
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(
            walk,
            Layout {
                flow: Flow::Down,
                ..Layout::default()
            },
        );

        // Render each line with its appropriate color (using constants defined above)
        for line in &self.lines {
            let text_color = match line.line_type {
                DiffLineType::Addition => DIFF_COLOR_ADD,
                DiffLineType::Deletion => DIFF_COLOR_DEL,
                DiffLineType::Context => DIFF_COLOR_CONTEXT,
                DiffLineType::Header => DIFF_COLOR_HEADER,
            };

            self.draw_text.color = text_color;
            self.draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), &line.text);
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

impl ColoredDiffText {
    /// Parse the diff text and categorize each line by type
    pub fn set_diff_text(&mut self, cx: &mut Cx, text: &str) {
        self.lines.clear();

        for line in text.lines() {
            // Determine line type based on prefix
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

            let display_text = if line.starts_with(' ') {
                format!("·{}", &line[1..])
            } else {
                line.to_string()
            };

            self.lines.push(DiffLine {
                text: display_text,
                line_type,
            });
        }

        // Trigger redraw to display the new content
        self.view.redraw(cx);
    }
}

/// API trait for controlling ColoredDiffText widget
pub trait ColoredDiffTextApi {
    /// Set the diff text content and trigger a redraw
    fn set_diff_text(&self, cx: &mut Cx, text: &str);
}

impl ColoredDiffTextApi for ColoredDiffTextRef {
    fn set_diff_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_diff_text(cx, text);
        }
    }
}
