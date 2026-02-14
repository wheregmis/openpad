use makepad_widgets::*;
use openpad_protocol::{
    Agent, Event as OcEvent, FileDiff, HealthResponse, Message, MessageWithParts, Part,
    PermissionReply, PermissionRequest, Project, ProvidersResponse, SecretString, Session, Skill,
};

#[derive(Clone, Debug, DefaultNone)]
pub enum AppAction {
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
        value: SecretString,
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
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ProjectsPanelAction {
    None,
    SelectSession(String),
    CreateSession(Option<String>),
    DeleteSession(String),
    RenameSession(String),
    AbortSession(String),
    BranchSession(String),
}
