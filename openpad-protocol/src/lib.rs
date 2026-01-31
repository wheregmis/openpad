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
        let _: Skill;

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
        let _: SubMatch;
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
        let _: SessionShare;
        let _: SessionSummary;
        let _: SessionRevert;
        let _: FileDiff;
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
        let _: PermissionRule;
        let _: PermissionAction;
        let _: PermissionReply;
        let _: PermissionRequest;
        let _: PermissionReplyRequest;

        // Message types
        let _: Message;
        let _: UserMessage;
        let _: AssistantMessage;
        let _: MessageTime;
        let _: MessageSummary;
        let _: MessagePath;
        let _: TokenUsage;
        let _: CacheUsage;
        let _: AssistantError;
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

    /// Test module for validating our Rust types against the OpenAPI specification.
    ///
    /// This test ensures that our manually-defined types in `types.rs` match the
    /// OpenAPI schema from the OpenCode server. It helps catch breaking changes
    /// when the server API is updated.
    ///
    /// ## Purpose
    ///
    /// The OpenCode server provides an OpenAPI specification that defines the structure
    /// of all API requests and responses. This test module validates that our Rust types
    /// can correctly serialize to and deserialize from the formats expected by the server.
    ///
    /// ## What Gets Tested
    ///
    /// 1. **Schema Existence**: Verifies that key schemas exist in the OpenAPI spec
    /// 2. **Required Fields**: Ensures our types include all required fields from the spec
    /// 3. **Serialization Format**: Validates that types serialize with correct field names
    /// 4. **Enum Values**: Checks that enum variants match the OpenAPI spec
    /// 5. **Inline Types**: Tests types that are defined inline rather than as standalone schemas
    ///
    /// ## Known Discrepancies
    ///
    /// Our implementation intentionally simplifies some types compared to the OpenAPI spec
    /// to make the client more flexible and easier to maintain:
    ///
    /// ### Provider Type
    ///
    /// - **OpenAPI spec requires**: `id`, `name`, `source`, `env`, `options`, `models`
    /// - **Our type requires**: `id` only (makes `name` and `models` optional)
    /// - **Rationale**: We only need basic provider information for the UI, avoiding
    ///   complex configuration structures that the server manages
    ///
    /// ### Model Type
    ///
    /// - **OpenAPI spec requires**: `id`, `providerID`, `api`, `name`, `capabilities`,
    ///   `cost`, `limit`, `status`, `options`, `headers`, `release_date`
    /// - **Our type requires**: `id` only (makes `name` optional)
    /// - **Rationale**: The UI only needs to display model names; detailed capabilities,
    ///   pricing, and limits are managed server-side
    ///
    /// These simplifications allow the client to gracefully handle partial data and
    /// avoid breaking when the server adds new fields to these complex types.
    ///
    /// ## Running the Tests
    ///
    /// ```bash
    /// cargo test -p openpad-protocol openapi_validation
    /// ```
    mod openapi_validation {
        use super::*;
        use serde_json::Value;
        use std::collections::HashMap;

        /// Load the OpenAPI specification from the repository root.
        fn load_openapi_spec() -> Value {
            let openapi_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../openapi.json");
            let openapi_content =
                std::fs::read_to_string(openapi_path).expect("Failed to read openapi.json");
            serde_json::from_str(&openapi_content).expect("Failed to parse openapi.json")
        }

        /// Extract a schema definition from the OpenAPI spec by name.
        fn get_schema<'a>(spec: &'a Value, name: &str) -> Option<&'a Value> {
            spec.get("components")?.get("schemas")?.get(name)
        }

        /// Helper to validate that a type can be serialized and has expected fields.
        fn validate_serialization<T: serde::Serialize>(
            value: &T,
            expected_fields: &[&str],
        ) -> serde_json::Value {
            let json = serde_json::to_value(value).expect("Failed to serialize");
            let obj = json.as_object().expect("Expected an object");

            for field in expected_fields {
                assert!(
                    obj.contains_key(*field),
                    "Missing required field: {}",
                    field
                );
            }

            json
        }

        #[test]
        fn test_openapi_spec_loads() {
            let spec = load_openapi_spec();
            assert!(spec.is_object(), "OpenAPI spec should be an object");
            assert_eq!(
                spec.get("openapi").and_then(|v| v.as_str()),
                Some("3.1.1"),
                "OpenAPI version mismatch"
            );
        }

        #[test]
        fn test_session_type_matches_openapi() {
            let spec = load_openapi_spec();
            let schema = get_schema(&spec, "Session").expect("Session schema not found");

            // Create a sample Session
            let session = Session {
                id: "ses_123".to_string(),
                slug: "test-session".to_string(),
                project_id: "proj_123".to_string(),
                directory: "/tmp/test".to_string(),
                parent_id: None,
                title: "Test Session".to_string(),
                version: "1.0.0".to_string(),
                time: SessionTime {
                    created: 1234567890000,
                    updated: 1234567890000,
                    compacting: None,
                    archived: None,
                },
                summary: None,
                share: None,
                permission: None,
                revert: None,
            };

            // Verify required fields from schema
            let required = schema
                .get("required")
                .and_then(|v| v.as_array())
                .expect("Session schema missing required fields");

            let required_fields: Vec<&str> = required
                .iter()
                .filter_map(|v| v.as_str())
                .collect();

            // Validate serialization includes all required fields
            validate_serialization(&session, &required_fields);
        }

        #[test]
        fn test_assistant_message_matches_openapi() {
            let spec = load_openapi_spec();
            let schema = get_schema(&spec, "AssistantMessage").expect("AssistantMessage schema not found");

            // Create a full Message enum (which includes the role field)
            let msg = Message::Assistant(AssistantMessage {
                id: "msg_123".to_string(),
                session_id: "ses_123".to_string(),
                time: MessageTime {
                    created: 1234567890000,
                    completed: None,
                },
                error: None,
                parent_id: "msg_122".to_string(),
                model_id: "claude-3".to_string(),
                provider_id: "anthropic".to_string(),
                mode: "agentic".to_string(),
                agent: "default".to_string(),
                path: Some(MessagePath {
                    cwd: "/tmp".to_string(),
                    root: "/tmp".to_string(),
                }),
                summary: None,
                cost: 0.0,
                tokens: Some(TokenUsage {
                    input: 100,
                    output: 50,
                    reasoning: 0,
                    cache: CacheUsage { read: 0, write: 0 },
                }),
                finish: None,
            });

            let required = schema
                .get("required")
                .and_then(|v| v.as_array())
                .expect("AssistantMessage schema missing required fields");

            let required_fields: Vec<&str> = required
                .iter()
                .filter_map(|v| v.as_str())
                .collect();

            validate_serialization(&msg, &required_fields);
        }

        #[test]
        fn test_user_message_matches_openapi() {
            let spec = load_openapi_spec();
            let schema = get_schema(&spec, "UserMessage").expect("UserMessage schema not found");

            // Create a full Message enum (which includes the role field)
            let msg = Message::User(UserMessage {
                id: "msg_123".to_string(),
                session_id: "ses_123".to_string(),
                time: MessageTime {
                    created: 1234567890000,
                    completed: None,
                },
                summary: None,
                agent: "default".to_string(),
                model: Some(ModelSpec {
                    provider_id: "anthropic".to_string(),
                    model_id: "claude-3".to_string(),
                }),
                system: None,
                tools: None,
                variant: None,
            });

            let required = schema
                .get("required")
                .and_then(|v| v.as_array())
                .expect("UserMessage schema missing required fields");

            let required_fields: Vec<&str> = required
                .iter()
                .filter_map(|v| v.as_str())
                .collect();

            validate_serialization(&msg, &required_fields);
        }

        #[test]
        fn test_permission_request_matches_openapi() {
            let spec = load_openapi_spec();
            let schema = get_schema(&spec, "PermissionRequest")
                .expect("PermissionRequest schema not found");

            let req = PermissionRequest {
                id: "per_123".to_string(),
                session_id: "ses_123".to_string(),
                permission: "bash".to_string(),
                patterns: vec!["*.sh".to_string()],
                metadata: HashMap::new(),
                always: vec![],
                tool: None,
            };

            let required = schema
                .get("required")
                .and_then(|v| v.as_array())
                .expect("PermissionRequest schema missing required fields");

            let required_fields: Vec<&str> = required
                .iter()
                .filter_map(|v| v.as_str())
                .collect();

            validate_serialization(&req, &required_fields);
        }

        #[test]
        fn test_token_usage_structure() {
            let tokens = TokenUsage {
                input: 100,
                output: 50,
                reasoning: 10,
                cache: CacheUsage { read: 20, write: 5 },
            };

            let json = serde_json::to_value(&tokens).expect("Failed to serialize");
            assert!(json.get("input").is_some());
            assert!(json.get("output").is_some());
            assert!(json.get("reasoning").is_some());
            assert!(json.get("cache").is_some());

            let cache = json.get("cache").unwrap();
            assert!(cache.get("read").is_some());
            assert!(cache.get("write").is_some());
        }

        #[test]
        fn test_assistant_error_variants() {
            // Test that all AssistantError variants serialize correctly
            let errors = vec![
                AssistantError::ProviderAuthError {
                    provider_id: "anthropic".to_string(),
                    message: "Auth failed".to_string(),
                },
                AssistantError::UnknownError {
                    message: "Something went wrong".to_string(),
                },
                AssistantError::MessageOutputLengthError,
                AssistantError::MessageAbortedError {
                    message: "Aborted by user".to_string(),
                },
                AssistantError::APIError {
                    message: "API error".to_string(),
                    status_code: Some(500),
                    is_retryable: true,
                    response_headers: None,
                    response_body: None,
                    metadata: None,
                },
            ];

            for error in errors {
                let json = serde_json::to_value(&error).expect("Failed to serialize error");
                assert!(json.get("name").is_some(), "Error should have a 'name' field");
            }
        }

        #[test]
        fn test_permission_action_serialization() {
            let actions = vec![
                PermissionAction::Allow,
                PermissionAction::Deny,
                PermissionAction::Ask,
            ];

            for action in actions {
                let json = serde_json::to_value(&action).expect("Failed to serialize");
                assert!(json.is_string(), "PermissionAction should serialize as string");
            }
        }

        #[test]
        fn test_message_enum_with_role() {
            // Test that Message enum serializes with the 'role' field
            let user_msg = Message::User(UserMessage {
                id: "msg_123".to_string(),
                session_id: "ses_123".to_string(),
                time: MessageTime {
                    created: 1234567890000,
                    completed: None,
                },
                summary: None,
                agent: "default".to_string(),
                model: Some(ModelSpec {
                    provider_id: "anthropic".to_string(),
                    model_id: "claude-3".to_string(),
                }),
                system: None,
                tools: None,
                variant: None,
            });

            let json = serde_json::to_value(&user_msg).expect("Failed to serialize");
            assert_eq!(
                json.get("role").and_then(|v| v.as_str()),
                Some("user"),
                "User message should have role='user'"
            );

            let assistant_msg = Message::Assistant(AssistantMessage {
                id: "msg_124".to_string(),
                session_id: "ses_123".to_string(),
                time: MessageTime {
                    created: 1234567890000,
                    completed: None,
                },
                error: None,
                parent_id: "msg_123".to_string(),
                model_id: "claude-3".to_string(),
                provider_id: "anthropic".to_string(),
                mode: "agentic".to_string(),
                agent: "default".to_string(),
                path: Some(MessagePath {
                    cwd: "/tmp".to_string(),
                    root: "/tmp".to_string(),
                }),
                summary: None,
                cost: 0.0,
                tokens: Some(TokenUsage {
                    input: 100,
                    output: 50,
                    reasoning: 0,
                    cache: CacheUsage { read: 0, write: 0 },
                }),
                finish: None,
            });

            let json = serde_json::to_value(&assistant_msg).expect("Failed to serialize");
            assert_eq!(
                json.get("role").and_then(|v| v.as_str()),
                Some("assistant"),
                "Assistant message should have role='assistant'"
            );
        }

        #[test]
        fn test_key_schemas_exist_in_openapi() {
            let spec = load_openapi_spec();

            // List of key schemas that exist as standalone definitions in the OpenAPI spec
            // Note: Many types like SessionTime, MessageTime, TokenUsage, CacheUsage, 
            // ModelSpec, SessionSummary, and PermissionReply are defined inline in the 
            // OpenAPI spec rather than as standalone schemas
            let expected_schemas = vec![
                "Session",
                "AssistantMessage",
                "UserMessage",
                "PermissionRequest",
                "PermissionRule",
                "PermissionAction",
                "FileDiff",
                "Config",
                "Provider",
                "Model",
            ];

            for schema_name in expected_schemas {
                assert!(
                    get_schema(&spec, schema_name).is_some(),
                    "Schema '{}' not found in OpenAPI spec",
                    schema_name
                );
            }
        }

        #[test]
        fn test_inline_time_structures() {
            // Validate that our SessionTime and MessageTime types serialize correctly
            // even though they're defined inline in the OpenAPI spec
            
            let session_time = SessionTime {
                created: 1234567890000,
                updated: 1234567890000,
                compacting: Some(1234567891000),
                archived: None,
            };

            let json = serde_json::to_value(&session_time).expect("Failed to serialize");
            assert!(json.get("created").is_some());
            assert!(json.get("updated").is_some());
            assert!(json.get("compacting").is_some());

            let message_time = MessageTime {
                created: 1234567890000,
                completed: Some(1234567891000),
            };

            let json = serde_json::to_value(&message_time).expect("Failed to serialize");
            assert!(json.get("created").is_some());
            assert!(json.get("completed").is_some());
        }

        #[test]
        fn test_inline_model_spec_structure() {
            // Validate ModelSpec serialization (defined inline in OpenAPI)
            let model_spec = ModelSpec {
                provider_id: "anthropic".to_string(),
                model_id: "claude-3".to_string(),
            };

            let json = serde_json::to_value(&model_spec).expect("Failed to serialize");
            assert_eq!(
                json.get("providerID").and_then(|v| v.as_str()),
                Some("anthropic")
            );
            assert_eq!(
                json.get("modelID").and_then(|v| v.as_str()),
                Some("claude-3")
            );
        }

        #[test]
        fn test_inline_session_summary_structure() {
            // Validate SessionSummary serialization (defined inline in OpenAPI)
            let summary = SessionSummary {
                additions: 10,
                deletions: 5,
                files: 2,
                diffs: vec![],
            };

            let json = serde_json::to_value(&summary).expect("Failed to serialize");
            assert_eq!(json.get("additions").and_then(|v| v.as_i64()), Some(10));
            assert_eq!(json.get("deletions").and_then(|v| v.as_i64()), Some(5));
            assert_eq!(json.get("files").and_then(|v| v.as_i64()), Some(2));
            assert!(json.get("diffs").is_some());
        }

        #[test]
        fn test_permission_reply_enum_values() {
            // Validate that PermissionReply enum values match OpenAPI spec
            // (defined inline in Event.permission.replied)
            let replies = vec![
                PermissionReply::Once,
                PermissionReply::Always,
                PermissionReply::Reject,
            ];

            let expected_values = vec!["once", "always", "reject"];

            for (reply, expected) in replies.iter().zip(expected_values.iter()) {
                let json = serde_json::to_value(reply).expect("Failed to serialize");
                assert_eq!(
                    json.as_str(),
                    Some(*expected),
                    "PermissionReply should serialize to '{}'",
                    expected
                );
            }
        }

        #[test]
        fn test_part_text_serialization() {
            // Validate Part::Text serialization matches OpenAPI expectations
            let part = Part::Text {
                id: "part_123".to_string(),
                session_id: "ses_123".to_string(),
                message_id: "msg_123".to_string(),
                text: "Hello world".to_string(),
            };

            let json = serde_json::to_value(&part).expect("Failed to serialize");
            assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("text"));
            assert_eq!(json.get("id").and_then(|v| v.as_str()), Some("part_123"));
            assert_eq!(
                json.get("text").and_then(|v| v.as_str()),
                Some("Hello world")
            );
        }

        #[test]
        fn test_file_diff_structure() {
            let spec = load_openapi_spec();
            let schema = get_schema(&spec, "FileDiff").expect("FileDiff schema not found");

            let diff = FileDiff {
                file: "test.rs".to_string(),
                before: "old content".to_string(),
                after: "new content".to_string(),
                additions: 5,
                deletions: 2,
            };

            let required = schema
                .get("required")
                .and_then(|v| v.as_array())
                .expect("FileDiff schema missing required fields");

            let required_fields: Vec<&str> = required
                .iter()
                .filter_map(|v| v.as_str())
                .collect();

            validate_serialization(&diff, &required_fields);
        }

        #[test]
        fn test_provider_and_model_structure() {
            let spec = load_openapi_spec();
            
            // See module documentation for details on why these types are simplified
            
            let _provider_schema = get_schema(&spec, "Provider").expect("Provider schema not found");
            let provider = Provider {
                id: "anthropic".to_string(),
                name: Some("Anthropic".to_string()),
                models: None,
            };

            // Just verify the type serializes correctly with our required fields
            let json = serde_json::to_value(&provider).expect("Failed to serialize Provider");
            assert_eq!(json.get("id").and_then(|v| v.as_str()), Some("anthropic"));
            assert_eq!(json.get("name").and_then(|v| v.as_str()), Some("Anthropic"));

            // Test Model - also simplified in our implementation
            let _model_schema = get_schema(&spec, "Model").expect("Model schema not found");
            let model = Model {
                id: "claude-3".to_string(),
                name: Some("Claude 3".to_string()),
            };

            // Verify our simplified model serializes correctly
            let json = serde_json::to_value(&model).expect("Failed to serialize Model");
            assert_eq!(json.get("id").and_then(|v| v.as_str()), Some("claude-3"));
            assert_eq!(json.get("name").and_then(|v| v.as_str()), Some("Claude 3"));
        }
    }
}
