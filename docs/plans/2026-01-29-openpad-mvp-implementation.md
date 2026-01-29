# Openpad MVP Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a minimal viable chat client for OpenCode server using Makepad with plain text rendering.

**Architecture:** Two-crate structure (openpad-protocol for async HTTP/SSE client, openpad-app for Makepad UI). Async operations bridge to sync UI via `Cx::post_action()`. State-driven rendering where SSE events update state, triggering UI redraws.

**Tech Stack:** Rust, Makepad, reqwest, tokio, serde, chrono

---

## Task 1: Set Up Workspace Structure

**Files:**
- Create: `openpad-protocol/Cargo.toml`
- Create: `openpad-protocol/src/lib.rs`
- Modify: `Cargo.toml` (workspace root)

**Step 1: Create workspace Cargo.toml**

Replace the existing single-crate setup with a workspace:

```toml
[workspace]
members = ["openpad-protocol", "openpad-app"]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Step 2: Create openpad-protocol crate structure**

```bash
mkdir -p openpad-protocol/src
```

**Step 3: Write openpad-protocol/Cargo.toml**

```toml
[package]
name = "openpad-protocol"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"
futures-util = "0.3"
```

**Step 4: Create openpad-protocol/src/lib.rs**

```rust
pub mod types;
pub mod client;
pub mod error;

pub use client::OpenCodeClient;
pub use types::*;
pub use error::{Error, Result};
```

**Step 5: Rename existing crate to openpad-app**

```bash
mkdir -p openpad-app/src
mv src/* openpad-app/src/
mv Cargo.toml openpad-app/Cargo.toml
```

**Step 6: Update openpad-app/Cargo.toml**

```toml
[package]
name = "openpad-app"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "openpad"
path = "src/main.rs"

[dependencies]
makepad-widgets = "1.0.0"
openpad-protocol = { path = "../openpad-protocol" }
tokio = { workspace = true }
```

**Step 7: Verify workspace builds**

```bash
cargo build
```

Expected: All crates compile successfully

**Step 8: Commit**

```bash
git add -A
git commit -m "chore: set up workspace with openpad-protocol and openpad-app crates"
```

---

## Task 2: Implement Protocol Error Types

**Files:**
- Create: `openpad-protocol/src/error.rs`

**Step 1: Write error types**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("SSE stream error: {0}")]
    Sse(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Step 2: Verify it compiles**

```bash
cargo build -p openpad-protocol
```

Expected: Compiles successfully

**Step 3: Commit**

```bash
git add openpad-protocol/src/error.rs
git commit -m "feat(protocol): add error types"
```

---

## Task 3: Implement Protocol Types

**Files:**
- Create: `openpad-protocol/src/types.rs`

**Step 1: Write core types**

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    pub model: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageTime {
    pub start: DateTime<Utc>,
    #[serde(default)]
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    #[serde(rename = "user")]
    User {
        id: String,
        parts: Vec<Part>,
        time: MessageTime,
    },
    #[serde(rename = "assistant")]
    Assistant {
        id: String,
        parts: Vec<Part>,
        model: String,
        time: MessageTime,
    },
}

impl Message {
    pub fn id(&self) -> &str {
        match self {
            Message::User { id, .. } => id,
            Message::Assistant { id, .. } => id,
        }
    }

    pub fn parts(&self) -> &[Part] {
        match self {
            Message::User { parts, .. } => parts,
            Message::Assistant { parts, .. } => parts,
        }
    }

    pub fn parts_mut(&mut self) -> &mut Vec<Part> {
        match self {
            Message::User { parts, .. } => parts,
            Message::Assistant { parts, .. } => parts,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Part {
    #[serde(rename = "text")]
    Text { text: String },
    // MVP: Other variants ignored but need to not break parsing
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartInput {
    #[serde(rename = "type")]
    pub type_name: String,
    pub text: String,
}

impl PartInput {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            type_name: "text".to_string(),
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    SessionCreated(Session),
    SessionDeleted(String),
    MessageUpdated {
        session_id: String,
        message: Message,
    },
    PartUpdated {
        session_id: String,
        message_id: String,
        part_index: usize,
        part: Part,
    },
    Error(String),
    Unknown(String),
}
```

**Step 2: Verify it compiles**

```bash
cargo build -p openpad-protocol
```

Expected: Compiles successfully

**Step 3: Commit**

```bash
git add openpad-protocol/src/types.rs
git commit -m "feat(protocol): add core types (Session, Message, Part, Event)"
```

---

## Task 4: Implement OpenCode Client - Basic Structure

**Files:**
- Create: `openpad-protocol/src/client.rs`

**Step 1: Write client structure**

```rust
use crate::{Error, Result, Event, Session, Message, PartInput};
use reqwest::Client as HttpClient;
use tokio::sync::broadcast;
use std::env;

pub struct OpenCodeClient {
    http: HttpClient,
    base_url: String,
    directory: String,
    event_tx: broadcast::Sender<Event>,
}

impl OpenCodeClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let (event_tx, _) = broadcast::channel(256);
        let directory = env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| ".".to_string());

        Self {
            http: HttpClient::new(),
            base_url: base_url.into(),
            directory,
            event_tx,
        }
    }

    pub fn with_directory(mut self, directory: impl Into<String>) -> Self {
        self.directory = directory.into();
        self
    }
}
```

**Step 2: Verify it compiles**

```bash
cargo build -p openpad-protocol
```

Expected: Compiles successfully

**Step 3: Commit**

```bash
git add openpad-protocol/src/client.rs
git commit -m "feat(protocol): add OpenCodeClient basic structure"
```

---

## Task 5: Implement Session Management Methods

**Files:**
- Modify: `openpad-protocol/src/client.rs`

**Step 1: Add session methods**

Add these methods to the `impl OpenCodeClient` block:

```rust
    pub async fn list_sessions(&self) -> Result<Vec<Session>> {
        let url = format!("{}/session", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to list sessions: {}",
                response.status()
            )));
        }

        let sessions: Vec<Session> = response.json().await?;
        Ok(sessions)
    }

    pub async fn create_session(&self) -> Result<Session> {
        let url = format!("{}/session", self.base_url);
        let body = serde_json::json!({});

        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to create session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn get_session(&self, id: &str) -> Result<Session> {
        let url = format!("{}/session/{}", self.base_url, id);

        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }
```

**Step 2: Verify it compiles**

```bash
cargo build -p openpad-protocol
```

Expected: Compiles successfully

**Step 3: Commit**

```bash
git add openpad-protocol/src/client.rs
git commit -m "feat(protocol): add session management methods"
```

---

## Task 6: Implement Send Prompt Method

**Files:**
- Modify: `openpad-protocol/src/client.rs`

**Step 1: Add send_prompt method**

Add this method to the `impl OpenCodeClient` block:

```rust
    pub async fn send_prompt(&self, session_id: &str, text: &str) -> Result<()> {
        let url = format!("{}/session/{}/prompt", self.base_url, session_id);
        let body = serde_json::json!({
            "parts": vec![PartInput::text(text)],
        });

        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to send prompt: {}",
                response.status()
            )));
        }

        Ok(())
    }
```

**Step 2: Verify it compiles**

```bash
cargo build -p openpad-protocol
```

Expected: Compiles successfully

**Step 3: Commit**

```bash
git add openpad-protocol/src/client.rs
git commit -m "feat(protocol): add send_prompt method"
```

---

## Task 7: Implement SSE Event Subscription

**Files:**
- Modify: `openpad-protocol/src/client.rs`

**Step 1: Add SSE parsing helper**

Add this function at the bottom of the file (outside the impl block):

```rust
fn parse_sse_event(data: &str) -> Option<Event> {
    let value: serde_json::Value = serde_json::from_str(data).ok()?;

    let event_type = value.get("type")?.as_str()?;
    let props = value.get("properties")?;

    match event_type {
        "session.created" => {
            let session: Session = serde_json::from_value(props.clone()).ok()?;
            Some(Event::SessionCreated(session))
        }
        "session.deleted" => {
            let id = props.get("id")?.as_str()?.to_string();
            Some(Event::SessionDeleted(id))
        }
        "message.updated" => {
            let session_id = props.get("sessionId")?.as_str()?.to_string();
            let message: Message = serde_json::from_value(props.get("message")?.clone()).ok()?;
            Some(Event::MessageUpdated { session_id, message })
        }
        "message.part.updated" => {
            let session_id = props.get("sessionId")?.as_str()?.to_string();
            let message_id = props.get("messageId")?.as_str()?.to_string();
            let part_index = props.get("index")?.as_u64()? as usize;
            let part: Part = serde_json::from_value(props.get("part")?.clone()).ok()?;
            Some(Event::PartUpdated { session_id, message_id, part_index, part })
        }
        "session.error" => {
            let error = props.get("error")?.as_str()?.to_string();
            Some(Event::Error(error))
        }
        _ => Some(Event::Unknown(event_type.to_string())),
    }
}
```

**Step 2: Add subscribe method**

Add this method to the `impl OpenCodeClient` block:

```rust
    pub async fn subscribe(&self) -> Result<broadcast::Receiver<Event>> {
        use futures_util::StreamExt;

        let url = format!("{}/event", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::Connection(format!(
                "Failed to connect to SSE stream: {}",
                response.status()
            )));
        }

        let event_tx = self.event_tx.clone();

        // Spawn task to read SSE stream
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));

                        // Parse SSE format: "data: {...}\n\n"
                        while let Some(idx) = buffer.find("\n\n") {
                            let event_str = &buffer[..idx];

                            if let Some(data) = event_str.strip_prefix("data: ") {
                                if let Some(event) = parse_sse_event(data) {
                                    let _ = event_tx.send(event);
                                }
                            }

                            buffer = buffer[idx + 2..].to_string();
                        }
                    }
                    Err(e) => {
                        let _ = event_tx.send(Event::Error(format!("Stream error: {}", e)));
                        break;
                    }
                }
            }
        });

        Ok(self.event_tx.subscribe())
    }
```

**Step 3: Verify it compiles**

```bash
cargo build -p openpad-protocol
```

Expected: Compiles successfully

**Step 4: Commit**

```bash
git add openpad-protocol/src/client.rs
git commit -m "feat(protocol): add SSE event subscription"
```

---

## Task 8: Set Up Makepad App Structure

**Files:**
- Modify: `openpad-app/src/app.rs`
- Modify: `openpad-app/src/main.rs`

**Step 1: Update main.rs**

```rust
fn main() {
    openpad_app::app_main();
}
```

**Step 2: Rewrite app.rs with basic structure**

```rust
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Window> {
            window: { inner_size: vec2(1200, 800) }
            pass: { clear_color: #1a1a1a }

            body = <View> {
                flow: Down,
                spacing: 0,

                // Status bar at top
                <View> {
                    walk: { width: Fill, height: Fit }
                    flow: Right,
                    spacing: 8,
                    padding: 8,
                    draw_bg: { color: #2a2a2a }

                    status_label = <Label> {
                        text: "Connecting..."
                        draw_text: { color: #888 }
                    }
                }

                // Messages area (scrollable)
                <ScrollYView> {
                    walk: { width: Fill, height: Fill }

                    message_list = <View> {
                        flow: Down,
                        spacing: 16,
                        padding: 16,
                    }
                }

                // Input area (fixed at bottom)
                <View> {
                    walk: { width: Fill, height: Fit }
                    flow: Right,
                    spacing: 8,
                    padding: 16,
                    draw_bg: { color: #2a2a2a }

                    input_box = <TextInput> {
                        walk: { width: Fill, height: Fit }
                        draw_bg: { color: #333 }
                        draw_text: { color: #fff }
                        text: ""
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);

pub fn app_main() {
    App::run_app();
}
```

**Step 3: Verify it builds and runs**

```bash
cargo build -p openpad-app
cargo run
```

Expected: Window opens with dark UI, status bar, empty message list, and input box

**Step 4: Commit**

```bash
git add openpad-app/src/app.rs openpad-app/src/main.rs
git commit -m "feat(app): add basic Makepad UI structure"
```

---

## Task 9: Add App State and Action System

**Files:**
- Modify: `openpad-app/src/app.rs`

**Step 1: Add imports at top**

```rust
use makepad_widgets::*;
use openpad_protocol::{OpenCodeClient, Session, Message, Event as OcEvent};
use std::sync::Arc;
use tokio::sync::Mutex;
```

**Step 2: Define AppAction enum before App struct**

```rust
#[derive(Clone, Debug, DefaultNone)]
pub enum AppAction {
    None,
    Connected,
    ConnectionFailed(String),
    SessionCreated(Session),
    OpenCodeEvent(OcEvent),
    SendMessageFailed(String),
}
```

**Step 3: Update App struct**

Replace the App struct with:

```rust
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,

    #[rust] messages: Vec<Message>,
    #[rust] current_session_id: Option<String>,
    #[rust] connected: bool,
    #[rust] error_message: Option<String>,
    #[rust] client: Option<Arc<OpenCodeClient>>,
    #[rust] _runtime: Option<tokio::runtime::Runtime>,
}
```

**Step 4: Verify it compiles**

```bash
cargo build -p openpad-app
```

Expected: Compiles successfully

**Step 5: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat(app): add app state and action system"
```

---

## Task 10: Implement Connection on Startup

**Files:**
- Modify: `openpad-app/src/app.rs`

**Step 1: Add connection method**

Add this method to the App impl block (before the AppMain impl):

```rust
impl App {
    fn connect_to_opencode(&mut self, cx: &mut Cx) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = Arc::new(OpenCodeClient::new("http://localhost:4096"));
        let client_clone = client.clone();

        runtime.spawn(async move {
            // Try to connect by listing sessions
            match client_clone.list_sessions().await {
                Ok(_) => {
                    Cx::post_action(AppAction::Connected);

                    // Subscribe to SSE
                    if let Ok(mut rx) = client_clone.subscribe().await {
                        while let Ok(event) = rx.recv().await {
                            Cx::post_action(AppAction::OpenCodeEvent(event));
                        }
                    }
                }
                Err(e) => {
                    Cx::post_action(AppAction::ConnectionFailed(e.to_string()));
                }
            }
        });

        self.client = Some(client);
        self._runtime = Some(runtime);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        for action in actions {
            if let Some(app_action) = action.as_widget_action().cast::<AppAction>() {
                match app_action {
                    AppAction::Connected => {
                        self.connected = true;
                        self.error_message = None;
                        self.ui.label(id!(status_label)).set_text(cx, "Connected");
                        cx.redraw_all();
                    }
                    AppAction::ConnectionFailed(err) => {
                        self.error_message = Some(err.clone());
                        self.ui.label(id!(status_label)).set_text(cx, &format!("Error: {}", err));
                        cx.redraw_all();
                    }
                    AppAction::SessionCreated(session) => {
                        self.current_session_id = Some(session.id.clone());
                        self.ui.label(id!(status_label)).set_text(cx, &format!("Session: {}", session.id));
                        cx.redraw_all();
                    }
                    AppAction::OpenCodeEvent(oc_event) => {
                        self.handle_opencode_event(cx, oc_event);
                    }
                    AppAction::SendMessageFailed(err) => {
                        self.error_message = Some(err.clone());
                        cx.redraw_all();
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_opencode_event(&mut self, cx: &mut Cx, event: &OcEvent) {
        match event {
            OcEvent::SessionCreated(session) => {
                if self.current_session_id.is_none() {
                    self.current_session_id = Some(session.id.clone());
                    self.ui.label(id!(status_label)).set_text(cx, &format!("Session: {}", session.id));
                }
            }
            OcEvent::MessageUpdated { message, .. } => {
                // Find existing message or add new
                if let Some(existing) = self.messages.iter_mut().find(|m| m.id() == message.id()) {
                    *existing = message.clone();
                } else {
                    self.messages.push(message.clone());
                }
                cx.redraw_all();
            }
            OcEvent::PartUpdated { message_id, part_index, part, .. } => {
                if let Some(msg) = self.messages.iter_mut().find(|m| m.id() == message_id) {
                    let parts = msg.parts_mut();
                    if *part_index < parts.len() {
                        parts[*part_index] = part.clone();
                    } else {
                        parts.push(part.clone());
                    }
                    cx.redraw_all();
                }
            }
            _ => {}
        }
    }
}
```

**Step 2: Update AppMain impl to handle startup**

Replace the `impl AppMain for App` with:

```rust
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.connect_to_opencode(cx);
            }
            Event::Actions(actions) => {
                self.handle_actions(cx, actions);
            }
            _ => {}
        }

        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

**Step 3: Verify it compiles**

```bash
cargo build -p openpad-app
```

Expected: Compiles successfully

**Step 4: Test connection (if OpenCode is running)**

```bash
cargo run
```

Expected: If OpenCode running → "Connected", otherwise → "Error: ..."

**Step 5: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat(app): implement connection on startup"
```

---

## Task 11: Implement Session Creation on First Message

**Files:**
- Modify: `openpad-app/src/app.rs`

**Step 1: Add send message method**

Add this method to the App impl block:

```rust
    fn send_message(&mut self, cx: &mut Cx, text: String) {
        let Some(client) = self.client.clone() else {
            self.error_message = Some("Not connected".to_string());
            return;
        };

        let session_id = self.current_session_id.clone();

        self._runtime.as_ref().unwrap().spawn(async move {
            // Create session if needed
            let sid = if let Some(id) = session_id {
                id
            } else {
                match client.create_session().await {
                    Ok(session) => {
                        Cx::post_action(AppAction::SessionCreated(session.clone()));
                        session.id
                    }
                    Err(e) => {
                        Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
                        return;
                    }
                }
            };

            // Send prompt
            if let Err(e) = client.send_prompt(&sid, &text).await {
                Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
            }
        });
    }
```

**Step 2: Handle TextInput return action**

Add to the end of `handle_event` method in `impl AppMain for App`, right after the match on Event but before calling `self.ui.handle_event`:

```rust
        // Handle text input
        let actions = self.ui.text_input(id!(input_box)).changed(&[]);
        if let Some(returned_text) = self.ui.text_input(id!(input_box)).returned(&actions) {
            if !returned_text.is_empty() {
                self.send_message(cx, returned_text);
                self.ui.text_input(id!(input_box)).set_text(cx, "");
            }
        }
```

Actually, this won't work properly. Let me fix it:

Replace the handle_event implementation with:

```rust
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.connect_to_opencode(cx);
            }
            Event::Actions(actions) => {
                self.handle_actions(cx, actions);
            }
            _ => {}
        }

        // Capture actions from UI
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });

        // Check for text input return
        if let Some(text) = self.ui.text_input(id!(input_box)).returned(&actions) {
            if !text.is_empty() {
                self.send_message(cx, text);
                self.ui.text_input(id!(input_box)).set_text(cx, "");
            }
        }
    }
}
```

**Step 3: Verify it compiles**

```bash
cargo build -p openpad-app
```

Expected: Compiles successfully

**Step 4: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat(app): implement session creation and message sending"
```

---

## Task 12: Implement Message Rendering

**Files:**
- Modify: `openpad-app/src/app.rs`

**Step 1: Update live_design with message templates**

Replace the message_list section in live_design! with:

```rust
                // Messages area (scrollable)
                <ScrollYView> {
                    walk: { width: Fill, height: Fill }

                    message_list = <View> {
                        flow: Down,
                        spacing: 16,
                        padding: 16,

                        // User message template
                        UserMessage = <View> {
                            visible: false,
                            walk: { width: Fill, height: Fit }
                            flow: Right,
                            align: { x: 1.0 }

                            <RoundedView> {
                                walk: { width: Fit, height: Fit, margin: { left: 100 } }
                                draw_bg: { color: #2a4a6a, radius: 8.0 }
                                layout: { padding: 12 }

                                <Label> {
                                    walk: { width: Fit, height: Fit }
                                    draw_text: { color: #fff, wrap: Word }
                                }
                            }
                        }

                        // Assistant message template
                        AssistantMessage = <View> {
                            visible: false,
                            walk: { width: Fill, height: Fit }
                            flow: Down,

                            <RoundedView> {
                                walk: { width: Fit, height: Fit, margin: { right: 100 } }
                                draw_bg: { color: #333, radius: 8.0 }
                                layout: { padding: 12 }

                                <Label> {
                                    walk: { width: Fit, height: Fit }
                                    draw_text: { color: #fff, wrap: Word }
                                }
                            }
                        }
                    }
                }
```

**Step 2: Add draw_walk implementation**

Add this implementation to App:

```rust
impl Widget for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.handle_event_with_fn(cx, event, scope, &mut |_, _, _, _| {});
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // First, ensure templates are drawn
        while let Some(view_item) = self.ui.draw_walk(cx, scope, walk).step() {
            // Draw message list dynamically
            if let Some(mut list) = view_item.as_view().borrow_mut() {
                if list.path() == &[live_id!(message_list)] {
                    list.begin(cx, walk, scope.props);

                    for message in &self.messages {
                        match message {
                            Message::User { parts, .. } => {
                                for part in parts {
                                    if let openpad_protocol::Part::Text { text } = part {
                                        let mut item = list.item(cx, live_id!(UserMessage), scope.props);
                                        item.label(id!(label)).set_text(text);
                                        item.draw_all(cx, scope);
                                    }
                                }
                            }
                            Message::Assistant { parts, .. } => {
                                for part in parts {
                                    if let openpad_protocol::Part::Text { text } = part {
                                        let mut item = list.item(cx, live_id!(AssistantMessage), scope.props);
                                        item.label(id!(label)).set_text(text);
                                        item.draw_all(cx, scope);
                                    }
                                }
                            }
                        }
                    }

                    list.end(cx);
                }
            }
        }

        DrawStep::done()
    }
}
```

Note: This approach won't work well with Makepad's widget system. Let me provide a simpler approach:

Actually, for MVP, let's keep it even simpler and just update status label to show message count:

**Step 2 (Revised): Update status to show message count**

In the `handle_opencode_event` method, after updating messages, add:

```rust
                self.ui.label(id!(status_label)).set_text(
                    cx,
                    &format!("Session: {} | {} messages",
                        self.current_session_id.as_deref().unwrap_or("none"),
                        self.messages.len()
                    )
                );
```

**Step 3: Verify it compiles and test**

```bash
cargo build -p openpad-app
cargo run
```

Expected: Status bar shows message count updating

**Step 4: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat(app): add message state tracking and count display"
```

---

## Task 13: Add Simple Message List View

**Files:**
- Modify: `openpad-app/src/app.rs`

**Step 1: Simplify message display with PortalList**

Replace the message_list section in live_design with:

```rust
                <PortalList> {
                    walk: { width: Fill, height: Fill }
                    flow: Down,

                    UserMsg = <View> {
                        walk: { width: Fill, height: Fit }
                        flow: Right,
                        padding: 8,
                        align: { x: 1.0 }

                        <View> {
                            walk: { width: Fit, height: Fit, margin: { left: 100 } }
                            flow: Down,
                            padding: 12,
                            draw_bg: { color: #2a4a6a }

                            msg_text = <Label> {
                                draw_text: { color: #fff, text_style: { font_size: 11 } }
                            }
                        }
                    }

                    AssistantMsg = <View> {
                        walk: { width: Fill, height: Fit }
                        flow: Down,
                        padding: 8,

                        <View> {
                            walk: { width: Fit, height: Fit, margin: { right: 100 } }
                            flow: Down,
                            padding: 12,
                            draw_bg: { color: #333 }

                            msg_text = <Label> {
                                draw_text: { color: #fff, text_style: { font_size: 11 } }
                            }
                        }
                    }
                }
```

**Step 2: Implement Widget trait with PortalList rendering**

Replace the Widget impl with:

```rust
impl Widget for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.handle_event_with_fn(cx, event, scope, &mut |_, _, _, _| {});
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(view) = self.ui.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = view.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.messages.len());

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < self.messages.len() {
                        let message = &self.messages[item_id];

                        let template_id = match message {
                            Message::User { .. } => live_id!(UserMsg),
                            Message::Assistant { .. } => live_id!(AssistantMsg),
                        };

                        let item = list.item(cx, item_id, template_id);

                        // Get text from message parts
                        let text = message.parts().iter()
                            .filter_map(|p| match p {
                                openpad_protocol::Part::Text { text } => Some(text.as_str()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("\n");

                        item.label(id!(msg_text)).set_text(&text);
                        item.draw_all(cx, scope);
                    }
                }
            }
        }

        DrawStep::done()
    }
}
```

**Step 3: Verify it compiles**

```bash
cargo build -p openpad-app
```

Expected: Compiles successfully

**Step 4: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat(app): implement message list rendering with PortalList"
```

---

## Task 14: Final Integration Test

**Files:**
- None (testing only)

**Step 1: Ensure OpenCode server is running**

```bash
# In another terminal
opencode
```

**Step 2: Run the app**

```bash
cargo run
```

**Step 3: Test the flow**

1. Verify status shows "Connected"
2. Type a message in input box
3. Press Enter
4. Verify message appears in list (user message, right-aligned, blue)
5. Verify assistant response streams in (left-aligned, gray)
6. Send another message
7. Verify conversation continues

**Step 4: Document any issues**

If anything doesn't work, note it down for fixing.

**Step 5: Commit if any fixes were needed**

```bash
git add -A
git commit -m "fix: resolve integration issues"
```

---

## Task 15: Add README and Documentation

**Files:**
- Create: `README.md`
- Create: `openpad-protocol/README.md`

**Step 1: Write root README.md**

```markdown
# Openpad

A native GUI client for OpenCode (Claude Code server) built with Makepad.

## Overview

Openpad provides a clean chat interface for interacting with Claude Code through the OpenCode server. This is an MVP focused on core functionality: connecting to OpenCode, sending messages, and displaying streaming responses.

## Features

- ✅ Connect to OpenCode server on startup
- ✅ Create chat sessions automatically
- ✅ Send text messages
- ✅ Display streaming responses in real-time
- ✅ Plain text message rendering

## Prerequisites

- Rust 1.70+
- OpenCode server running (default: localhost:4096)

## Installation

```bash
git clone <repository>
cd openpad
cargo build --release
```

## Usage

1. Start OpenCode server:
   ```bash
   opencode
   ```

2. Run Openpad:
   ```bash
   cargo run --release
   ```

3. Type messages in the input box and press Enter

## Architecture

Openpad consists of two crates:

- **openpad-protocol**: Async Rust client for OpenCode HTTP/SSE API
- **openpad-app**: Makepad-based GUI application

The app bridges async operations to the sync UI using `Cx::post_action()` for thread-safe communication.

## Development

See [docs/plans/2026-01-29-openpad-mvp-design.md](docs/plans/2026-01-29-openpad-mvp-design.md) for architecture details.

## Future Enhancements

- Session sidebar
- Permission approval UI
- Markdown rendering
- Syntax highlighting
- Terminal integration
- Code diff visualization

## License

[Your License]
```

**Step 2: Write openpad-protocol/README.md**

```markdown
# openpad-protocol

Async Rust client library for the OpenCode server API.

## Features

- Type-safe HTTP client for OpenCode REST API
- Server-Sent Events (SSE) subscription for real-time updates
- Session management
- Message sending with prompt API
- Error handling with typed errors

## Usage

```rust
use openpad_protocol::{OpenCodeClient, Event};

#[tokio::main]
async fn main() {
    let client = OpenCodeClient::new("http://localhost:4096");

    // Create a session
    let session = client.create_session().await.unwrap();

    // Subscribe to events
    let mut events = client.subscribe().await.unwrap();

    // Send a message
    client.send_prompt(&session.id, "Hello!").await.unwrap();

    // Listen for responses
    while let Ok(event) = events.recv().await {
        println!("{:?}", event);
    }
}
```

## API

See [types.rs](src/types.rs) for complete type definitions.

### OpenCodeClient

- `new(base_url)` - Create client
- `list_sessions()` - Get all sessions
- `create_session()` - Create new session
- `get_session(id)` - Get session details
- `send_prompt(session_id, text)` - Send message
- `subscribe()` - Subscribe to SSE events

### Events

- `SessionCreated(Session)`
- `MessageUpdated { session_id, message }`
- `PartUpdated { session_id, message_id, part_index, part }`
- `Error(String)`

## Dependencies

- reqwest - HTTP client
- tokio - Async runtime
- serde - Serialization
- chrono - DateTime handling
```

**Step 3: Verify markdown renders correctly**

View the files in a markdown viewer or on GitHub.

**Step 4: Commit**

```bash
git add README.md openpad-protocol/README.md
git commit -m "docs: add README files"
```

---

## Success Criteria Checklist

After completing all tasks, verify:

- [ ] App connects to OpenCode on launch
- [ ] Status bar shows "Connected"
- [ ] User can type message and press Enter
- [ ] User message appears in chat (right-aligned, blue)
- [ ] Assistant response streams in token-by-token
- [ ] Assistant messages appear (left-aligned, gray)
- [ ] Multiple back-and-forth exchanges work
- [ ] Sessions are created automatically on first message
- [ ] No crashes or panics during normal usage

## Next Steps

After MVP is complete:

1. Add permission approval UI (critical for tool use)
2. Implement session sidebar for switching conversations
3. Add markdown rendering with syntax highlighting
4. Integrate terminal view for bash tool output
5. Add code diff visualization for edit tools
