use super::*;

impl App {
    pub(super) fn normalize_project_directory(worktree: &str) -> String {
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

    /// Helper to get a session's directory by session ID
    pub(super) fn get_session_directory(&self, session_id: &str) -> Option<String> {
        self.state
            .sessions
            .iter()
            .find(|s| s.id == session_id)
            .map(|s| s.directory.clone())
    }

    pub(super) fn connect_to_opencode(&mut self, _cx: &mut Cx) {
        if self.client.is_some() || self._runtime.is_some() {
            return;
        }
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = Arc::new(OpenCodeClient::new(OPENCODE_SERVER_URL));

        // Spawn background tasks
        async_runtime::spawn_sse_subscriber(&runtime, client.clone());
        async_runtime::spawn_health_checker(&runtime, client.clone());
        async_runtime::spawn_project_loader(&runtime, client.clone());

        self.client = Some(client);
        self._runtime = Some(runtime);
        self.connected_once = true;
    }

    pub(super) fn load_providers_and_agents(&mut self) {
        if self.providers_loaded_once {
            return;
        }
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };
        async_runtime::spawn_providers_loader(runtime, client.clone());
        async_runtime::spawn_agents_loader(runtime, client.clone());
        async_runtime::spawn_skills_loader(runtime, client.clone());
        async_runtime::spawn_config_loader(runtime, client);
        self.providers_loaded_once = true;
    }

    pub(super) fn load_pending_permissions(&mut self) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_pending_permissions_loader(runtime, client);
    }

    pub(super) fn load_all_sessions(&mut self, projects: Vec<openpad_protocol::Project>) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };
        async_runtime::spawn_all_sessions_loader(runtime, client, projects);
    }

    pub(super) fn load_messages(&mut self, session_id: String) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the session to get its directory
        let directory = self.get_session_directory(&session_id);

        async_runtime::spawn_message_loader(runtime, client, session_id, directory);
    }
}
