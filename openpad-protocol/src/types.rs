//! Type definitions for the OpenCode API.
//!
//! This module contains all request and response types used by the OpenCode server API,
//! organized by API category.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Global API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthResponse {
    pub healthy: bool,
    pub version: String,
}

// ============================================================================
// App API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogRequest {
    pub service: String,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Agent {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

// ============================================================================
// Project API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
}

// ============================================================================
// Path API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PathInfo {
    pub path: String,
    #[serde(default)]
    pub exists: Option<bool>,
}

// ============================================================================
// Config API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Provider {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub models: Option<Vec<Model>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Model {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProvidersResponse {
    pub providers: Vec<Provider>,
    #[serde(default)]
    pub default: HashMap<String, String>,
}

// ============================================================================
// File/Find API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextSearchRequest {
    pub pattern: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextSearchResult {
    pub path: String,
    pub lines: String,
    pub line_number: usize,
    pub absolute_offset: usize,
    #[serde(default)]
    pub submatches: Vec<SubMatch>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubMatch {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilesSearchRequest {
    pub query: String,
    #[serde(default, rename = "type")]
    pub type_filter: Option<String>,  // "file" or "directory"
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,  // 1-200
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolsSearchRequest {
    pub query: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Symbol {
    pub name: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    pub path: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileReadRequest {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileReadResponse {
    #[serde(rename = "type")]
    pub type_name: String,  // "raw" or "patch"
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileStatusRequest {
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub path: String,
    #[serde(default)]
    pub status: Option<String>,
}

// ============================================================================
// TUI API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppendPromptRequest {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecuteCommandRequest {
    pub command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShowToastRequest {
    pub message: String,
    #[serde(default)]
    pub variant: Option<String>,  // e.g., "success", "error", "info"
}

// ============================================================================
// Auth API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthSetRequest {
    #[serde(rename = "type")]
    pub auth_type: String,  // "api"
    pub key: String,
}

// ============================================================================
// Permission API types
// ============================================================================

/// A permission rule that controls access to specific operations.
///
/// Permission rules use pattern matching to determine whether an operation
/// should be allowed, denied, or require user confirmation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionRule {
    /// The permission type being controlled (e.g., "read", "write", "bash")
    pub permission: String,
    /// Pattern to match against (e.g., "*.rs", "/home/user/*")
    pub pattern: String,
    /// Action to take when the pattern matches
    pub action: PermissionAction,
}

/// Action to take for a permission request.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionAction {
    /// Automatically allow the operation
    Allow,
    /// Automatically deny the operation
    Deny,
    /// Ask the user for confirmation
    Ask,
}

/// A collection of permission rules for a session.
///
/// Rules are evaluated in order, with the first matching rule determining
/// the action to take.
pub type PermissionRuleset = Vec<PermissionRule>;

// ============================================================================
// Session API types
// ============================================================================

/// Session timing information with millisecond-precision timestamps.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionTime {
    /// When the session was created (milliseconds since Unix epoch)
    pub created: i64,
    /// When the session was last updated (milliseconds since Unix epoch)
    pub updated: i64,
    /// When the session started compacting (optional, milliseconds since Unix epoch)
    #[serde(default)]
    pub compacting: Option<i64>,
    /// When the session was archived (optional, milliseconds since Unix epoch)
    #[serde(default)]
    pub archived: Option<i64>,
}

/// Summary of changes made during a session.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionSummary {
    /// Total lines added across all files
    pub additions: i64,
    /// Total lines deleted across all files
    pub deletions: i64,
    /// Number of files modified
    pub files: i64,
    /// Detailed diff information for each file
    pub diffs: Vec<FileDiff>,
}

/// Diff information for a single file.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileDiff {
    /// Path to the file
    pub file: String,
    /// Content before changes
    pub before: String,
    /// Content after changes
    pub after: String,
    /// Number of lines added
    pub additions: i64,
    /// Number of lines deleted
    pub deletions: i64,
}

/// Share settings for a session.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionShare {
    /// URL where the session can be viewed publicly
    pub url: String,
}

/// Information about a reverted state in a session.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionRevert {
    /// ID of the message that was reverted
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// ID of the specific part that was reverted (optional)
    #[serde(default, rename = "partID")]
    pub part_id: Option<String>,
    /// Snapshot of the session state at the revert point (optional)
    #[serde(default)]
    pub snapshot: Option<String>,
    /// Diff showing what was reverted (optional)
    #[serde(default)]
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: String,
    pub slug: String,
    #[serde(rename = "projectID")]
    pub project_id: String,
    pub directory: String,
    #[serde(default, rename = "parentID")]
    pub parent_id: Option<String>,
    pub title: String,
    pub version: String,
    pub time: SessionTime,
    #[serde(default)]
    pub summary: Option<SessionSummary>,
    #[serde(default)]
    pub share: Option<SessionShare>,
    #[serde(default)]
    pub permission: Option<PermissionRuleset>,
    #[serde(default)]
    pub revert: Option<SessionRevert>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionCreateRequest {
    #[serde(default, rename = "parentID")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub permission: Option<PermissionRuleset>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionUpdateRequest {
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionInitRequest {
    #[serde(default)]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionSummarizeRequest {
    #[serde(default)]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageWithParts {
    pub info: Message,
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelSpec {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PromptRequest {
    #[serde(default)]
    pub model: Option<ModelSpec>,
    pub parts: Vec<PartInput>,
    #[serde(default, rename = "noReply")]
    pub no_reply: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandRequest {
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShellRequest {
    pub command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevertRequest {
    pub message_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionResponse {
    pub allow: bool,
}

/// Message timing information with millisecond-precision timestamps.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageTime {
    /// When the message was created (milliseconds since Unix epoch)
    pub created: i64,
    /// When the message was completed (milliseconds since Unix epoch, optional)
    #[serde(default)]
    pub completed: Option<i64>,
}

/// Token usage statistics for an AI response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsage {
    /// Number of input tokens consumed
    pub input: i64,
    /// Number of output tokens generated
    pub output: i64,
    /// Number of reasoning tokens used (for reasoning-capable models)
    pub reasoning: i64,
    /// Cache usage statistics
    pub cache: CacheUsage,
}

/// Cache usage statistics for prompt caching.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheUsage {
    /// Number of tokens read from cache
    pub read: i64,
    /// Number of tokens written to cache
    pub write: i64,
}

/// Error information for failed assistant messages.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "name", content = "data")]
pub enum AssistantError {
    /// Authentication error with the AI provider
    ProviderAuthError {
        /// The provider that failed authentication
        #[serde(rename = "providerID")]
        provider_id: String,
        /// Error message describing the authentication failure
        message: String,
    },
    /// An unknown or unexpected error occurred
    UnknownError {
        /// Error message describing what went wrong
        message: String,
    },
    /// The AI response exceeded the maximum output length
    MessageOutputLengthError,
    /// The message generation was aborted by the user or system
    MessageAbortedError {
        /// Reason for the abort
        message: String,
    },
    /// An error occurred while communicating with the AI API
    APIError {
        /// Error message from the API
        message: String,
        /// HTTP status code (if applicable)
        #[serde(default, rename = "statusCode")]
        status_code: Option<i64>,
        /// Whether this error can be retried
        #[serde(rename = "isRetryable")]
        is_retryable: bool,
        /// HTTP response headers from the failed request
        #[serde(default, rename = "responseHeaders")]
        response_headers: Option<HashMap<String, String>>,
        /// Response body from the failed request
        #[serde(default, rename = "responseBody")]
        response_body: Option<String>,
        /// Additional metadata about the error
        #[serde(default)]
        metadata: Option<HashMap<String, String>>,
    },
}

/// A user-authored message in a conversation session.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserMessage {
    /// Unique message identifier
    pub id: String,
    /// ID of the session this message belongs to
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Message role (always "user")
    pub role: String,
    /// When the message was created and completed
    pub time: MessageTime,
    /// Optional summary of the message and its effects
    #[serde(default)]
    pub summary: Option<MessageSummary>,
    /// Agent that handled this message
    pub agent: String,
    /// Model specification used for this message
    pub model: ModelSpec,
    /// Optional system prompt override
    #[serde(default)]
    pub system: Option<String>,
    /// Tool permissions for this message (deprecated)
    #[serde(default)]
    pub tools: Option<HashMap<String, bool>>,
    /// Model variant to use (e.g., "extended")
    #[serde(default)]
    pub variant: Option<String>,
}

/// Summary of a message and its effects.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSummary {
    /// Brief title summarizing the message
    pub title: String,
    /// Detailed description of what the message accomplished
    pub body: String,
    /// File changes resulting from this message
    pub diffs: Vec<FileDiff>,
}

/// An AI assistant-generated message in a conversation session.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssistantMessage {
    /// Unique message identifier
    pub id: String,
    /// ID of the session this message belongs to
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Message role (always "assistant")
    pub role: String,
    /// When the message was created and completed
    pub time: MessageTime,
    /// Error information if the message generation failed
    #[serde(default)]
    pub error: Option<AssistantError>,
    /// ID of the parent message this is responding to
    #[serde(rename = "parentID")]
    pub parent_id: String,
    /// Model ID that generated this message
    #[serde(rename = "modelID")]
    pub model_id: String,
    /// Provider ID for the model
    #[serde(rename = "providerID")]
    pub provider_id: String,
    /// Execution mode (e.g., "agentic", "chat")
    pub mode: String,
    /// Agent that handled this message
    pub agent: String,
    /// Path information for where the message was generated
    pub path: MessagePath,
    /// Whether this message has been summarized/compacted
    #[serde(default)]
    pub summary: Option<bool>,
    /// Estimated cost in USD for this message
    pub cost: f64,
    /// Token usage statistics
    pub tokens: TokenUsage,
    /// How the message generation finished (e.g., "stop", "length")
    #[serde(default)]
    pub finish: Option<String>,
}

/// Path context for where a message was generated.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessagePath {
    /// Current working directory when the message was generated
    pub cwd: String,
    /// Root directory of the project
    pub root: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    #[serde(rename = "user")]
    User(UserMessage),
    #[serde(rename = "assistant")]
    Assistant(AssistantMessage),
}

impl Message {
    pub fn id(&self) -> &str {
        match self {
            Message::User(msg) => &msg.id,
            Message::Assistant(msg) => &msg.id,
        }
    }
    
    pub fn session_id(&self) -> &str {
        match self {
            Message::User(msg) => &msg.session_id,
            Message::Assistant(msg) => &msg.session_id,
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

/// Server-sent events from the OpenCode server.
///
/// These events are emitted in real-time as sessions progress and can be
/// subscribed to via the `/event` SSE endpoint.
#[derive(Debug, Clone)]
pub enum Event {
    /// A new session was created
    SessionCreated(Session),
    /// An existing session was updated (title, permissions, etc.)
    SessionUpdated(Session),
    /// A session was deleted
    SessionDeleted(Session),
    /// A message was added or updated in a session
    MessageUpdated(Message),
    /// A message was removed from a session
    MessageRemoved {
        /// ID of the session the message was removed from
        session_id: String,
        /// ID of the message that was removed
        message_id: String,
    },
    /// A part within a message was updated (typically during streaming)
    PartUpdated {
        /// The updated part
        part: Part,
        /// Optional delta text for streaming updates
        delta: Option<String>,
    },
    /// A part was removed from a message
    PartRemoved {
        /// ID of the session
        session_id: String,
        /// ID of the message
        message_id: String,
        /// ID of the part that was removed
        part_id: String,
    },
    /// An error occurred within a session
    SessionError {
        /// ID of the session that encountered the error
        session_id: String,
        /// Detailed error information
        error: AssistantError,
    },
    /// A generic error occurred
    Error(String),
    /// An unknown event type was received
    Unknown(String),
}
