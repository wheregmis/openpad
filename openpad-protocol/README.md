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
