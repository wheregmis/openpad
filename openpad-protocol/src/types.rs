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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionRule {
    pub permission: String,
    pub pattern: String,
    pub action: PermissionAction,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}

pub type PermissionRuleset = Vec<PermissionRule>;

// ============================================================================
// Session API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionTime {
    pub created: i64,  // milliseconds timestamp
    pub updated: i64,  // milliseconds timestamp
    #[serde(default)]
    pub compacting: Option<i64>,
    #[serde(default)]
    pub archived: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionSummary {
    pub additions: i64,
    pub deletions: i64,
    pub files: i64,
    pub diffs: Vec<FileDiff>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileDiff {
    pub file: String,
    pub before: String,
    pub after: String,
    pub additions: i64,
    pub deletions: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionShare {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionRevert {
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(default, rename = "partID")]
    pub part_id: Option<String>,
    #[serde(default)]
    pub snapshot: Option<String>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageTime {
    pub created: i64,  // milliseconds timestamp
    #[serde(default)]
    pub completed: Option<i64>,  // milliseconds timestamp
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsage {
    pub input: i64,
    pub output: i64,
    pub reasoning: i64,
    pub cache: CacheUsage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheUsage {
    pub read: i64,
    pub write: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "name", content = "data")]
pub enum AssistantError {
    ProviderAuthError {
        #[serde(rename = "providerID")]
        provider_id: String,
        message: String,
    },
    UnknownError {
        message: String,
    },
    MessageOutputLengthError,
    MessageAbortedError {
        message: String,
    },
    APIError {
        message: String,
        #[serde(default, rename = "statusCode")]
        status_code: Option<i64>,
        #[serde(rename = "isRetryable")]
        is_retryable: bool,
        #[serde(default, rename = "responseHeaders")]
        response_headers: Option<HashMap<String, String>>,
        #[serde(default, rename = "responseBody")]
        response_body: Option<String>,
        #[serde(default)]
        metadata: Option<HashMap<String, String>>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserMessage {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub role: String, // "user"
    pub time: MessageTime,
    #[serde(default)]
    pub summary: Option<MessageSummary>,
    pub agent: String,
    pub model: ModelSpec,
    #[serde(default)]
    pub system: Option<String>,
    #[serde(default)]
    pub tools: Option<HashMap<String, bool>>,
    #[serde(default)]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSummary {
    pub title: String,
    pub body: String,
    pub diffs: Vec<FileDiff>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssistantMessage {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub role: String, // "assistant"
    pub time: MessageTime,
    #[serde(default)]
    pub error: Option<AssistantError>,
    #[serde(rename = "parentID")]
    pub parent_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
    pub mode: String,
    pub agent: String,
    pub path: MessagePath,
    #[serde(default)]
    pub summary: Option<bool>,
    pub cost: f64,
    pub tokens: TokenUsage,
    #[serde(default)]
    pub finish: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessagePath {
    pub cwd: String,
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
