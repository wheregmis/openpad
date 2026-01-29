use crate::{Error, Result, Event, Session, Message, PartInput, Part};
use reqwest::Client as HttpClient;
use tokio::sync::broadcast;
use std::env;

pub struct OpenCodeClient {
    http: HttpClient,
    base_url: String,
    directory: String,
    event_tx: broadcast::Sender<Event>,
}

impl OpenCodeClient {
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
