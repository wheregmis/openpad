use super::*;
use regex::Regex;
use std::sync::OnceLock;

// Lazy-initialized regex for detecting image data URLs
static IMAGE_DATA_URL_REGEX: OnceLock<Regex> = OnceLock::new();

pub(crate) fn get_image_data_url_regex() -> &'static Regex {
    IMAGE_DATA_URL_REGEX.get_or_init(|| {
        Regex::new(r"data:(image/(?:png|jpeg|jpg|gif|webp|tiff|svg\+xml));base64,([A-Za-z0-9+/=]+)")
            .expect("Failed to compile image data URL regex")
    })
}

fn image_extension_for_mime(mime_type: &str) -> &'static str {
    match mime_type {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/tiff" => "tiff",
        "image/svg+xml" => "svg",
        _ => "png",
    }
}

impl App {
    /// Extract data URLs from text and add them as attachments.
    /// Returns the text with data URLs removed.
    pub(super) fn process_pasted_content(&mut self, cx: &mut Cx, text: &str) -> String {
        let data_url_pattern = get_image_data_url_regex();

        let mut remaining_text = String::new();
        let mut last_end = 0;

        for (attachment_count, captures) in data_url_pattern.captures_iter(text).enumerate() {
            let full_match = &captures[0];
            let mime_type = &captures[1];

            // Add text before the data URL
            remaining_text.push_str(&text[last_end..captures.get(0).unwrap().start()]);
            last_end = captures.get(0).unwrap().end();

            // Determine file extension from mime type
            let extension = image_extension_for_mime(mime_type);

            // Generate a unique filename using timestamp and counter
            let filename = format!(
                "attachment_{}_{}.{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis(),
                attachment_count,
                extension
            );

            // Add the file as an attachment
            self.state.attached_files.push(crate::state::AttachedFile {
                filename: filename.clone(),
                mime_type: mime_type.to_string(),
                data_url: full_match.to_string(),
                raw_text: None,
            });

            log!("Detected pasted image: {} ({})", mime_type, filename);
        }

        // Add remaining text after last data URL
        remaining_text.push_str(&text[last_end..]);

        // Update UI to show attachments
        self.update_attachments_ui(cx);

        remaining_text
    }

    pub(super) fn send_message(&mut self, cx: &mut Cx, text: String) {
        let Some(client) = self.client_or_error() else { return; };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        let Some(session_id) = self.state.current_session_id.clone() else {
            return;
        };
        let directory = self
            .state
            .sessions
            .iter()
            .find(|session| session.id == session_id)
            .map(|session| {
                log!(
                    "Sending message to session: id={}, directory={}, project_id={}",
                    session.id,
                    session.directory,
                    session.project_id
                );
                session.directory.clone()
            });
        let model_spec = self.state.selected_model_spec();
        let agent = self.state.selected_agent_name();
        let permission = self.state.selected_agent_permission();
        let system = self.state.selected_skill_prompt();

        self.state.is_working = true;
        crate::ui::state_updates::update_work_indicator(&self.ui, cx, true);

        // Convert attached files to PartInput
        let attachments: Vec<openpad_protocol::PartInput> = self
            .state
            .attached_files
            .iter()
            .map(|file| {
                if let Some(raw_text) = &file.raw_text {
                    // Text attachments are sent as text parts
                    openpad_protocol::PartInput::text(raw_text)
                } else {
                    openpad_protocol::PartInput::file_with_filename(
                        file.mime_type.clone(),
                        file.filename.clone(),
                        file.data_url.clone(),
                    )
                }
            })
            .collect();

        async_runtime::spawn_message_sender(
            runtime,
            client,
            Some(session_id),
            text,
            model_spec,
            agent,
            system,
            directory,
            attachments,
            permission,
        );

        // Clear attached files after sending
        self.state.attached_files.clear();
        self.update_attachments_ui(cx);
    }
}
