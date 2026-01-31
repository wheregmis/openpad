//! Terminal widget - A simple terminal emulator widget.
//!
//! This component provides a basic terminal interface using portable-pty.

use makepad_widgets::*;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub Terminal = {{Terminal}} {
        width: Fill, height: 200
        flow: Down
        show_bg: true
        draw_bg: {
            color: #1a1d23
        }

        terminal_header = <View> {
            width: Fill, height: Fit
            flow: Right
            padding: { left: 12, right: 12, top: 8, bottom: 8 }
            spacing: 8
            align: { y: 0.5 }
            show_bg: true
            draw_bg: {
                color: #22262c
            }

            <Label> {
                text: "Terminal"
                draw_text: { color: #e6e9ee, text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
            }
            <View> { width: Fill }
            clear_button = <Button> {
                width: Fit, height: 24
                text: "Clear"
                draw_bg: {
                    color: #2a2f36
                    color_hover: #313843
                    border_radius: 4.0
                }
                draw_text: { color: #cbd3dc, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
            }
        }

        terminal_output = <View> {
            width: Fill, height: Fill
            scroll_bars: <ScrollBars> {}
            <PortalList> {
                width: Fill
                flow: Down

                output_label = <Label> {
                    width: Fill
                    text: ""
                    draw_text: {
                        wrap: Word
                        color: #e6e9ee
                        text_style: <THEME_FONT_CODE> { font_size: 11 }
                    }
                }
            }
        }

        terminal_input = <View> {
            width: Fill, height: Fit
            flow: Right
            padding: { left: 12, right: 12, top: 6, bottom: 8 }
            spacing: 8
            show_bg: true
            draw_bg: {
                color: #1f2329
            }

            input_field = <TextInput> {
                width: Fill, height: Fit
                empty_text: "Enter command..."
                draw_bg: {
                    color: #2a2f36
                    border_radius: 4.0
                }
                draw_text: { color: #e6e9ee, text_style: <THEME_FONT_CODE> { font_size: 11 } }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct Terminal {
    #[deref]
    view: View,
    
    #[rust]
    output_text: String,
    
    #[rust]
    pty_writer: Option<Arc<Mutex<Box<dyn Write + Send>>>>,
}

impl LiveHook for Terminal {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.output_text = String::new();
    }
}

impl Widget for Terminal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // Handle input field Enter key
            if let Some((input, _modifiers)) = self.view.text_input(id!(input_field)).returned(actions) {
                if !input.is_empty() {
                    self.send_command(cx, &input);
                    self.view.text_input(id!(input_field)).set_text(cx, "");
                }
            }
            
            // Handle clear button
            if self.view.button(id!(clear_button)).clicked(actions) {
                self.clear_output(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Terminal {
    pub fn init_pty(&mut self, cx: &mut Cx) {
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
                self.append_output(cx, &format!("Failed to create PTY: {}\n", e));
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
        cmd.cwd(cwd);
        
        if let Err(e) = pair.slave.spawn_command(cmd) {
            self.append_output(cx, &format!("Failed to spawn shell: {}\n", e));
            return;
        }
        
        // Store writer
        let writer = match pair.master.take_writer() {
            Ok(w) => Arc::new(Mutex::new(w)),
            Err(e) => {
                self.append_output(cx, &format!("Failed to get PTY writer: {}\n", e));
                return;
            }
        };
        self.pty_writer = Some(writer);
        
        // Start reading output in background
        // NOTE: This thread will run until the PTY reader is closed or an error occurs.
        // Future enhancement: Store thread handle and implement proper cleanup on widget drop.
        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(e) => {
                self.append_output(cx, &format!("Failed to get PTY reader: {}\n", e));
                return;
            }
        };
        
        std::thread::spawn(move || {
            let buf_reader = BufReader::new(reader);
            for line in buf_reader.lines() {
                match line {
                    Ok(text) => {
                        // Post output to main thread
                        Cx::post_action(TerminalAction::OutputReceived(text + "\n"));
                    }
                    Err(_) => break,
                }
            }
        });
        
        self.append_output(cx, &format!("Terminal initialized with shell: {}\n", shell));
    }
    
    fn send_command(&mut self, cx: &mut Cx, command: &str) {
        self.append_output(cx, &format!("> {}\n", command));
        
        if let Some(writer) = &self.pty_writer {
            let result = {
                let mut w = match writer.lock() {
                    Ok(w) => w,
                    Err(_) => return,
                };
                writeln!(w, "{}", command)
            };
            
            if let Err(e) = result {
                self.append_output(cx, &format!("Failed to send command: {}\n", e));
            }
        }
    }
    
    fn append_output(&mut self, cx: &mut Cx, text: &str) {
        // NOTE: output_text grows unbounded. Future enhancement: implement buffer size limit
        // and auto-trim old content to prevent excessive memory usage in long sessions.
        self.output_text.push_str(text);
        self.view.label(id!(output_label)).set_text(cx, &self.output_text);
        cx.redraw_all();
    }
    
    fn clear_output(&mut self, cx: &mut Cx) {
        self.output_text.clear();
        self.view.label(id!(output_label)).set_text(cx, "");
        cx.redraw_all();
    }
    
    pub fn handle_action(&mut self, cx: &mut Cx, action: &TerminalAction) {
        match action {
            TerminalAction::OutputReceived(text) => {
                self.append_output(cx, text);
            }
            TerminalAction::None => {}
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum TerminalAction {
    OutputReceived(String),
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
