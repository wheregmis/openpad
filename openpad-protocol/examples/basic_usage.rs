//! Basic usage example of the OpenCode protocol client.
//!
//! This example demonstrates:
//! - Creating a client
//! - Checking server health
//! - Creating a session
//! - Sending prompts
//! - Subscribing to events
//!
//! To run this example, make sure an OpenCode server is running on localhost:4096:
//! ```bash
//! cargo run --example basic_usage
//! ```

use openpad_protocol::{
    Event, LogRequest, ModelSpec, OpenCodeClient, Part, PartInput, PromptRequest,
    SessionCreateRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OpenCode Protocol Client - Basic Usage Example");
    println!("===============================================\n");

    // Create the client
    let client = OpenCodeClient::new("http://localhost:4096");
    println!("✓ Client created");

    // 1. Check server health
    println!("\n1. Checking server health...");
    match client.health().await {
        Ok(health) => {
            println!("   ✓ Server is healthy");
            println!("   Version: {}", health.version);
        }
        Err(e) => {
            eprintln!("   ✗ Failed to connect to server: {}", e);
            eprintln!("   Make sure OpenCode server is running on http://localhost:4096");
            return Err(e.into());
        }
    }

    // 2. Log a message
    println!("\n2. Writing log entry...");
    client
        .log(LogRequest {
            service: "basic_usage_example".to_string(),
            level: "info".to_string(),
            message: "Starting basic usage example".to_string(),
        })
        .await?;
    println!("   ✓ Log entry written");

    // 3. Create a session
    println!("\n3. Creating session...");
    let session = client
        .create_session_with_options(SessionCreateRequest {
            title: Some("Example Session".to_string()),
            parent_id: None,
            permission: None,
        })
        .await?;
    println!("   ✓ Session created");
    println!("   ID: {}", session.id);
    println!("   Title: {}", session.title);

    // 4. Subscribe to events
    println!("\n4. Subscribing to events...");
    let mut events = client.subscribe().await?;
    println!("   ✓ Subscribed to server events");

    // 5. Send a prompt
    println!("\n5. Sending prompt...");
    let prompt = PromptRequest {
        model: Some(ModelSpec {
            provider_id: "anthropic".to_string(),
            model_id: "claude-3-5-sonnet-20241022".to_string(),
        }),
        parts: vec![PartInput::text(
            "Hello! Can you help me with Rust programming?",
        )],
        no_reply: None,
    };

    match client.send_prompt_with_options(&session.id, prompt).await {
        Ok(message) => {
            println!("   ✓ Prompt sent");
            println!("   Message ID: {}", message.id());
        }
        Err(e) => {
            eprintln!("   ✗ Failed to send prompt: {}", e);
        }
    }

    // 6. Listen for events (for a short time)
    println!("\n6. Listening for events (10 events or 30 seconds)...");
    let mut count = 0;
    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(30));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            result = events.recv() => {
                match result {
                    Ok(event) => {
                        count += 1;
                        match event {
                            Event::SessionCreated(s) => {
                                println!("   → Session created: {}", s.id);
                            }
                            Event::SessionUpdated(s) => {
                                println!("   → Session updated: {}", s.id);
                            }
                            Event::SessionDeleted(s) => {
                                println!("   → Session deleted: {}", s.id);
                            }
                            Event::MessageUpdated(message) => {
                                println!("   → Message updated: {} (session {})", message.id(), message.session_id());
                            }
                            Event::MessageRemoved { session_id, message_id } => {
                                println!("   → Message {} removed from session {}", message_id, session_id);
                            }
                            Event::PartUpdated { part, delta } => {
                                println!("   → Part updated (delta: {})", delta.is_some());
                                match part {
                                    Part::Text { text, .. } => println!("      Text: {}...", text.chars().take(50).collect::<String>()),
                                    Part::Unknown => println!("      Unknown part type"),
                                }
                            }
                            Event::PartRemoved { session_id, message_id, part_id } => {
                                println!("   → Part {} removed from message {} (session {})",
                                    part_id, message_id, session_id);
                            }
                            Event::SessionError { session_id, error } => {
                                eprintln!("   → Session {} error: {:?}", session_id, error);
                            }
                            Event::Error(err) => {
                                eprintln!("   → Error event: {}", err);
                            }
                            Event::Unknown(event_type) => {
                                println!("   → Unknown event: {}", event_type);
                            }
                        }

                        if count >= 10 {
                            println!("   Received 10 events, stopping...");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("   ✗ Error receiving event: {}", e);
                        break;
                    }
                }
            }
            _ = &mut timeout => {
                println!("   Timeout after 30 seconds");
                break;
            }
        }
    }

    // 7. List sessions
    println!("\n7. Listing all sessions...");
    let sessions = client.list_sessions().await?;
    println!("   Found {} session(s)", sessions.len());
    for s in &sessions {
        println!("   - {} ({})", s.id, s.title);
    }

    println!("\n✓ Example completed successfully!");
    Ok(())
}
