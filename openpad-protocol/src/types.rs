use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionTime {
    pub created: i64,  // milliseconds timestamp
    pub updated: i64,  // milliseconds timestamp
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, rename = "projectID")]
    pub project_id: Option<String>,
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    pub time: SessionTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageTime {
    pub start: DateTime<Utc>,
    #[serde(default)]
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    #[serde(rename = "user")]
    User {
        id: String,
        parts: Vec<Part>,
        time: MessageTime,
    },
    #[serde(rename = "assistant")]
    Assistant {
        id: String,
        parts: Vec<Part>,
        model: String,
        time: MessageTime,
    },
}

impl Message {
    pub fn id(&self) -> &str {
        match self {
            Message::User { id, .. } => id,
            Message::Assistant { id, .. } => id,
        }
    }

    pub fn parts(&self) -> &[Part] {
        match self {
            Message::User { parts, .. } => parts,
            Message::Assistant { parts, .. } => parts,
        }
    }

    pub fn parts_mut(&mut self) -> &mut Vec<Part> {
        match self {
            Message::User { parts, .. } => parts,
            Message::Assistant { parts, .. } => parts,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Part {
    #[serde(rename = "text")]
    Text { text: String },
    // MVP: Other variants ignored but need to not break parsing
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartInput {
    #[serde(rename = "type")]
    pub type_name: String,
    pub text: String,
}

impl PartInput {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            type_name: "text".to_string(),
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    SessionCreated(Session),
    SessionDeleted(String),
    MessageUpdated {
        session_id: String,
        message: Message,
    },
    PartUpdated {
        session_id: String,
        message_id: String,
        part_index: usize,
        part: Part,
    },
    Error(String),
    Unknown(String),
}
