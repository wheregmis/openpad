use crate::state::actions::AppAction;
use makepad_widgets::Cx;
use openpad_protocol::{
    OpenCodeClient, PermissionAction, PermissionReply, PermissionReplyRequest, PermissionRule,
    PermissionRuleset, Session, SessionCreateRequest,
};
use std::sync::Arc;

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

/// Spawns a task to load messages for a session
pub fn spawn_message_loader(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
) {
    runtime.spawn(async move {
        match client.list_messages(&session_id).await {
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
) {
    runtime.spawn(async move {
        // Create session if needed
        let sid = if let Some(id) = session_id {
            id
        } else {
            let request = SessionCreateRequest {
                parent_id: None,
                title: None,
                permission: Some(default_permission_ruleset()),
            };

            match client.create_session_with_options(request).await {
                Ok(session) => {
                    Cx::post_action(AppAction::SessionCreated(session.clone()));
                    session.id
                }
                Err(e) => {
                    Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
                    return;
                }
            }
        };

        // Send prompt
        if let Err(e) = client.send_prompt(&sid, &text).await {
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
        // If a specific directory is provided, create a new client for this request
        // Otherwise, use the default client
        let session_result = if let Some(directory) = project_directory {
            let project_client = OpenCodeClient::new("http://localhost:4096")
                .with_directory(directory);
            
            let request = SessionCreateRequest {
                parent_id: None,
                title: None,
                permission: Some(default_permission_ruleset()),
            };
            
            project_client.create_session_with_options(request).await
        } else {
            let request = SessionCreateRequest {
                parent_id: None,
                title: None,
                permission: Some(default_permission_ruleset()),
            };
            
            client.create_session_with_options(request).await
        };

        match session_result {
            Ok(session) => {
                Cx::post_action(AppAction::SessionCreated(session));
                if let Ok(sessions) = client.list_sessions().await {
                    Cx::post_action(AppAction::SessionsLoaded(sessions));
                }
            }
            Err(e) => {
                Cx::post_action(AppAction::SendMessageFailed(e.to_string()));
            }
        }
    });
}

/// Spawns a task to respond to a permission request
pub fn spawn_permission_reply(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    request_id: String,
    reply: PermissionReply,
) {
    runtime.spawn(async move {
        let response = PermissionReplyRequest { reply };
        if let Err(e) = client.reply_to_permission(&request_id, response).await {
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

fn default_permission_ruleset() -> PermissionRuleset {
    vec![PermissionRule {
        permission: "*".to_string(),
        pattern: "*".to_string(),
        action: PermissionAction::Ask,
    }]
}

/// Spawns a task to delete a session
pub fn spawn_session_deleter(
    runtime: &tokio::runtime::Runtime,
    client: Arc<OpenCodeClient>,
    session_id: String,
) {
    runtime.spawn(async move {
        match client.delete_session(&session_id).await {
            Ok(_) => {
                Cx::post_action(AppAction::SessionDeleted(session_id.clone()));
                // Reload sessions list
                if let Ok(sessions) = client.list_sessions().await {
                    Cx::post_action(AppAction::SessionsLoaded(sessions));
                }
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
) {
    use openpad_protocol::SessionUpdateRequest;

    runtime.spawn(async move {
        let request = SessionUpdateRequest {
            title: Some(new_title),
        };
        match client.update_session(&session_id, request).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionUpdated(session.clone()));
                // Reload sessions list to ensure consistency
                if let Ok(sessions) = client.list_sessions().await {
                    Cx::post_action(AppAction::SessionsLoaded(sessions));
                }
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
) {
    runtime.spawn(async move {
        let request = SessionCreateRequest {
            parent_id: Some(parent_session_id),
            title: None,
            permission: Some(default_permission_ruleset()),
        };

        match client.create_session_with_options(request).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionCreated(session));
                if let Ok(sessions) = client.list_sessions().await {
                    Cx::post_action(AppAction::SessionsLoaded(sessions));
                }
            }
            Err(e) => {
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
) {
    use openpad_protocol::RevertRequest;

    runtime.spawn(async move {
        let request = RevertRequest { message_id };
        match client.revert_message(&session_id, request).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionUpdated(session));
                // Reload messages for the session
                if let Ok(messages) = client.list_messages(&session_id).await {
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
) {
    runtime.spawn(async move {
        match client.unrevert_session(&session_id).await {
            Ok(session) => {
                Cx::post_action(AppAction::SessionUpdated(session));
                // Reload messages for the session
                if let Ok(messages) = client.list_messages(&session_id).await {
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
