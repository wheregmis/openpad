use makepad_widgets::*;
use openpad_protocol::{
    Event as OcEvent, HealthResponse, Message, MessageWithParts, Part, PermissionReply, Project,
    Session,
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
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ProjectsPanelAction {
    None,
    SelectSession(String),
    CreateSession(Option<String>),
    RunSession(String),
    DeleteSession(String),
    RenameSession(String),
    AbortSession(String),
    BranchSession(String),
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MessageListAction {
    None,
    RevertToMessage(String),
}
