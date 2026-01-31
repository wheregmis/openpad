use crate::actions::AppAction;
use makepad_widgets::Cx;
use openpad_protocol::{OpenCodeClient, PermissionReply, PermissionReplyRequest, Session};
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
            match client.create_session().await {
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
pub fn spawn_session_creator(runtime: &tokio::runtime::Runtime, client: Arc<OpenCodeClient>) {
    runtime.spawn(async move {
        match client.create_session().await {
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
        let response = PermissionReplyRequest { response: reply };
        if let Err(e) = client.reply_to_permission(&request_id, response).await {
            Cx::post_action(AppAction::SendMessageFailed(format!(
                "Permission response failed: {}",
                e
            )));
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
