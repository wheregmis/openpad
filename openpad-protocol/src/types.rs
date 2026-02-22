//! Type definitions for the OpenCode API.
//!
//! This module contains all request and response types used by the OpenCode server API,
//! organized by API category.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, DerefMut};

/// A string wrapper that masks its content in Debug output.
///
/// Use this for sensitive data like API keys or tokens to prevent them from
/// being accidentally logged or displayed in debug views.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"<REDACTED>\"")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for SecretString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for SecretString {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

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

/// Output format for an AI response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputFormat {
    /// Plain text output
    Text,
    /// Structured JSON output matching a schema
    JsonSchema {
        /// JSON schema for the output
        schema: serde_json::Value,
        /// Number of times to retry if output doesn't match schema
        #[serde(default, rename = "retryCount")]
        retry_count: Option<i64>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogRequest {
    pub service: String,
    pub level: String,
    pub message: String,
    #[serde(default)]
    pub extra: ExtraMaskedMap<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Agent {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub mode: String,
    #[serde(default)]
    pub native: Option<bool>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(rename = "topP")]
    #[serde(default)]
    pub top_p: Option<f64>,
    #[serde(default)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub color: Option<String>,
    pub permission: PermissionRuleset,
    #[serde(default)]
    pub model: Option<AgentModelSpec>,
    #[serde(default)]
    pub variant: Option<String>,
    #[serde(default)]
    pub prompt: Option<String>,
    pub options: ExtraMaskedMap<serde_json::Value>,
    #[serde(default)]
    pub steps: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentModelSpec {
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub location: String,
    pub content: String,
}

// ============================================================================
// Project API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub worktree: String,
    #[serde(default)]
    pub vcs: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub icon: Option<ProjectIcon>,
    #[serde(default)]
    pub commands: Option<ProjectCommands>,
    pub time: ProjectTime,
    #[serde(default)]
    pub sandboxes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectIcon {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default, rename = "override")]
    pub r#override: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectCommands {
    #[serde(default)]
    pub start: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ProjectIcon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<ProjectCommands>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectSummary {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    pub worktree: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectTime {
    pub created: f64,
    pub updated: f64,
    #[serde(default)]
    pub initialized: Option<f64>,
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

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("model", &self.model)
            .field("extra", &ExtraMasked(&self.extra))
            .finish()
    }
}

/// Checks if a key name suggests it contains sensitive information (e.g. credentials).
fn is_sensitive_key(key: &str) -> bool {
    let k_lower = key.to_lowercase();
    k_lower == "key"
        || k_lower == "token"
        || k_lower == "secret"
        || k_lower == "password"
        || k_lower == "auth"
        || k_lower == "authorization"
        || k_lower == "cookie"
        || k_lower == "set-cookie"
        || k_lower == "signature"
        || k_lower == "credential"
        || k_lower == "passphrase"
        || k_lower == "pwd"
        || k_lower == "sessionid"
        || k_lower == "sid"
        || k_lower.ends_with("_key")
        || k_lower.ends_with("-key")
        || k_lower.ends_with("apikey")
        || k_lower.ends_with("_token")
        || k_lower.ends_with("-token")
        || k_lower.ends_with("_secret")
        || k_lower.ends_with("-secret")
        || k_lower.ends_with("_password")
        || k_lower.ends_with("-password")
        || k_lower.ends_with("_auth")
        || k_lower.ends_with("-auth")
}

/// A wrapper for HashMaps that masks sensitive keys in its Debug implementation.
///
/// Use this for any HashMap that might contain credentials or other sensitive
/// information that shouldn't be leaked in logs.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExtraMaskedMap<V>(HashMap<String, V>);

impl<V: fmt::Debug> fmt::Debug for ExtraMaskedMap<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (k, v) in &self.0 {
            if is_sensitive_key(k) {
                map.entry(k, &"<REDACTED>");
            } else {
                map.entry(k, v);
            }
        }
        map.finish()
    }
}

impl<V> Deref for ExtraMaskedMap<V> {
    type Target = HashMap<String, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for ExtraMaskedMap<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V> From<HashMap<String, V>> for ExtraMaskedMap<V> {
    fn from(map: HashMap<String, V>) -> Self {
        Self(map)
    }
}

/// Helper to mask sensitive keys in a HashMap when formatting for Debug.
struct ExtraMasked<'a, V>(&'a HashMap<String, V>);

impl<'a, V: fmt::Debug> fmt::Debug for ExtraMasked<'a, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (k, v) in self.0 {
            if is_sensitive_key(k) {
                map.entry(k, &"<REDACTED>");
            } else {
                map.entry(k, v);
            }
        }
        map.finish()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub source: String, // "env", "config", "custom", "api"
    pub env: Vec<String>,
    #[serde(default)]
    pub key: Option<SecretString>,
    pub options: ExtraMaskedMap<serde_json::Value>,
    pub models: HashMap<String, Model>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Model {
    pub id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
    pub api: ModelApi,
    pub name: String,
    #[serde(default)]
    pub family: Option<String>,
    pub capabilities: ModelCapabilities,
    pub cost: ModelCost,
    pub limit: ModelLimit,
    pub status: String,
    pub options: ExtraMaskedMap<serde_json::Value>,
    pub headers: HashMap<String, SecretString>,
    pub release_date: String,
    #[serde(default)]
    pub variants: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelApi {
    pub id: String,
    pub url: String,
    pub npm: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelCapabilities {
    pub temperature: bool,
    pub reasoning: bool,
    pub attachment: bool,
    pub toolcall: bool,
    pub input: ModelModalities,
    pub output: ModelModalities,
    pub interleaved: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelModalities {
    pub text: bool,
    pub audio: bool,
    pub image: bool,
    pub video: bool,
    pub pdf: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelCost {
    pub input: f64,
    pub output: f64,
    pub cache: ModelCacheCost,
    #[serde(default, rename = "experimentalOver200K")]
    pub experimental_over_200k: Option<ModelCostOver200K>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelCacheCost {
    pub read: f64,
    pub write: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelCostOver200K {
    pub input: f64,
    pub output: f64,
    pub cache: ModelCacheCost,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelLimit {
    pub context: f64,
    #[serde(default)]
    pub input: Option<f64>,
    pub output: f64,
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
    pub type_filter: Option<String>, // "file" or "directory"
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>, // 1-200
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolsSearchRequest {
    pub query: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Symbol {
    pub name: String,
    pub kind: f64,
    pub location: SymbolLocation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolLocation {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileReadRequest {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileReadResponse {
    #[serde(rename = "type")]
    pub type_name: String, // "text" or "binary"
    pub content: String,
    #[serde(default)]
    pub diff: Option<String>,
    #[serde(default)]
    pub patch: Option<serde_json::Value>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default, rename = "mimeType")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileStatusRequest {
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub path: String,
    pub added: i64,
    pub removed: i64,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub absolute: String,
    #[serde(rename = "type")]
    pub type_name: String, // "file" or "directory"
    pub ignored: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
    pub content: String,
    pub status: String,
    pub priority: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pty {
    pub id: String,
    pub title: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub status: String,
    pub pid: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LSPStatus {
    pub id: String,
    pub name: String,
    pub root: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormatterStatus {
    pub name: String,
    pub extensions: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Command {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    pub template: serde_json::Value,
    #[serde(default)]
    pub subtask: Option<bool>,
    pub hints: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Worktree {
    pub name: String,
    pub branch: String,
    pub directory: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PtyCreateRequest {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub env: ExtraMaskedMap<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PtyUpdateRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub size: Option<PtySize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PtySize {
    pub rows: f64,
    pub cols: f64,
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
    #[serde(default)]
    pub title: Option<String>,
    pub message: String,
    pub variant: String, // "info", "success", "warning", "error"
    #[serde(default)]
    pub duration: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SessionStatus {
    Idle,
    Retry {
        attempt: f64,
        message: String,
        next: f64,
    },
    Busy,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestionRequest {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub questions: Vec<QuestionInfo>,
    #[serde(default)]
    pub tool: Option<PermissionToolContext>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestionInfo {
    pub question: String,
    pub header: String,
    pub options: Vec<QuestionOption>,
    #[serde(default)]
    pub multiple: Option<bool>,
    #[serde(default)]
    pub custom: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestionOption {
    pub label: String,
    pub description: String,
}

// ============================================================================
// Auth API types
// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthSetRequest {
    #[serde(rename = "type")]
    pub auth_type: String, // "api"
    pub key: SecretString,
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

/// Reply to a permission request.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionReply {
    Once,
    Always,
    Reject,
}

/// Tool context for a permission request.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionToolContext {
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "callID")]
    pub call_id: String,
}

/// A pending permission request emitted by the server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionRequest {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub permission: String,
    #[serde(default)]
    pub patterns: Vec<String>,
    #[serde(default)]
    pub metadata: ExtraMaskedMap<serde_json::Value>,
    #[serde(default)]
    pub always: Vec<String>,
    #[serde(default)]
    pub tool: Option<PermissionToolContext>,
}

/// Request body for replying to a permission request.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionReplyRequest {
    pub reply: PermissionReply,
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
    #[serde(default)]
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
    /// Status of the file change (added, deleted, modified)
    #[serde(default)]
    pub status: Option<String>,
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
    #[serde(default)]
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
    #[serde(default, rename = "parentID", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalSession {
    #[serde(flatten)]
    pub info: Session,
    pub project: Option<ProjectSummary>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionUpdateRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub time: Option<SessionUpdateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionUpdateTime {
    #[serde(default)]
    pub archived: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionInitRequest {
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionSummarizeRequest {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(default)]
    pub auto: bool,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub parts: Vec<PartInput>,
    #[serde(default, rename = "noReply", skip_serializing_if = "Option::is_none")]
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
#[serde(rename_all = "snake_case")]
pub enum PermissionDecision {
    Allow,
    Reject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionResponse {
    pub response: PermissionDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remember: Option<bool>,
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
    /// Total number of tokens used
    #[serde(default)]
    pub total: Option<i64>,
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
    /// Error when generating structured output
    StructuredOutputError {
        /// Error message
        message: String,
        /// Number of retries attempted
        retries: i64,
    },
    /// Error when the context limit is exceeded
    ContextOverflowError {
        /// Error message
        message: String,
        /// Optional response body
        #[serde(default, rename = "responseBody")]
        response_body: Option<SecretString>,
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
        response_headers: Option<HashMap<String, SecretString>>,
        /// Response body from the failed request
        #[serde(default, rename = "responseBody")]
        response_body: Option<SecretString>,
        /// Additional metadata about the error
        #[serde(default)]
        metadata: Option<HashMap<String, SecretString>>,
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
    /// When the message was created and completed
    pub time: MessageTime,
    /// Optional summary of the message and its effects
    #[serde(default)]
    pub summary: Option<MessageSummary>,
    /// Output format requested for this message
    #[serde(default)]
    pub format: Option<OutputFormat>,
    /// Agent that handled this message
    #[serde(default)]
    pub agent: String,
    /// Model specification used for this message
    #[serde(default)]
    pub model: Option<ModelSpec>,
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
    #[serde(default)]
    pub title: String,
    /// Detailed description of what the message accomplished
    #[serde(default)]
    pub body: String,
    /// File changes resulting from this message
    #[serde(default)]
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
    /// When the message was created and completed
    pub time: MessageTime,
    /// Error information if the message generation failed
    #[serde(default)]
    pub error: Option<AssistantError>,
    /// ID of the parent message this is responding to
    #[serde(default, rename = "parentID")]
    pub parent_id: String,
    /// Model ID that generated this message
    #[serde(default, rename = "modelID")]
    pub model_id: String,
    /// Provider ID for the model
    #[serde(default, rename = "providerID")]
    pub provider_id: String,
    /// Execution mode (e.g., "agentic", "chat")
    #[serde(default)]
    pub mode: String,
    /// Agent that handled this message
    #[serde(default)]
    pub agent: String,
    /// Path information for where the message was generated
    #[serde(default)]
    pub path: Option<MessagePath>,
    /// Whether this message has been summarized/compacted
    #[serde(default)]
    pub summary: Option<bool>,
    /// Estimated cost in USD for this message
    #[serde(default)]
    pub cost: f64,
    /// Token usage statistics
    #[serde(default)]
    pub tokens: Option<TokenUsage>,
    /// Structured output if requested
    #[serde(default)]
    pub structured: Option<serde_json::Value>,
    /// Model variant to use
    #[serde(default)]
    pub variant: Option<String>,
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
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Part {
    #[serde(rename = "text")]
    Text {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        #[serde(default)]
        text: String,
        #[serde(default)]
        synthetic: Option<bool>,
        #[serde(default)]
        ignored: Option<bool>,
        #[serde(default)]
        time: Option<PartTime>,
        #[serde(default)]
        metadata: Option<ExtraMaskedMap<serde_json::Value>>,
    },
    #[serde(rename = "subtask")]
    Subtask {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        prompt: String,
        description: String,
        agent: String,
        model: AgentModelSpec,
        #[serde(default)]
        command: Option<String>,
    },
    #[serde(rename = "reasoning")]
    Reasoning {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        text: String,
        #[serde(default)]
        metadata: Option<ExtraMaskedMap<serde_json::Value>>,
        time: PartTime,
    },
    #[serde(rename = "file")]
    File(FilePart),
    #[serde(rename = "tool")]
    Tool {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        #[serde(default, rename = "callID")]
        call_id: String,
        #[serde(default)]
        tool: String,
        state: ToolState,
        #[serde(default)]
        metadata: Option<ExtraMaskedMap<serde_json::Value>>,
    },
    #[serde(rename = "step-start")]
    StepStart {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        #[serde(default)]
        snapshot: Option<String>,
    },
    #[serde(rename = "step-finish")]
    StepFinish {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        #[serde(default)]
        reason: String,
        #[serde(default)]
        snapshot: Option<String>,
        #[serde(default)]
        cost: f64,
        #[serde(default)]
        tokens: Option<TokenUsage>,
    },
    #[serde(rename = "snapshot")]
    Snapshot {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        snapshot: String,
    },
    #[serde(rename = "patch")]
    Patch {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        hash: String,
        files: Vec<String>,
    },
    #[serde(rename = "agent")]
    Agent {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        name: String,
        #[serde(default)]
        source: Option<PartSourceValue>,
    },
    #[serde(rename = "retry")]
    Retry {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        attempt: f64,
        error: AssistantError,
        time: PartTimeCreated,
    },
    #[serde(rename = "compaction")]
    Compaction {
        #[serde(default)]
        id: String,
        #[serde(default, rename = "sessionID")]
        session_id: String,
        #[serde(default, rename = "messageID")]
        message_id: String,
        auto: bool,
    },
    // Other part types — we don't render them but must not break parsing
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartTime {
    pub start: f64,
    #[serde(default)]
    pub end: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartTimeCreated {
    pub created: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePart {
    #[serde(default)]
    pub id: String,
    #[serde(default, rename = "sessionID")]
    pub session_id: String,
    #[serde(default, rename = "messageID")]
    pub message_id: String,
    #[serde(default)]
    pub mime: String,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub source: Option<FilePartSource>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartSourceValue {
    pub value: String,
    pub start: i64,
    pub end: i64,
}

/// Tool execution state (pending / running / completed / error).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ToolState {
    Pending {
        #[serde(default)]
        input: ExtraMaskedMap<serde_json::Value>,
        #[serde(default)]
        raw: String,
    },
    Running {
        #[serde(default)]
        input: ExtraMaskedMap<serde_json::Value>,
        #[serde(default)]
        title: String,
        #[serde(default)]
        metadata: ExtraMaskedMap<serde_json::Value>,
        #[serde(default)]
        time: ToolStateTime,
    },
    Completed {
        #[serde(default)]
        input: ExtraMaskedMap<serde_json::Value>,
        #[serde(default)]
        output: String,
        #[serde(default)]
        title: String,
        #[serde(default)]
        metadata: ExtraMaskedMap<serde_json::Value>,
        #[serde(default)]
        time: ToolStateTime,
        #[serde(default)]
        attachments: Vec<FilePart>,
    },
    Error {
        #[serde(default)]
        input: ExtraMaskedMap<serde_json::Value>,
        #[serde(default)]
        error: String,
        #[serde(default)]
        metadata: ExtraMaskedMap<serde_json::Value>,
        #[serde(default)]
        time: ToolStateTime,
    },
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToolStateTime {
    #[serde(default)]
    pub start: Option<f64>,
    #[serde(default)]
    pub end: Option<f64>,
    #[serde(default)]
    pub compacted: Option<f64>,
}

impl Part {
    /// Extract displayable text content, if any.
    pub fn text_content(&self) -> Option<&str> {
        match self {
            Part::Text { text, .. } if !text.is_empty() => Some(text),
            Part::Reasoning { text, .. } if !text.is_empty() => Some(text),
            _ => None,
        }
    }

    /// Get file attachment info, if this is a file part.
    pub fn file_info(&self) -> Option<(&str, Option<&str>, &str)> {
        match self {
            Part::File(file) => Some((file.mime.as_str(), file.filename.as_deref(), file.url.as_str())),
            _ => None,
        }
    }

    /// Get the message ID this part belongs to, if available.
    pub fn message_id(&self) -> Option<&str> {
        match self {
            Part::Text { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Subtask { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Reasoning { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::File(file) if !file.message_id.is_empty() => Some(&file.message_id),
            Part::StepStart { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::StepFinish { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Tool { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Snapshot { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Patch { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Agent { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Retry { message_id, .. } if !message_id.is_empty() => Some(message_id),
            Part::Compaction { message_id, .. } if !message_id.is_empty() => Some(message_id),
            _ => None,
        }
    }

    /// If this is a step-finish part, return (reason, cost, tokens).
    pub fn step_finish_info(&self) -> Option<(&str, f64, Option<&TokenUsage>)> {
        match self {
            Part::StepFinish {
                reason,
                cost,
                tokens,
                ..
            } => Some((reason.as_str(), *cost, tokens.as_ref())),
            _ => None,
        }
    }

    /// If this is a tool part, return (tool_name, input_summary, result) for display.
    /// result is either the output string or "Error: {error}".
    pub fn tool_display(&self) -> Option<(String, String, String)> {
        match self {
            Part::Tool { tool, state, .. } => {
                let input_summary = summarize_tool_input(state.input());
                let result = state.output_or_error();
                Some((tool.clone(), input_summary, result))
            }
            _ => None,
        }
    }
}

pub(crate) fn summarize_tool_input(input: &HashMap<String, serde_json::Value>) -> String {
    if input.is_empty() {
        return String::new();
    }
    // Prefer human-readable keys: path, offset, limit, command, etc.
    let keys = ["path", "offset", "limit", "command", "arguments", "name"];
    let mut parts: Vec<String> = Vec::new();
    for k in keys {
        if let Some(v) = input.get(k) {
            if is_sensitive_key(k) {
                parts.push(format!("{}=<REDACTED>", k));
                continue;
            }
            let s = match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => v.to_string(),
            };
            if !s.is_empty() {
                parts.push(format!("{}={}", k, truncate_display(&s, 60)));
            }
        }
    }
    if parts.is_empty() {
        // Redact sensitive keys before serializing the whole map to JSON for display
        let mut masked = input.clone();
        for (k, v) in masked.iter_mut() {
            if is_sensitive_key(k) {
                *v = serde_json::Value::String("<REDACTED>".to_string());
            }
        }
        let single = serde_json::to_string(&masked).unwrap_or_default();
        truncate_display(&single, 80).to_string()
    } else {
        parts.join(" ")
    }
}

fn truncate_display(s: &str, max: usize) -> String {
    let s = s.trim();
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max).collect::<String>())
    }
}

impl ToolState {
    fn input(&self) -> &HashMap<String, serde_json::Value> {
        match self {
            ToolState::Pending { input, .. } => input,
            ToolState::Running { input, .. } => input,
            ToolState::Completed { input, .. } => input,
            ToolState::Error { input, .. } => input,
        }
    }

    fn output_or_error(&self) -> String {
        match self {
            ToolState::Pending { .. } => "(pending)".to_string(),
            ToolState::Running { .. } => "(running)".to_string(),
            ToolState::Completed { output, .. } => truncate_display(output, 200).to_string(),
            ToolState::Error { error, .. } => format!("Error: {}", truncate_display(error, 200)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PartInput {
    Text {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        synthetic: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ignored: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time: Option<PartTime>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<ExtraMaskedMap<serde_json::Value>>,
    },
    File {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        mime: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<FilePartSource>,
    },
    Agent {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<PartSourceValue>,
    },
    Subtask {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        prompt: String,
        description: String,
        agent: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        model: Option<AgentModelSpec>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        command: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FilePartSource {
    File {
        text: FilePartSourceText,
        path: String,
    },
    Symbol {
        text: FilePartSourceText,
        path: String,
        range: Range,
        name: String,
        kind: i64,
    },
    Resource {
        text: FilePartSourceText,
        #[serde(rename = "clientName")]
        client_name: String,
        uri: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePartSourceText {
    pub value: String,
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Position {
    pub line: f64,
    pub character: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McpResource {
    pub name: String,
    pub uri: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "mimeType")]
    pub mime_type: Option<String>,
    pub client: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum MCPStatus {
    Connected,
    Disabled,
    Failed {
        error: String,
    },
    NeedsAuth,
    NeedsClientRegistration {
        error: String,
    },
}

pub type ToolIDs = Vec<String>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolListItem {
    pub id: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub type ToolList = Vec<ToolListItem>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McpAddRequest {
    pub name: String,
    pub config: McpConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpConfig {
    Local(McpLocalConfig),
    Remote(McpRemoteConfig),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McpLocalConfig {
    pub command: Vec<String>,
    #[serde(default)]
    pub environment: ExtraMaskedMap<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub timeout: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McpRemoteConfig {
    pub url: String,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub headers: HashMap<String, SecretString>,
    #[serde(default)]
    pub oauth: Option<serde_json::Value>, // McpOAuthConfig or bool
    #[serde(default)]
    pub timeout: Option<f64>,
}

impl PartInput {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text {
            id: None,
            text: text.into(),
            synthetic: None,
            ignored: None,
            time: None,
            metadata: None,
        }
    }

    pub fn file(mime: impl Into<String>, url: impl Into<String>) -> Self {
        Self::File {
            id: None,
            mime: mime.into(),
            filename: None,
            url: url.into(),
            source: None,
        }
    }

    pub fn file_with_filename(
        mime: impl Into<String>,
        filename: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self::File {
            id: None,
            mime: mime.into(),
            filename: Some(filename.into()),
            url: url.into(),
            source: None,
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
    /// A session's status changed
    SessionStatus {
        session_id: String,
        status: SessionStatus,
    },
    /// A session became idle
    SessionIdle { session_id: String },
    /// A session was compacted
    SessionCompacted { session_id: String },
    /// File changes (diff) for a session
    SessionDiff {
        session_id: String,
        diff: Vec<FileDiff>,
    },
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
    /// A incremental delta update for a message part
    PartDelta {
        session_id: String,
        message_id: String,
        part_id: String,
        field: String,
        delta: String,
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
    /// A permission request was issued by the assistant
    PermissionAsked(PermissionRequest),
    /// A permission request was replied to
    PermissionReplied {
        /// ID of the session
        session_id: String,
        /// ID of the permission request
        request_id: String,
        /// Reply type
        reply: PermissionReply,
    },
    /// A question request was issued by the assistant
    QuestionAsked(QuestionRequest),
    /// A question request was replied to
    QuestionReplied {
        session_id: String,
        request_id: String,
        answers: Vec<Vec<String>>,
    },
    /// A question request was rejected
    QuestionRejected {
        session_id: String,
        request_id: String,
    },
    /// Todo list was updated
    TodoUpdated {
        session_id: String,
        todos: Vec<Todo>,
    },
    /// TUI prompt append
    TuiPromptAppend { text: String },
    /// TUI command execute
    TuiCommandExecute { command: String },
    /// TUI toast show
    TuiToastShow {
        title: Option<String>,
        message: String,
        variant: String,
        duration: Option<f64>,
    },
    /// TUI session select
    TuiSessionSelect { session_id: String },
    /// PTY session created
    PtyCreated(Pty),
    /// PTY session updated
    PtyUpdated(Pty),
    /// PTY session exited
    PtyExited { id: String, exit_code: i64 },
    /// PTY session deleted
    PtyDeleted { id: String },
    /// Project information was updated
    ProjectUpdated(Project),
    /// VCS branch was updated
    VcsBranchUpdated { branch: String },
    /// File was edited
    FileEdited { file: String },
    /// File watcher event
    FileWatcherUpdated { file: String, event: String },
    /// LSP server updated
    LspUpdated,
    /// LSP diagnostics received
    LspDiagnostics { server_id: String, path: String },
    /// Worktree is ready
    WorktreeReady { name: String, branch: String },
    /// Worktree creation failed
    WorktreeFailed { message: String },
    /// MCP tools changed
    McpToolsChanged { server: String },
    /// MCP browser open failed
    McpBrowserOpenFailed { mcp_name: String, url: String },
    /// Command was executed
    CommandExecuted {
        name: String,
        session_id: String,
        arguments: String,
        message_id: String,
    },
    /// Installation was updated
    InstallationUpdated { version: String },
    /// Update is available
    InstallationUpdateAvailable { version: String },
    /// Server connected
    ServerConnected,
    /// Global instance disposed
    GlobalDisposed,
    /// Server instance disposed
    ServerInstanceDisposed { directory: String },
    /// A generic error occurred
    Error(String),
    /// An unknown event type was received
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize_tool_input_redaction() {
        let mut input = HashMap::new();
        input.insert("token".into(), serde_json::Value::String("secret".into()));
        assert!(summarize_tool_input(&input).contains("<REDACTED>"));
    }

    #[test]
    fn test_extra_masked_map_debug() {
        let mut map = HashMap::new();
        map.insert("api_key".to_string(), "secret123".to_string());
        map.insert("normal_field".to_string(), "value".to_string());

        let masked: ExtraMaskedMap<String> = map.into();
        let debug = format!("{:?}", masked);

        assert!(debug.contains("api_key"));
        assert!(debug.contains("<REDACTED>"));
        assert!(!debug.contains("secret123"));
        assert!(debug.contains("normal_field"));
        assert!(debug.contains("value"));
    }
}
