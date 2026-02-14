use crate::async_runtime;
use crate::constants::OPENCODE_SERVER_URL;
use crate::state::{self, AppAction, AppState, ProjectsPanelAction};
use makepad_widgets::*;
use openpad_protocol::OpenCodeClient;
use openpad_widgets::message_list::MessageListWidgetRefExt;
use openpad_widgets::permission_card::PermissionCardAction;
use openpad_widgets::simple_dialog::SimpleDialogWidgetRefExt;
use openpad_widgets::terminal_panel::TerminalPanelWidgetRefExt;
use openpad_widgets::UpDropDownWidgetRefExt;
use openpad_widgets::{
    MessageListAction as WidgetMessageListAction, PermissionDialogAction, SettingsDialogAction,
    SidePanelWidgetRefExt, SimpleDialogAction,
};
use regex::Regex;
use std::sync::{Arc, OnceLock};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum SidebarMode {
    #[default]
    Projects,
    Settings,
}

// Lazy-initialized regex for detecting image data URLs
static IMAGE_DATA_URL_REGEX: OnceLock<Regex> = OnceLock::new();
const SIDEBAR_DEFAULT_WIDTH: f32 = 260.0;
const SIDEBAR_MIN_WIDTH: f32 = 200.0;
const SIDEBAR_MAX_WIDTH: f32 = 420.0;

fn get_image_data_url_regex() -> &'static Regex {
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

app_main!(App);

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    let ChatPanel = View {
        width: Fill, height: Fill
        flow: Down
        spacing: 0

        session_summary := View {
            visible: false
            width: Fill, height: Fit
            flow: Down
            spacing: 6

            summary_header := View {
                width: Fill, height: Fit
                flow: Right
                summary_title := Label { text: "Session Summary" }
                View { width: Fill }
            }

            summary_stats_label := Label { width: Fill, height: Fit, text: "" }
            summary_diff := Markdown { width: Fill, height: Fit }
        }

        View {
            width: Fill, height: Fill
            message_list := MessageList { width: Fill, height: Fill }
        }

        input_row := View {
            width: Fill, height: Fit
            flow: Down, spacing: 8

            attachments_preview := View {
                visible: false
                width: Fill, height: Fit
                flow: Right, spacing: 8
                attachments_label := Label { text: "Attached:" }
                attachments_list := Label { text: "" }
                View { width: Fill }
                clear_attachments_button := Button { width: Fit, height: 20, text: "Clear" }
            }

            skill_preview := View {
                visible: false
                width: Fill, height: Fit
                flow: Down, spacing: 4
                skill_header := View {
                    width: Fill, height: Fit
                    flow: Right, spacing: 8
                    skill_name_label := Label { text: "Skill" }
                    View { width: Fill }
                    clear_skill_button := Button { width: Fit, height: 20, text: "Clear" }
                }
                skill_desc_label := Label { text: "" }
            }

            InputBar {
                width: Fill
                input_box := InputField {}
                input_bar_toolbar := InputBarToolbar {
                    agent_dropdown := InputBarDropDown { labels: ["Agent"] }
                    skill_dropdown := InputBarDropDown { width: 120 labels: ["Skill"] }
                    provider_dropdown := InputBarDropDown { width: 120 labels: ["Provider"] }
                    model_dropdown := InputBarDropDown { width: 150 labels: ["Model"] }
                    View { width: Fill }
                    send_button := SendButton { margin: Inset{ left: 0 } }
                }
            }
        }
    }

    let TerminalPanelWrap = TerminalPanel {}

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(1200 800)
                window.title: "Openpad"
                pass.clear_color: #1a1a1a
                body +: {
                    width: Fill, height: Fill
                    flow: Overlay

                    View {
                        width: Fill, height: Fill
                        flow: Right

                        side_panel := SidePanel {
                            width: 260.0, height: Fill
                            open_size: 260.0

                            projects_panel := ProjectsPanel { visible: true }
                            settings_panel := SettingsDialog { visible: false width: Fill height: Fill }
                        }

                        sidebar_resize_handle := View { width: 6, height: Fill }

                        View {
                            width: Fill, height: Fill
                            flow: Down

                            main_header := HeaderBar {
                                width: Fill, height: 36
                                traffic_light_spacer := SidePanel { width: 0.0 height: Fill open_size: 80.0 close_size: 0.0 }
                                hamburger_button := HamburgerButton { width: 32, height: 32 }
                                app_title := Label { text: "Openpad" }
                                View { width: Fill }
                                status_row := View {
                                    width: Fit, height: Fit
                                    flow: Right, spacing: 8
                                    work_indicator := View { visible: false Label { text: "Working..." } }
                                    status_dot := StatusDot {}
                                    status_label := Label { text: "Connected" }
                                }
                            }

                            session_info := View {
                                width: Fill, height: 32
                                flow: Right, spacing: 8
                                show_bg: true
                                draw_bg +: {
                                    color: #171a20
                                    border_size: 1.0
                                    border_color: #262c35
                                }
                                padding: Inset{left: 10 right: 10 top: 6 bottom: 6}
                                project_row := View {
                                    width: Fit, height: Fit
                                    project_badge := View { project_badge_label := Label { text: "No project" } }
                                }
                                Label { text: "/" }
                                session_row := View { session_title := Label { text: "New Session" } }
                                project_path_wrap := View { visible: false project_path_label := Label { text: "" } }
                                View { width: Fill }
                                share_wrap := View {
                                    width: Fit, height: Fit
                                    flow: Right, spacing: 6
                                    share_button := Button { width: Fit, height: 20, text: "Share" }
                                    unshare_button := Button { width: Fit, height: 20, visible: false, text: "Unshare" }
                                    copy_share_button := Button { width: Fit, height: 20, visible: false, text: "Copy link" }
                                    share_url_label := Label { text: "" }
                                }
                                summarize_button := Button { width: Fit, height: 20, text: "Summarize" }
                                revert_indicator := View { visible: false revert_indicator_label := Label { text: "Reverted" } }
                                unrevert_wrap := View { visible: false unrevert_button := Button { width: Fit, height: 20, text: "Unrevert" } }
                            }

                            ChatPanel {}
                            terminal_panel_wrap := TerminalPanelWrap {}
                        }
                    }

                    simple_dialog := SimpleDialog {}
                }
            }
        }
    }
}
#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    state: AppState,
    #[rust]
    sidebar_open: bool,
    #[rust]
    sidebar_width: f32,
    #[rust]
    sidebar_drag_start: Option<(f64, f32)>,
    #[rust]
    sidebar_mode: SidebarMode,
    #[rust]
    terminal_open: bool,
    #[rust]
    client: Option<Arc<OpenCodeClient>>,
    #[rust]
    _runtime: Option<tokio::runtime::Runtime>,
    #[rust]
    connected_once: bool,
    #[rust]
    providers_loaded_once: bool,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        makepad_widgets::script_mod(vm);
        makepad_code_editor::script_mod(vm);
        openpad_widgets::script_mod(vm);
        crate::components::projects_panel::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }

    fn normalize_project_directory(worktree: &str) -> String {
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
    fn get_session_directory(&self, session_id: &str) -> Option<String> {
        self.state
            .sessions
            .iter()
            .find(|s| s.id == session_id)
            .map(|s| s.directory.clone())
    }

    /// Update the attachments preview UI
    fn update_attachments_ui(&self, cx: &mut Cx) {
        let has_attachments = !self.state.attached_files.is_empty();
        self.ui
            .view(cx, &[id!(attachments_preview)])
            .set_visible(cx, has_attachments);

        if has_attachments {
            let filenames: Vec<String> = self
                .state
                .attached_files
                .iter()
                .map(|f| f.filename.clone())
                .collect();
            let text = filenames.join(", ");
            self.ui
                .label(cx, &[id!(attachments_list)])
                .set_text(cx, &text);
        }
        self.ui.redraw(cx);
    }

    fn update_skill_ui(&self, cx: &mut Cx) {
        let selected = self.state.selected_skill();
        let has_skill = selected.is_some();
        self.ui
            .view(cx, &[id!(skill_preview)])
            .set_visible(cx, has_skill);

        if let Some(skill) = selected {
            self.ui
                .label(cx, &[id!(skill_name_label)])
                .set_text(cx, &skill.name);
            self.ui
                .label(cx, &[id!(skill_desc_label)])
                .set_text(cx, &skill.description);
        }
        self.ui.redraw(cx);
    }

    fn set_sidebar_width(&mut self, cx: &mut Cx, width: f32) {
        let clamped = width.clamp(SIDEBAR_MIN_WIDTH, SIDEBAR_MAX_WIDTH);
        self.sidebar_width = clamped;
        self.ui
            .side_panel(cx, &[id!(side_panel)])
            .set_open_size(cx, clamped);
        self.ui.redraw(cx);
    }

    fn update_sidebar_handle_visibility(&self, cx: &mut Cx) {
        self.ui
            .view(cx, &[id!(sidebar_resize_handle)])
            .set_visible(cx, self.sidebar_open);
    }

    fn toggle_sidebar(&mut self, cx: &mut Cx) {
        self.sidebar_open = !self.sidebar_open;

        if self.sidebar_open && self.sidebar_width <= 0.0 {
            self.sidebar_width = SIDEBAR_DEFAULT_WIDTH;
        }
        if self.sidebar_open {
            self.set_sidebar_width(cx, self.sidebar_width);
        }
        self.ui
            .side_panel(cx, &[id!(side_panel)])
            .set_open(cx, self.sidebar_open);
        self.ui
            .side_panel(cx, &[id!(traffic_light_spacer)])
            .set_open(cx, !self.sidebar_open);
        self.update_sidebar_handle_visibility(cx);
        self.update_sidebar_panel_visibility(cx);

        if self.sidebar_open {
            self.ui
                .view(cx, &[id!(hamburger_button)])
                .animator_play(cx, &[id!(open), id!(on)]);
        } else {
            self.ui
                .view(cx, &[id!(hamburger_button)])
                .animator_play(cx, &[id!(open), id!(off)]);
        }
    }

    fn update_sidebar_panel_visibility(&mut self, cx: &mut Cx) {
        let show_projects = self.sidebar_mode == SidebarMode::Projects;
        let show_settings = self.sidebar_mode == SidebarMode::Settings;

        // Use widget() for custom widgets (ProjectsPanel, SettingsDialog).
        self.ui
            .widget(cx, &[id!(side_panel), id!(projects_panel)])
            .set_visible(cx, show_projects);
        self.ui
            .widget(cx, &[id!(side_panel), id!(settings_panel)])
            .set_visible(cx, show_settings);

        // Force redraw of the side panel to ensure visibility changes take effect
        self.ui.view(cx, &[id!(side_panel)]).redraw(cx);
        self.ui.redraw(cx);
    }

    fn toggle_terminal(&mut self, cx: &mut Cx) {
        self.terminal_open = !self.terminal_open;
        self.ui
            .terminal_panel(cx, &[id!(terminal_panel_wrap)])
            .set_open(cx, self.terminal_open);
    }

    fn handle_sidebar_resize(&mut self, cx: &mut Cx, event: &Event) {
        if !self.sidebar_open {
            self.sidebar_drag_start = None;
            return;
        }

        let handle_area = self.ui.view(cx, &[id!(sidebar_resize_handle)]).area();
        let hit = event.hits_with_options(
            cx,
            handle_area,
            HitOptions::new().with_margin(Inset {
                left: 4.0,
                right: 4.0,
                top: 0.0,
                bottom: 0.0,
            }),
        );

        match hit {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::ColResize);
            }
            Hit::FingerDown(f) => {
                cx.set_cursor(MouseCursor::ColResize);
                self.sidebar_drag_start = Some((f.abs.x, self.sidebar_width));
            }
            Hit::FingerMove(f) => {
                if let Some((start_x, start_width)) = self.sidebar_drag_start {
                    let delta = (f.abs.x - start_x) as f32;
                    self.set_sidebar_width(cx, start_width + delta);
                }
            }
            Hit::FingerUp(_) => {
                self.sidebar_drag_start = None;
            }
            _ => {}
        }
    }

    /// Extract data URLs from text and add them as attachments
    /// Returns the text with data URLs removed
    fn process_pasted_content(&mut self, cx: &mut Cx, text: &str) -> String {
        use crate::state::handlers::AttachedFile;

        let data_url_pattern = get_image_data_url_regex();

        let mut remaining_text = String::new();
        let mut last_end = 0;
        let mut attachment_count = 0;

        for captures in data_url_pattern.captures_iter(text) {
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
            attachment_count += 1;

            // Add the file as an attachment
            self.state.attached_files.push(AttachedFile {
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

    fn connect_to_opencode(&mut self, _cx: &mut Cx) {
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

    fn handle_actions(&mut self, cx: &mut Cx, actions: &ActionsBuf) {
        for action in actions {
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                match app_action {
                    AppAction::OpenCodeEvent(oc_event) => {
                        state::handle_opencode_event(&mut self.state, &self.ui, cx, oc_event);
                    }
                    AppAction::PermissionResponded {
                        session_id,
                        request_id,
                        reply,
                    } => {
                        state::handle_permission_responded(
                            &mut self.state,
                            &self.ui,
                            cx,
                            request_id,
                        );
                        self.respond_to_permission(
                            cx,
                            session_id.clone(),
                            request_id.clone(),
                            reply.clone(),
                        );
                    }
                    AppAction::RevertToMessage {
                        session_id,
                        message_id,
                    } => {
                        self.revert_to_message(cx, session_id.clone(), message_id.clone());
                    }
                    AppAction::UnrevertSession(session_id) => {
                        self.unrevert_session(cx, session_id.clone());
                    }
                    AppAction::RequestSessionDiff {
                        session_id,
                        message_id,
                    } => {
                        self.load_session_diff(cx, session_id.clone(), message_id.clone());
                    }
                    AppAction::DialogConfirmed { dialog_type, value } => {
                        self.handle_dialog_confirmed(cx, dialog_type.clone(), value.clone());
                    }
                    AppAction::Connected => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.load_providers_and_agents();
                    }
                    AppAction::ProjectsLoaded(projects) => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.load_all_sessions(projects.clone());
                    }
                    _ => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                    }
                }
            }
        }
    }

    fn load_providers_and_agents(&mut self) {
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

    fn load_pending_permissions(&mut self) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_pending_permissions_loader(runtime, client);
    }

    fn load_all_sessions(&mut self, projects: Vec<openpad_protocol::Project>) {
        let Some(client) = self.client.clone() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };
        async_runtime::spawn_all_sessions_loader(runtime, client, projects);
    }

    fn load_messages(&mut self, session_id: String) {
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

    fn send_message(&mut self, cx: &mut Cx, text: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        let session_id = self.state.current_session_id.clone();
        let directory = session_id
            .as_ref()
            .and_then(|sid| {
                self.state
                    .sessions
                    .iter()
                    .find(|session| &session.id == sid)
                    .map(|session| {
                        log!(
                            "Sending message to session: id={}, directory={}, project_id={}",
                            session.id,
                            session.directory,
                            session.project_id
                        );
                        session.directory.clone()
                    })
            })
            .or_else(|| {
                self.state.current_project.as_ref().map(|project| {
                    let dir = Self::normalize_project_directory(&project.worktree);
                    log!(
                        "No session - using current_project: id={}, worktree={}, normalized_dir={}",
                        project.id,
                        project.worktree,
                        dir
                    );
                    dir
                })
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
            session_id,
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

    fn create_session(&mut self, _cx: &mut Cx, project_id: Option<String>) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Look up project directory if project_id is provided
        let project_directory = project_id.as_ref().and_then(|pid| {
            self.state
                .projects
                .iter()
                .find(|p| &p.id == pid)
                .map(|p| {
                    let normalized = Self::normalize_project_directory(&p.worktree);
                    log!(
                        "Creating session for project: id={}, name={:?}, worktree={}, normalized_directory={}",
                        pid,
                        p.name,
                        p.worktree,
                        normalized
                    );
                    normalized
                })
        });

        let permission = self.state.selected_agent_permission();
        async_runtime::spawn_session_creator(runtime, client, project_directory, permission);
    }

    fn respond_to_permission(
        &mut self,
        _cx: &mut Cx,
        session_id: String,
        request_id: String,
        reply: openpad_protocol::PermissionReply,
    ) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_permission_reply(runtime, client, session_id, request_id, reply);
    }

    fn delete_session(&mut self, cx: &mut Cx, session_id: String) {
        // Show confirmation dialog
        self.ui
            .simple_dialog(cx, &[id!(simple_dialog)])
            .show_confirm(
                cx,
                "Delete Session",
                "Are you sure you want to delete this session? This action cannot be undone.",
                format!("delete_session:{}", session_id),
            );
    }

    fn rename_session(&mut self, cx: &mut Cx, session_id: String) {
        // Get current title
        let current_title = self
            .state
            .sessions
            .iter()
            .find(|s| s.id == session_id)
            .map(|s| async_runtime::get_session_title(s))
            .unwrap_or_else(|| "Session".to_string());

        // Show input dialog
        self.ui.simple_dialog(cx, &[id!(simple_dialog)]).show_input(
            cx,
            "Rename Session",
            "Enter a new name for this session:",
            &current_title,
            format!("rename_session:{}", session_id),
        );
    }

    fn abort_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_aborter(runtime, client, session_id);
    }

    fn share_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_sharer(runtime, client, session_id);
    }

    fn unshare_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_unsharer(runtime, client, session_id);
    }

    fn summarize_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_summarizer(runtime, client, session_id, false);
    }

    fn load_session_diff(&mut self, _cx: &mut Cx, session_id: String, message_id: Option<String>) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_diff_loader(runtime, client, session_id, message_id);
    }

    fn branch_session(&mut self, _cx: &mut Cx, parent_session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the parent session to get its directory for the new branched session
        let directory = self.get_session_directory(&parent_session_id);

        async_runtime::spawn_session_brancher(runtime, client, parent_session_id, directory);
    }

    fn revert_to_message(&mut self, _cx: &mut Cx, session_id: String, message_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the session to get its directory
        let directory = self.get_session_directory(&session_id);

        async_runtime::spawn_message_reverter(runtime, client, session_id, message_id, directory);
    }

    fn unrevert_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the session to get its directory
        let directory = self.get_session_directory(&session_id);

        async_runtime::spawn_session_unreverter(runtime, client, session_id, directory);
    }

    fn handle_dialog_confirmed(&mut self, _cx: &mut Cx, dialog_type: String, value: String) {
        // Parse the dialog_type which is in format "action:data"
        let Some((action, data)) = dialog_type.split_once(':') else {
            return;
        };

        let Some(client) = self.client.clone() else {
            self.state.error_message = Some("Not connected".to_string());
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        let directory = self.get_session_directory(data);

        match action {
            "delete_session" => {
                async_runtime::spawn_session_deleter(runtime, client, data.to_string(), directory);
            }
            "rename_session" => {
                if !value.is_empty() {
                    async_runtime::spawn_session_updater(
                        runtime,
                        client,
                        data.to_string(),
                        value,
                        directory,
                    );
                }
            }
            "set_auth" => {
                async_runtime::spawn_auth_setter(runtime, client, data.to_string(), value);
            }
            _ => {}
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if self.state.is_working {
            if let Event::NextFrame(_) = event {
                self.ui.view(cx, &[id!(work_indicator)]).redraw(cx);
            }
            cx.new_next_frame();
        }

        match event {
            Event::Startup => {
                self.connect_to_opencode(cx);
                if !cx.in_makepad_studio() {
                    #[cfg(not(target_os = "macos"))]
                    if let Some(mut window) = self.ui.borrow_mut::<Window>() {
                        window.set_fullscreen(cx);
                    }
                }
                // Initialize sidebar open, terminal collapsed by default
                self.sidebar_open = true;
                self.sidebar_mode = SidebarMode::Projects;
                self.terminal_open = false;
                self.ui
                    .terminal_panel(cx, &[id!(terminal_panel_wrap)])
                    .set_open(cx, false);
                self.sidebar_width = SIDEBAR_DEFAULT_WIDTH;
                self.set_sidebar_width(cx, self.sidebar_width);
                self.ui
                    .side_panel(cx, &[id!(side_panel)])
                    .set_open(cx, true);
                self.ui
                    .side_panel(cx, &[id!(traffic_light_spacer)])
                    .set_open(cx, false);
                self.ui
                    .view(cx, &[id!(hamburger_button)])
                    .animator_play(cx, &[id!(open), id!(on)]);
                self.update_sidebar_handle_visibility(cx);
                self.update_sidebar_panel_visibility(cx);
            }
            Event::Actions(actions) => {
                self.handle_actions(cx, actions);
            }
            Event::KeyDown(ke) => {
                if (ke.modifiers.logo || ke.modifiers.control)
                    && !ke.modifiers.shift
                    && !ke.modifiers.alt
                {
                    match ke.key_code {
                        KeyCode::KeyD => {
                            self.toggle_sidebar(cx);
                        }
                        KeyCode::KeyT => {
                            self.toggle_terminal(cx);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        self.handle_sidebar_resize(cx, event);

        // Handle UI events and capture actions
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });

        // Process widget actions (e.g. ProjectsPanelAction from sidebar clicks)
        for action in &actions {
            if let Some(panel_action) = action.downcast_ref::<ProjectsPanelAction>() {
                match panel_action {
                    ProjectsPanelAction::SelectSession(session_id) => {
                        self.state.selected_session_id = Some(session_id.clone());
                        self.state.current_session_id = Some(session_id.clone());
                        self.state.is_working = false;
                        self.state.messages_data.clear();
                        self.ui.message_list(cx, &[id!(message_list)]).set_messages(
                            cx,
                            &self.state.messages_data,
                            None,
                        );
                        crate::ui::state_updates::update_work_indicator(&self.ui, cx, false);
                        self.state.update_projects_panel(&self.ui, cx);
                        self.state.update_session_title_ui(&self.ui, cx);
                        self.state.update_project_context_ui(&self.ui, cx);
                        self.state.update_session_meta_ui(&self.ui, cx);
                        self.load_messages(session_id.clone());
                        self.load_pending_permissions();
                    }
                    ProjectsPanelAction::CreateSession(project_id) => {
                        log!(
                            "UI action: create session button clicked (project_id={:?})",
                            project_id
                        );
                        self.create_session(cx, project_id.clone());
                    }
                    ProjectsPanelAction::DeleteSession(session_id) => {
                        self.delete_session(cx, session_id.clone());
                    }
                    ProjectsPanelAction::RenameSession(session_id) => {
                        self.rename_session(cx, session_id.clone());
                    }
                    ProjectsPanelAction::AbortSession(session_id) => {
                        self.abort_session(cx, session_id.clone());
                    }
                    ProjectsPanelAction::BranchSession(session_id) => {
                        self.branch_session(cx, session_id.clone());
                    }
                    _ => {}
                }
            }

            // Handle MessageListAction
            if let Some(msg_action) = action.downcast_ref::<WidgetMessageListAction>() {
                match msg_action {
                    WidgetMessageListAction::RevertToMessage(message_id) => {
                        if let Some(session_id) = &self.state.current_session_id {
                            self.revert_to_message(cx, session_id.clone(), message_id.clone());
                        }
                    }
                    _ => {}
                }
            }

            // Handle AppAction from captured UI actions (e.g. DialogConfirmed, PermissionResponded)
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                match app_action {
                    AppAction::DialogConfirmed { dialog_type, value } => {
                        self.handle_dialog_confirmed(cx, dialog_type.clone(), value.clone());
                    }
                    AppAction::PermissionResponded {
                        session_id,
                        request_id,
                        reply,
                    } => {
                        state::handle_permission_responded(
                            &mut self.state,
                            &self.ui,
                            cx,
                            request_id,
                        );
                        self.respond_to_permission(
                            cx,
                            session_id.clone(),
                            request_id.clone(),
                            reply.clone(),
                        );
                    }
                    _ => {}
                }
            }

            // Handle PermissionCardAction from inline permission cards
            if let Some(action) = action.downcast_ref::<PermissionCardAction>() {
                match action {
                    PermissionCardAction::Approved {
                        session_id,
                        request_id,
                    } => {
                        if let Some(client) = &self.client {
                            let client = client.clone();
                            let session_id = session_id.clone();
                            let request_id = request_id.clone();
                            if let Some(runtime) = self._runtime.as_ref() {
                                async_runtime::spawn_permission_reply(
                                    runtime,
                                    client,
                                    session_id,
                                    request_id.clone(),
                                    openpad_protocol::PermissionReply::Once,
                                );
                            }
                            state::handle_permission_responded(
                                &mut self.state,
                                &self.ui,
                                cx,
                                &request_id,
                            );
                        }
                    }
                    PermissionCardAction::AlwaysApproved {
                        session_id,
                        request_id,
                    } => {
                        if let Some(client) = &self.client {
                            let client = client.clone();
                            let session_id = session_id.clone();
                            let request_id = request_id.clone();
                            if let Some(runtime) = self._runtime.as_ref() {
                                async_runtime::spawn_permission_reply(
                                    runtime,
                                    client,
                                    session_id,
                                    request_id.clone(),
                                    openpad_protocol::PermissionReply::Always,
                                );
                            }
                            state::handle_permission_responded(
                                &mut self.state,
                                &self.ui,
                                cx,
                                &request_id,
                            );
                        }
                    }
                    PermissionCardAction::Rejected {
                        session_id,
                        request_id,
                    } => {
                        if let Some(client) = &self.client {
                            let client = client.clone();
                            let session_id = session_id.clone();
                            let request_id = request_id.clone();
                            if let Some(runtime) = self._runtime.as_ref() {
                                async_runtime::spawn_permission_reply(
                                    runtime,
                                    client,
                                    session_id,
                                    request_id.clone(),
                                    openpad_protocol::PermissionReply::Reject,
                                );
                            }
                            state::handle_permission_responded(
                                &mut self.state,
                                &self.ui,
                                cx,
                                &request_id,
                            );
                        }
                    }
                    _ => {}
                }
            }

            // Handle PermissionDialogAction
            if let Some(action) = action.downcast_ref::<PermissionDialogAction>() {
                match action {
                    PermissionDialogAction::Responded {
                        session_id,
                        request_id,
                        reply,
                    } => {
                        state::handle_permission_responded(
                            &mut self.state,
                            &self.ui,
                            cx,
                            request_id,
                        );
                        self.respond_to_permission(
                            cx,
                            session_id.clone(),
                            request_id.clone(),
                            reply.clone(),
                        );
                    }
                    _ => {}
                }
            }

            // Handle SettingsDialogAction
            if let Some(action) = action.downcast_ref::<SettingsDialogAction>() {
                match action {
                    SettingsDialogAction::UpdateKey { provider_id, key } => {
                        self.handle_dialog_confirmed(
                            cx,
                            format!("set_auth:{}", provider_id),
                            key.clone(),
                        );
                    }
                    _ => {}
                }
            }

            // Handle SimpleDialogAction from openpad-widgets
            if let Some(dialog_action) = action.downcast_ref::<SimpleDialogAction>() {
                match dialog_action {
                    SimpleDialogAction::Confirmed { dialog_type, value } => {
                        self.handle_dialog_confirmed(cx, dialog_type.clone(), value.clone());
                    }
                    SimpleDialogAction::Cancelled => {
                        // Dialog was cancelled, no action needed
                    }
                    SimpleDialogAction::None => {}
                }
            }
        }

        // Detect pasted images (data URLs) and long text on input change,
        // converting them to attachments immediately.
        const LONG_TEXT_THRESHOLD: usize = 2000;
        if let Some(new_text) = self.ui.text_input(cx, &[id!(input_box)]).changed(&actions) {
            let remaining = self.process_pasted_content(cx, &new_text);
            if remaining.len() > LONG_TEXT_THRESHOLD {
                use crate::state::handlers::AttachedFile;

                let filename = format!(
                    "pasted_text_{}.txt",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis(),
                );

                self.state.attached_files.push(AttachedFile {
                    filename,
                    mime_type: "text/plain".to_string(),
                    data_url: String::new(),
                    raw_text: Some(remaining.clone()),
                });

                self.ui.text_input(cx, &[id!(input_box)]).set_text(cx, "");
                self.update_attachments_ui(cx);
            } else if remaining != new_text {
                // Images were extracted — update the input with remaining text
                self.ui
                    .text_input(cx, &[id!(input_box)])
                    .set_text(cx, &remaining);
            }
        }

        // Check for text input return
        if let Some((text, _modifiers)) =
            self.ui.text_input(cx, &[id!(input_box)]).returned(&actions)
        {
            if !text.is_empty() {
                let processed_text = self.process_pasted_content(cx, &text);
                self.send_message(cx, processed_text);
                self.ui.text_input(cx, &[id!(input_box)]).set_text(cx, "");
            }
        }

        // Handle unrevert button
        if self
            .ui
            .button(cx, &[id!(unrevert_button)])
            .clicked(&actions)
        {
            if let Some(session_id) = &self.state.current_session_id {
                self.unrevert_session(cx, session_id.clone());
            }
        }

        if self.ui.button(cx, &[id!(share_button)]).clicked(&actions) {
            if let Some(session_id) = &self.state.current_session_id {
                self.share_session(cx, session_id.clone());
            }
        }

        if self.ui.button(cx, &[id!(unshare_button)]).clicked(&actions) {
            if let Some(session_id) = &self.state.current_session_id {
                self.unshare_session(cx, session_id.clone());
            }
        }

        if self
            .ui
            .button(cx, &[id!(copy_share_button)])
            .clicked(&actions)
        {
            if let Some(url) = self.state.current_share_url() {
                cx.copy_to_clipboard(&url);
            }
        }

        if self
            .ui
            .button(cx, &[id!(summarize_button)])
            .clicked(&actions)
        {
            let session_id = self.state.current_session_id.clone();
            if let Some(session_id) = session_id {
                let message_id =
                    self.state
                        .messages_data
                        .iter()
                        .rev()
                        .find_map(|mwp| match &mwp.info {
                            openpad_protocol::Message::User(msg) => Some(msg.id.clone()),
                            _ => None,
                        });
                self.summarize_session(cx, session_id.clone());
                self.load_session_diff(cx, session_id, message_id);
            }
        }

        if self
            .ui
            .button(cx, &[id!(hamburger_button)])
            .clicked(&actions)
        {
            self.toggle_sidebar(cx);
        }

        if self.ui.button(cx, &[id!(send_button)]).clicked(&actions) {
            let text = self.ui.text_input(cx, &[id!(input_box)]).text();
            if !text.is_empty() {
                let processed_text = self.process_pasted_content(cx, &text);
                self.send_message(cx, processed_text);
                self.ui.text_input(cx, &[id!(input_box)]).set_text(cx, "");
            }
        }

        // Handle clear attachments button
        if self
            .ui
            .button(cx, &[id!(clear_attachments_button)])
            .clicked(&actions)
        {
            self.state.attached_files.clear();
            self.update_attachments_ui(cx);
        }

        if self
            .ui
            .button(cx, &[id!(clear_skill_button)])
            .clicked(&actions)
        {
            self.state.selected_skill_idx = None;
            self.ui
                .up_drop_down(cx, &[id!(input_bar_toolbar), id!(skill_dropdown)])
                .set_selected_item(cx, 0);
            self.update_skill_ui(cx);
        }

        // Handle dropdown selections (main input bar only)
        // Provider selection changed - update model list
        if let Some(idx) = self
            .ui
            .up_drop_down(cx, &[id!(input_bar_toolbar), id!(provider_dropdown)])
            .changed(&actions)
        {
            self.state.selected_provider_idx = idx;
            self.state.update_model_list_for_provider();
            self.ui
                .up_drop_down(cx, &[id!(input_bar_toolbar), id!(model_dropdown)])
                .set_labels(cx, self.state.model_labels.clone());
            self.ui
                .up_drop_down(cx, &[id!(input_bar_toolbar), id!(model_dropdown)])
                .set_selected_item(cx, 0);
        }

        // Model selection changed
        if let Some(idx) = self
            .ui
            .up_drop_down(cx, &[id!(input_bar_toolbar), id!(model_dropdown)])
            .changed(&actions)
        {
            self.state.selected_model_idx = idx;
        }
        if let Some(idx) = self
            .ui
            .up_drop_down(cx, &[id!(input_bar_toolbar), id!(agent_dropdown)])
            .changed(&actions)
        {
            self.state.selected_agent_idx = if idx > 0 { Some(idx - 1) } else { None };
        }
        if let Some(idx) = self
            .ui
            .up_drop_down(cx, &[id!(input_bar_toolbar), id!(skill_dropdown)])
            .changed(&actions)
        {
            self.state.selected_skill_idx = if idx > 0 { Some(idx - 1) } else { None };
            self.update_skill_ui(cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_image_data_url_regex;

    #[test]
    fn test_data_url_detection() {
        let data_url_pattern = get_image_data_url_regex();

        // Test simple PNG data URL
        let text1 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        assert!(data_url_pattern.is_match(text1));

        // Test JPEG data URL
        let text2 = "data:image/jpeg;base64,/9j/4AAQSkZJRg==";
        assert!(data_url_pattern.is_match(text2));

        // Test mixed content
        let text3 = "Here is an image: data:image/png;base64,ABC123== and some text after";
        let matches: Vec<_> = data_url_pattern.find_iter(text3).collect();
        assert_eq!(matches.len(), 1);

        // Test no match
        let text4 = "This is just plain text";
        assert!(!data_url_pattern.is_match(text4));

        // Test extraction
        let text5 = "Before data:image/png;base64,ABC123== After";
        let captures = data_url_pattern.captures(text5).unwrap();
        assert_eq!(&captures[1], "image/png");
        assert_eq!(&captures[2], "ABC123==");
    }

    #[test]
    fn test_text_extraction() {
        let data_url_pattern = get_image_data_url_regex();

        let text = "Start data:image/png;base64,ABC== Middle data:image/jpeg;base64,DEF== End";
        let result = data_url_pattern.replace_all(text, "");
        assert_eq!(result, "Start  Middle  End");
    }
}
