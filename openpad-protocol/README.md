# openpad-protocol

Comprehensive async Rust client library for the OpenCode server API.

## Features

- **Complete API coverage** - All OpenCode server endpoints implemented
- **Type-safe** - Full type safety with serde for all requests and responses
- **Real-time updates** - Server-Sent Events (SSE) subscription
- **Session management** - Create, update, delete, share sessions
- **File operations** - Search text, files, symbols; read files; get status
- **TUI control** - Programmatic control of OpenCode TUI interface
- **Error handling** - Typed errors with detailed messages

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
openpad-protocol = { path = "../openpad-protocol" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use openpad_protocol::{OpenCodeClient, PromptRequest, PartInput, ModelSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenCodeClient::new("http://localhost:4096");

    // Create a session
    let session = client.create_session().await?;

    // Send a prompt with model specification
    let prompt = PromptRequest {
        model: Some(ModelSpec {
            provider_id: "anthropic".to_string(),
            model_id: "claude-3-5-sonnet-20241022".to_string(),
        }),
        parts: vec![PartInput::text("Hello, world!")],
        no_reply: None,
    };
    
    let message = client.send_prompt_with_options(&session.id, prompt).await?;
    println!("Response: {:?}", message);

    Ok(())
}
```

## API Documentation

### Global APIs

```rust
// Check server health and version
let health = client.health().await?;
println!("Server version: {}", health.version);
```

### App APIs

```rust
// Write a log entry
client.log(LogRequest {
    service: "my-app".to_string(),
    level: "info".to_string(),
    message: "Operation completed".to_string(),
}).await?;

// List available agents
let agents = client.agents().await?;
```

### Project APIs

```rust
// List all projects
let projects = client.list_projects().await?;

// Get current project
let current = client.current_project().await?;
```

### Path APIs

```rust
// Get current path information
let path_info = client.get_path().await?;
```

### Config APIs

```rust
// Get configuration
let config = client.get_config().await?;

// Get providers and models
let providers = client.get_providers().await?;
for provider in providers.providers {
    println!("Provider: {:?}", provider);
}
```

### Session APIs

#### Basic Operations
```rust
// List all sessions
let sessions = client.list_sessions().await?;

// Create session with options
let session = client.create_session_with_options(SessionCreateRequest {
    title: Some("My Session".to_string()),
}).await?;

// Get session details
let session = client.get_session(&session_id).await?;

// Update session
let updated = client.update_session(&session_id, SessionUpdateRequest {
    title: Some("New Title".to_string()),
}).await?;

// Delete session
client.delete_session(&session_id).await?;
```

#### Advanced Operations
```rust
// Get child sessions
let children = client.get_session_children(&session_id).await?;

// Initialize session (create AGENTS.md)
client.init_session(&session_id, SessionInitRequest {
    force: Some(false),
}).await?;

// Abort running session
client.abort_session(&session_id).await?;

// Share session
let shared = client.share_session(&session_id).await?;

// Unshare session
let unshared = client.unshare_session(&session_id).await?;

// Summarize session
client.summarize_session(&session_id, SessionSummarizeRequest {
    force: Some(false),
}).await?;
```

#### Messages
```rust
// List all messages in a session
let messages = client.list_messages(&session_id).await?;

// Get specific message
let message = client.get_message(&session_id, &message_id).await?;

// Send prompt with context (no AI response)
let prompt = PromptRequest {
    model: None,
    parts: vec![PartInput::text("Context information")],
    no_reply: Some(true),
};
client.send_prompt_with_options(&session_id, prompt).await?;

// Send command
let response = client.send_command(&session_id, CommandRequest {
    command: "ls".to_string(),
    args: Some(vec!["-la".to_string()]),
}).await?;

// Send shell command
let output = client.send_shell(&session_id, ShellRequest {
    command: "echo hello".to_string(),
}).await?;

// Revert a message
let reverted = client.revert_message(&session_id, RevertRequest {
    message_id: message_id.to_string(),
}).await?;

// Restore reverted messages
let restored = client.unrevert_session(&session_id).await?;

// Respond to permission request
client.respond_to_permission(&session_id, &permission_id, PermissionResponse {
    allow: true,
}).await?;
```

### File & Find APIs

```rust
// Search for text in files
let results = client.search_text(TextSearchRequest {
    pattern: "function.*opencode".to_string(),
}).await?;

// Find files and directories
let files = client.search_files(FilesSearchRequest {
    query: "*.rs".to_string(),
    type_filter: Some("file".to_string()),
    directory: None,
    limit: Some(50),
}).await?;

// Search for symbols
let symbols = client.search_symbols(SymbolsSearchRequest {
    query: "OpenCodeClient".to_string(),
}).await?;

// Read a file
let content = client.read_file(FileReadRequest {
    path: "src/main.rs".to_string(),
}).await?;
println!("Type: {}, Content: {}", content.type_name, content.content);

// Get file status (git status)
let files = client.get_file_status(Some(FileStatusRequest {
    path: Some("src/".to_string()),
})).await?;
```

### TUI APIs

```rust
// Append text to the prompt
client.append_prompt(AppendPromptRequest {
    text: "Add this to the prompt".to_string(),
}).await?;

// Open various dialogs
client.open_help().await?;
client.open_sessions().await?;
client.open_themes().await?;
client.open_models().await?;

// Prompt control
client.submit_prompt().await?;
client.clear_prompt().await?;

// Execute a command in TUI
client.execute_command(ExecuteCommandRequest {
    command: "refresh".to_string(),
}).await?;

// Show toast notification
client.show_toast(ShowToastRequest {
    message: "Task completed!".to_string(),
    variant: Some("success".to_string()),
}).await?;
```

### Auth APIs

```rust
// Set authentication credentials
client.set_auth("anthropic", AuthSetRequest {
    auth_type: "api".to_string(),
    key: "your-api-key".to_string(),
}).await?;
```

### Event Subscription (SSE)

```rust
// Subscribe to real-time events
let mut events = client.subscribe().await?;

// Handle events
tokio::spawn(async move {
    while let Ok(event) = events.recv().await {
        match event {
            Event::SessionCreated(session) => {
                println!("New session: {}", session.id);
            }
            Event::MessageUpdated { session_id, message } => {
                println!("Message updated in session {}", session_id);
            }
            Event::PartUpdated { session_id, message_id, part_index, part } => {
                println!("Part {} updated in message {}", part_index, message_id);
            }
            Event::SessionDeleted(id) => {
                println!("Session deleted: {}", id);
            }
            Event::Error(err) => {
                eprintln!("Error: {}", err);
            }
            Event::Unknown(event_type) => {
                println!("Unknown event: {}", event_type);
            }
        }
    }
});
```

## Configuration

The client automatically uses the current working directory as the `directory` parameter for all requests. You can override this:

```rust
let client = OpenCodeClient::new("http://localhost:4096")
    .with_directory("/path/to/project");
```

## Error Handling

All methods return `Result<T, Error>` where `Error` is defined in the error module:

```rust
use openpad_protocol::Error;

match client.health().await {
    Ok(health) => println!("Server is healthy: {}", health.version),
    Err(Error::Http(e)) => eprintln!("HTTP error: {}", e),
    Err(Error::Connection(e)) => eprintln!("Connection error: {}", e),
    Err(Error::InvalidResponse(e)) => eprintln!("Invalid response: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Type Definitions

See [types.rs](src/types.rs) for complete type definitions including:

- Session types (Session, SessionTime, Message, Part)
- Request/response types for all endpoints
- Event types for SSE subscription
- Configuration and provider types
- File and search result types

## Dependencies

- **reqwest** - HTTP client with JSON and streaming support
- **tokio** - Async runtime
- **serde** / **serde_json** - Serialization
- **chrono** - DateTime handling
- **thiserror** - Error type derivation
- **futures-util** - Async utilities for streaming

## Examples

Check the main openpad-app crate for a complete example of using this library in a GUI application.
