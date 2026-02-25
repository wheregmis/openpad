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
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

const SIDEBAR_DEFAULT_WIDTH: f32 = 260.0;
const SIDEBAR_MIN_WIDTH: f32 = 200.0;
const SIDEBAR_MAX_WIDTH: f32 = 420.0;
const RIGHT_SIDEBAR_DEFAULT_WIDTH: f32 = 260.0;
const RIGHT_SIDEBAR_MIN_WIDTH: f32 = 200.0;
const RIGHT_SIDEBAR_MAX_WIDTH: f32 = 420.0;

fn is_probably_binary(bytes: &[u8]) -> bool {
    bytes.iter().take(8192).any(|b| *b == 0)
}

app_main!(App);
mod action_dispatch;
pub(crate) mod composer;
mod dock_controller;
mod lifecycle;
mod sidebar;
mod ui_sync;

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
                                            work_label := Label { text: "Working..." }
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
                                width: Fill, height: Fill
                                flow: Down
                                View { width: Fill, height: Fill }
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
    #[rust]
    frame_count: u64,
    #[rust]
    last_share_copy_at: Option<std::time::Instant>,
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

    fn create_session(&mut self, _cx: &mut Cx, project_id: Option<String>) {
        let Some(client) = self.client_or_error() else {
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
        let Some(client) = self.client_or_error() else {
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
        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_aborter(runtime, client, session_id);
    }

    fn share_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_sharer(runtime, client, session_id);
    }

    fn unshare_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_unsharer(runtime, client, session_id);
    }

    fn summarize_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        let model_spec = self.state.selected_model_spec();
        let (provider_id, model_id) = match model_spec {
            Some(spec) => (spec.provider_id, spec.model_id),
            None => {
                // If no model is explicitly selected (e.g. "Default" is selected),
                // we try to use the first entry from model_entries if available.
                if let Some((pid, mid)) = self.state.model_entries.first() {
                    (pid.clone(), mid.clone())
                } else {
                    // Fallback to avoid crash, though ideally this shouldn't happen if connected
                    ("openai".to_string(), "gpt-4o".to_string())
                }
            }
        };

        async_runtime::spawn_session_summarizer(runtime, client, session_id, provider_id, model_id);
    }

    fn load_session_diff(&mut self, _cx: &mut Cx, session_id: String, message_id: Option<String>) {
        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        async_runtime::spawn_session_diff_loader(runtime, client, session_id, message_id);
    }

    fn branch_session(&mut self, _cx: &mut Cx, parent_session_id: String) {
        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the parent session to get its directory for the new branched session
        let directory = self.get_session_directory(&parent_session_id);

        async_runtime::spawn_session_brancher(runtime, client, parent_session_id, directory);
    }

    fn revert_to_message(&mut self, cx: &mut Cx, session_id: String, message_id: String) {
        // Show confirmation dialog
        self.ui
            .simple_dialog(cx, &[id!(simple_dialog)])
            .show_confirm(
                cx,
                "Revert Session",
                "Are you sure you want to revert to this message? All subsequent messages and actions will be lost.",
                format!("revert_session:{}:{}", session_id, message_id),
            );
    }

    fn unrevert_session(&mut self, _cx: &mut Cx, session_id: String) {
        let Some(client) = self.client_or_error() else {
            return;
        };
        let Some(runtime) = self._runtime.as_ref() else {
            return;
        };

        // Find the session to get its directory
        let directory = self.get_session_directory(&session_id);

        async_runtime::spawn_session_unreverter(runtime, client, session_id, directory);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if self.last_share_copy_at.is_some() {
            if let Event::NextFrame(_) = event {
                if let Some(time) = self.last_share_copy_at {
                    if time.elapsed().as_secs() >= 2 {
                        self.last_share_copy_at = None;
                        self.ui
                            .button(cx, &[id!(copy_share_button)])
                            .set_text(cx, "Copy link");
                        self.ui.view(cx, &[id!(session_info)]).redraw(cx);
                    }
                }
            }
            cx.new_next_frame();
        }

        if self.state.is_working {
            if let Event::NextFrame(_) = event {
                self.frame_count += 1;
                if self.frame_count % 6 == 0 {
                    let thinking_frame = (self.frame_count / 6 % 6) as usize;
                    let icon = crate::constants::SPINNER_FRAMES[thinking_frame];
                    self.ui
                        .label(cx, &[id!(work_indicator), id!(work_label)])
                        .set_text(cx, &format!("{} Working...", icon));
                    self.ui.view(cx, &[id!(work_indicator)]).redraw(cx);
                }
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
            || self
                .ui
                .button(cx, &[id!(session_options_btn)])
                .clicked(&actions);
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
                            self.center_dock(cx).accept_drag(
                                cx,
                                drag_event.clone(),
                                DragResponse::Move,
                            );
                        }
                    }
                    DockAction::Drop(drop_event) => {
                        if let DragItem::FilePath { internal_id, .. } = &drop_event.items[0] {
                            if let Some(internal_id) = internal_id {
                                self.center_dock(cx)
                                    .drop_move(cx, drop_event.abs, *internal_id);
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
                            target_session = self.state.messages_by_session.iter().find_map(
                                |(sid, messages)| {
                                    messages
                                        .iter()
                                        .any(|m| m.info.id() == *message_id)
                                        .then(|| sid.clone())
                                },
                            );
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
                        state::handle_app_action(&mut self.state, &self.ui, cx, app_action);
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
                                    session_id.clone(),
                                    request_id.clone(),
                                    openpad_protocol::PermissionReply::Once,
                                );
                            }
                            state::handle_app_action(
                                &mut self.state,
                                &self.ui,
                                cx,
                                &AppAction::PermissionDismissed {
                                    session_id: session_id.clone(),
                                    request_id: request_id.clone(),
                                },
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
                                    session_id.clone(),
                                    request_id.clone(),
                                    openpad_protocol::PermissionReply::Always,
                                );
                            }
                            state::handle_app_action(
                                &mut self.state,
                                &self.ui,
                                cx,
                                &AppAction::PermissionDismissed {
                                    session_id: session_id.clone(),
                                    request_id: request_id.clone(),
                                },
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
                                    session_id.clone(),
                                    request_id.clone(),
                                    openpad_protocol::PermissionReply::Reject,
                                );
                            }
                            state::handle_app_action(
                                &mut self.state,
                                &self.ui,
                                cx,
                                &AppAction::PermissionDismissed {
                                    session_id: session_id.clone(),
                                    request_id: request_id.clone(),
                                },
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
                        state::handle_app_action(
                            &mut self.state,
                            &self.ui,
                            cx,
                            &AppAction::PermissionResponded {
                                session_id: session_id.clone(),
                                request_id: request_id.clone(),
                                reply: reply.clone(),
                            },
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
                use crate::state::AttachedFile;

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
                self.last_share_copy_at = Some(std::time::Instant::now());
                self.ui
                    .button(cx, &[id!(copy_share_button)])
                    .set_text(cx, "Copied!");
                self.ui.view(cx, &[id!(session_info)]).redraw(cx);
            }
        }

        if self
            .ui
            .button(cx, &[id!(summarize_button)])
            .clicked(&actions)
        {
            let session_id = self.state.current_session_id.clone();
            if let Some(session_id) = session_id {
                let message_id = self
                    .state
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
    use super::composer::get_image_data_url_regex;

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
