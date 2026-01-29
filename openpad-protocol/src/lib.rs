//! OpenCode protocol client library.
//!
//! This crate provides a complete, type-safe async Rust client for the OpenCode server API.
//!
//! ## Features
//!
//! - Complete API coverage for all OpenCode endpoints
//! - Type-safe requests and responses with serde
//! - Real-time event subscription via Server-Sent Events
//! - Comprehensive error handling
//!
//! ## Quick Start
//!
//! ```no_run
//! use openpad_protocol::{OpenCodeClient, PromptRequest, PartInput};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = OpenCodeClient::new("http://localhost:4096");
//!     
//!     // Check server health
//!     let health = client.health().await?;
//!     println!("Server version: {}", health.version);
//!     
//!     // Create a session
//!     let session = client.create_session().await?;
//!     
//!     // Send a prompt
//!     client.send_prompt(&session.id, "Hello!").await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! See the [README](../README.md) for detailed API documentation.

pub mod client;
pub mod error;
pub mod types;

pub use client::OpenCodeClient;
pub use error::{Error, Result};
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_types_are_public() {
        // Global types
        let _: HealthResponse;
        
        // App types
        let _: LogRequest;
        let _: Agent;
        
        // Project types
        let _: Project;
        
        // Path types
        let _: PathInfo;
        
        // Config types
        let _: Config;
        let _: Provider;
        let _: Model;
        let _: ProvidersResponse;
        
        // File/Find types
        let _: TextSearchRequest;
        let _: TextSearchResult;
        let _: FilesSearchRequest;
        let _: SymbolsSearchRequest;
        let _: Symbol;
        let _: Location;
        let _: FileReadRequest;
        let _: FileReadResponse;
        let _: FileStatusRequest;
        let _: File;
        
        // TUI types
        let _: AppendPromptRequest;
        let _: ExecuteCommandRequest;
        let _: ShowToastRequest;
        
        // Auth types
        let _: AuthSetRequest;
        
        // Session types
        let _: Session;
        let _: SessionTime;
        let _: SessionCreateRequest;
        let _: SessionUpdateRequest;
        let _: SessionInitRequest;
        let _: SessionSummarizeRequest;
        let _: MessageWithParts;
        let _: ModelSpec;
        let _: PromptRequest;
        let _: CommandRequest;
        let _: ShellRequest;
        let _: RevertRequest;
        let _: PermissionResponse;
        
        // Message types
        let _: Message;
        let _: MessageTime;
        let _: Part;
        let _: PartInput;
        
        // Event types
        let _: Event;
        
        // Error types
        let _: Error;
    }

    #[test]
    fn test_client_can_be_created() {
        let client = OpenCodeClient::new("http://localhost:4096");
        let client_with_dir = client.with_directory("/path/to/project");
        // Just ensure these compile
        drop(client_with_dir);
    }
}

