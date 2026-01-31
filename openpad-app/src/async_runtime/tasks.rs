use crate::constants::OPENCODE_SERVER_URL;
use crate::state::actions::AppAction;
use makepad_widgets::{log, Cx};
use openpad_protocol::{
    ModelSpec, OpenCodeClient, PartInput, PermissionReply, PermissionReplyRequest, Project,
    PromptRequest, Session, SessionCreateRequest,
};
use std::sync::Arc;

/// Helper to create a directory-specific client if a directory is provided
fn get_directory_client(
    base_client: Arc<OpenCodeClient>,
    directory: Option<String>,
) -> Arc<OpenCodeClient> {
    if let Some(dir) = directory {
        Arc::new(OpenCodeClient::new(OPENCODE_SERVER_URL).with_directory(dir))
    } else {
        base_client
    }
}

/// Spawns a task to subscribe to SSE events
pub fn spawn_sse_subscriber(runtime: &tokio::runtime::Runtime, client: Arc<OpenCodeClient>) {
    runtime.spawn(async move {
        use tokio::time::{sleep, Duration};

        // Retry connecting until successful
        let sessions = loop {
            match client.list_sessions().await {
                Ok(sessions) => break sessions,
                Err(_) => {
                    sleep(Duration::from_secs(2)).await;
                }
            }
        };

        Cx::post_action(AppAction::Connected);
        Cx::post_action(AppAction::SessionsLoaded(sessions));

        // Subscribe to SSE
        if let Ok(mut rx) = client.subscribe().await {
            while let Ok(event) = rx.recv().await {
                Cx::post_action(AppAction::OpenCodeEvent(event));
            }
        }
    });
}

/// Spawns a task to periodically check health status
pub fn spawn_health_checker(runtime: &tokio::runtime::Runtime, client: Arc<OpenCodeClient>) {
    runtime.spawn(async move {
        use openpad_protocol::HealthResponse;
        use tokio::time::{sleep, Duration};

        loop {
            match client.health().await {
                Ok(health) => Cx::post_action(AppAction::HealthUpdated(health)),
                Err(_) => Cx::post_action(AppAction::HealthUpdated(HealthResponse {
                    healthy: false,
                    version: "unknown".to_string(),
                })),
            }
            sleep(Duration::from_secs(5)).await;
        }
    });
}

/// Spawns a task to load projects and current project
pub fn spawn_project_loader(runtime: &tokio::runtime::Runtime, client: Arc<OpenCodeClient>) {
    runtime.spawn(async move {
        if let Ok(projects) = client.list_projects().await {
            Cx::post_action(AppAction::ProjectsLoaded(projects));
        }
        if let Ok(current) = client.current_project().await {
            Cx::post_action(AppAction::CurrentProjectLoaded(current));
        }
    });
}

/// Normalize a worktree path to an absolute directory, matching how sessions are created.
fn normalize_worktree(worktree: &str) -> String {
    if worktree == "." {
        if let Ok(current_dir) = std::env::current_dir() {
            return current_dir.to_string_lossy().to_string();
        }
    }
    match std::fs::canonicalize(worktree) {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => worktree.to_string(),
    }
}

/// Spawns a task to load sessions for all projects by querying each project's directory
pub fn spawn_all_sessions_loader(
    runtime: &tokio::runtime::Runtime,
    _client: Arc<OpenCodeClient>,
    projects: Vec<Project>,
) {
    // Normalize worktree paths on the main thread (needs filesystem access)
    let normalized: Vec<String> = projects
        .iter()
        .filter(|p| p.worktree != "/" && !p.worktree.is_empty())
        .map(|p| normalize_worktree(&p.worktree))
        .collect();

    runtime.spawn(async move {
        let mut all_sessions: Vec<Session> = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for directory in &normalized {
            let project_client = OpenCodeClient::new(OPENCODE_SERVER_URL).with_directory(directory);

            match project_client.list_sessions().await {
                Ok(sessions) => {
                    for session in sessions {
                        if seen_ids.insert(session.id.clone()) {
                            all_sessions.push(session);
                        }
                    }
                }
                Err(e) => {
                    log!("Failed to load sessions for project {}: {}", directory, e);
                }
            }
        }

        Cx::post_action(AppAction::SessionsLoaded(all_sessions));
    });
}

/// Spawns a task to load messages for a session
pub fn spawn_message_loader(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
    directory: Option<String>,
) {
    runtime.spawn(async move {
        // Use session-specific directory if provided
        let target_client = get_directory_client(client, directory);

        match target_client.list_messages(&session_id).await {
            Ok(messages) => {
                Cx::post_action(AppAction::MessagesLoaded(messages));
            }
            Err(_) => {}
        }
    });
}

/// Spawns a task to send a message (creating session if needed)
pub fn spawn_message_sender(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: Option<String>,
    text: String,
    model_spec: Option<ModelSpec>,
    agent: Option<String>,
    system: Option<String>,
    directory: Option<String>,
    attachments: Vec<PartInput>,
) {
    runtime.spawn(async move {
        let target_client = if let Some(dir) = directory.clone() {
            Arc::new(OpenCodeClient::new(OPENCODE_SERVER_URL).with_directory(dir))
        } else {
            client.clone()
        };

        // Create session if needed
        let sid = if let Some(id) = session_id {
            id
        } else {
            let request = SessionCreateRequest {
                parent_id: None,
                title: None,
                permission: None,
            };

            match target_client.create_session_with_options(request).await {
                Ok(session) => {
                    Cx::post_action(AppAction::SessionCreated(session.clone()));
                    session.id
                }
                Err(e) => {
                    log!("Failed to create session for message send: {}", e);
                    Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
                    return;
                }
            }
        };

        // Build parts: text first, then attachments
        let mut parts = vec![PartInput::text(&text)];
        parts.extend(attachments);

        // Send prompt with optional model selection
        let request = PromptRequest {
            model: model_spec,
            agent,
            system,
            parts,
            no_reply: None,
        };
        if let Err(e) = target_client.send_prompt_with_options(&sid, request).await {
            log!("Failed to send prompt on session {}: {}", sid, e);
            Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
        }
    });
}

/// Spawns a task to create a new session
pub fn spawn_session_creator(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    project_directory: Option<String>,
) {
    runtime.spawn(async move {
        // Create the session request
        let request = SessionCreateRequest {
            parent_id: None,
            title: None,
            permission: None,
        };

        // If a specific directory is provided, create a new client for this request
        // Otherwise, use the default client
        let session_result = if let Some(directory) = project_directory {
            let project_client = OpenCodeClient::new(OPENCODE_SERVER_URL).with_directory(directory);
            project_client.create_session_with_options(request).await
        } else {
            client.create_session_with_options(request).await
        };

        match session_result {
            Ok(session) => {
                Cx::post_action(AppAction::SessionCreated(session));
                // Don't reload sessions here - let SSE handle it to avoid race conditions
                // The SessionCreated SSE event will arrive and add the session to the list
            }
            Err(e) => {
                log!("Failed to create session (new session request): {}", e);
                Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
            }
        }
    });
}

/// Spawns a task to respond to a permission request
pub fn spawn_permission_reply(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
    request_id: String,
    reply: PermissionReply,
) {
    runtime.spawn(async move {
        let response = match reply {
            PermissionReply::Reject => openpad_protocol::PermissionDecision::Reject,
            PermissionReply::Once | PermissionReply::Always => {
                openpad_protocol::PermissionDecision::Allow
            }
        };
        let remember = matches!(reply, PermissionReply::Always);
        let request = PermissionReplyRequest { reply };
        if client
            .reply_to_permission(&request_id, request)
            .await
            .is_ok()
        {
            return;
        }

        let response = openpad_protocol::PermissionResponse {
            response,
            remember: Some(remember),
        };
        if let Err(e) = client
            .respond_to_permission(&session_id, &request_id, response)
            .await
        {
            Cx::post_action(AppAction::SendMessageFailed(format!(
                "Permission response failed: {}",
                e
            )));
        }
    });
}

/// Spawns a task to load pending permission requests
pub fn spawn_pending_permissions_loader(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
) {
    runtime.spawn(async move {
        match client.list_pending_permissions().await {
            Ok(permissions) => {
                Cx::post_action(AppAction::PendingPermissionsLoaded(permissions));
            }
            Err(_) => {}
        }
    });
}

/// Helper to get or create a session title from a session
pub fn get_session_title(session: &Session) -> String {
    if !session.title.is_empty() {
        session.title.clone()
    } else if !session.slug.is_empty() {
        session.slug.clone()
    } else {
        session.id.clone()
    }
}

/// Spawns a task to fetch providers (and their models)
pub fn spawn_providers_loader(runtime: &tokio::runtime::Runtime, client: Arc<OpenCodeClient>) {
    runtime.spawn(async move {
        match client.get_providers().await {
            Ok(providers_response) => {
                Cx::post_action(AppAction::ProvidersLoaded(providers_response));
            }
            Err(e) => {
                eprintln!("Failed to load providers: {}", e);
            }
        }
    });
}

/// Spawns a task to fetch available agents
pub fn spawn_agents_loader(runtime: &tokio::runtime::Runtime, client: Arc<OpenCodeClient>) {
    runtime.spawn(async move {
        match client.agents().await {
            Ok(agents) => {
                Cx::post_action(AppAction::AgentsLoaded(agents));
            }
            Err(e) => {
                eprintln!("Failed to load agents: {}", e);
            }
        }
    });
}

/// Spawns a task to fetch available skills
pub fn spawn_skills_loader(runtime: &tokio::runtime::Runtime, client: Arc<OpenCodeClient>) {
    runtime.spawn(async move {
        match client.list_skills().await {
            Ok(skills) => {
                Cx::post_action(AppAction::SkillsLoaded(skills));
            }
            Err(e) => {
                eprintln!("Failed to load skills: {}", e);
            }
        }
    });
}

/// Spawns a task to delete a session
pub fn spawn_session_deleter(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
    directory: Option<String>,
) {
    runtime.spawn(async move {
        let target_client = get_directory_client(client, directory);
        match target_client.delete_session(&session_id).await {
            Ok(_) => {
                Cx::post_action(AppAction::SessionDeleted(session_id.clone()));
                // Don't reload sessions here - let SSE handle it
            }
            Err(e) => {
                Cx::post_action(AppAction::SendMessageFailed(format!(
                    "Failed to delete session: {}",
                    e
                )));
            }
        }
    });
}

/// Spawns a task to update a session (e.g., rename)
pub fn spawn_session_updater(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
    new_title: String,
    directory: Option<String>,
) {
    use openpad_protocol::SessionUpdateRequest;

    runtime.spawn(async move {
        let target_client = get_directory_client(client, directory);
        let request = SessionUpdateRequest {
            title: Some(new_title),
        };
        match target_client.update_session(&session_id, request).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionUpdated(session.clone()));
                // Don't reload sessions here - let SSE handle it
            }
            Err(e) => {
                Cx::post_action(AppAction::SendMessageFailed(format!(
                    "Failed to rename session: {}",
                    e
                )));
            }
        }
    });
}

/// Spawns a task to abort an ongoing session
pub fn spawn_session_aborter(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
) {
    runtime.spawn(async move {
        match client.abort_session(&session_id).await {
            Ok(_) => {
                // Session aborted successfully
                // SSE will handle the session state update
            }
            Err(e) => {
                Cx::post_action(AppAction::SendMessageFailed(format!(
                    "Failed to abort session: {}",
                    e
                )));
            }
        }
    });
}

/// Spawns a task to branch a session (create child from parent)
pub fn spawn_session_brancher(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    parent_session_id: String,
    directory: Option<String>,
) {
    runtime.spawn(async move {
        let request = SessionCreateRequest {
            parent_id: Some(parent_session_id.clone()),
            title: None,
            permission: None,
        };

        // Use session-specific directory if provided
        let target_client = get_directory_client(client.clone(), directory);

        match target_client.create_session_with_options(request).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionCreated(session));
                // Don't reload sessions here - let SSE handle it to avoid race conditions
                // The SessionCreated SSE event will arrive and add the session to the list
            }
            Err(e) => {
                log!(
                    "Failed to branch session from {}: {}",
                    parent_session_id.clone(),
                    e
                );
                Cx::post_action(AppAction::SendMessageFailed(format!(
                    "Failed to branch session: {}",
                    e
                )));
            }
        }
    });
}

/// Spawns a task to revert session to a specific message
pub fn spawn_message_reverter(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
    message_id: String,
    directory: Option<String>,
) {
    use openpad_protocol::RevertRequest;

    runtime.spawn(async move {
        // Use session-specific directory if provided
        let target_client = get_directory_client(client, directory);

        let request = RevertRequest { message_id };
        match target_client.revert_message(&session_id, request).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionUpdated(session));
                // Reload messages for the session using the same directory-aware client
                if let Ok(messages) = target_client.list_messages(&session_id).await {
                    Cx::post_action(AppAction::MessagesLoaded(messages));
                }
            }
            Err(e) => {
                Cx::post_action(AppAction::SendMessageFailed(format!(
                    "Failed to revert to message: {}",
                    e
                )));
            }
        }
    });
}

/// Spawns a task to unrevert a session
pub fn spawn_session_unreverter(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
    directory: Option<String>,
) {
    runtime.spawn(async move {
        // Use session-specific directory if provided
        let target_client = get_directory_client(client, directory);

        match target_client.unrevert_session(&session_id).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionUpdated(session));
                // Reload messages for the session using the same directory-aware client
                if let Ok(messages) = target_client.list_messages(&session_id).await {
                    Cx::post_action(AppAction::MessagesLoaded(messages));
                }
            }
            Err(e) => {
                Cx::post_action(AppAction::SendMessageFailed(format!(
                    "Failed to unrevert session: {}",
                    e
                )));
            }
        }
    });
}
