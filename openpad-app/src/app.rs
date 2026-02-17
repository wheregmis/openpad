use crate::async_runtime;
use crate::components::editor_panel::{EditorPanelAction, EditorPanelWidgetRefExt};
use crate::components::session_options_popup::SessionOptionsPopupWidgetRefExt;
use crate::constants::OPENCODE_SERVER_URL;
use crate::state::{
    self, AppAction, AppState, CenterTabKind, OpenFileState, PendingCenterIntent,
    ProjectsPanelAction, SidebarMode,
};
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
use std::path::Path;
use std::sync::{Arc, OnceLock};

// Lazy-initialized regex for detecting image data URLs
static IMAGE_DATA_URL_REGEX: OnceLock<Regex> = OnceLock::new();
const SIDEBAR_DEFAULT_WIDTH: f32 = 260.0;
const SIDEBAR_MIN_WIDTH: f32 = 200.0;
const SIDEBAR_MAX_WIDTH: f32 = 420.0;
const RIGHT_SIDEBAR_DEFAULT_WIDTH: f32 = 260.0;
const RIGHT_SIDEBAR_MIN_WIDTH: f32 = 200.0;
const RIGHT_SIDEBAR_MAX_WIDTH: f32 = 420.0;

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

fn is_probably_binary(bytes: &[u8]) -> bool {
    bytes.iter().take(8192).any(|b| *b == 0)
}

app_main!(App);

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    let ChatTabView = View {
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
    }

    let CodeTabView = View {
        width: Fill, height: Fill
        flow: Down

        editor_panel := EditorPanel {
            width: Fill
            height: Fill
        }
    }

    let CenterHomeView = View {
        width: Fill, height: Fill
        align: Align{ x: 0.5, y: 0.5 }
        flow: Down
        spacing: 10
        Label {
            text: "Openpad"
            draw_text +: { color: #ffffff, text_style: theme.font_bold { font_size: 16 } }
        }
        Label {
            text: "Open a session or file from the sidebars"
            draw_text +: { color: #aab3bd, text_style: theme.font_regular { font_size: 11 } }
        }
    }

    let CenterDock = Dock {
        width: Fill, height: Fill
        root := DockTabs {
            tabs: [@center_home_tab]
            selected: 0
            closable: true
        }

        center_home_tab := DockTab {
            name: "Home"
            template: @PermanentTab
            kind: @CenterHomeTab
        }

        CenterHomeTab := CenterHomeView {}
        CenterChatTab := ChatTabView {}
        CenterCodeTab := CodeTabView {}
    }

    let ChatComposer = View {
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
                            flow: Down

                            sidebar_header := SidebarHeader {}

                            files_panel := FilesPanel { visible: true }
                            settings_panel := SettingsDialog { visible: false width: Fill height: Fill }
                        }

                        sidebar_resize_handle := View { width: 6, height: Fill }

                        main_content := View {
                            width: Fill, height: Fill
                            flow: Overlay

                            content_column := View {
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
                                        align: Align{ y: 0.5 }
                                        work_indicator := View {
                                            width: Fit, height: Fit
                                            flow: Right
                                            align: Align{ y: 0.5 }
                                            visible: false
                                            Label { text: "Working..." }
                                        }
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
                                    session_options_btn := Button {
                                        width: 28, height: 24
                                        text: "Session options"
                                        draw_text +: { color: #0000 }
                                        draw_bg +: {
                                            color: #0000
                                            color_dots: #9ca3af
                                            color_hover: #333
                                            border_radius: 6.0
                                            border_size: 0.0

                                            pixel: fn() {
                                                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                                                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius)
                                                sdf.fill(mix(self.color, self.color_hover, self.hover))

                                                let cx = self.rect_size.x * 0.5
                                                let cy = self.rect_size.y * 0.5
                                                let r = 1.2
                                                let gap = 4.0
                                                sdf.circle(cx - gap, cy, r)
                                                sdf.fill(self.color_dots)
                                                sdf.circle(cx, cy, r)
                                                sdf.fill(self.color_dots)
                                                sdf.circle(cx + gap, cy, r)
                                                sdf.fill(self.color_dots)

                                                return sdf.result
                                            }
                                        }
                                    }
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

                                editor_info := View {
                                    width: Fill, height: 32
                                    flow: Right, spacing: 8
                                    show_bg: true
                                    visible: false
                                    draw_bg +: {
                                        color: #171a20
                                        border_size: 1.0
                                        border_color: #262c35
                                    }
                                    padding: Inset{left: 10 right: 10 top: 6 bottom: 6}
                                    editor_file_label := Label {
                                        width: Fit, height: Fit
                                        text: "No file selected"
                                    }
                                    View { width: Fill }
                                    editor_dirty_dot := Label {
                                        width: Fit, height: Fit
                                        text: ""
                                        draw_text +: { color: #f59e0b, text_style: theme.font_bold { font_size: 12 } }
                                    }
                                }

                                center_dock := CenterDock {}
                                chat_composer := ChatComposer {}
                            }

                            terminal_overlay := View {
                                width: Fill, height: Fit
                                align: Align{x: 0.0 y: 1.0}
                                flow: Down
                                terminal_panel_wrap := TerminalPanelWrap {}
                            }
                        }

                        right_sidebar_resize_handle := View { width: 6, height: Fill }

                        right_side_panel := SidePanel {
                            width: 260.0, height: Fill
                            open_size: 260.0
                            flow: Down
                            sessions_panel := SessionsPanel { width: Fill, height: Fill }
                        }
                    }

                    simple_dialog := SimpleDialog {}

                    session_options_popup := SessionOptionsPopup { visible: false }
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
    right_sidebar_open: bool,
    #[rust]
    right_sidebar_width: f32,
    #[rust]
    right_sidebar_drag_start: Option<(f64, f32)>,
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
        crate::components::files_panel::script_mod(vm);
        crate::components::editor_panel::script_mod(vm);
        crate::components::sessions_panel::script_mod(vm);
        crate::components::sidebar_header::script_mod(vm);
        crate::components::session_options_popup::script_mod(vm);
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

    fn center_dock(&self, cx: &mut Cx) -> DockRef {
        self.ui.dock(cx, &[id!(center_dock)])
    }

    fn set_top_surfaces_for_active_kind(&self, cx: &mut Cx, kind: Option<&CenterTabKind>) {
        match kind {
            Some(CenterTabKind::Chat { .. }) => {
                self.ui.view(cx, &[id!(session_info)]).set_visible(cx, true);
                self.ui.view(cx, &[id!(editor_info)]).set_visible(cx, false);
                self.ui.view(cx, &[id!(chat_composer)]).set_visible(cx, true);
            }
            Some(CenterTabKind::File { .. }) => {
                self.ui.view(cx, &[id!(session_info)]).set_visible(cx, false);
                self.ui.view(cx, &[id!(editor_info)]).set_visible(cx, true);
                self.ui.view(cx, &[id!(chat_composer)]).set_visible(cx, false);
            }
            Some(CenterTabKind::Home) | None => {
                self.ui.view(cx, &[id!(session_info)]).set_visible(cx, false);
                self.ui.view(cx, &[id!(editor_info)]).set_visible(cx, false);
                self.ui.view(cx, &[id!(chat_composer)]).set_visible(cx, false);
            }
        }
    }

    fn active_center_kind(&self) -> Option<&CenterTabKind> {
        self.state
            .active_center_tab
            .and_then(|tab_id| self.state.center_tabs_by_id.get(&tab_id))
    }

    fn current_active_file_tab_id(&self) -> Option<LiveId> {
        let tab_id = self.state.active_center_tab?;
        match self.state.center_tabs_by_id.get(&tab_id) {
            Some(CenterTabKind::File { .. }) => Some(tab_id),
            _ => None,
        }
    }

    fn update_editor_header_ui_for_tab(&self, cx: &mut Cx, tab_id: LiveId) {
        let Some(CenterTabKind::File { open_file }) = self.state.center_tabs_by_id.get(&tab_id) else {
            return;
        };
        let item = self.center_dock(cx).item(tab_id);
        self.ui
            .label(cx, &[id!(editor_file_label)])
            .set_text(cx, &open_file.absolute_path);
        let current_text = item.editor_panel(cx, &[id!(editor_panel)]).get_text();
        let is_dirty = current_text != open_file.text_cache;
        self.ui
            .label(cx, &[id!(editor_dirty_dot)])
            .set_text(cx, if is_dirty { "●" } else { "" });
    }

    fn render_chat_tab(&mut self, cx: &mut Cx, tab_id: LiveId, session_id: &str) {
        let item = self.center_dock(cx).item(tab_id);
        let messages = self
            .state
            .messages_by_session
            .get(session_id)
            .cloned()
            .unwrap_or_default();
        let revert = self.state.current_revert_message_id_for_session(session_id);
        item.message_list(cx, &[id!(message_list)])
            .set_messages(cx, &messages, revert);
        let working = self
            .state
            .working_by_session
            .get(session_id)
            .copied()
            .unwrap_or(false);
        item.message_list(cx, &[id!(message_list)])
            .set_working(cx, working);

        let displays: Vec<openpad_widgets::message_list::PendingPermissionDisplay> = self
            .state
            .pending_permissions
            .iter()
            .filter(|p| p.session_id == session_id)
            .map(|p| openpad_widgets::message_list::PendingPermissionDisplay {
                session_id: p.session_id.clone(),
                request_id: p.id.clone(),
                permission: p.permission.clone(),
                patterns: p.patterns.clone(),
            })
            .collect();
        item.message_list(cx, &[id!(message_list)])
            .set_pending_permissions(cx, &displays);

        if let Some(session) = self.state.find_session(session_id) {
            if let Some(summary) = &session.summary {
                item.view(cx, &[id!(session_summary)]).set_visible(cx, true);
                item.label(cx, &[id!(summary_stats_label)]).set_text(
                    cx,
                    &format!(
                        "{} files, +{}, -{}",
                        summary.files, summary.additions, summary.deletions
                    ),
                );
                item.message_list(cx, &[id!(message_list)])
                    .set_session_diffs(cx, &summary.diffs);
            } else {
                item.view(cx, &[id!(session_summary)]).set_visible(cx, false);
                item.message_list(cx, &[id!(message_list)])
                    .set_session_diffs(cx, &[]);
            }
        } else {
            item.view(cx, &[id!(session_summary)]).set_visible(cx, false);
            item.message_list(cx, &[id!(message_list)])
                .set_session_diffs(cx, &[]);
        }
    }

    fn refresh_open_center_tabs(&mut self, cx: &mut Cx) {
        let tabs: Vec<(LiveId, CenterTabKind)> = self
            .state
            .center_tabs_by_id
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
        for (tab_id, kind) in tabs {
            match kind {
                CenterTabKind::Chat { session_id } => self.render_chat_tab(cx, tab_id, &session_id),
                CenterTabKind::File { .. } => self.update_editor_header_ui_for_tab(cx, tab_id),
                CenterTabKind::Home => {}
            }
        }
    }

    fn sync_active_center_ui(&mut self, cx: &mut Cx) {
        let active_kind = self.active_center_kind().cloned();
        self.set_top_surfaces_for_active_kind(cx, active_kind.as_ref());
        match active_kind {
            Some(CenterTabKind::Chat { session_id }) => {
                self.state.current_session_id = Some(session_id.clone());
                self.state.selected_session_id = Some(session_id.clone());
                self.state.messages_data = self
                    .state
                    .messages_by_session
                    .get(&session_id)
                    .cloned()
                    .unwrap_or_default();
                self.state.update_files_panel(&self.ui, cx);
                self.state.update_sessions_panel(&self.ui, cx);
                self.state.update_session_title_ui(&self.ui, cx);
                self.state.update_project_context_ui(&self.ui, cx);
                self.state.update_session_meta_ui(&self.ui, cx);
                let working = self
                    .state
                    .working_by_session
                    .get(&session_id)
                    .copied()
                    .unwrap_or(false);
                crate::ui::state_updates::update_work_indicator(&self.ui, cx, working);
            }
            Some(CenterTabKind::File { .. }) => {
                self.state.current_session_id = None;
                crate::ui::state_updates::update_work_indicator(&self.ui, cx, false);
                if let Some(tab_id) = self.state.active_center_tab {
                    self.update_editor_header_ui_for_tab(cx, tab_id);
                }
            }
            Some(CenterTabKind::Home) | None => {
                self.state.current_session_id = None;
                crate::ui::state_updates::update_work_indicator(&self.ui, cx, false);
            }
        }
    }

    fn has_unsaved_file_tab_changes(&self, cx: &mut Cx, tab_id: LiveId) -> bool {
        let Some(CenterTabKind::File { open_file }) = self.state.center_tabs_by_id.get(&tab_id) else {
            return false;
        };
        let item = self.center_dock(cx).item(tab_id);
        let current_text = item.editor_panel(cx, &[id!(editor_panel)]).get_text();
        current_text != open_file.text_cache
    }

    fn save_file_tab(&mut self, cx: &mut Cx, tab_id: LiveId) -> bool {
        let Some(CenterTabKind::File { open_file }) = self.state.center_tabs_by_id.get(&tab_id).cloned() else {
            return false;
        };
        let item = self.center_dock(cx).item(tab_id);
        let text = item.editor_panel(cx, &[id!(editor_panel)]).get_text();
        if text == open_file.text_cache {
            return true;
        }
        if let Err(err) = std::fs::write(&open_file.absolute_path, text.as_bytes()) {
            self.state.error_message = Some(format!(
                "Failed to save {}: {}",
                open_file.absolute_path, err
            ));
            crate::ui::state_updates::set_status_error(
                &self.ui,
                cx,
                &format!("save failed: {}", err),
            );
            return false;
        }
        if let Some(CenterTabKind::File { open_file }) = self.state.center_tabs_by_id.get_mut(&tab_id)
        {
            open_file.text_cache = text;
            open_file.last_saved_revision = open_file.last_saved_revision.saturating_add(1);
        }
        self.update_editor_header_ui_for_tab(cx, tab_id);
        true
    }

    fn discard_file_tab_changes(&mut self, cx: &mut Cx, tab_id: LiveId) {
        let Some(CenterTabKind::File { open_file }) = self.state.center_tabs_by_id.get(&tab_id).cloned() else {
            return;
        };
        let item = self.center_dock(cx).item(tab_id);
        item.editor_panel(cx, &[id!(editor_panel)])
            .set_text(cx, &open_file.text_cache);
        self.update_editor_header_ui_for_tab(cx, tab_id);
    }

    fn show_unsaved_editor_dialog(&self, cx: &mut Cx) {
        self.ui
            .simple_dialog(cx, &[id!(simple_dialog)])
            .show_confirm_with_secondary(
                cx,
                "Unsaved changes",
                "You have unsaved changes in the open file.",
                "Save",
                "Discard",
                "Cancel",
                "unsaved_editor".to_string(),
            );
    }

    fn queue_or_open_file(&mut self, cx: &mut Cx, project_id: String, absolute_path: String) {
        if let Some(tab_id) = self.current_active_file_tab_id() {
            if self.has_unsaved_file_tab_changes(cx, tab_id) {
                self.state.pending_center_intent = Some(PendingCenterIntent::OpenFile {
                    project_id,
                    absolute_path,
                });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.open_file_now(cx, project_id, absolute_path);
    }

    fn queue_or_select_session(&mut self, cx: &mut Cx, session_id: String) {
        if let Some(tab_id) = self.current_active_file_tab_id() {
            if self.has_unsaved_file_tab_changes(cx, tab_id) {
                self.state.pending_center_intent = Some(PendingCenterIntent::OpenSession {
                    session_id,
                });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.select_session_now(cx, session_id);
    }

    fn queue_or_switch_tab(&mut self, cx: &mut Cx, tab_id: LiveId) {
        if let Some(active_file_tab_id) = self.current_active_file_tab_id() {
            if active_file_tab_id != tab_id && self.has_unsaved_file_tab_changes(cx, active_file_tab_id)
            {
                self.state.pending_center_intent = Some(PendingCenterIntent::SwitchTab { tab_id });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.activate_center_tab(cx, tab_id);
    }

    fn queue_or_close_tab(&mut self, cx: &mut Cx, tab_id: LiveId) {
        if tab_id == live_id!(center_home_tab) {
            return;
        }
        if let Some(CenterTabKind::File { .. }) = self.state.center_tabs_by_id.get(&tab_id) {
            if self.has_unsaved_file_tab_changes(cx, tab_id) {
                self.state.pending_center_intent = Some(PendingCenterIntent::CloseTab { tab_id });
                self.show_unsaved_editor_dialog(cx);
                return;
            }
        }
        self.close_tab_now(cx, tab_id);
    }

    fn close_tab_now(&mut self, cx: &mut Cx, tab_id: LiveId) {
        let tab_bar_id = self
            .center_dock(cx)
            .find_tab_bar_of_tab(tab_id)
            .map(|(tab_bar, _)| tab_bar);

        let kind = self.state.center_tabs_by_id.remove(&tab_id);
        if let Some(kind) = kind {
            match kind {
                CenterTabKind::Chat { session_id } => {
                    self.state.tab_by_session.remove(&session_id);
                }
                CenterTabKind::File { open_file } => {
                    self.state.tab_by_file.remove(&open_file.absolute_path);
                }
                CenterTabKind::Home => {}
            }
        }
        self.center_dock(cx).close_tab(cx, tab_id);
        if self.state.active_center_tab == Some(tab_id) {
            let mut fallback = None;
            if let Some(tab_bar_id) = tab_bar_id {
                if let Some(dock_items) = self.center_dock(cx).clone_state() {
                    if let Some(DockItem::Tabs { tabs, selected, .. }) = dock_items.get(&tab_bar_id)
                    {
                        fallback = tabs.get(*selected).copied();
                    }
                }
            }
            if fallback.is_none() {
                fallback = Some(live_id!(center_home_tab));
            }
            if let Some(fallback_tab) = fallback {
                self.state.active_center_tab = Some(fallback_tab);
                self.center_dock(cx).select_tab(cx, fallback_tab);
            }
            self.sync_active_center_ui(cx);
        }
    }

    fn activate_center_tab(&mut self, cx: &mut Cx, tab_id: LiveId) {
        self.center_dock(cx).select_tab(cx, tab_id);
        self.state.active_center_tab = Some(tab_id);
        self.sync_active_center_ui(cx);
        if let Some(kind) = self.state.center_tabs_by_id.get(&tab_id).cloned() {
            match kind {
                CenterTabKind::Chat { session_id } => self.render_chat_tab(cx, tab_id, &session_id),
                CenterTabKind::File { .. } => self.update_editor_header_ui_for_tab(cx, tab_id),
                CenterTabKind::Home => {}
            }
        }
    }

    fn open_file_now(&mut self, cx: &mut Cx, project_id: String, absolute_path: String) {
        if let Some(existing_tab_id) = self.state.tab_by_file.get(&absolute_path).copied() {
            self.activate_center_tab(cx, existing_tab_id);
            return;
        }

        let path = Path::new(&absolute_path);
        if !path.is_file() {
            return;
        }

        let Ok(bytes) = std::fs::read(path) else {
            return;
        };
        if is_probably_binary(&bytes) {
            return;
        }

        let Ok(content) = String::from_utf8(bytes) else {
            return;
        };

        let display_name = std::path::Path::new(&absolute_path)
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or(&absolute_path)
            .to_string();
        let open_file = OpenFileState {
            project_id,
            absolute_path: absolute_path.clone(),
            display_name: display_name.clone(),
            text_cache: content.clone(),
            last_saved_revision: 0,
        };

        let dock = self.center_dock(cx);
        let (tab_bar, pos) = dock
            .find_tab_bar_of_tab(live_id!(center_home_tab))
            .unwrap_or((live_id!(root), 0));
        let tab_id = dock.unique_id(LiveId::from_str(&format!("file:{}", absolute_path)).0);
        let _ = dock.create_and_select_tab(
            cx,
            tab_bar,
            tab_id,
            live_id!(CenterCodeTab),
            display_name,
            live_id!(CloseableTab),
            Some(pos),
        );

        self.state.center_tabs_by_id.insert(
            tab_id,
            CenterTabKind::File {
                open_file: open_file.clone(),
            },
        );
        self.state.tab_by_file.insert(absolute_path, tab_id);

        let item = dock.item(tab_id);
        item.editor_panel(cx, &[id!(editor_panel)])
            .set_read_only(cx, false);
        item.editor_panel(cx, &[id!(editor_panel)]).set_text(cx, &content);
        item.editor_panel(cx, &[id!(editor_panel)]).focus_editor(cx);
        self.update_editor_header_ui_for_tab(cx, tab_id);
        self.activate_center_tab(cx, tab_id);
    }

    fn select_session_now(&mut self, cx: &mut Cx, session_id: String) {
        self.state.selected_session_id = Some(session_id.clone());

        if let Some(tab_id) = self.state.tab_by_session.get(&session_id).copied() {
            self.activate_center_tab(cx, tab_id);
            self.load_pending_permissions();
            return;
        }

        let title = self
            .state
            .find_session(&session_id)
            .map(async_runtime::get_session_title)
            .unwrap_or_else(|| "Session".to_string());

        let dock = self.center_dock(cx);
        let (tab_bar, pos) = dock
            .find_tab_bar_of_tab(live_id!(center_home_tab))
            .unwrap_or((live_id!(root), 0));
        let tab_id = dock.unique_id(LiveId::from_str(&format!("chat:{}", session_id)).0);
        let _ = dock.create_and_select_tab(
            cx,
            tab_bar,
            tab_id,
            live_id!(CenterChatTab),
            title,
            live_id!(CloseableTab),
            Some(pos),
        );

        self.state.center_tabs_by_id.insert(
            tab_id,
            CenterTabKind::Chat {
                session_id: session_id.clone(),
            },
        );
        self.state.tab_by_session.insert(session_id.clone(), tab_id);
        self.state.active_center_tab = Some(tab_id);

        self.sync_active_center_ui(cx);
        self.render_chat_tab(cx, tab_id, &session_id);

        if !self.state.messages_by_session.contains_key(&session_id) {
            self.load_messages(session_id.clone());
        } else {
            self.state.messages_data = self
                .state
                .messages_by_session
                .get(&session_id)
                .cloned()
                .unwrap_or_default();
        }
        self.load_pending_permissions();
    }

    fn run_pending_center_intent(&mut self, cx: &mut Cx) {
        let Some(intent) = self.state.pending_center_intent.clone() else {
            return;
        };
        self.state.pending_center_intent = None;
        match intent {
            PendingCenterIntent::OpenFile {
                project_id,
                absolute_path,
            } => self.open_file_now(cx, project_id, absolute_path),
            PendingCenterIntent::OpenSession { session_id } => self.select_session_now(cx, session_id),
            PendingCenterIntent::SwitchTab { tab_id } => self.activate_center_tab(cx, tab_id),
            PendingCenterIntent::CloseTab { tab_id } => self.close_tab_now(cx, tab_id),
        }
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
        self.ui
            .view(cx, &[id!(right_sidebar_resize_handle)])
            .set_visible(cx, self.right_sidebar_open);
    }

    fn set_right_sidebar_width(&mut self, cx: &mut Cx, width: f32) {
        let clamped = width.clamp(RIGHT_SIDEBAR_MIN_WIDTH, RIGHT_SIDEBAR_MAX_WIDTH);
        self.right_sidebar_width = clamped;
        self.ui
            .side_panel(cx, &[id!(right_side_panel)])
            .set_open_size(cx, clamped);
        self.ui.redraw(cx);
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
        let show_files = self.sidebar_mode == SidebarMode::Files;
        let show_settings = self.sidebar_mode == SidebarMode::Settings;

        // Use widget() for custom widgets (FilesPanel, SettingsDialog).
        self.ui
            .widget(cx, &[id!(side_panel), id!(files_panel)])
            .set_visible(cx, show_files);
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

    fn toggle_right_sidebar(&mut self, cx: &mut Cx) {
        self.right_sidebar_open = !self.right_sidebar_open;

        if self.right_sidebar_open && self.right_sidebar_width <= 0.0 {
            self.right_sidebar_width = RIGHT_SIDEBAR_DEFAULT_WIDTH;
        }
        if self.right_sidebar_open {
            self.set_right_sidebar_width(cx, self.right_sidebar_width);
        }
        self.ui
            .side_panel(cx, &[id!(right_side_panel)])
            .set_open(cx, self.right_sidebar_open);
        self.update_sidebar_handle_visibility(cx);
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

    fn handle_right_sidebar_resize(&mut self, cx: &mut Cx, event: &Event) {
        if !self.right_sidebar_open {
            self.right_sidebar_drag_start = None;
            return;
        }

        let handle_area = self
            .ui
            .view(cx, &[id!(right_sidebar_resize_handle)])
            .area();
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
                self.right_sidebar_drag_start = Some((f.abs.x, self.right_sidebar_width));
            }
            Hit::FingerMove(f) => {
                if let Some((start_x, start_width)) = self.right_sidebar_drag_start {
                    let delta = (f.abs.x - start_x) as f32;
                    // Right panel: drag left = bigger, drag right = smaller
                    self.set_right_sidebar_width(cx, start_width - delta);
                }
            }
            Hit::FingerUp(_) => {
                self.right_sidebar_drag_start = None;
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
        let mut saw_app_action = false;
        for action in actions {
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                saw_app_action = true;
                match app_action {
                    AppAction::SessionCreated(session) => {
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        self.queue_or_select_session(cx, session.id.clone());
                    }
                    AppAction::SessionDeleted(session_id) => {
                        let tab_to_close = self.state.tab_by_session.get(session_id).copied();
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
                        if let Some(tab_id) = tab_to_close {
                            self.close_tab_now(cx, tab_id);
                        } else {
                            self.sync_active_center_ui(cx);
                        }
                    }
                    AppAction::OpenCodeEvent(oc_event) => {
                        let deleted_tab_id = match oc_event {
                            openpad_protocol::Event::SessionDeleted(session) => {
                                self.state.tab_by_session.get(&session.id).copied()
                            }
                            _ => None,
                        };
                        state::handle_opencode_event(&mut self.state, &self.ui, cx, oc_event);
                        if let Some(tab_id) = deleted_tab_id {
                            self.close_tab_now(cx, tab_id);
                        }
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
        if saw_app_action {
            self.refresh_open_center_tabs(cx);
            self.sync_active_center_ui(cx);
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

    fn handle_dialog_confirmed(&mut self, cx: &mut Cx, dialog_type: String, value: String) {
        if dialog_type == "unsaved_editor" {
            let saved = if let Some(tab_id) = self.current_active_file_tab_id() {
                self.save_file_tab(cx, tab_id)
            } else {
                true
            };
            if saved {
                self.run_pending_center_intent(cx);
            } else {
                self.state.pending_center_intent = None;
            }
            return;
        }

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

    fn handle_dialog_secondary(&mut self, cx: &mut Cx, dialog_type: String) {
        if dialog_type == "unsaved_editor" {
            if let Some(tab_id) = self.current_active_file_tab_id() {
                self.discard_file_tab_changes(cx, tab_id);
            }
            self.run_pending_center_intent(cx);
        }
    }

    fn handle_dialog_cancelled(&mut self, _cx: &mut Cx) {
        self.state.pending_center_intent = None;
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
                self.sidebar_mode = SidebarMode::Files;
                self.terminal_open = false;
                self.ui
                    .terminal_panel(cx, &[id!(terminal_panel_wrap)])
                    .set_open(cx, false);
                self.sidebar_width = SIDEBAR_DEFAULT_WIDTH;
                self.set_sidebar_width(cx, self.sidebar_width);
                self.right_sidebar_open = true;
                self.right_sidebar_width = RIGHT_SIDEBAR_DEFAULT_WIDTH;
                self.set_right_sidebar_width(cx, self.right_sidebar_width);
                self.ui
                    .side_panel(cx, &[id!(side_panel)])
                    .set_open(cx, true);
                self.ui
                    .side_panel(cx, &[id!(traffic_light_spacer)])
                    .set_open(cx, false);
                self.ui
                    .side_panel(cx, &[id!(right_side_panel)])
                    .set_open(cx, true);
                self.ui
                    .view(cx, &[id!(hamburger_button)])
                    .animator_play(cx, &[id!(open), id!(on)]);
                self.update_sidebar_handle_visibility(cx);
                self.update_sidebar_panel_visibility(cx);
                self.state
                    .center_tabs_by_id
                    .insert(live_id!(center_home_tab), CenterTabKind::Home);
                self.state.active_center_tab = Some(live_id!(center_home_tab));
                self.set_top_surfaces_for_active_kind(cx, Some(&CenterTabKind::Home));
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
                        KeyCode::KeyI => {
                            self.toggle_right_sidebar(cx);
                        }
                        KeyCode::KeyS => {
                            if let Some(tab_id) = self.current_active_file_tab_id() {
                                self.save_file_tab(cx, tab_id);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        self.handle_sidebar_resize(cx, event);
        self.handle_right_sidebar_resize(cx, event);

        // Handle UI events and capture actions
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });

        // Session options button in header (reliable way to open session menu)
        let opts_clicked = self
            .ui
            .button(cx, &[id!(session_info), id!(session_options_btn)])
            .clicked(&actions)
            || self.ui.button(cx, &[id!(session_options_btn)]).clicked(&actions);
        if opts_clicked {
            log!("Session options button clicked");
            if let Some(session_id) = &self.state.current_session_id {
                let working = self
                    .state
                    .working_by_session
                    .get(session_id)
                    .copied()
                    .unwrap_or(false);
                self.ui
                    .widget(cx, &[id!(session_options_popup)])
                    .set_visible(cx, true);
                self.ui
                    .session_options_popup(cx, &[id!(session_options_popup)])
                    .show(cx, session_id.clone(), working);
            }
        }

        // Process widget actions (e.g. ProjectsPanelAction from sidebar clicks)
        let mut needs_center_refresh = false;
        for action in &actions {
            if let Some(panel_action) = action.downcast_ref::<ProjectsPanelAction>() {
                match panel_action {
                    ProjectsPanelAction::SelectSession(session_id) => {
                        self.queue_or_select_session(cx, session_id.clone());
                    }
                    ProjectsPanelAction::OpenFile {
                        project_id,
                        absolute_path,
                    } => {
                        self.queue_or_open_file(cx, project_id.clone(), absolute_path.clone());
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
                    ProjectsPanelAction::OpenSessionContextMenu {
                        session_id,
                        x: _,
                        y: _,
                        working,
                    } => {
                        log!(
                            "ProjectsPanelAction::OpenSessionContextMenu session_id={} working={}",
                            session_id,
                            working
                        );
                        let popup_ref = self.ui.widget(cx, &[id!(session_options_popup)]);
                        popup_ref.set_visible(cx, true);
                        popup_ref.redraw(cx);
                        self.ui
                            .session_options_popup(cx, &[id!(session_options_popup)])
                            .show(cx, session_id.clone(), *working);
                        popup_ref.redraw(cx);
                    }
                    ProjectsPanelAction::OpenProjectContextMenu { project_id } => {
                        log!(
                            "ProjectsPanelAction::OpenProjectContextMenu project_id={:?}",
                            project_id
                        );
                        let popup_ref = self.ui.widget(cx, &[id!(session_options_popup)]);
                        popup_ref.set_visible(cx, true);
                        popup_ref.redraw(cx);
                        self.ui
                            .session_options_popup(cx, &[id!(session_options_popup)])
                            .show_project(cx, project_id.clone());
                        popup_ref.redraw(cx);
                    }
                    ProjectsPanelAction::CloseSessionContextMenu => {
                        log!("ProjectsPanelAction::CloseSessionContextMenu");
                        self.ui
                            .widget(cx, &[id!(session_options_popup)])
                            .set_visible(cx, false);
                        self.ui
                            .session_options_popup(cx, &[id!(session_options_popup)])
                            .hide(cx);
                    }
                    _ => {}
                }
            }

            if let Some(widget_action) = action.as_widget_action() {
                match widget_action.cast() {
                    DockAction::TabWasPressed(tab_id) => {
                        self.queue_or_switch_tab(cx, tab_id);
                    }
                    DockAction::TabCloseWasPressed(tab_id) => {
                        self.queue_or_close_tab(cx, tab_id);
                    }
                    DockAction::ShouldTabStartDrag(tab_id) => {
                        self.center_dock(cx).tab_start_drag(
                            cx,
                            tab_id,
                            DragItem::FilePath {
                                path: String::new(),
                                internal_id: Some(tab_id),
                            },
                        );
                    }
                    DockAction::Drag(drag_event) => {
                        if drag_event.items.len() == 1 {
                            self.center_dock(cx)
                                .accept_drag(cx, drag_event.clone(), DragResponse::Move);
                        }
                    }
                    DockAction::Drop(drop_event) => {
                        if let DragItem::FilePath { internal_id, .. } = &drop_event.items[0] {
                            if let Some(internal_id) = internal_id {
                                self.center_dock(cx).drop_move(cx, drop_event.abs, *internal_id);
                            }
                        }
                    }
                    DockAction::SplitPanelChanged { .. } | DockAction::None => {}
                }
            }

            if let Some(editor_action) = action.downcast_ref::<EditorPanelAction>() {
                match editor_action {
                    EditorPanelAction::TextDidChange => {
                        if let Some(tab_id) = self.current_active_file_tab_id() {
                            self.update_editor_header_ui_for_tab(cx, tab_id);
                        }
                    }
                    EditorPanelAction::None => {}
                }
            }

            // Handle MessageListAction
            if let Some(msg_action) = action.downcast_ref::<WidgetMessageListAction>() {
                match msg_action {
                    WidgetMessageListAction::RevertToMessage(message_id) => {
                        let mut target_session = self.state.current_session_id.clone();
                        if target_session.is_none() {
                            target_session = self
                                .state
                                .messages_by_session
                                .iter()
                                .find_map(|(sid, messages)| {
                                    messages
                                        .iter()
                                        .any(|m| m.info.id() == *message_id)
                                        .then(|| sid.clone())
                                });
                        }
                        if let Some(session_id) = target_session {
                            self.revert_to_message(cx, session_id, message_id.clone());
                        }
                    }
                    _ => {}
                }
            }

            // Handle AppAction from captured UI actions (e.g. DialogConfirmed, PermissionResponded)
            if let Some(app_action) = action.downcast_ref::<AppAction>() {
                match app_action {
                    AppAction::SetSidebarMode(mode) => {
                        self.sidebar_mode = *mode;
                        self.update_sidebar_panel_visibility(cx);
                    }
                    AppAction::DialogConfirmed { dialog_type, value } => {
                        self.handle_dialog_confirmed(cx, dialog_type.clone(), value.clone());
                    }
                    AppAction::PermissionResponded {
                        session_id,
                        request_id,
                        reply,
                    } => {
                        needs_center_refresh = true;
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
                        needs_center_refresh = true;
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
                        needs_center_refresh = true;
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
                        needs_center_refresh = true;
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
                        needs_center_refresh = true;
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
                        needs_center_refresh = true;
                    }
                    SimpleDialogAction::Secondary { dialog_type } => {
                        self.handle_dialog_secondary(cx, dialog_type.clone());
                        needs_center_refresh = true;
                    }
                    SimpleDialogAction::Cancelled => {
                        self.handle_dialog_cancelled(cx);
                        needs_center_refresh = true;
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
                        .messages_by_session
                        .get(&session_id)
                        .map(|v| {
                            v.iter().rev().find_map(|mwp| match &mwp.info {
                                openpad_protocol::Message::User(msg) => Some(msg.id.clone()),
                                _ => None,
                            })
                        })
                        .flatten();
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

        if needs_center_refresh {
            self.refresh_open_center_tabs(cx);
            self.sync_active_center_ui(cx);
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
