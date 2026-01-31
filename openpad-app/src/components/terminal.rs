//! Terminal widget - A simple terminal emulator widget.
//!
//! This component provides a basic terminal interface using portable-pty.

use makepad_widgets::text_input::TextInputAction;
use makepad_widgets::*;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub Terminal = {{Terminal}} {
        width: Fill, height: Fill
        flow: Down
        spacing: 0

        // Tab bar at the top using PortalList
        tab_bar = <View> {
            width: Fill, height: 32
            flow: Right
            spacing: 0
            show_bg: true
            draw_bg: {
                color: #1a1a1a
            }

            tabs_list = <PortalList> {
                width: Fit, height: Fill
                flow: Right

                TerminalTab = <Button> {
                    width: Fit, height: 32
                    padding: { left: 12, right: 28, top: 8, bottom: 8 }
                    text: "user — zsh"
                    draw_bg: {
                        instance selected: 0.0
                        color: #1a1a1a
                        color_hover: #2a2a2a

                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let selected_color = #252526;
                            let base_color = mix(self.color, self.color_hover, self.hover);
                            let final_color = mix(base_color, selected_color, self.selected);
                            sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                            sdf.fill(final_color);
                            
                            // Bottom border for selected tab
                            if self.selected > 0.5 {
                                sdf.move_to(0.0, self.rect_size.y - 2.0);
                                sdf.line_to(self.rect_size.x, self.rect_size.y - 2.0);
                                sdf.stroke(#4a90e2, 2.0);
                            }
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                        color: #888
                    }
                }
            }

            <View> { width: Fill }

            new_tab_button = <Button> {
                width: 32, height: 32
                margin: { right: 4 }
                padding: 0
                text: "+"
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 16 }
                    color: #888
                    color_hover: #fff
                }
                draw_bg: {
                    color: #0000
                    color_hover: #2a2a2a
                    border_radius: 0.0
                }
            }
        }

        terminal_output = <View> {
            width: Fill, height: Fill
            flow: Down
            spacing: 0
            show_bg: true
            draw_bg: {
                color: #1a1a1a
            }

            output_list = <PortalList> {
                auto_tail: true
                OutputLine = <View> {
                    width: Fill, height: Fit
                    padding: { left: 10, right: 10, top: 0, bottom: 0 }

                    line_label = <Label> {
                        width: Fill, height: Fit
                        draw_text: {
                            color: #cccccc
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
                            color: #cccccc
                            text_style: <THEME_FONT_CODE> { font_size: 10 }
                        }
                    }

                    input_field = <TextInput> {
                        width: Fill, height: Fit
                        padding: { left: 0, right: 10, top: 4, bottom: 4 }
                        empty_text: ""
                        draw_bg: {
                            color: #0000
                            color_focus: #0000
                            color_empty: #0000
                            border_radius: 0.0
                            border_size: 0.0
                        }
                        draw_text: {
                            color: #ffffff
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

/// Individual terminal instance data
struct TerminalInstance {
    id: usize,
    shell_name: String,
    output_lines: Vec<Vec<TerminalSpan>>,
    partial_spans: Vec<TerminalSpan>,
    current_color: Vec4,
    prompt_string: String,
    pty_writer: Option<Arc<Mutex<Box<dyn Write + Send>>>>,
}

impl TerminalInstance {
    fn new(id: usize) -> Self {
        Self {
            id,
            shell_name: Self::get_tab_label(),
            output_lines: Vec::new(),
            partial_spans: Vec::new(),
            current_color: Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            },
            prompt_string: Terminal::build_prompt_string(&std::path::PathBuf::from(".")),
            pty_writer: None,
        }
    }

    fn get_shell_name() -> String {
        #[cfg(target_os = "windows")]
        {
            "powershell".to_string()
        }
        #[cfg(not(target_os = "windows"))]
        {
            std::env::var("SHELL")
                .ok()
                .and_then(|s| std::path::Path::new(&s).file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| "sh".to_string())
        }
    }

    fn get_tab_label() -> String {
        let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
        let shell = Self::get_shell_name();
        format!("{} — {}", user, shell)
    }
}

#[derive(Live, Widget)]
pub struct Terminal {
    #[deref]
    view: View,

    #[rust]
    terminals: Vec<TerminalInstance>,

    #[rust]
    active_terminal_index: usize,

    #[rust]
    next_terminal_id: usize,
}

impl LiveHook for Terminal {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.terminals = vec![TerminalInstance::new(0)];
        self.active_terminal_index = 0;
        self.next_terminal_id = 1;
    }
}

impl Widget for Terminal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle new tab button
        if self.view.button(id!(new_tab_button)).clicked(&actions) {
            self.create_new_terminal(cx);
            self.redraw(cx);
        }

        // Handle tab clicks using items_with_actions
        let tabs_list = self.view.portal_list(id!(tabs_list));
        for (item_id, widget) in tabs_list.items_with_actions(&actions) {
            if item_id < self.terminals.len() {
                // TerminalTab is a Button, so we can treat the widget as a button directly
                // We need to convert WidgetRef to ButtonRef
                let button_ref = widget.as_button();
                if button_ref.clicked(&actions) {
                    if self.active_terminal_index != item_id {
                        self.active_terminal_index = item_id;
                        self.redraw(cx);
                    }
                }
            }
        }

        // Handle input field for active terminal
        if let TextInputAction::Returned(input, _modifiers) =
            actions.widget_action(&[live_id!(input_field)]).cast()
        {
            if !input.is_empty() {
                self.send_command(cx, &input);
                self.view.text_input(id!(input_field)).set_text(cx, "");
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let mut is_first_list = true;
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // First PortalList is tabs, second is output
                if is_first_list {
                    is_first_list = false;
                    // Draw tabs
                    list.set_item_range(cx, 0, self.terminals.len());
                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < self.terminals.len() {
                            let terminal = &self.terminals[item_id];
                            let is_selected = item_id == self.active_terminal_index;
                            
                            let tab_widget = list.item(cx, item_id, live_id!(TerminalTab));
                            tab_widget.set_text(cx, &terminal.shell_name);
                            tab_widget.apply_over(cx, live! {
                                draw_bg: { selected: (if is_selected { 1.0 } else { 0.0 }) }
                            });
                            
                            tab_widget.draw_all(cx, scope);
                        }
                    }
                } else if let Some(active_terminal) = self.terminals.get(self.active_terminal_index) {
                    // Draw output
                    let total_items = active_terminal.output_lines.len() + 1;
                    list.set_item_range(cx, 0, total_items);

                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < active_terminal.output_lines.len() {
                            let spans = &active_terminal.output_lines[item_id];
                            let item_widget = list.item(cx, item_id, live_id!(OutputLine));

                            let full_text: String = spans.iter().map(|s| s.text.as_str()).collect();
                            let label = item_widget.label(id!(line_label));
                            label.set_text(cx, &full_text);
                            if let Some(first) = spans.first() {
                                label.apply_over(
                                    cx,
                                    live! {
                                        draw_text: { color: (first.color) }
                                    },
                                );
                            }

                            item_widget.draw_all(cx, scope);
                        } else if item_id == active_terminal.output_lines.len() {
                            let item_widget = list.item(cx, item_id, live_id!(InputLine));
                            item_widget
                                .label(id!(prompt_label))
                                .set_text(cx, &active_terminal.prompt_string);
                            item_widget.draw_all(cx, scope);
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl Terminal {
    /// Build full prompt string: "• user@host dirname % "
    fn build_prompt_string(cwd: &std::path::Path) -> String {
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

    fn color_from_ansi(code: u8) -> Vec4 {
        match code {
            0 => Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            }, // Reset/Gray
            30 => Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }, // Black
            31 => Vec4 {
                x: 0.9,
                y: 0.3,
                z: 0.3,
                w: 1.0,
            }, // Red
            32 => Vec4 {
                x: 0.3,
                y: 0.8,
                z: 0.3,
                w: 1.0,
            }, // Green
            33 => Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.2,
                w: 1.0,
            }, // Yellow
            34 => Vec4 {
                x: 0.3,
                y: 0.3,
                z: 0.9,
                w: 1.0,
            }, // Blue
            35 => Vec4 {
                x: 0.8,
                y: 0.3,
                z: 0.8,
                w: 1.0,
            }, // Magenta
            36 => Vec4 {
                x: 0.3,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            }, // Cyan
            37 => Vec4 {
                x: 0.9,
                y: 0.9,
                z: 0.9,
                w: 1.0,
            }, // White
            90 => Vec4 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
                w: 1.0,
            }, // Bright Black
            91 => Vec4 {
                x: 1.0,
                y: 0.4,
                z: 0.4,
                w: 1.0,
            }, // Bright Red
            92 => Vec4 {
                x: 0.4,
                y: 1.0,
                z: 0.4,
                w: 1.0,
            }, // Bright Green
            93 => Vec4 {
                x: 1.0,
                y: 1.0,
                z: 0.4,
                w: 1.0,
            }, // Bright Yellow
            94 => Vec4 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
                w: 1.0,
            }, // Bright Blue
            95 => Vec4 {
                x: 1.0,
                y: 0.5,
                z: 1.0,
                w: 1.0,
            }, // Bright Magenta
            96 => Vec4 {
                x: 0.5,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            }, // Bright Cyan
            97 => Vec4 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            }, // Bright White
            _ => Vec4 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
                w: 1.0,
            },
        }
    }

    pub fn init_pty(&mut self, cx: &mut Cx) {
        // Initialize PTY for the first terminal
        if !self.terminals.is_empty() {
            self.init_pty_for_terminal(cx, 0);
        }
    }

    fn init_pty_for_terminal(&mut self, cx: &mut Cx, terminal_index: usize) {
        let Some(terminal) = self.terminals.get_mut(terminal_index) else {
            return;
        };

        if terminal.pty_writer.is_some() {
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
                self.append_output_to_terminal(cx, terminal_index, &format!("Failed to create PTY: {}\n", e));
                return;
            }
        };

        // Spawn shell
        #[cfg(target_os = "windows")]
        let shell = "powershell.exe";
        #[cfg(not(target_os = "windows"))]
        let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

        #[cfg(not(target_os = "windows"))]
        let shell = shell_path.as_str();

        let mut cmd = CommandBuilder::new(shell);
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        cmd.cwd(&cwd);
        
        if let Some(terminal) = self.terminals.get_mut(terminal_index) {
            terminal.prompt_string = Self::build_prompt_string(&cwd);
        }

        if let Err(e) = pair.slave.spawn_command(cmd) {
            self.append_output_to_terminal(cx, terminal_index, &format!("Failed to spawn shell: {}\n", e));
            return;
        }

        // Store writer
        let writer = match pair.master.take_writer() {
            Ok(w) => Arc::new(Mutex::new(w)),
            Err(e) => {
                self.append_output_to_terminal(cx, terminal_index, &format!("Failed to get PTY writer: {}\n", e));
                return;
            }
        };
        
        if let Some(terminal) = self.terminals.get_mut(terminal_index) {
            terminal.pty_writer = Some(writer);
        }

        // Start reading output in background
        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(e) => {
                self.append_output_to_terminal(cx, terminal_index, &format!("Failed to get PTY reader: {}\n", e));
                return;
            }
        };

        let terminal_id = self.terminals[terminal_index].id;
        std::thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let text = String::from_utf8_lossy(&buffer[0..n]).to_string();
                        Cx::post_action(TerminalAction::OutputReceived { terminal_id, text });
                    }
                    Ok(_) => break, // EOF
                    Err(_) => break,
                }
            }
        });

        self.append_output_to_terminal(cx, terminal_index, &format!("Terminal initialized with shell: {}\n", shell));
    }

    fn create_new_terminal(&mut self, cx: &mut Cx) {
        let new_id = self.next_terminal_id;
        self.next_terminal_id += 1;
        
        self.terminals.push(TerminalInstance::new(new_id));
        self.active_terminal_index = self.terminals.len() - 1;
        
        // Initialize PTY for new terminal
        self.init_pty_for_terminal(cx, self.active_terminal_index);
    }

    fn close_terminal(&mut self, _cx: &mut Cx, terminal_index: usize) {
        // Don't allow closing the last terminal
        if self.terminals.len() <= 1 {
            return;
        }

        self.terminals.remove(terminal_index);
        
        // Adjust active index if needed
        if self.active_terminal_index >= self.terminals.len() {
            self.active_terminal_index = self.terminals.len() - 1;
        } else if self.active_terminal_index > terminal_index {
            self.active_terminal_index -= 1;
        }
    }

    fn send_command(&mut self, cx: &mut Cx, command: &str) {
        let Some(terminal) = self.terminals.get(self.active_terminal_index) else {
            return;
        };

        if let Some(writer) = &terminal.pty_writer {
            let result = {
                let mut w = match writer.lock() {
                    Ok(w) => w,
                    Err(_) => return,
                };
                writeln!(w, "{}", command)
            };

            if let Err(e) = result {
                self.append_output_to_terminal(cx, self.active_terminal_index, &format!("Failed to send command: {}\n", e));
            }
        }
    }

    /// Normalize line for prompt check: aggressively strip ANSI and control chars.
    fn normalize_for_prompt_check(line: &str) -> String {
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

    /// True if the line is only a shell prompt or looks like one.
    fn is_prompt_only_line(line: &str, our_prompt: &str) -> bool {
        let t = Self::normalize_for_prompt_check(line);
        if t.is_empty() {
            return true;
        }

        // Exact or trimmed match to our expected prompt
        let our_trimmed = our_prompt.trim();
        if t == our_trimmed {
            return true;
        }

        // Only filter lines that end with a prompt character and nothing after it
        if t.ends_with('%')
            || t.ends_with("% ")
            || t.ends_with('$')
            || t.ends_with("$ ")
            || t.ends_with('#')
            || t.ends_with("# ")
        {
            // Check if it looks like just a prompt (user@host dir %)
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

    fn append_output_to_terminal(&mut self, cx: &mut Cx, terminal_index: usize, text: &str) {
        let Some(terminal) = self.terminals.get_mut(terminal_index) else {
            return;
        };

        let mut chars = text.chars().peekable();
        let mut current_text = String::new();

        while let Some(ch) = chars.next() {
            match ch {
                '\x1b' => {
                    // Flush current text if any
                    if !current_text.is_empty() {
                        terminal.partial_spans.push(TerminalSpan {
                            text: current_text.clone(),
                            color: terminal.current_color,
                        });
                        current_text.clear();
                    }

                    if chars.next() == Some('[') {
                        // Skip private mode prefix characters (?, >, =, etc.)
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
                            // SGR (Select Graphic Rendition)
                            for p in param.split(';') {
                                if let Ok(code) = p.parse::<u8>() {
                                    terminal.current_color = Self::color_from_ansi(code);
                                }
                            }
                        } else if command == Some('K') {
                            // Erase in Line - ignore for now
                        } else if command == Some('H') || command == Some('f') {
                            // Cursor Position - ignore for now
                        } else if command == Some('J') {
                            // Erase in Display - ignore for now
                        }
                        // Skip other unknown CSI sequences
                    }
                }
                '\n' => {
                    if !current_text.is_empty() {
                        terminal.partial_spans.push(TerminalSpan {
                            text: current_text.clone(),
                            color: terminal.current_color,
                        });
                        current_text.clear();
                    }

                    let line_spans = std::mem::take(&mut terminal.partial_spans);
                    // Simple prompt check on the concatenated text
                    let full_text: String = line_spans.iter().map(|s| s.text.as_str()).collect();
                    if !Self::is_prompt_only_line(&full_text, &terminal.prompt_string) {
                        terminal.output_lines.push(line_spans);
                    }
                }
                '\r' => {
                    // \r\n or \r\r\n = line ending, preserve content and let \n handle it.
                    // \r at end of chunk = preserve (the \n may arrive in the next chunk).
                    // \r followed by visible text = shell is redrawing the line, clear and overwrite.
                    match chars.peek() {
                        Some(&'\n') | Some(&'\r') | None => {
                            // Line ending or end of chunk - preserve content
                        }
                        Some(&'\x1b') => {
                            // ANSI escape after \r typically means prompt redraw
                            terminal.partial_spans.clear();
                            current_text.clear();
                        }
                        _ => {
                            // Standalone \r followed by text: shell is redrawing
                            terminal.partial_spans.clear();
                            current_text.clear();
                        }
                    }
                }
                '\x08' => {
                    // Backspace: remove last character
                    if !current_text.is_empty() {
                        current_text.pop();
                    } else if let Some(last_span) = terminal.partial_spans.last_mut() {
                        last_span.text.pop();
                        if last_span.text.is_empty() {
                            terminal.partial_spans.pop();
                        }
                    }
                }
                '\t' => {
                    current_text.push_str("    ");
                }
                ch if ch.is_control() => {
                    // Skip other control characters
                }
                _ => {
                    current_text.push(ch);
                }
            }
        }

        if !current_text.is_empty() {
            terminal.partial_spans.push(TerminalSpan {
                text: current_text,
                color: terminal.current_color,
            });
        }

        // Limit line count
        const MAX_LINES: usize = 2000;
        if terminal.output_lines.len() > MAX_LINES {
            let remove_count = terminal.output_lines.len() - MAX_LINES;
            terminal.output_lines.drain(0..remove_count);
        }

        self.redraw(cx);
        Cx::post_action(TerminalAction::ScrollToBottom);
    }

    fn clear_output(&mut self, cx: &mut Cx) {
        if let Some(terminal) = self.terminals.get_mut(self.active_terminal_index) {
            terminal.output_lines.clear();
            terminal.partial_spans.clear();
            self.redraw(cx);
        }
    }

    pub fn handle_action(&mut self, cx: &mut Cx, action: &TerminalAction) {
        match action {
            TerminalAction::OutputReceived { terminal_id, text } => {
                // Find the terminal with this ID and append output
                if let Some(idx) = self.terminals.iter().position(|t| t.id == *terminal_id) {
                    self.append_output_to_terminal(cx, idx, text);
                }
            }
            TerminalAction::ScrollToBottom => {
                // Scroll PortalList so the input line (last item) is visible
                if let Some(terminal) = self.terminals.get(self.active_terminal_index) {
                    let list = self.view.portal_list(id!(output_list));
                    let total_items = terminal.output_lines.len() + 1;
                    if total_items > 0 {
                        // Show last ~40 items so user sees recent output + input line
                        list.set_first_id(total_items.saturating_sub(40));
                    }
                }
            }
            TerminalAction::None => {}
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum TerminalAction {
    OutputReceived { terminal_id: usize, text: String },
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
