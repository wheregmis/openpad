//! OpenCode HTTP client implementation.
//!
//! This module provides a complete client for the OpenCode server API,
//! including REST endpoints and Server-Sent Events (SSE) subscription.

use crate::{
    Agent, AppendPromptRequest, AuthSetRequest, CommandRequest, Config, ExecuteCommandRequest,
    File, FileDiff, FileNode, FileReadRequest, FileReadResponse, FileStatusRequest,
    FilesSearchRequest, GlobalSession, HealthResponse, LogRequest, MCPStatus, McpAddRequest,
    McpResource, MessageWithParts, PathInfo, PermissionReply, PermissionReplyRequest,
    PermissionRequest,
    PermissionResponse, Project, ProjectUpdateRequest, PromptRequest, ProvidersResponse, Pty,
    RevertRequest, SessionCreateRequest, SessionInitRequest, SessionSummarizeRequest,
    SessionUpdateRequest, ShellRequest, ShowToastRequest, Skill, Symbol,
    SymbolsSearchRequest, TextSearchRequest, TextSearchResult, Todo, ToolIDs, ToolList,
};
use crate::{AssistantError, Error, Event, Message, Part, PartInput, Result, Session};
use reqwest::Client as HttpClient;
use std::env;
use tokio::sync::broadcast;

/// OpenCode HTTP client.
///
/// Provides async methods for all OpenCode server API endpoints including:
/// - Global APIs (health)
/// - App APIs (log, agents)
/// - Project APIs (list, current)
/// - Path APIs (get)
/// - Config APIs (get, providers)
/// - Session APIs (create, list, get, update, delete, messages, prompts, etc.)
/// - File/Find APIs (search text, files, symbols, read, status)
/// - TUI APIs (control TUI interface)
/// - Auth APIs (set credentials)
/// - Event subscription (SSE)
// Maximum size of the SSE buffer to prevent memory exhaustion.
const MAX_SSE_BUFFER_SIZE: usize = 1024 * 1024; // 1MB

pub struct OpenCodeClient {
    http: HttpClient,
    base_url: String,
    directory: String,
    event_tx: broadcast::Sender<Event>,
}

impl OpenCodeClient {
    /// Creates a new OpenCode client.
    ///
    /// The client will use the current working directory as the default
    /// `directory` parameter for all requests.
    ///
    /// # Example
    /// ```no_run
    /// use openpad_protocol::OpenCodeClient;
    ///
    /// let client = OpenCodeClient::new("http://localhost:4096");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        let (event_tx, _) = broadcast::channel(256);
        let directory = env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| ".".to_string());

        Self {
            http: HttpClient::new(),
            base_url: base_url.into(),
            directory,
            event_tx,
        }
    }

    /// Sets the directory for all API requests.
    ///
    /// This overrides the default current working directory.
    ///
    /// # Example
    /// ```no_run
    /// use openpad_protocol::OpenCodeClient;
    ///
    /// let client = OpenCodeClient::new("http://localhost:4096")
    ///     .with_directory("/path/to/project");
    /// ```
    pub fn with_directory(mut self, directory: impl Into<String>) -> Self {
        self.directory = directory.into();
        self
    }

    // ========================================================================
    // Private helper methods to reduce code duplication
    // ========================================================================

    /// Helper to check response status and return an error if not successful.
    async fn check_response(
        response: reqwest::Response,
        action: &str,
    ) -> Result<reqwest::Response> {
        if !response.status().is_success() {
            let status = response.status();
            let mut body = response.text().await.unwrap_or_default();
            if body.is_empty() {
                body = "no response body".to_string();
            } else if body.len() > 1024 {
                // Truncate overly large error bodies to prevent memory exhaustion and log spam.
                // We ensure we truncate at a valid UTF-8 character boundary.
                let mut truncate_idx = 1024;
                while !body.is_char_boundary(truncate_idx) {
                    truncate_idx -= 1;
                }
                body.truncate(truncate_idx);
                body.push_str("... (truncated)");
            }
            return Err(Error::InvalidResponse(format!(
                "Failed to {}: {} ({})",
                action, status, body
            )));
        }
        Ok(response)
    }

    /// Helper for GET requests that return JSON.
    async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        action: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        Ok(response.json().await?)
    }

    /// Helper for POST requests with JSON body that return JSON.
    async fn post_json<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &B,
        action: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        Ok(response.json().await?)
    }

    /// Helper for POST requests with JSON body that return boolean (success indicator).
    async fn post_json_bool<B: serde::Serialize>(
        &self,
        endpoint: &str,
        body: &B,
        action: &str,
    ) -> Result<bool> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        // Consume the response body to allow connection reuse
        let _ = response.bytes().await?;
        Ok(true)
    }

    /// Helper for POST requests without body that return JSON.
    async fn post_no_body_json<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        action: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        Ok(response.json().await?)
    }

    /// Helper for POST requests without body that return boolean.
    async fn post_no_body_bool(&self, endpoint: &str, action: &str) -> Result<bool> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        // Consume the response body to allow connection reuse
        let _ = response.bytes().await?;
        Ok(true)
    }

    /// Helper for PATCH requests with JSON body that return JSON.
    async fn patch_json<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &B,
        action: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .patch(&url)
            .query(&[("directory", &self.directory)])
            .json(body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        Ok(response.json().await?)
    }

    /// Helper for DELETE requests that return JSON.
    async fn delete_json<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        action: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .delete(&url)
            .query(&[("directory", &self.directory)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        Ok(response.json().await?)
    }

    /// Helper for DELETE requests that return boolean.
    async fn delete_bool(&self, endpoint: &str, action: &str) -> Result<bool> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .delete(&url)
            .query(&[("directory", &self.directory)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, action).await?;
        // Consume the response body to allow connection reuse
        let _ = response.bytes().await?;
        Ok(true)
    }

    // ========================================================================
    // Public API methods
    // ========================================================================

    pub async fn list_sessions(&self) -> Result<Vec<Session>> {
        self.get_json("/session", "list sessions").await
    }

    pub async fn list_global_sessions(
        &self,
        roots: Option<bool>,
        start: Option<f64>,
        cursor: Option<f64>,
        search: Option<&str>,
        limit: Option<f64>,
        archived: Option<bool>,
    ) -> Result<Vec<GlobalSession>> {
        let url = format!("{}/experimental/session", self.base_url);
        let mut query = vec![("directory", self.directory.clone())];

        if let Some(roots) = roots {
            query.push(("roots", roots.to_string()));
        }
        if let Some(start) = start {
            query.push(("start", start.to_string()));
        }
        if let Some(cursor) = cursor {
            query.push(("cursor", cursor.to_string()));
        }
        if let Some(search) = search {
            query.push(("search", search.to_string()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(archived) = archived {
            query.push(("archived", archived.to_string()));
        }

        let response = self
            .http
            .get(&url)
            .query(&query)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "list global sessions").await?;
        Ok(response.json().await?)
    }

    pub async fn create_session(&self) -> Result<Session> {
        let body = serde_json::json!({});
        self.post_json("/session", &body, "create session").await
    }

    pub async fn get_session(&self, id: &str) -> Result<Session> {
        let endpoint = format!("/session/{}", id);
        self.get_json(&endpoint, "get session").await
    }

    pub async fn send_prompt(&self, session_id: &str, text: &str) -> Result<()> {
        let endpoint = format!("/session/{}/message", session_id);
        let body = serde_json::json!({
            "parts": vec![PartInput::text(text)],
        });
        self.post_json_bool(&endpoint, &body, "send prompt").await?;
        Ok(())
    }

    // ========================================================================
    // Global APIs
    // ========================================================================

    /// Check server health and version.
    ///
    /// Uses the `/global/health` endpoint.
    pub async fn health(&self) -> Result<HealthResponse> {
        let url = format!("{}/global/health", self.base_url);
        let response = self
            .http
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;
        let response = Self::check_response(response, "get health").await?;
        Ok(response.json().await?)
    }

    pub async fn get_global_config(&self) -> Result<Config> {
        let url = format!("{}/global/config", self.base_url);
        let response = self
            .http
            .get(&url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;
        let response = Self::check_response(response, "get global config").await?;
        Ok(response.json().await?)
    }

    pub async fn update_global_config(&self, config: &Config) -> Result<Config> {
        let url = format!("{}/global/config", self.base_url);
        let response = self
            .http
            .patch(&url)
            .json(config)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;
        let response = Self::check_response(response, "update global config").await?;
        Ok(response.json().await?)
    }

    pub async fn dispose_global(&self) -> Result<bool> {
        self.post_no_body_bool("/global/dispose", "dispose global")
            .await
    }

    // ========================================================================
    // App APIs
    // ========================================================================

    pub async fn log(&self, request: LogRequest) -> Result<bool> {
        self.post_json_bool("/log", &request, "log").await
    }

    pub async fn agents(&self) -> Result<Vec<Agent>> {
        self.get_json("/agent", "get agents").await
    }

    pub async fn list_skills(&self) -> Result<Vec<Skill>> {
        self.get_json("/skill", "list skills").await
    }

    // ========================================================================
    // Project APIs
    // ========================================================================

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.get_json("/project", "list projects").await
    }

    pub async fn current_project(&self) -> Result<Project> {
        self.get_json("/project/current", "get current project")
            .await
    }

    pub async fn update_project(
        &self,
        project_id: &str,
        request: ProjectUpdateRequest,
    ) -> Result<Project> {
        let endpoint = format!("/project/{}", project_id);
        self.patch_json(&endpoint, &request, "update project").await
    }

    // ========================================================================
    // Path APIs
    // ========================================================================

    pub async fn get_path(&self) -> Result<PathInfo> {
        self.get_json("/path", "get path").await
    }

    // ========================================================================
    // Config APIs
    // ========================================================================

    pub async fn get_config(&self) -> Result<Config> {
        self.get_json("/config", "get config").await
    }

    pub async fn get_providers(&self) -> Result<ProvidersResponse> {
        self.get_json("/config/providers", "get providers").await
    }

    // ========================================================================
    // Extended Session APIs
    // ========================================================================

    pub async fn create_session_with_options(
        &self,
        request: SessionCreateRequest,
    ) -> Result<Session> {
        self.post_json("/session", &request, "create session").await
    }

    pub async fn get_session_children(&self, session_id: &str) -> Result<Vec<Session>> {
        let endpoint = format!("/session/{}/children", session_id);
        self.get_json(&endpoint, "get session children").await
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<bool> {
        let endpoint = format!("/session/{}", session_id);
        self.delete_bool(&endpoint, "delete session").await
    }

    pub async fn update_session(
        &self,
        session_id: &str,
        request: SessionUpdateRequest,
    ) -> Result<Session> {
        let endpoint = format!("/session/{}", session_id);
        self.patch_json(&endpoint, &request, "update session").await
    }

    pub async fn init_session(
        &self,
        session_id: &str,
        request: SessionInitRequest,
    ) -> Result<bool> {
        let endpoint = format!("/session/{}/init", session_id);
        self.post_json_bool(&endpoint, &request, "init session")
            .await
    }

    pub async fn abort_session(&self, session_id: &str) -> Result<bool> {
        let endpoint = format!("/session/{}/abort", session_id);
        self.post_no_body_bool(&endpoint, "abort session").await
    }

    pub async fn share_session(&self, session_id: &str) -> Result<Session> {
        let endpoint = format!("/session/{}/share", session_id);
        self.post_no_body_json(&endpoint, "share session").await
    }

    pub async fn unshare_session(&self, session_id: &str) -> Result<Session> {
        let endpoint = format!("/session/{}/share", session_id);
        self.delete_json(&endpoint, "unshare session").await
    }

    pub async fn summarize_session(
        &self,
        session_id: &str,
        request: SessionSummarizeRequest,
    ) -> Result<bool> {
        let endpoint = format!("/session/{}/summarize", session_id);
        self.post_json_bool(&endpoint, &request, "summarize session")
            .await
    }

    pub async fn session_diff(
        &self,
        session_id: &str,
        message_id: Option<&str>,
    ) -> Result<Vec<FileDiff>> {
        let endpoint = if let Some(message_id) = message_id {
            format!("/session/{}/diff?messageID={}", session_id, message_id)
        } else {
            format!("/session/{}/diff", session_id)
        };
        self.get_json(&endpoint, "get session diff").await
    }

    pub async fn list_messages(&self, session_id: &str) -> Result<Vec<MessageWithParts>> {
        let endpoint = format!("/session/{}/message", session_id);
        self.get_json(&endpoint, "list messages").await
    }

    pub async fn get_message(
        &self,
        session_id: &str,
        message_id: &str,
    ) -> Result<MessageWithParts> {
        let endpoint = format!("/session/{}/message/{}", session_id, message_id);
        self.get_json(&endpoint, "get message").await
    }

    pub async fn send_prompt_with_options(
        &self,
        session_id: &str,
        request: PromptRequest,
    ) -> Result<Message> {
        let endpoint = format!("/session/{}/message", session_id);
        self.post_json(&endpoint, &request, "send prompt").await
    }

    pub async fn send_command(
        &self,
        session_id: &str,
        request: CommandRequest,
    ) -> Result<MessageWithParts> {
        let endpoint = format!("/session/{}/command", session_id);
        self.post_json(&endpoint, &request, "send command").await
    }

    pub async fn send_shell(&self, session_id: &str, request: ShellRequest) -> Result<Message> {
        let endpoint = format!("/session/{}/shell", session_id);
        self.post_json(&endpoint, &request, "send shell command")
            .await
    }

    pub async fn revert_message(
        &self,
        session_id: &str,
        request: RevertRequest,
    ) -> Result<Session> {
        let endpoint = format!("/session/{}/revert", session_id);
        self.post_json(&endpoint, &request, "revert message").await
    }

    pub async fn unrevert_session(&self, session_id: &str) -> Result<Session> {
        let endpoint = format!("/session/{}/unrevert", session_id);
        self.post_no_body_json(&endpoint, "unrevert session").await
    }

    pub async fn respond_to_permission(
        &self,
        session_id: &str,
        permission_id: &str,
        permission_response: PermissionResponse,
    ) -> Result<bool> {
        let endpoint = format!("/session/{}/permissions/{}", session_id, permission_id);
        self.post_json_bool(&endpoint, &permission_response, "respond to permission")
            .await
    }

    pub async fn reply_to_permission(
        &self,
        request_id: &str,
        permission_reply: PermissionReplyRequest,
    ) -> Result<bool> {
        let endpoint = format!("/permission/{}/reply", request_id);
        self.post_json_bool(&endpoint, &permission_reply, "reply to permission")
            .await
    }

    /// List all pending permission requests.
    pub async fn list_pending_permissions(&self) -> Result<Vec<PermissionRequest>> {
        self.get_json("/permission", "list pending permissions")
            .await
    }

    pub async fn list_mcp_resources(&self) -> Result<std::collections::HashMap<String, McpResource>> {
        self.get_json("/experimental/resource", "list mcp resources")
            .await
    }

    pub async fn list_mcp_status(&self) -> Result<std::collections::HashMap<String, MCPStatus>> {
        self.get_json("/mcp", "list mcp status").await
    }

    pub async fn add_mcp_server(
        &self,
        request: McpAddRequest,
    ) -> Result<std::collections::HashMap<String, MCPStatus>> {
        self.post_json("/mcp", &request, "add mcp server").await
    }

    pub async fn list_tool_ids(&self) -> Result<ToolIDs> {
        self.get_json("/experimental/tool/ids", "list tool ids").await
    }

    pub async fn list_tools(&self, provider: &str, model: &str) -> Result<ToolList> {
        let url = format!("{}/experimental/tool", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("provider", provider)])
            .query(&[("model", model)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "list tools").await?;
        Ok(response.json().await?)
    }

    // ========================================================================
    // File/Find APIs
    // ========================================================================

    pub async fn search_text(&self, request: TextSearchRequest) -> Result<Vec<TextSearchResult>> {
        let url = format!("{}/find", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("pattern", &request.pattern)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "search text").await?;
        Ok(response.json().await?)
    }

    pub async fn search_files(&self, request: FilesSearchRequest) -> Result<Vec<String>> {
        let url = format!("{}/find/file", self.base_url);

        // Use request.directory if provided, otherwise use self.directory
        let directory = request
            .directory
            .as_ref()
            .unwrap_or(&self.directory)
            .to_string();
        let mut query = vec![("directory", directory), ("query", request.query.clone())];

        if let Some(type_filter) = &request.type_filter {
            query.push(("type", type_filter.clone()));
        }
        if let Some(limit) = request.limit {
            query.push(("limit", limit.to_string()));
        }

        let response = self
            .http
            .get(&url)
            .query(&query)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "search files").await?;
        Ok(response.json().await?)
    }

    pub async fn search_symbols(&self, request: SymbolsSearchRequest) -> Result<Vec<Symbol>> {
        let url = format!("{}/find/symbol", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("query", &request.query)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "search symbols").await?;
        Ok(response.json().await?)
    }

    pub async fn list_files(&self, path: &str) -> Result<Vec<FileNode>> {
        let url = format!("{}/file", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("path", path)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "list files").await?;
        Ok(response.json().await?)
    }

    pub async fn read_file(&self, request: FileReadRequest) -> Result<FileReadResponse> {
        let url = format!("{}/file/content", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("path", &request.path)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "read file").await?;
        Ok(response.json().await?)
    }

    pub async fn get_file_status(&self, request: Option<FileStatusRequest>) -> Result<Vec<File>> {
        let url = format!("{}/file/status", self.base_url);
        let mut query = vec![("directory", self.directory.clone())];

        if let Some(req) = request {
            if let Some(path) = req.path {
                query.push(("path", path));
            }
        }

        let response = self
            .http
            .get(&url)
            .query(&query)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let response = Self::check_response(response, "get file status").await?;
        Ok(response.json().await?)
    }

    // ========================================================================
    // TUI APIs
    // ========================================================================

    pub async fn append_prompt(&self, request: AppendPromptRequest) -> Result<bool> {
        self.post_json_bool("/tui/append-prompt", &request, "append prompt")
            .await
    }

    pub async fn open_help(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/open-help", "open help").await
    }

    pub async fn open_sessions(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/open-sessions", "open sessions")
            .await
    }

    pub async fn open_themes(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/open-themes", "open themes")
            .await
    }

    pub async fn open_models(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/open-models", "open models")
            .await
    }

    pub async fn submit_prompt(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/submit-prompt", "submit prompt")
            .await
    }

    pub async fn clear_prompt(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/clear-prompt", "clear prompt")
            .await
    }

    pub async fn execute_command(&self, request: ExecuteCommandRequest) -> Result<bool> {
        self.post_json_bool("/tui/execute-command", &request, "execute command")
            .await
    }

    pub async fn show_toast(&self, request: ShowToastRequest) -> Result<bool> {
        self.post_json_bool("/tui/show-toast", &request, "show toast")
            .await
    }

    // ========================================================================
    // Auth APIs
    // ========================================================================

    pub async fn set_auth(&self, provider_id: &str, request: AuthSetRequest) -> Result<bool> {
        let endpoint = format!("/auth/{}", provider_id);
        self.post_json_bool(&endpoint, &request, "set auth").await
    }

    // ========================================================================
    // SSE Event Subscription
    // ========================================================================

    pub async fn subscribe(&self) -> Result<broadcast::Receiver<Event>> {
        use futures_util::StreamExt;

        let url = format!("{}/global/event", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            // No timeout for the long-running SSE stream.
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::Connection(format!(
                "Failed to connect to SSE stream: {}",
                response.status()
            )));
        }

        let event_tx = self.event_tx.clone();

        // Spawn task to read SSE stream
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));

                        // Prevent memory exhaustion from oversized buffer
                        if buffer.len() > MAX_SSE_BUFFER_SIZE {
                            let _ = event_tx.send(Event::Error(
                                "SSE buffer limit exceeded (potential DoS)".to_string(),
                            ));
                            break;
                        }

                        // Parse SSE format: "data: {...}\n\n"
                        while let Some(idx) = buffer.find("\n\n") {
                            let event_str = &buffer[..idx];

                            if let Some(data) = event_str.strip_prefix("data: ") {
                                if let Some(event) = parse_sse_event(data) {
                                    let _ = event_tx.send(event);
                                }
                            }

                            buffer = buffer[idx + 2..].to_string();
                        }
                    }
                    Err(e) => {
                        let _ = event_tx.send(Event::Error(format!("Stream error: {}", e)));
                        break;
                    }
                }
            }
        });

        Ok(self.event_tx.subscribe())
    }
}

fn parse_sse_event(data: &str) -> Option<Event> {
    let value: serde_json::Value = serde_json::from_str(data).ok()?;

    // SSE events are wrapped in a "payload" envelope
    let payload = value.get("payload").unwrap_or(&value);

    let event_type = payload.get("type")?.as_str()?;
    let props = payload.get("properties")?;

    match event_type {
        "session.created" => {
            let session: Session = serde_json::from_value(props.get("info")?.clone()).ok()?;
            Some(Event::SessionCreated(session))
        }
        "session.updated" => {
            let session: Session = serde_json::from_value(props.get("info")?.clone()).ok()?;
            Some(Event::SessionUpdated(session))
        }
        "session.deleted" => {
            let session: Session = serde_json::from_value(props.get("info")?.clone()).ok()?;
            Some(Event::SessionDeleted(session))
        }
        "session.status" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let status: crate::SessionStatus =
                serde_json::from_value(props.get("status")?.clone()).ok()?;
            Some(Event::SessionStatus { session_id, status })
        }
        "session.idle" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            Some(Event::SessionIdle { session_id })
        }
        "session.compacted" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            Some(Event::SessionCompacted { session_id })
        }
        "session.diff" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let diff: Vec<FileDiff> = serde_json::from_value(props.get("diff")?.clone()).ok()?;
            Some(Event::SessionDiff { session_id, diff })
        }
        "message.updated" => {
            let message: Message = serde_json::from_value(props.get("info")?.clone()).ok()?;
            Some(Event::MessageUpdated(message))
        }
        "message.removed" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let message_id = props.get("messageID")?.as_str()?.to_string();
            Some(Event::MessageRemoved {
                session_id,
                message_id,
            })
        }
        "message.part.updated" => {
            let part: Part = serde_json::from_value(props.get("part")?.clone()).ok()?;
            let delta = props
                .get("delta")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            Some(Event::PartUpdated { part, delta })
        }
        "message.part.delta" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let message_id = props.get("messageID")?.as_str()?.to_string();
            let part_id = props.get("partID")?.as_str()?.to_string();
            let field = props.get("field")?.as_str()?.to_string();
            let delta = props.get("delta")?.as_str()?.to_string();
            Some(Event::PartDelta {
                session_id,
                message_id,
                part_id,
                field,
                delta,
            })
        }
        "message.part.removed" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let message_id = props.get("messageID")?.as_str()?.to_string();
            let part_id = props.get("partID")?.as_str()?.to_string();
            Some(Event::PartRemoved {
                session_id,
                message_id,
                part_id,
            })
        }
        "session.error" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let error: AssistantError = serde_json::from_value(props.get("error")?.clone()).ok()?;
            Some(Event::SessionError { session_id, error })
        }
        "permission.asked" => {
            let request: PermissionRequest = serde_json::from_value(props.clone()).ok()?;
            Some(Event::PermissionAsked(request))
        }
        "permission.replied" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let request_id = props.get("requestID")?.as_str()?.to_string();
            let reply: PermissionReply =
                serde_json::from_value(props.get("reply")?.clone()).ok()?;
            Some(Event::PermissionReplied {
                session_id,
                request_id,
                reply,
            })
        }
        "question.asked" => {
            let request: crate::QuestionRequest = serde_json::from_value(props.clone()).ok()?;
            Some(Event::QuestionAsked(request))
        }
        "question.replied" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let request_id = props.get("requestID")?.as_str()?.to_string();
            let answers: Vec<Vec<String>> =
                serde_json::from_value(props.get("answers")?.clone()).ok()?;
            Some(Event::QuestionReplied {
                session_id,
                request_id,
                answers,
            })
        }
        "question.rejected" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let request_id = props.get("requestID")?.as_str()?.to_string();
            Some(Event::QuestionRejected {
                session_id,
                request_id,
            })
        }
        "todo.updated" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let todos: Vec<Todo> = serde_json::from_value(props.get("todos")?.clone()).ok()?;
            Some(Event::TodoUpdated { session_id, todos })
        }
        "tui.prompt.append" => {
            let text = props.get("text")?.as_str()?.to_string();
            Some(Event::TuiPromptAppend { text })
        }
        "tui.command.execute" => {
            let command = props.get("command")?.as_str()?.to_string();
            Some(Event::TuiCommandExecute { command })
        }
        "tui.toast.show" => {
            let title = props
                .get("title")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let message = props.get("message")?.as_str()?.to_string();
            let variant = props.get("variant")?.as_str()?.to_string();
            let duration = props.get("duration").and_then(|v| v.as_f64());
            Some(Event::TuiToastShow {
                title,
                message,
                variant,
                duration,
            })
        }
        "tui.session.select" => {
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            Some(Event::TuiSessionSelect { session_id })
        }
        "pty.created" => {
            let info: Pty = serde_json::from_value(props.get("info")?.clone()).ok()?;
            Some(Event::PtyCreated(info))
        }
        "pty.updated" => {
            let info: Pty = serde_json::from_value(props.get("info")?.clone()).ok()?;
            Some(Event::PtyUpdated(info))
        }
        "pty.exited" => {
            let id = props.get("id")?.as_str()?.to_string();
            let exit_code = props.get("exitCode")?.as_i64()?;
            Some(Event::PtyExited { id, exit_code })
        }
        "pty.deleted" => {
            let id = props.get("id")?.as_str()?.to_string();
            Some(Event::PtyDeleted { id })
        }
        "project.updated" => {
            let info: Project = serde_json::from_value(props.clone()).ok()?;
            Some(Event::ProjectUpdated(info))
        }
        "vcs.branch.updated" => {
            let branch = props.get("branch")?.as_str()?.to_string();
            Some(Event::VcsBranchUpdated { branch })
        }
        "file.edited" => {
            let file = props.get("file")?.as_str()?.to_string();
            Some(Event::FileEdited { file })
        }
        "file.watcher.updated" => {
            let file = props.get("file")?.as_str()?.to_string();
            let event = props.get("event")?.as_str()?.to_string();
            Some(Event::FileWatcherUpdated { file, event })
        }
        "lsp.updated" => Some(Event::LspUpdated),
        "lsp.client.diagnostics" => {
            let server_id = props.get("serverID")?.as_str()?.to_string();
            let path = props.get("path")?.as_str()?.to_string();
            Some(Event::LspDiagnostics { server_id, path })
        }
        "worktree.ready" => {
            let name = props.get("name")?.as_str()?.to_string();
            let branch = props.get("branch")?.as_str()?.to_string();
            Some(Event::WorktreeReady { name, branch })
        }
        "worktree.failed" => {
            let message = props.get("message")?.as_str()?.to_string();
            Some(Event::WorktreeFailed { message })
        }
        "mcp.tools.changed" => {
            let server = props.get("server")?.as_str()?.to_string();
            Some(Event::McpToolsChanged { server })
        }
        "mcp.browser.open.failed" => {
            let mcp_name = props.get("mcpName")?.as_str()?.to_string();
            let url = props.get("url")?.as_str()?.to_string();
            Some(Event::McpBrowserOpenFailed { mcp_name, url })
        }
        "command.executed" => {
            let name = props.get("name")?.as_str()?.to_string();
            let session_id = props.get("sessionID")?.as_str()?.to_string();
            let arguments = props.get("arguments")?.as_str()?.to_string();
            let message_id = props.get("messageID")?.as_str()?.to_string();
            Some(Event::CommandExecuted {
                name,
                session_id,
                arguments,
                message_id,
            })
        }
        "installation.updated" => {
            let version = props.get("version")?.as_str()?.to_string();
            Some(Event::InstallationUpdated { version })
        }
        "installation.update-available" => {
            let version = props.get("version")?.as_str()?.to_string();
            Some(Event::InstallationUpdateAvailable { version })
        }
        "server.connected" => Some(Event::ServerConnected),
        "global.disposed" => Some(Event::GlobalDisposed),
        "server.instance.disposed" => {
            let directory = props.get("directory")?.as_str()?.to_string();
            Some(Event::ServerInstanceDisposed { directory })
        }
        _ => Some(Event::Unknown(event_type.to_string())),
    }
}
