Openpad Architecture - Full Rust + Makepad
Protocol Summary
OpenCode exposes a REST API + SSE on localhost (default port 4096):
┌─────────────────────────────────────────────────────────────────┐
│                    OpenCode Server (Go)                        │
│  localhost:4096                                                 │
├─────────────────────────────────────────────────────────────────┤
│  REST Endpoints:                                                │
│    GET  /session              - List sessions                   │
│    POST /session              - Create session                  │
│    GET  /session/{id}         - Get session                     │
│    DELETE /session/{id}       - Delete session                  │
│    POST /session/{id}/prompt  - Send prompt (main interaction)  │
│    POST /session/{id}/abort   - Abort running session           │
│    GET  /session/{id}/messages - Get all messages               │
│    POST /session/{id}/permission/{pid}/respond - Approve tools  │
│                                                                 │
│    GET  /event?directory=...  - SSE stream (real-time events)   │
│    GET  /config               - Get configuration               │
│    GET  /app/providers        - List AI providers/models        │
│    GET  /file?path=...        - Read file                       │
│    GET  /file/status          - Git status                      │
│    POST /pty                  - Create PTY (WebSocket upgrade)  │
├─────────────────────────────────────────────────────────────────┤
│  SSE Events (30+ types):                                        │
│    session.created, session.deleted, session.error              │
│    message.updated, message.part.updated                        │
│    permission.updated, permission.replied                       │
│    file.edited, file.watcher.updated                            │
│    ...                                                          │
└─────────────────────────────────────────────────────────────────┘

Refined Openpad Architecture
openpad/
├── Cargo.toml                    # Workspace
├── crates/
│   ├── openpad-protocol/         # Pure Rust OpenCode client
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── client.rs         # HTTP + SSE client
│   │       ├── types/
│   │       │   ├── mod.rs
│   │       │   ├── session.rs    # Session, Message, Part
│   │       │   ├── event.rs      # All 30+ SSE event types
│   │       │   ├── config.rs     # Config, Provider, Model
│   │       │   ├── tool.rs       # ToolPart, ToolState
│   │       │   └── permission.rs # Permission types
│   │       └── error.rs
│   │
│   ├── openpad-terminal/         # PTY + ANSI rendering
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pty.rs            # portable-pty wrapper
│   │       ├── parser.rs         # ANSI escape sequence parser
│   │       ├── grid.rs           # Terminal cell grid
│   │       └── websocket.rs      # WS connection to /pty
│   │
│   ├── openpad-syntax/           # Code highlighting + diff
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── highlight.rs      # tree-sitter or syntect
│   │       ├── diff.rs           # similar crate wrapper
│   │       └── languages.rs      # Language detection
│   │
│   └── openpad-app/              # Makepad UI
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── app.rs            # Root LiveDesign + routing
│           ├── state/
│           │   ├── mod.rs
│           │   ├── store.rs      # Central reactive state
│           │   └── actions.rs    # Commands to async runtime
│           ├── widgets/
│           │   ├── mod.rs
│           │   ├── chat/
│           │   │   ├── mod.rs
│           │   │   ├── message_list.rs
│           │   │   ├── message_item.rs
│           │   │   ├── input_box.rs
│           │   │   └── part_view.rs    # Renders Part variants
│           │   ├── terminal/
│           │   │   ├── mod.rs
│           │   │   └── terminal_view.rs
│           │   ├── code/
│           │   │   ├── mod.rs
│           │   │   ├── code_view.rs    # Syntax highlighted
│           │   │   └── diff_view.rs    # Side-by-side diff
│           │   ├── sidebar/
│           │   │   ├── mod.rs
│           │   │   ├── session_list.rs
│           │   │   └── file_tree.rs
│           │   ├── toolbar/
│           │   │   ├── mod.rs
│           │   │   └── model_picker.rs
│           │   └── permission_card.rs  # Tool approval UI
│           └── theme/
│               ├── mod.rs
│               └── flexoki.rs          # Color scheme

Core Types (openpad-protocol)
rust// types/session.rs
#[derive(Debug, Clone, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: Option<String>,
    pub agent: String,
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub share: Option<SessionShare>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    User(UserMessage),
    Assistant(AssistantMessage),
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMessage {
    pub id: String,
    pub parts: Vec<Part>,
    pub time: MessageTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssistantMessage {
    pub id: String,
    pub parts: Vec<Part>,
    pub model: String,
    pub tokens: Option<TokenUsage>,
    pub time: MessageTime,
    pub error: Option<AssistantError>,
}

// types/part.rs - The core building blocks
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Part {
    Text(TextPart),
    File(FilePart),
    Tool(ToolPart),
    Reasoning(ReasoningPart),
    StepStart(StepStartPart),
    StepFinish(StepFinishPart),
    Snapshot(SnapshotPart),
    Patch(PatchPart),
    Retry(RetryPart),
    // Agent part (for subagent spawning)
    Agent(AgentPart),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolPart {
    pub id: String,
    pub tool: String,           // "bash", "edit", "read", etc.
    pub input: serde_json::Value,
    pub state: ToolState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ToolState {
    Pending,
    Running { time: TimeRange },
    Completed { time: TimeRange, output: serde_json::Value },
    Error { time: TimeRange, error: String },
}

Client Implementation
rust// client.rs
use reqwest::{Client as HttpClient, Url};
use tokio::sync::{broadcast, mpsc};
use futures_util::StreamExt;

pub struct OpenCodeClient {
    http: HttpClient,
    base_url: Url,
    directory: String,
    event_tx: broadcast::Sender<Event>,
}

impl OpenCodeClient {
    pub fn new(base_url: &str, directory: &str) -> Self {
        let (event_tx, _) = broadcast::channel(256);
        Self {
            http: HttpClient::new(),
            base_url: Url::parse(base_url).unwrap(),
            directory: directory.to_string(),
            event_tx,
        }
    }

    // Start SSE event subscription
    pub async fn subscribe(&self) -> Result<broadcast::Receiver<Event>> {
        let url = self.base_url.join("/event")?;
        let response = self.http
            .get(url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        let event_tx = self.event_tx.clone();
        
        // Spawn SSE reader task
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();
            
            while let Some(chunk) = stream.next().await {
                // Parse SSE format: "data: {...}\n\n"
                if let Ok(bytes) = chunk {
                    buffer.push_str(&String::from_utf8_lossy(&bytes));
                    
                    while let Some(idx) = buffer.find("\n\n") {
                        let line = &buffer[..idx];
                        if let Some(data) = line.strip_prefix("data: ") {
                            if let Ok(event) = serde_json::from_str::<Event>(data) {
                                let _ = event_tx.send(event);
                            }
                        }
                        buffer = buffer[idx + 2..].to_string();
                    }
                }
            }
        });

        Ok(self.event_tx.subscribe())
    }

    // Session operations
    pub async fn list_sessions(&self) -> Result<Vec<Session>> {
        let url = self.base_url.join("/session")?;
        let resp: Vec<Session> = self.http
            .get(url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn create_session(&self, agent: Option<&str>) -> Result<Session> {
        let url = self.base_url.join("/session")?;
        let body = serde_json::json!({ "agent": agent });
        let resp: Session = self.http
            .post(url)
            .query(&[("directory", &self.directory)])
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn send_prompt(
        &self,
        session_id: &str,
        parts: Vec<PartInput>,
        model: Option<&str>,
    ) -> Result<()> {
        let url = self.base_url.join(&format!("/session/{}/prompt", session_id))?;
        let body = serde_json::json!({
            "parts": parts,
            "model": model,
        });
        self.http
            .post(url)
            .query(&[("directory", &self.directory)])
            .json(&body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn respond_permission(
        &self,
        session_id: &str,
        permission_id: &str,
        approved: bool,
    ) -> Result<()> {
        let url = self.base_url.join(
            &format!("/session/{}/permission/{}/respond", session_id, permission_id)
        )?;
        let response = if approved { "yes" } else { "no" };
        self.http
            .post(url)
            .query(&[("directory", &self.directory)])
            .json(&serde_json::json!({ "response": response }))
            .send()
            .await?;
        Ok(())
    }

    pub async fn abort_session(&self, session_id: &str) -> Result<()> {
        let url = self.base_url.join(&format!("/session/{}/abort", session_id))?;
        self.http
            .post(url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;
        Ok(())
    }
}

State Management Bridge
rust// state/store.rs
use std::collections::HashMap;
use makepad_widgets::*;

#[derive(Default)]
pub struct AppState {
    // Data
    pub sessions: HashMap<String, Session>,
    pub active_session_id: Option<String>,
    pub messages: HashMap<String, Vec<Message>>,  // session_id -> messages
    pub pending_permissions: HashMap<String, Permission>,
    
    // UI state
    pub sidebar_open: bool,
    pub current_view: View,
    
    // Connection
    pub connected: bool,
    pub project_path: String,
}

pub enum View {
    Chat,
    Terminal,
    Settings,
}

// Actions sent to async runtime
pub enum Command {
    Connect { url: String, directory: String },
    CreateSession { agent: Option<String> },
    SendPrompt { session_id: String, text: String, files: Vec<PathBuf> },
    ApprovePermission { session_id: String, permission_id: String },
    DenyPermission { session_id: String, permission_id: String },
    AbortSession { session_id: String },
    SelectSession { session_id: String },
}

// Events received from async runtime (mapped from OpenCode SSE)
pub enum AppEvent {
    Connected,
    Disconnected,
    SessionsLoaded(Vec<Session>),
    SessionCreated(Session),
    SessionDeleted(String),
    MessageUpdated { session_id: String, message: Message },
    PartUpdated { session_id: String, message_id: String, part: Part },
    PermissionRequested(Permission),
    PermissionResolved(String),
    Error(String),
}

Makepad Widget Example (Chat)
rust// widgets/chat/message_list.rs
live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    
    MessageList = {{MessageList}} {
        walk: { width: Fill, height: Fill }
        layout: { flow: Down, spacing: 8, padding: 16 }
        
        list = <PortalList> {
            auto_tail: true  // Scroll to bottom on new messages
            
            UserMessage = <View> {
                walk: { width: Fill, height: Fit }
                layout: { flow: Right, align: { x: 1.0 } }
                
                bubble = <RoundedView> {
                    walk: { width: Fit, height: Fit }
                    draw_bg: { color: #2a4a6a }
                    layout: { padding: 12 }
                    
                    text = <Label> {
                        walk: { width: Fit, height: Fit }
                        draw_text: { color: #fff }
                    }
                }
            }
            
            AssistantMessage = <View> {
                walk: { width: Fill, height: Fit }
                layout: { flow: Down, spacing: 4 }
                
                // Parts rendered dynamically
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct MessageList {
    #[deref] view: View,
    #[rust] messages: Vec<Message>,
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let list = self.view.portal_list(id!(list));
        
        for (idx, message) in self.messages.iter().enumerate() {
            let item_id = match message {
                Message::User(_) => live_id!(UserMessage),
                Message::Assistant(_) => live_id!(AssistantMessage),
            };
            
            let item = list.item(cx, idx, item_id);
            self.draw_message(cx, &item, message);
            item.draw_all(cx, scope);
        }
        
        DrawStep::done()
    }
}

Key Crate Dependencies
toml# openpad-protocol/Cargo.toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
futures-util = "0.3"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"

# openpad-terminal/Cargo.toml
[dependencies]
portable-pty = "0.8"
vt100 = "0.15"                    # ANSI parser
tokio-tungstenite = "0.21"        # WebSocket for /pty

# openpad-syntax/Cargo.toml
[dependencies]
syntect = "5"                      # Syntax highlighting
similar = "2"                      # Diff algorithm
tree-sitter = "0.22"              # Optional: better parsing

# openpad-app/Cargo.toml
[dependencies]
makepad-widgets = "0.7"
openpad-protocol = { path = "../openpad-protocol" }
openpad-terminal = { path = "../openpad-terminal" }
openpad-syntax = { path = "../openpad-syntax" }

Next Steps

Start with openpad-protocol - Get the Rust client talking to OpenCode
Build minimal Makepad shell - Session list + chat view
Add SSE event handling - Real-time message updates
Permission cards - Critical for tool approval flow
Terminal integration - The hardest part
Diff views - For edit tool visualization

Want me to scaffold the initial openpad-protocol crate with full type definitions, or dive deeper into any specific component?