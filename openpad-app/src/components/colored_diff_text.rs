use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::theme::*;

    ColoredDiffText = {{ColoredDiffText}} {
        width: Fill, height: Fit
        
        draw_text: {
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
    
    #[rust]
    lines: Vec<DiffLine>,
}

#[derive(Clone)]
struct DiffLine {
    text: String,
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
        cx.begin_turtle(walk, Layout::default());
        
        // Define colors matching the theme
        let text_color_add = vec4(0.301, 0.792, 0.301, 1.0); // #4dca4d - bright green
        let text_color_del = vec4(0.878, 0.376, 0.376, 1.0); // #e06060 - soft red
        let text_color_context = vec4(0.733, 0.757, 0.788, 1.0); // #bbc1c9 - lighter gray for better readability
        let text_color_header = vec4(0.533, 0.690, 0.859, 1.0); // #88b0db - soft blue for headers
        
        for line in &self.lines {
            let text_color = match line.line_type {
                DiffLineType::Addition => text_color_add,
                DiffLineType::Deletion => text_color_del,
                DiffLineType::Context => text_color_context,
                DiffLineType::Header => text_color_header,
            };
            
            self.draw_text.color = text_color;
            let line_text = format!("{}\n", line.text);
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), &line_text);
        }
        
        cx.end_turtle();
        DrawStep::done()
    }
}

impl ColoredDiffText {
    pub fn set_diff_text(&mut self, cx: &mut Cx, text: &str) {
        self.lines.clear();
        
        for line in text.lines() {
            let line_type = if line.starts_with('+') {
                DiffLineType::Addition
            } else if line.starts_with('-') {
                DiffLineType::Deletion
            } else if line.starts_with("──") || line.starts_with("...") {
                DiffLineType::Header
            } else {
                DiffLineType::Context
            };
            
            self.lines.push(DiffLine {
                text: line.to_string(),
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

// Extension trait for accessing ColoredDiffText from View
pub trait ColoredDiffTextWidgetRefExt {
    fn colored_diff_text(&self, path: &[LiveId]) -> ColoredDiffTextRef;
}

impl ColoredDiffTextWidgetRefExt for WidgetRef {
    fn colored_diff_text(&self, path: &[LiveId]) -> ColoredDiffTextRef {
        self.widget(path)
    }
}
