//! OpenCode HTTP client implementation.
//!
//! This module provides a complete client for the OpenCode server API,
//! including REST endpoints and Server-Sent Events (SSE) subscription.

use crate::{
    Agent, AppendPromptRequest, AuthSetRequest, CommandRequest, Config, ExecuteCommandRequest,
    File, FileReadRequest, FileReadResponse, FileStatusRequest, FilesSearchRequest, HealthResponse,
    LogRequest, MessageWithParts, PathInfo, PermissionReply, PermissionReplyRequest,
    PermissionRequest, PermissionResponse, Project, PromptRequest, ProvidersResponse,
    RevertRequest, SessionCreateRequest, SessionInitRequest, SessionSummarizeRequest,
    SessionUpdateRequest, ShellRequest, ShowToastRequest, Symbol, SymbolsSearchRequest,
    TextSearchRequest, TextSearchResult,
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
    fn check_response(response: &reqwest::Response, action: &str) -> Result<()> {
        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to {}: {}",
                action,
                response.status()
            )));
        }
        Ok(())
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
            .send()
            .await?;

        Self::check_response(&response, action)?;
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
            .send()
            .await?;

        Self::check_response(&response, action)?;
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
            .send()
            .await?;

        Self::check_response(&response, action)?;
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
            .send()
            .await?;

        Self::check_response(&response, action)?;
        Ok(response.json().await?)
    }

    /// Helper for POST requests without body that return boolean.
    async fn post_no_body_bool(&self, endpoint: &str, action: &str) -> Result<bool> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        Self::check_response(&response, action)?;
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
            .send()
            .await?;

        Self::check_response(&response, action)?;
        Ok(response.json().await?)
    }

    /// Helper for DELETE requests that return boolean.
    async fn delete_bool(&self, endpoint: &str, action: &str) -> Result<bool> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http
            .delete(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        Self::check_response(&response, action)?;
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
        let response = self.http.get(&url).send().await?;
        Self::check_response(&response, "get health")?;
        Ok(response.json().await?)
    }

    // ========================================================================
    // App APIs
    // ========================================================================

    pub async fn log(&self, request: LogRequest) -> Result<bool> {
        self.post_json_bool("/app/log", &request, "log").await
    }

    pub async fn agents(&self) -> Result<Vec<Agent>> {
        self.get_json("/app/agents", "get agents").await
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
        let endpoint = format!("/session/{}/unshare", session_id);
        self.post_no_body_json(&endpoint, "unshare session").await
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

    // ========================================================================
    // File/Find APIs
    // ========================================================================

    pub async fn search_text(&self, request: TextSearchRequest) -> Result<Vec<TextSearchResult>> {
        let url = format!("{}/find/text", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("pattern", &request.pattern)])
            .send()
            .await?;

        Self::check_response(&response, "search text")?;
        Ok(response.json().await?)
    }

    pub async fn search_files(&self, request: FilesSearchRequest) -> Result<Vec<String>> {
        let url = format!("{}/find/files", self.base_url);

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

        let response = self.http.get(&url).query(&query).send().await?;

        Self::check_response(&response, "search files")?;
        Ok(response.json().await?)
    }

    pub async fn search_symbols(&self, request: SymbolsSearchRequest) -> Result<Vec<Symbol>> {
        let url = format!("{}/find/symbols", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("query", &request.query)])
            .send()
            .await?;

        Self::check_response(&response, "search symbols")?;
        Ok(response.json().await?)
    }

    pub async fn read_file(&self, request: FileReadRequest) -> Result<FileReadResponse> {
        let url = format!("{}/file/read", self.base_url);
        let response = self
            .http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("path", &request.path)])
            .send()
            .await?;

        Self::check_response(&response, "read file")?;
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

        let response = self.http.get(&url).query(&query).send().await?;

        Self::check_response(&response, "get file status")?;
        Ok(response.json().await?)
    }

    // ========================================================================
    // TUI APIs
    // ========================================================================

    pub async fn append_prompt(&self, request: AppendPromptRequest) -> Result<bool> {
        self.post_json_bool("/tui/appendPrompt", &request, "append prompt")
            .await
    }

    pub async fn open_help(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/openHelp", "open help").await
    }

    pub async fn open_sessions(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/openSessions", "open sessions")
            .await
    }

    pub async fn open_themes(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/openThemes", "open themes")
            .await
    }

    pub async fn open_models(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/openModels", "open models")
            .await
    }

    pub async fn submit_prompt(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/submitPrompt", "submit prompt")
            .await
    }

    pub async fn clear_prompt(&self) -> Result<bool> {
        self.post_no_body_bool("/tui/clearPrompt", "clear prompt")
            .await
    }

    pub async fn execute_command(&self, request: ExecuteCommandRequest) -> Result<bool> {
        self.post_json_bool("/tui/executeCommand", &request, "execute command")
            .await
    }

    pub async fn show_toast(&self, request: ShowToastRequest) -> Result<bool> {
        self.post_json_bool("/tui/showToast", &request, "show toast")
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
        _ => Some(Event::Unknown(event_type.to_string())),
    }
}
