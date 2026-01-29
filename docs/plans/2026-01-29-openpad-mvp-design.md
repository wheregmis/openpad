# Openpad MVP Design

**Date:** 2026-01-29
**Status:** Approved
**Goal:** Build a minimal viable chat client for OpenCode server using Makepad

## Overview

Openpad is a native GUI client for OpenCode (Claude Code server) built with Makepad. The MVP focuses on validating the core concept: connecting to OpenCode, sending prompts, and displaying streaming responses in a chat interface.

## Scope

### In Scope (MVP)
- Connect to OpenCode server on startup
- Create/list sessions via REST API
- Send text prompts
- Receive and display streaming message responses via SSE
- Plain text rendering only
- Single-view UI (message list + input box)

### Out of Scope (Post-MVP)
- Permission approval UI
- Terminal integration
- Code diff rendering
- Syntax highlighting
- Multi-session sidebar
- Tool output visualization
- Markdown rendering

## Architecture

### Two-Crate Structure

**openpad-protocol** - Reusable async Rust client
- HTTP client using reqwest
- SSE event stream subscription
- Type-safe models matching OpenCode API
- No UI dependencies

**openpad-app** - Makepad GUI application
- Depends on openpad-protocol
- Tokio runtime for async operations
- Bridges async → sync UI via `Cx::post_action()`
- State-driven reactive rendering

### Communication Flow

```
OpenCode Server (localhost:4096)
    ↕ HTTP/SSE
openpad-protocol (tokio async)
    ↕ Cx::post_action()
openpad-app (Makepad event loop)
```

## openpad-protocol Crate

### Structure
```
openpad-protocol/
├── src/
│   ├── lib.rs           # Re-exports
│   ├── client.rs        # OpenCodeClient
│   ├── types.rs         # Session, Message, Part, Event
│   └── error.rs         # Error types
└── Cargo.toml
```

### Dependencies
```toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
```

### Key Types

```rust
pub struct Session {
    pub id: String,
    pub title: Option<String>,
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum Message {
    User {
        id: String,
        parts: Vec<Part>,
        time: MessageTime,
    },
    Assistant {
        id: String,
        parts: Vec<Part>,
        model: String,
        time: MessageTime,
    },
}

pub enum Part {
    Text { text: String },
    // MVP: Other variants exist but ignored
}

pub enum Event {
    SessionCreated(Session),
    SessionDeleted(String),
    MessageUpdated { session_id: String, message: Message },
    PartUpdated { session_id: String, message_id: String, part: Part },
    Error(String),
    // ... 30+ other event types
}
```

### Client API

```rust
pub struct OpenCodeClient {
    http: reqwest::Client,
    base_url: String,
    event_tx: broadcast::Sender<Event>,
}

impl OpenCodeClient {
    pub fn new(base_url: &str) -> Self;

    // Session management
    pub async fn list_sessions(&self) -> Result<Vec<Session>>;
    pub async fn create_session(&self) -> Result<Session>;
    pub async fn get_session(&self, id: &str) -> Result<Session>;

    // Messaging
    pub async fn send_prompt(&self, session_id: &str, text: &str) -> Result<()>;

    // SSE subscription
    pub async fn subscribe(&self) -> Result<broadcast::Receiver<Event>>;
}
```

### SSE Implementation

The `subscribe()` method:
1. Opens GET /event?directory=<cwd> connection
2. Spawns tokio task to parse SSE stream
3. Returns broadcast receiver for events
4. Parses "data: {...}\n\n" format into Event enum

## openpad-app Crate

### Structure
```
openpad-app/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # App widget + async bridge
│   └── ui/
│       ├── mod.rs
│       ├── message_list.rs  # Message display
│       └── input_box.rs     # Text input
└── Cargo.toml
```

### Dependencies
```toml
[dependencies]
makepad-widgets = "1.0.0"
openpad-protocol = { path = "../openpad-protocol" }
tokio = { version = "1", features = ["full"] }
```

### App State

```rust
#[derive(Live, LiveHook)]
struct App {
    #[live] ui: WidgetRef,

    // State
    #[rust] messages: Vec<Message>,
    #[rust] current_session_id: Option<String>,
    #[rust] connected: bool,
    #[rust] error_message: Option<String>,

    // Runtime handle (for cleanup)
    #[rust] _runtime_handle: Option<tokio::runtime::Runtime>,
}
```

### Action System

```rust
#[derive(Clone, Debug, DefaultNone)]
enum AppAction {
    None,
    Connected,
    ConnectionFailed(String),
    SessionCreated(Session),
    SessionsLoaded(Vec<Session>),
    OpenCodeEvent(OcEvent),
    SendMessageFailed(String),
}
```

### Async Bridge Pattern

**Startup Flow:**
```rust
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event {
            Event::Startup => {
                self.connect_to_opencode(cx);
            }
            Event::Actions(actions) => {
                self.handle_actions(cx, actions);
            }
            _ => {}
        }

        // Forward to UI
        self.ui.handle_event(cx, event, scope);
    }
}

fn connect_to_opencode(&mut self, cx: &mut Cx) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.spawn(async move {
        match OpenCodeClient::new("http://localhost:4096").connect().await {
            Ok(client) => {
                Cx::post_action(AppAction::Connected);

                // Subscribe to SSE
                let mut rx = client.subscribe().await.unwrap();
                while let Ok(event) = rx.recv().await {
                    Cx::post_action(AppAction::OpenCodeEvent(event));
                }
            }
            Err(e) => {
                Cx::post_action(AppAction::ConnectionFailed(e.to_string()));
            }
        }
    });

    self._runtime_handle = Some(runtime);
}
```

**Handling Actions:**
```rust
fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
    for action in actions.iter() {
        if let Some(app_action) = action.downcast_ref::<AppAction>() {
            match app_action {
                AppAction::Connected => {
                    self.connected = true;
                    cx.redraw_all();
                }
                AppAction::OpenCodeEvent(oc_event) => {
                    self.handle_opencode_event(cx, oc_event);
                }
                AppAction::ConnectionFailed(err) => {
                    self.error_message = Some(err.clone());
                    cx.redraw_all();
                }
                _ => {}
            }
        }
    }
}
```

### UI Structure

```rust
live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Window> {
            window: { inner_size: vec2(1200, 800) }
            body = <View> {
                flow: Down,
                spacing: 0,

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

                    input_box = <TextInput> {
                        walk: { width: Fill, height: Fit }
                        placeholder: "Type a message..."
                    }
                }
            }
        }
    }
}
```

### Message Flow

**Sending:**
1. User types in `<TextInput>`, presses Enter
2. `TextInputAction::Returned(text)` captured
3. Spawn async task: `client.send_prompt(session_id, text)`
4. Add user message to `self.messages` immediately
5. Clear input, redraw

**Receiving:**
1. SSE events arrive: `MessageUpdated`, `PartUpdated`
2. Posted as `AppAction::OpenCodeEvent(event)`
3. Update `self.messages` Vec:
   - Find message by ID, or append new
   - Update parts with new content
4. `cx.redraw_all()` triggers UI update
5. Message list re-renders from state

### Message Rendering

**User messages:** Right-aligned, blue background
**Assistant messages:** Left-aligned, gray background

```rust
// Simplified - just show text
for msg in &self.messages {
    match msg {
        Message::User { parts, .. } => {
            for part in parts {
                if let Part::Text { text } = part {
                    // Render right-aligned label with text
                }
            }
        }
        Message::Assistant { parts, .. } => {
            for part in parts {
                if let Part::Text { text } = part {
                    // Render left-aligned label with text
                }
            }
        }
    }
}
```

## Error Handling

**Connection Failures:**
- Show error message in UI: "Failed to connect to OpenCode at localhost:4096"
- No auto-retry in MVP (just restart app)

**Send Failures:**
- Show toast/error message
- Keep message in input box so user can retry

**SSE Disconnection:**
- Post `AppAction::Disconnected`
- Show warning in UI
- No auto-reconnect in MVP

## Open Questions

1. Should we auto-create a session on startup, or wait for user to send first message?
2. How to handle window close (cleanup tokio runtime)?
3. Directory parameter for OpenCode - hardcode to CWD or configurable?

## Success Criteria

MVP is successful when:
1. ✅ App connects to OpenCode on launch
2. ✅ User can type message and press Enter
3. ✅ Message appears in chat (user side)
4. ✅ Assistant response streams in token-by-token
5. ✅ Multiple back-and-forth exchanges work
6. ✅ Can create new session and continue conversation

## Future Enhancements

- Session sidebar for switching between conversations
- Permission approval cards for tool use
- Markdown/syntax highlighting
- Terminal view integration
- Settings panel (API key, model selection)
- Export conversations
