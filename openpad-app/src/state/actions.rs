use openpad_protocol::{
    Agent, Event as OcEvent, FileDiff, HealthResponse, Message, MessageWithParts, Part,
    PermissionReply, PermissionRequest, Project, ProvidersResponse, Session, Skill,
};

#[derive(Clone, Debug, Default)]
pub enum AppAction {
    #[default]
    None,
    Connected,
    ConnectionFailed(String),
    HealthUpdated(HealthResponse),
    ProjectsLoaded(Vec<Project>),
    CurrentProjectLoaded(Project),
    SessionsLoaded(Vec<Session>),
    SessionCreated(Session),
    SessionLoaded(Session),
    SessionDeleted(String),
    SessionUpdated(Session),
    SessionDiffLoaded {
        session_id: String,
        diffs: Vec<FileDiff>,
    },
    RequestSessionDiff {
        session_id: String,
        message_id: Option<String>,
    },
    MessagesLoaded(Vec<MessageWithParts>),
    MessageReceived(Message),
    PartReceived {
        part: Part,
        delta: Option<String>,
    },
    OpenCodeEvent(OcEvent),
    SendMessageFailed(String),
    PermissionRequested {
        session_id: String,
        permission_id: String,
        permission: String,
        pattern: String,
    },
    PermissionResponded {
        session_id: String,
        request_id: String,
        reply: PermissionReply,
    },
    RevertToMessage {
        session_id: String,
        message_id: String,
    },
    UnrevertSession(String),
    DialogConfirmed {
        dialog_type: String,
        value: String,
    },
    PendingPermissionsLoaded(Vec<PermissionRequest>),
    PermissionDismissed {
        session_id: String,
        request_id: String,
    },
    ProvidersLoaded(ProvidersResponse),
    AgentsLoaded(Vec<Agent>),
    SkillsLoaded(Vec<Skill>),
    ConfigLoaded(openpad_protocol::Config),
    AuthSet {
        provider_id: String,
        success: bool,
    },
    SetSidebarMode(SidebarMode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SidebarMode {
    #[default]
    Files,
    Settings,
}

#[derive(Clone, Debug, Default)]
pub enum ProjectsPanelAction {
    #[default]
    None,
    SelectSession(String),
    CreateSession(Option<String>),
    DeleteSession(String),
    RenameSession(String),
    AbortSession(String),
    BranchSession(String),
    /// Open the session context menu at the given position (avoids full list redraw).
    OpenSessionContextMenu {
        session_id: String,
        x: f32,
        y: f32,
        working: bool,
    },
    OpenProjectContextMenu {
        project_id: Option<String>,
    },
    CloseSessionContextMenu,
}
