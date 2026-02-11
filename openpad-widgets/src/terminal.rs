use makepad_widgets::text_input::TextInputAction;
use makepad_widgets::*;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::openpad::*;
    use crate::theme::*;

    pub Terminal = {{Terminal}} {
        width: Fill, height: Fill
        terminal_output = <View> {
            width: Fill, height: Fill
            flow: Down
            spacing: 0
            show_bg: true
            draw_bg: {
                color: (THEME_COLOR_BG_DARKER)
            }

            output_list = <PortalList> {
                auto_tail: true
                OutputLine = <View> {
                    width: Fill, height: Fit
                    padding: { left: 10, right: 10, top: 0, bottom: 0 }

                    line_label = <Label> {
                        width: Fill, height: Fit
                        draw_text: {
                            color: (THEME_COLOR_SHADE_11)
                            text_style: <THEME_FONT_CODE> { font_size: 10 }
                        }
                    }
                }

                InputLine = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    padding: { left: 10, right: 10, top: 0, bottom: 0 }
                    spacing: 0
                    align: { y: 0.5 }

                    prompt_label = <Label> {
                        width: Fit, height: Fit
                        text: " % "
                        draw_text: {
                            color: (THEME_COLOR_SHADE_11)
                            text_style: <THEME_FONT_CODE> { font_size: 10 }
                        }
                    }

                    input_field = <TextInput> {
                        width: Fill, height: Fit
                        padding: { left: 0, right: 10, top: 4, bottom: 4 }
                        empty_text: ""
                        draw_bg: {
                            color: (THEME_COLOR_TRANSPARENT)
                            color_focus: (THEME_COLOR_TRANSPARENT)
                            color_empty: (THEME_COLOR_TRANSPARENT)
                            border_radius: 0.0
                            border_size: 0.0
                        }
                        draw_text: {
                            color: (THEME_COLOR_TEXT_BRIGHT)
                            text_style: <THEME_FONT_CODE> { font_size: 10 }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TerminalSpan {
    pub text: String,
    pub color: Vec4,
}

#[derive(Clone, Debug)]
pub struct TerminalLine {
    pub spans: Vec<TerminalSpan>,
    pub cached_text: String,
}

pub struct TerminalBackend {
    pub output_lines: Vec<TerminalLine>,
    pub partial_spans: Vec<TerminalSpan>,
    pub current_color: Vec4,
    pub prompt_string: String,
    pub pty_writer: Option<Arc<Mutex<Box<dyn Write + Send>>>>,
}

impl Default for TerminalBackend {
    fn default() -> Self {
        Self {
            output_lines: Vec::new(),
            partial_spans: Vec::new(),
            current_color: Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            },
            prompt_string: Self::build_prompt_string(&std::path::PathBuf::from(".")),
            pty_writer: None,
        }
    }
}

impl TerminalBackend {
    pub fn build_prompt_string(cwd: &std::path::Path) -> String {
        let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
        let host = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "localhost".into());
        let dir_name = cwd
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".".into());
        format!("{}@{} {} % ", user, host, dir_name)
    }

    pub fn color_from_ansi(code: u8) -> Vec4 {
        match code {
            0 => Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            },
            30 => Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            31 => Vec4 {
                x: 0.9,
                y: 0.3,
                z: 0.3,
                w: 1.0,
            },
            32 => Vec4 {
                x: 0.3,
                y: 0.8,
                z: 0.3,
                w: 1.0,
            },
            33 => Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.2,
                w: 1.0,
            },
            34 => Vec4 {
                x: 0.3,
                y: 0.3,
                z: 0.9,
                w: 1.0,
            },
            35 => Vec4 {
                x: 0.8,
                y: 0.3,
                z: 0.8,
                w: 1.0,
            },
            36 => Vec4 {
                x: 0.3,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            },
            37 => Vec4 {
                x: 0.9,
                y: 0.9,
                z: 0.9,
                w: 1.0,
            },
            90 => Vec4 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
                w: 1.0,
            },
            91 => Vec4 {
                x: 1.0,
                y: 0.4,
                z: 0.4,
                w: 1.0,
            },
            92 => Vec4 {
                x: 0.4,
                y: 1.0,
                z: 0.4,
                w: 1.0,
            },
            93 => Vec4 {
                x: 1.0,
                y: 1.0,
                z: 0.4,
                w: 1.0,
            },
            94 => Vec4 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
                w: 1.0,
            },
            95 => Vec4 {
                x: 1.0,
                y: 0.5,
                z: 1.0,
                w: 1.0,
            },
            96 => Vec4 {
                x: 0.5,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            97 => Vec4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            _ => Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            },
        }
    }

    pub fn normalize_for_prompt_check(line: &str) -> String {
        let mut result = String::with_capacity(line.len());
        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                if chars.peek() == Some(&'[') {
                    chars.next();
                    while let Some(&next) = chars.peek() {
                        chars.next();
                        if next.is_ascii_alphabetic() || next == '~' {
                            break;
                        }
                    }
                }
            } else if !ch.is_control() || ch == '\n' {
                result.push(ch);
            }
        }
        result.trim().to_string()
    }

    pub fn is_prompt_only_line(line: &str, our_prompt: &str) -> bool {
        let t = Self::normalize_for_prompt_check(line);
        if t.is_empty() {
            return true;
        }
        let our_trimmed = our_prompt.trim();
        if t == our_trimmed {
            return true;
        }
        if t.ends_with('%')
            || t.ends_with("% ")
            || t.ends_with('$')
            || t.ends_with("$ ")
            || t.ends_with('#')
            || t.ends_with("# ")
        {
            if (t.contains('@') && t.len() < our_trimmed.len() + 5)
                || t == "%"
                || t == "$"
                || t == "#"
            {
                return true;
            }
        }
        false
    }

    fn push_current_text(&mut self, current_text: &mut String) {
        if !current_text.is_empty() {
            if let Some(last) = self.partial_spans.last_mut() {
                if last.color == self.current_color {
                    last.text.push_str(current_text);
                    current_text.clear();
                    return;
                }
            }
            self.partial_spans.push(TerminalSpan {
                text: current_text.clone(),
                color: self.current_color,
            });
            current_text.clear();
        }
    }

    pub fn append_output(&mut self, text: &str) {
        let mut chars = text.chars().peekable();
        let mut current_text = String::new();
        while let Some(ch) = chars.next() {
            match ch {
                '\x1b' => {
                    self.push_current_text(&mut current_text);
                    if chars.next() == Some('[') {
                        while let Some(&next) = chars.peek() {
                            if next == '?' || next == '>' || next == '=' || next == '!' {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        let mut param = String::new();
                        while let Some(&next) = chars.peek() {
                            if next.is_ascii_digit() || next == ';' {
                                param.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        let command = chars.next();
                        if command == Some('m') {
                            for p in param.split(';') {
                                if let Ok(code) = p.parse::<u8>() {
                                    self.current_color = Self::color_from_ansi(code);
                                }
                            }
                        }
                    }
                }
                '\n' => {
                    self.push_current_text(&mut current_text);
                    let line_spans = std::mem::take(&mut self.partial_spans);
                    let full_text: String = line_spans.iter().map(|s| s.text.as_str()).collect();
                    if !Self::is_prompt_only_line(&full_text, &self.prompt_string) {
                        self.output_lines.push(TerminalLine {
                            spans: line_spans,
                            cached_text: full_text,
                        });
                    }
                }
                '\r' => match chars.peek() {
                    Some(&'\n') | Some(&'\r') | None => {}
                    _ => {
                        self.partial_spans.clear();
                        current_text.clear();
                    }
                },
                '\x08' => {
                    if !current_text.is_empty() {
                        current_text.pop();
                    } else if let Some(last_span) = self.partial_spans.last_mut() {
                        last_span.text.pop();
                        if last_span.text.is_empty() {
                            self.partial_spans.pop();
                        }
                    }
                }
                '\t' => {
                    current_text.push_str("    ");
                }
                ch if ch.is_control() => {}
                _ => {
                    current_text.push(ch);
                }
            }
        }
        self.push_current_text(&mut current_text);
        const MAX_LINES: usize = 2000;
        if self.output_lines.len() > MAX_LINES {
            let remove_count = self.output_lines.len() - MAX_LINES;
            self.output_lines.drain(0..remove_count);
        }
    }

    pub fn send_command(&mut self, command: &str) -> Result<(), std::io::Error> {
        if let Some(writer) = &self.pty_writer {
            let mut w = writer
                .lock()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Lock failed"))?;
            writeln!(w, "{}", command)?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "PTY not initialized",
            ))
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Terminal {
    #[deref]
    view: View,

    #[rust]
    backend: TerminalBackend,
}

impl Widget for Terminal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if let TextInputAction::Returned(input, _modifiers) =
            actions.widget_action(&[live_id!(input_field)]).cast()
        {
            if !input.is_empty() {
                if let Err(e) = self.backend.send_command(&input) {
                    self.backend.append_output(&format!("Error: {}\n", e));
                }
                self.view.text_input(&[id!(input_field)]).set_text(cx, "");
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let total_items = self.backend.output_lines.len() + 1;
                list.set_item_range(cx, 0, total_items);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < self.backend.output_lines.len() {
                        let line = &self.backend.output_lines[item_id];
                        let item_widget = list.item(cx, item_id, live_id!(OutputLine));
                        let full_text = &line.cached_text;
                        let spans = &line.spans;
                        let label = item_widget.label(&[id!(line_label)]);
                        label.set_text(cx, &full_text);
                        if let Some(first) = spans.first() {
                            label.apply_over(cx, live! { draw_text: { color: (first.color) } });
                        }

                        item_widget.draw_all(cx, scope);
                    } else if item_id == self.backend.output_lines.len() {
                        let item_widget = list.item(cx, item_id, live_id!(InputLine));
                        item_widget
                            .label(&[id!(prompt_label)])
                            .set_text(cx, &self.backend.prompt_string);
                        item_widget.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl Terminal {
    pub fn init_pty(&mut self, cx: &mut Cx) {
        if self.backend.pty_writer.is_some() {
            return;
        }
        let pty_system = native_pty_system();
        let pty_size = PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        };
        let pair = match pty_system.openpty(pty_size) {
            Ok(pair) => pair,
            Err(e) => {
                self.backend
                    .append_output(&format!("Failed to create PTY: {}\n", e));
                self.redraw(cx);
                return;
            }
        };

        #[cfg(target_os = "windows")]
        let shell = "powershell.exe";
        #[cfg(not(target_os = "windows"))]
        let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        #[cfg(not(target_os = "windows"))]
        let shell = shell_path.as_str();

        let mut cmd = CommandBuilder::new(shell);
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        cmd.cwd(&cwd);
        self.backend.prompt_string = TerminalBackend::build_prompt_string(&cwd);

        if let Err(e) = pair.slave.spawn_command(cmd) {
            self.backend
                .append_output(&format!("Failed to spawn shell: {}\n", e));
            self.redraw(cx);
            return;
        }

        let writer = match pair.master.take_writer() {
            Ok(w) => Arc::new(Mutex::new(w)),
            Err(e) => {
                self.backend
                    .append_output(&format!("Failed to get PTY writer: {}\n", e));
                self.redraw(cx);
                return;
            }
        };
        self.backend.pty_writer = Some(writer);

        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(e) => {
                self.backend
                    .append_output(&format!("Failed to get PTY reader: {}\n", e));
                self.redraw(cx);
                return;
            }
        };

        std::thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let text = String::from_utf8_lossy(&buffer[0..n]).to_string();
                        Cx::post_action(TerminalAction::OutputReceived(text));
                    }
                    Ok(_) => break,
                    Err(_) => break,
                }
            }
        });

        self.backend
            .append_output(&format!("Terminal initialized with shell: {}\n", shell));
        self.redraw(cx);
    }

    pub fn handle_action(&mut self, cx: &mut Cx, action: &TerminalAction) {
        match action {
            TerminalAction::OutputReceived(text) => {
                self.backend.append_output(text);
                self.redraw(cx);
                Cx::post_action(TerminalAction::ScrollToBottom);
            }
            TerminalAction::ScrollToBottom => {
                let list = self.view.portal_list(&[id!(output_list)]);
                let total_items = self.backend.output_lines.len() + 1;
                if total_items > 0 {
                    list.set_first_id(total_items.saturating_sub(40));
                }
            }
            TerminalAction::None => {}
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum TerminalAction {
    OutputReceived(String),
    ScrollToBottom,
    None,
}

impl TerminalRef {
    pub fn init_pty(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.init_pty(cx);
        }
    }
    pub fn handle_action(&self, cx: &mut Cx, action: &TerminalAction) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.handle_action(cx, action);
        }
    }
}
