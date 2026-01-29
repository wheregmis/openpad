//! OpenCode HTTP client implementation.
//!
//! This module provides a complete client for the OpenCode server API,
//! including REST endpoints and Server-Sent Events (SSE) subscription.

use crate::{Error, Result, Event, Session, Message, PartInput, Part};
use crate::{
    HealthResponse, LogRequest, Agent, Project, PathInfo, Config, ProvidersResponse,
    TextSearchRequest, TextSearchResult, FilesSearchRequest, SymbolsSearchRequest, Symbol,
    FileReadRequest, FileReadResponse, FileStatusRequest, File,
    AppendPromptRequest, ExecuteCommandRequest, ShowToastRequest, AuthSetRequest,
    SessionCreateRequest, SessionUpdateRequest, SessionInitRequest, SessionSummarizeRequest,
    MessageWithParts, PromptRequest, CommandRequest, ShellRequest, RevertRequest, PermissionResponse,
};
use reqwest::Client as HttpClient;
use tokio::sync::broadcast;
use std::env;

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

    pub async fn list_sessions(&self) -> Result<Vec<Session>> {
        let url = format!("{}/session", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to list sessions: {}",
                response.status()
            )));
        }

        let sessions: Vec<Session> = response.json().await?;
        Ok(sessions)
    }

    pub async fn create_session(&self) -> Result<Session> {
        let url = format!("{}/session", self.base_url);
        let body = serde_json::json!({});

        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to create session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn get_session(&self, id: &str) -> Result<Session> {
        let url = format!("{}/session/{}", self.base_url, id);

        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn send_prompt(&self, session_id: &str, text: &str) -> Result<()> {
        let url = format!("{}/session/{}/prompt", self.base_url, session_id);
        let body = serde_json::json!({
            "parts": vec![PartInput::text(text)],
        });

        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to send prompt: {}",
                response.status()
            )));
        }

        Ok(())
    }

    // ========================================================================
    // Global APIs
    // ========================================================================

    pub async fn health(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.base_url);
        let response = self.http.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get health: {}",
                response.status()
            )));
        }

        let health: HealthResponse = response.json().await?;
        Ok(health)
    }

    // ========================================================================
    // App APIs
    // ========================================================================

    pub async fn log(&self, request: LogRequest) -> Result<bool> {
        let url = format!("{}/app/log", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to log: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn agents(&self) -> Result<Vec<Agent>> {
        let url = format!("{}/app/agents", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get agents: {}",
                response.status()
            )));
        }

        let agents: Vec<Agent> = response.json().await?;
        Ok(agents)
    }

    // ========================================================================
    // Project APIs
    // ========================================================================

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let url = format!("{}/project", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to list projects: {}",
                response.status()
            )));
        }

        let projects: Vec<Project> = response.json().await?;
        Ok(projects)
    }

    pub async fn current_project(&self) -> Result<Project> {
        let url = format!("{}/project/current", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get current project: {}",
                response.status()
            )));
        }

        let project: Project = response.json().await?;
        Ok(project)
    }

    // ========================================================================
    // Path APIs
    // ========================================================================

    pub async fn get_path(&self) -> Result<PathInfo> {
        let url = format!("{}/path", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get path: {}",
                response.status()
            )));
        }

        let path: PathInfo = response.json().await?;
        Ok(path)
    }

    // ========================================================================
    // Config APIs
    // ========================================================================

    pub async fn get_config(&self) -> Result<Config> {
        let url = format!("{}/config", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get config: {}",
                response.status()
            )));
        }

        let config: Config = response.json().await?;
        Ok(config)
    }

    pub async fn get_providers(&self) -> Result<ProvidersResponse> {
        let url = format!("{}/config/providers", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get providers: {}",
                response.status()
            )));
        }

        let providers: ProvidersResponse = response.json().await?;
        Ok(providers)
    }

    // ========================================================================
    // Extended Session APIs
    // ========================================================================

    pub async fn create_session_with_options(&self, request: SessionCreateRequest) -> Result<Session> {
        let url = format!("{}/session", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to create session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn get_session_children(&self, session_id: &str) -> Result<Vec<Session>> {
        let url = format!("{}/session/{}/children", self.base_url, session_id);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get session children: {}",
                response.status()
            )));
        }

        let sessions: Vec<Session> = response.json().await?;
        Ok(sessions)
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<bool> {
        let url = format!("{}/session/{}", self.base_url, session_id);
        let response = self.http
            .delete(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to delete session: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn update_session(&self, session_id: &str, request: SessionUpdateRequest) -> Result<Session> {
        let url = format!("{}/session/{}", self.base_url, session_id);
        let response = self.http
            .patch(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to update session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn init_session(&self, session_id: &str, request: SessionInitRequest) -> Result<bool> {
        let url = format!("{}/session/{}/init", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to init session: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn abort_session(&self, session_id: &str) -> Result<bool> {
        let url = format!("{}/session/{}/abort", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to abort session: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn share_session(&self, session_id: &str) -> Result<Session> {
        let url = format!("{}/session/{}/share", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to share session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn unshare_session(&self, session_id: &str) -> Result<Session> {
        let url = format!("{}/session/{}/unshare", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to unshare session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn summarize_session(&self, session_id: &str, request: SessionSummarizeRequest) -> Result<bool> {
        let url = format!("{}/session/{}/summarize", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to summarize session: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn list_messages(&self, session_id: &str) -> Result<Vec<MessageWithParts>> {
        let url = format!("{}/session/{}/messages", self.base_url, session_id);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to list messages: {}",
                response.status()
            )));
        }

        let messages: Vec<MessageWithParts> = response.json().await?;
        Ok(messages)
    }

    pub async fn get_message(&self, session_id: &str, message_id: &str) -> Result<MessageWithParts> {
        let url = format!("{}/session/{}/messages/{}", self.base_url, session_id, message_id);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get message: {}",
                response.status()
            )));
        }

        let message: MessageWithParts = response.json().await?;
        Ok(message)
    }

    pub async fn send_prompt_with_options(&self, session_id: &str, request: PromptRequest) -> Result<Message> {
        let url = format!("{}/session/{}/prompt", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to send prompt: {}",
                response.status()
            )));
        }

        let message: Message = response.json().await?;
        Ok(message)
    }

    pub async fn send_command(&self, session_id: &str, request: CommandRequest) -> Result<MessageWithParts> {
        let url = format!("{}/session/{}/command", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to send command: {}",
                response.status()
            )));
        }

        let message: MessageWithParts = response.json().await?;
        Ok(message)
    }

    pub async fn send_shell(&self, session_id: &str, request: ShellRequest) -> Result<Message> {
        let url = format!("{}/session/{}/shell", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to send shell command: {}",
                response.status()
            )));
        }

        let message: Message = response.json().await?;
        Ok(message)
    }

    pub async fn revert_message(&self, session_id: &str, request: RevertRequest) -> Result<Session> {
        let url = format!("{}/session/{}/revert", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to revert message: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn unrevert_session(&self, session_id: &str) -> Result<Session> {
        let url = format!("{}/session/{}/unrevert", self.base_url, session_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to unrevert session: {}",
                response.status()
            )));
        }

        let session: Session = response.json().await?;
        Ok(session)
    }

    pub async fn respond_to_permission(&self, session_id: &str, permission_id: &str, response: PermissionResponse) -> Result<bool> {
        let url = format!("{}/session/{}/permissions/{}", self.base_url, session_id, permission_id);
        let resp = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&response)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to respond to permission: {}",
                resp.status()
            )));
        }

        Ok(true)
    }

    // ========================================================================
    // File/Find APIs
    // ========================================================================

    pub async fn search_text(&self, request: TextSearchRequest) -> Result<Vec<TextSearchResult>> {
        let url = format!("{}/find/text", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("pattern", &request.pattern)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to search text: {}",
                response.status()
            )));
        }

        let results: Vec<TextSearchResult> = response.json().await?;
        Ok(results)
    }

    pub async fn search_files(&self, request: FilesSearchRequest) -> Result<Vec<String>> {
        let url = format!("{}/find/files", self.base_url);
        let mut query = vec![
            ("directory", self.directory.clone()),
            ("query", request.query.clone()),
        ];
        
        if let Some(type_filter) = &request.type_filter {
            query.push(("type", type_filter.clone()));
        }
        if let Some(dir) = &request.directory {
            query.push(("directory", dir.clone()));
        }
        if let Some(limit) = request.limit {
            query.push(("limit", limit.to_string()));
        }

        let response = self.http
            .get(&url)
            .query(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to search files: {}",
                response.status()
            )));
        }

        let files: Vec<String> = response.json().await?;
        Ok(files)
    }

    pub async fn search_symbols(&self, request: SymbolsSearchRequest) -> Result<Vec<Symbol>> {
        let url = format!("{}/find/symbols", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("query", &request.query)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to search symbols: {}",
                response.status()
            )));
        }

        let symbols: Vec<Symbol> = response.json().await?;
        Ok(symbols)
    }

    pub async fn read_file(&self, request: FileReadRequest) -> Result<FileReadResponse> {
        let url = format!("{}/file/read", self.base_url);
        let response = self.http
            .get(&url)
            .query(&[("directory", &self.directory)])
            .query(&[("path", &request.path)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to read file: {}",
                response.status()
            )));
        }

        let file: FileReadResponse = response.json().await?;
        Ok(file)
    }

    pub async fn get_file_status(&self, request: Option<FileStatusRequest>) -> Result<Vec<File>> {
        let url = format!("{}/file/status", self.base_url);
        let mut query = vec![("directory", self.directory.clone())];
        
        if let Some(req) = request {
            if let Some(path) = req.path {
                query.push(("path", path));
            }
        }

        let response = self.http
            .get(&url)
            .query(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to get file status: {}",
                response.status()
            )));
        }

        let files: Vec<File> = response.json().await?;
        Ok(files)
    }

    // ========================================================================
    // TUI APIs
    // ========================================================================

    pub async fn append_prompt(&self, request: AppendPromptRequest) -> Result<bool> {
        let url = format!("{}/tui/appendPrompt", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to append prompt: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn open_help(&self) -> Result<bool> {
        let url = format!("{}/tui/openHelp", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to open help: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn open_sessions(&self) -> Result<bool> {
        let url = format!("{}/tui/openSessions", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to open sessions: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn open_themes(&self) -> Result<bool> {
        let url = format!("{}/tui/openThemes", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to open themes: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn open_models(&self) -> Result<bool> {
        let url = format!("{}/tui/openModels", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to open models: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn submit_prompt(&self) -> Result<bool> {
        let url = format!("{}/tui/submitPrompt", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to submit prompt: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn clear_prompt(&self) -> Result<bool> {
        let url = format!("{}/tui/clearPrompt", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to clear prompt: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn execute_command(&self, request: ExecuteCommandRequest) -> Result<bool> {
        let url = format!("{}/tui/executeCommand", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to execute command: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn show_toast(&self, request: ShowToastRequest) -> Result<bool> {
        let url = format!("{}/tui/showToast", self.base_url);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to show toast: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    // ========================================================================
    // Auth APIs
    // ========================================================================

    pub async fn set_auth(&self, provider_id: &str, request: AuthSetRequest) -> Result<bool> {
        let url = format!("{}/auth/{}", self.base_url, provider_id);
        let response = self.http
            .post(&url)
            .query(&[("directory", &self.directory)])
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::InvalidResponse(format!(
                "Failed to set auth: {}",
                response.status()
            )));
        }

        Ok(true)
    }

    // ========================================================================
    // SSE Event Subscription
    // ========================================================================


    pub async fn subscribe(&self) -> Result<broadcast::Receiver<Event>> {
        use futures_util::StreamExt;

        let url = format!("{}/event", self.base_url);
        let response = self.http
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

    let event_type = value.get("type")?.as_str()?;
    let props = value.get("properties")?;

    match event_type {
        "session.created" => {
            let session: Session = serde_json::from_value(props.clone()).ok()?;
            Some(Event::SessionCreated(session))
        }
        "session.deleted" => {
            let id = props.get("id")?.as_str()?.to_string();
            Some(Event::SessionDeleted(id))
        }
        "message.updated" => {
            let session_id = props.get("sessionId")?.as_str()?.to_string();
            let message: Message = serde_json::from_value(props.get("message")?.clone()).ok()?;
            Some(Event::MessageUpdated { session_id, message })
        }
        "message.part.updated" => {
            let session_id = props.get("sessionId")?.as_str()?.to_string();
            let message_id = props.get("messageId")?.as_str()?.to_string();
            let part_index = props.get("index")?.as_u64()? as usize;
            let part: Part = serde_json::from_value(props.get("part")?.clone()).ok()?;
            Some(Event::PartUpdated { session_id, message_id, part_index, part })
        }
        "session.error" => {
            let error = props.get("error")?.as_str()?.to_string();
            Some(Event::Error(error))
        }
        _ => Some(Event::Unknown(event_type.to_string())),
    }
}
