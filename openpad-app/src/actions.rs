use makepad_widgets::*;
use openpad_protocol::{Event as OcEvent, HealthResponse, Project, Session};

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
    OpenCodeEvent(OcEvent),
    SendMessageFailed(String),
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ProjectsPanelAction {
    None,
    SelectSession(String),
    CreateSession(Option<String>),
}
