//! Type definitions for the OpenCode API.
//!
//! This module contains all request and response types used by the OpenCode server API,
//! organized by API category.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
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
// Session API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionTime {
    pub created: i64,  // milliseconds timestamp
    pub updated: i64,  // milliseconds timestamp
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, rename = "projectID")]
    pub project_id: Option<String>,
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    pub time: SessionTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionCreateRequest {
    #[serde(default)]
    pub title: Option<String>,
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
