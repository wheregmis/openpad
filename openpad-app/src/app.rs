use crate::async_runtime;
use crate::constants::OPENCODE_SERVER_URL;
use crate::state::{self, AppAction, AppState, ProjectsPanelAction};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use makepad_widgets::*;
use openpad_protocol::OpenCodeClient;
use openpad_widgets::message_list::MessageListWidgetRefExt;
use openpad_widgets::permission_card::PermissionCardAction;
use openpad_widgets::simple_dialog::SimpleDialogWidgetRefExt;
use openpad_widgets::terminal::{TerminalAction, TerminalWidgetRefExt};
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

fn detect_image_mime_type(data: &[u8]) -> Option<&'static str> {
    if data.starts_with(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]) {
        return Some("image/png");
    }
    if data.len() >= 3 && data[0] == 0xff && data[1] == 0xd8 && data[2] == 0xff {
        return Some("image/jpeg");
    }
    if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    if data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    if data.starts_with(b"II*\0") || data.starts_with(b"MM\0*") {
        return Some("image/tiff");
    }
    None
}

app_main!(App);

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;
    use openpad_widgets::app_bg::AppBg;
    use openpad_widgets::hamburger_button::HamburgerButton;
    use openpad_widgets::header_bar::HeaderBar;
    use openpad_widgets::input_bar::InputBar;
    use openpad_widgets::input_bar::InputBarDropDown;
    use openpad_widgets::input_bar::InputBarToolbar;
    use openpad_widgets::input_field::InputField;
    use openpad_widgets::send_button::SendButton;
    use openpad_widgets::side_panel::SidePanel;
    use openpad_widgets::simple_dialog::SimpleDialog;
    use openpad_widgets::status_dot::StatusDot;
    use makepad_code_editor::code_view::CodeView;

    // Import component DSL definitions
    use openpad_widgets::user_bubble::UserBubble;
    use openpad_widgets::assistant_bubble::AssistantBubble;
    use crate::components::projects_panel::ProjectsPanel;
    use openpad_widgets::permission_card::PermissionCard;
    use openpad_widgets::permission_dialog::PermissionDialog;
    use openpad_widgets::message_list::MessageList;
    use openpad_widgets::diff_view::DiffView;
    use openpad_widgets::terminal::Terminal;
    use openpad_widgets::terminal_panel::TerminalPanel;
    use openpad_widgets::settings_dialog::SettingsDialog;

    ChatPanel = <View> {
        width: Fill, height: Fill
        flow: Down
        spacing: 0

        <View> { width: Fill, height: 1, show_bg: true, draw_bg: { color: (THEME_COLOR_SHADE_2) } }

        session_summary = <RoundedView> {
            visible: false
            width: Fill, height: Fit
            padding: { left: 16, right: 16, top: 12, bottom: 12 }
            flow: Down
            spacing: 8
            show_bg: true
            draw_bg: {
                color: (THEME_COLOR_BG_DIALOG)
                border_color: (THEME_COLOR_BORDER_DIALOG)
                border_radius: 8.0
                border_size: 1.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, self.border_radius);
                    sdf.fill_keep(self.color);
                    sdf.stroke(self.border_color, self.border_size);
                    return sdf.result;
                }
            }

            summary_header = <View> {
                width: Fill, height: Fit
                flow: Right
                align: { y: 0.5 }

                summary_title = <Label> {
                    text: "Session Summary"
                    draw_text: { color: (THEME_COLOR_TEXT_PRIMARY), text_style: <THEME_FONT_BOLD> { font_size: 10 } }
                }
                <View> { width: Fill }
            }

            summary_stats_label = <Label> {
                width: Fill, height: Fit
                text: ""
                draw_text: { color: (THEME_COLOR_TEXT_DIM), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
            }

            summary_diff = <Markdown> {
                width: Fill, height: Fit
                font_size: 9
                font_color: (THEME_COLOR_TEXT_NORMAL)
                paragraph_spacing: 6
                pre_code_spacing: 6
                use_code_block_widget: true

                code_block = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: { left: 8, right: 8, top: 6, bottom: 6 }
                    margin: { top: 4, bottom: 4 }
                    draw_bg: {
                        color: (THEME_COLOR_BG_INPUT)
                        border_radius: 6.0
                    }

                    code_view = <CodeView> {
                        editor: {
                            width: Fill
                            height: Fit
                            draw_bg: { color: (THEME_COLOR_BG_INPUT) }
                        }
                    }
                }

                draw_normal: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.4 }
                    color: (THEME_COLOR_TEXT_NORMAL)
                }
                draw_italic: {
                    text_style: <THEME_FONT_ITALIC> { font_size: 9 }
                    color: (THEME_COLOR_TEXT_NORMAL)
                }
                draw_bold: {
                    text_style: <THEME_FONT_BOLD> { font_size: 9 }
                    color: (THEME_COLOR_TEXT_BOLD)
                }
                draw_fixed: {
                    text_style: <THEME_FONT_CODE> { font_size: 8 }
                    color: (THEME_COLOR_TEXT_CODE)
                }
            }
        }

        // Chat area - Unified
        <View> {
            width: Fill, height: Fill
            flow: Down
            spacing: 0
            show_bg: true
            draw_bg: { color: (THEME_COLOR_BG_APP) }

            <View> {
                width: Fill, height: Fill
                message_list = <MessageList> { width: Fill, height: Fill }
            }

            input_row = <View> {
                width: Fill, height: Fit
                padding: { left: 32, right: 32, top: 12, bottom: 20 }
                flow: Down, spacing: 8
                clip_y: false

                // Attachments preview area
                attachments_preview = <RoundedView> {
                    visible: false
                    width: Fill, height: Fit
                    flow: Right, spacing: 8
                    padding: { left: 18, right: 18, top: 8, bottom: 8 }
                    show_bg: true
                    draw_bg: {
                        color: (THEME_COLOR_SHADE_2)
                        border_radius: 8.0
                    }

                    attachments_label = <Label> {
                        text: "Attached:"
                        draw_text: { color: (THEME_COLOR_TEXT_MUTED), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                    }
                    attachments_list = <Label> {
                        text: ""
                        draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHTER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                    }
                    <View> { width: Fill }
                    clear_attachments_button = <Button> {
                        width: Fit, height: 20
                        text: "Clear"
                        draw_text: { color: (THEME_COLOR_ACCENT_AMBER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                        draw_bg: {
                            color: (THEME_COLOR_TRANSPARENT)
                            color_hover: (THEME_COLOR_HOVER_MEDIUM)
                        }
                    }
                }

                skill_preview = <RoundedView> {
                    visible: false
                    width: Fill, height: Fit
                    flow: Down, spacing: 4
                    padding: { left: 18, right: 18, top: 8, bottom: 8 }
                    show_bg: true
                    draw_bg: {
                        color: (THEME_COLOR_SHADE_2)
                        border_radius: 8.0
                    }

                    skill_header = <View> {
                        width: Fill, height: Fit
                        flow: Right, spacing: 8
                        align: { y: 0.5 }

                        skill_name_label = <Label> {
                            text: "Skill"
                            draw_text: { color: (THEME_COLOR_SHADE_8), text_style: <THEME_FONT_BOLD> { font_size: 9 } }
                        }
                        <View> { width: Fill }
                        clear_skill_button = <Button> {
                            width: Fit, height: 20
                            text: "Clear"
                            draw_text: { color: (THEME_COLOR_ACCENT_AMBER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                            }
                        }
                    }

                    skill_desc_label = <Label> {
                        text: ""
                        draw_text: { color: (THEME_COLOR_SHADE_9), text_style: <THEME_FONT_REGULAR> { font_size: 9, line_spacing: 1.3 }, word: Wrap }
                    }
                }

                <InputBar> {
                    width: Fill
                    input_box = <InputField> {}
                    input_bar_toolbar = <InputBarToolbar> {
                        agent_dropdown = <InputBarDropDown> {
                             labels: ["Agent"]
                         }
                         skill_dropdown = <InputBarDropDown> {
                             width: 120
                             labels: ["Skill"]
                         }
                         provider_dropdown = <InputBarDropDown> {
                             width: 120
                             labels: ["Provider"]
                         }
                         model_dropdown = <InputBarDropDown> {
                             width: 150
                             labels: ["Model"]
                         }
                        <View> { width: Fill }
                        send_button = <SendButton> {
                            margin: { left: 0 }
                        }
                    }
                }
            }
        }
    }

    TerminalPanelWrap = <TerminalPanel> {}

    App = {{App}} {
        ui: <Window> {
            window: { inner_size: vec2(1200, 800) }
            pass: { clear_color: (THEME_COLOR_BG_DARKER) }

            body = <View> {
                width: Fill, height: Fill
                flow: Overlay

                <AppBg> {
                width: Fill, height: Fill
                flow: Down,
                spacing: 0,
                padding: 0,

                <View> {
                    width: Fill, height: Fill
                    flow: Right,
                    spacing: 0,

                    side_panel = <SidePanel> {
                        width: 260.0, height: Fill
                        open_size: 260.0

                        <HeaderBar> {
                            height: 36
                            width: Fill
                            padding: { left: 80, right: 10 }
                            draw_bg: {
                                color: (THEME_COLOR_BG_APP)
                                border_color: (THEME_COLOR_BORDER_MEDIUM)
                                border_radius: 0.0
                                border_size: 1.0
                            }

                            <View> { width: Fill }

                            sidebar_tabs = <View> {
                                width: Fit, height: Fill
                                flow: Right
                                spacing: 4
                                align: { y: 0.5 }

                                projects_tab = <Button> {
                                    width: Fit, height: 24
                                    padding: { left: 8, right: 8, top: 4, bottom: 4 }
                                    text: "Projects"
                                    draw_bg: {
                                        color: (THEME_COLOR_TRANSPARENT)
                                        color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                        border_radius: 4.0
                                        border_size: 0.0
                                    }
                                    draw_text: {
                                        color: (THEME_COLOR_TEXT_MUTED)
                                        text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                                    }
                                }

                                settings_tab = <Button> {
                                    width: Fit, height: 24
                                    padding: { left: 8, right: 8, top: 4, bottom: 4 }
                                    text: "Settings"
                                    draw_bg: {
                                        color: (THEME_COLOR_TRANSPARENT)
                                        color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                        border_radius: 4.0
                                        border_size: 0.0
                                    }
                                    draw_text: {
                                        color: (THEME_COLOR_TEXT_MUTED)
                                        text_style: <THEME_FONT_REGULAR> { font_size: 10 }
                                    }
                                }
                            }
                        }

                        projects_panel = <ProjectsPanel> {
                            visible: true
                        }

                        settings_panel = <SettingsDialog> {
                            visible: false
                            width: Fill, height: Fill
                        }
                    }
                    sidebar_resize_handle = <View> {
                        width: 6, height: Fill
                        show_bg: true
                        draw_bg: { color: (THEME_COLOR_BORDER_MEDIUM) }
                    }

                    <View> {
                        width: Fill, height: Fill
                        flow: Down,
                        spacing: 0,

                        // Main Header
                        main_header = <HeaderBar> {
                            height: 36
                            padding: { left: 16, right: 10 }
                            draw_bg: {
                                color: (THEME_COLOR_BG_APP)
                                border_color: (THEME_COLOR_BORDER_MEDIUM)
                                border_radius: 0.0
                            }

                            // This spacer expands when the sidebar closes to keep traffic lights clear
                            traffic_light_spacer = <SidePanel> {
                                width: 0.0, height: Fill
                                open_size: 80.0
                                close_size: 0.0
                                draw_bg: { color: (THEME_COLOR_TRANSPARENT), border_size: 0.0 } // Transparent!
                            }

                            hamburger_button = <HamburgerButton> {
                                width: 32, height: 32
                            }
                            <View> { width: 4 }
                            app_title = <Label> {
                                text: "Openpad"
                                draw_text: { color: (THEME_COLOR_TEXT_MUTED), text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                            }
                            <View> { width: Fill }
                            status_row = <View> {
                                width: Fit, height: Fit
                                flow: Right
                                spacing: 8
                                align: { y: 0.5 }
                                status_dot = <StatusDot> {}
                                status_label = <Label> {
                                    text: "Connected"
                                    draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARK), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }
                            }
                        }

                        // Slim Breadcrumbs Bar
                        session_info = <View> {
                            width: Fill, height: 32
                            padding: { left: 16, right: 16 }
                            flow: Right,
                            spacing: 8,
                            align: { y: 0.5 }
                            show_bg: true
                            draw_bg: { color: (THEME_COLOR_BG_APP) }

                            project_row = <View> {
                                width: Fit, height: Fit
                                flow: Right, spacing: 4, align: {y: 0.5}
                                project_badge = <View> {
                                    width: Fit, height: Fit
                                    project_badge_label = <Label> {
                                        text: "No project"
                                        draw_text: { color: (THEME_COLOR_TEXT_MUTED), text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                                    }
                                }
                            }

                            <Label> { text: "/", draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 10 } } }

                            session_row = <View> {
                                width: Fit, height: Fit
                                session_title = <Label> {
                                    text: "New Session"
                                    draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHTER), text_style: <THEME_FONT_BOLD> { font_size: 10 } }
                                }
                            }

                            project_path_wrap = <View> {
                                visible: false
                                project_path_label = <Label> { text: "" }
                            }

                            <View> { width: Fill }

                            share_wrap = <View> {
                                width: Fit, height: Fit
                                flow: Right,
                                spacing: 6,
                                align: { y: 0.5 }

                                share_button = <Button> {
                                    width: Fit, height: 20
                                    text: "Share"
                                    draw_bg: {
                                        color: (THEME_COLOR_TRANSPARENT)
                                        color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                        border_radius: 4.0
                                        border_size: 0.0
                                    }
                                    draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }

                                unshare_button = <Button> {
                                    width: Fit, height: 20
                                    visible: false
                                    text: "Unshare"
                                    draw_bg: {
                                        color: (THEME_COLOR_TRANSPARENT)
                                        color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                        border_radius: 4.0
                                        border_size: 0.0
                                    }
                                    draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }

                                copy_share_button = <Button> {
                                    width: Fit, height: 20
                                    visible: false
                                    text: "Copy link"
                                    draw_bg: {
                                        color: (THEME_COLOR_TRANSPARENT)
                                        color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                        border_radius: 4.0
                                        border_size: 0.0
                                    }
                                    draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }

                                share_url_label = <Label> {
                                    width: Fit, height: Fit
                                    text: ""
                                    draw_text: { color: (THEME_COLOR_TEXT_MUTED_DARKER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }
                            }

                            summarize_button = <Button> {
                                width: Fit, height: 20
                                text: "Summarize"
                                draw_bg: {
                                    color: (THEME_COLOR_TRANSPARENT)
                                    color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                    border_radius: 4.0
                                    border_size: 0.0
                                }
                                draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                            }

                            revert_indicator = <View> {
                                visible: false
                                revert_indicator_label = <Label> {
                                    text: "⟲ Reverted"
                                    draw_text: { color: (THEME_COLOR_ACCENT_AMBER), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }
                            }
                            unrevert_wrap = <View> {
                                visible: false
                                unrevert_button = <Button> {
                                    width: Fit, height: 20
                                    text: "Unrevert"
                                    draw_text: { color: (THEME_COLOR_ACCENT_BLUE), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                                }
                            }
                        }
                        <ChatPanel> {}
                        terminal_panel_wrap = <TerminalPanelWrap> {}
                    }
                }
                }

                // Simple dialog for confirmations and inputs (shown as overlay)
                simple_dialog = <SimpleDialog> {}
            }
        }
    }
}

#[derive(Live, LiveHook)]
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

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        openpad_widgets::live_design(cx);
        makepad_code_editor::code_editor::live_design(cx);
        makepad_code_editor::code_view::live_design(cx);
        crate::components::projects_panel::live_design(cx);
    }
}

impl App {
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
            .view(&[id!(attachments_preview)])
            .set_visible(cx, has_attachments);

        if has_attachments {
            let filenames: Vec<String> = self
                .state
                .attached_files
                .iter()
                .map(|f| f.filename.clone())
                .collect();
            let text = filenames.join(", ");
            self.ui.label(&[id!(attachments_list)]).set_text(cx, &text);
        }
        self.ui.redraw(cx);
    }

    fn update_skill_ui(&self, cx: &mut Cx) {
        let selected = self.state.selected_skill();
        let has_skill = selected.is_some();
        self.ui
            .view(&[id!(skill_preview)])
            .set_visible(cx, has_skill);

        if let Some(skill) = selected {
            self.ui
                .label(&[id!(skill_name_label)])
                .set_text(cx, &skill.name);
            self.ui
                .label(&[id!(skill_desc_label)])
                .set_text(cx, &skill.description);
        }
        self.ui.redraw(cx);
    }

    fn set_sidebar_width(&mut self, cx: &mut Cx, width: f32) {
        let clamped = width.clamp(SIDEBAR_MIN_WIDTH, SIDEBAR_MAX_WIDTH);
        self.sidebar_width = clamped;
        self.ui
            .side_panel(&[id!(side_panel)])
            .apply_over(cx, live! { open_size: (clamped) });
        self.ui.redraw(cx);
    }

    fn update_sidebar_handle_visibility(&self, cx: &mut Cx) {
        let width = if self.sidebar_open { 6.0 } else { 0.0 };
        self.ui
            .view(&[id!(sidebar_resize_handle)])
            .apply_over(cx, live! { width: (width) });
        self.ui
            .view(&[id!(sidebar_resize_handle)])
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
            .side_panel(&[id!(side_panel)])
            .set_open(cx, self.sidebar_open);
        self.ui
            .side_panel(&[id!(traffic_light_spacer)])
            .set_open(cx, !self.sidebar_open);
        self.update_sidebar_handle_visibility(cx);
        self.update_sidebar_panel_visibility(cx);

        if self.sidebar_open {
            self.ui
                .view(&[id!(hamburger_button)])
                .animator_play(cx, &[id!(open), id!(on)]);
        } else {
            self.ui
                .view(&[id!(hamburger_button)])
                .animator_play(cx, &[id!(open), id!(off)]);
        }
    }

    fn update_sidebar_panel_visibility(&mut self, cx: &mut Cx) {
        let show_projects = self.sidebar_mode == SidebarMode::Projects;
        let show_settings = self.sidebar_mode == SidebarMode::Settings;

        // Use widget() for custom widgets (ProjectsPanel, SettingsDialog).
        self.ui
            .widget(&[id!(side_panel), id!(projects_panel)])
            .set_visible(cx, show_projects);
        self.ui
            .widget(&[id!(side_panel), id!(settings_panel)])
            .set_visible(cx, show_settings);

        // Update tab button styling to show active state
        // Use hex colors directly since theme constants aren't available in Rust code
        let projects_color = if show_projects {
            vec4(0.9, 0.91, 0.93, 1.0) // THEME_COLOR_TEXT_PRIMARY equivalent
        } else {
            vec4(0.53, 0.53, 0.53, 1.0) // THEME_COLOR_TEXT_MUTED equivalent (#888)
        };

        let settings_color = if show_settings {
            vec4(0.9, 0.91, 0.93, 1.0) // THEME_COLOR_TEXT_PRIMARY equivalent
        } else {
            vec4(0.53, 0.53, 0.53, 1.0) // THEME_COLOR_TEXT_MUTED equivalent (#888)
        };

        self.ui
            .button(&[id!(side_panel), id!(sidebar_tabs), id!(projects_tab)])
            .apply_over(
                cx,
                live! {
                    draw_text: {
                        color: (projects_color)
                    }
                },
            );

        self.ui
            .button(&[id!(side_panel), id!(sidebar_tabs), id!(settings_tab)])
            .apply_over(
                cx,
                live! {
                    draw_text: {
                        color: (settings_color)
                    }
                },
            );

        // Force redraw of the side panel to ensure visibility changes take effect
        self.ui.view(&[id!(side_panel)]).redraw(cx);
        self.ui.redraw(cx);
    }

    fn toggle_terminal(&mut self, cx: &mut Cx) {
        self.terminal_open = !self.terminal_open;
        self.ui
            .terminal_panel(&[id!(terminal_panel_wrap)])
            .set_open(cx, self.terminal_open);
    }

    fn handle_sidebar_resize(&mut self, cx: &mut Cx, event: &Event) {
        if !self.sidebar_open {
            self.sidebar_drag_start = None;
            return;
        }

        let handle_area = self.ui.view(&[id!(sidebar_resize_handle)]).area();
        let hit = event.hits_with_options(
            cx,
            handle_area,
            HitOptions::new().with_margin(Margin {
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

    fn handle_image_input(&mut self, cx: &mut Cx, image: &ImageInputEvent) {
        use crate::state::handlers::AttachedFile;

        if image.data.is_empty() {
            return;
        }

        if !self.ui.text_input(&[id!(input_box)]).key_focus(cx) {
            return;
        }

        let Some(mime_type) = detect_image_mime_type(&image.data) else {
            log!(
                "Unsupported clipboard image format ({} bytes)",
                image.data.len()
            );
            return;
        };

        let extension = image_extension_for_mime(mime_type);
        let filename = format!(
            "attachment_{}_{}.{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
            self.state.attached_files.len(),
            extension
        );
        let data_url = format!("data:{};base64,{}", mime_type, STANDARD.encode(&image.data));

        self.state.attached_files.push(AttachedFile {
            filename: filename.clone(),
            mime_type: mime_type.to_string(),
            data_url,
            raw_text: None,
        });

        log!(
            "Detected pasted image from clipboard: {} ({})",
            mime_type,
            filename
        );
        self.update_attachments_ui(cx);
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
            // Handle TerminalAction from background thread
            if let Some(terminal_action) = action.downcast_ref::<TerminalAction>() {
                self.ui
                    .terminal(&[id!(terminal_panel)])
                    .handle_action(cx, terminal_action);
            }

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
        self.ui.simple_dialog(&[id!(simple_dialog)]).show_confirm(
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
        self.ui.simple_dialog(&[id!(simple_dialog)]).show_input(
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
        match event {
            Event::Startup => {
                self.connect_to_opencode(cx);
                if !cx.in_makepad_studio() {
                    #[cfg(not(target_os = "macos"))]
                    if let Some(mut window) = self.ui.borrow_mut::<Window>() {
                        window.set_fullscreen(cx);
                    }
                }
                // Initialize terminal
                self.ui.terminal(&[id!(terminal_panel)]).init_pty(cx);

                // Initialize sidebar open, terminal collapsed by default
                self.sidebar_open = true;
                self.sidebar_mode = SidebarMode::Projects;
                self.terminal_open = false;
                self.ui
                    .terminal_panel(&[id!(terminal_panel_wrap)])
                    .set_open(cx, false);
                self.sidebar_width = SIDEBAR_DEFAULT_WIDTH;
                self.set_sidebar_width(cx, self.sidebar_width);
                self.ui.side_panel(&[id!(side_panel)]).set_open(cx, true);
                self.ui
                    .side_panel(&[id!(traffic_light_spacer)])
                    .set_open(cx, false);
                self.ui
                    .view(&[id!(hamburger_button)])
                    .animator_play(cx, &[id!(open), id!(on)]);
                self.update_sidebar_handle_visibility(cx);
                self.update_sidebar_panel_visibility(cx);
            }
            Event::ImageInput(image) => {
                self.handle_image_input(cx, image);
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
                        self.ui.message_list(&[id!(message_list)]).set_messages(
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

            // Handle TerminalAction
            if let Some(terminal_action) = action.downcast_ref::<TerminalAction>() {
                self.ui
                    .terminal(&[id!(terminal_panel)])
                    .handle_action(cx, terminal_action);
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
        if let Some(new_text) = self.ui.text_input(&[id!(input_box)]).changed(&actions) {
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

                self.ui.text_input(&[id!(input_box)]).set_text(cx, "");
                self.update_attachments_ui(cx);
            } else if remaining != new_text {
                // Images were extracted — update the input with remaining text
                self.ui
                    .text_input(&[id!(input_box)])
                    .set_text(cx, &remaining);
            }
        }

        // Check for text input return
        if let Some((text, _modifiers)) = self.ui.text_input(&[id!(input_box)]).returned(&actions) {
            if !text.is_empty() {
                let processed_text = self.process_pasted_content(cx, &text);
                self.send_message(cx, processed_text);
                self.ui.text_input(&[id!(input_box)]).set_text(cx, "");
            }
        }

        // Handle unrevert button
        if self.ui.button(&[id!(unrevert_button)]).clicked(&actions) {
            if let Some(session_id) = &self.state.current_session_id {
                self.unrevert_session(cx, session_id.clone());
            }
        }

        if self.ui.button(&[id!(share_button)]).clicked(&actions) {
            if let Some(session_id) = &self.state.current_session_id {
                self.share_session(cx, session_id.clone());
            }
        }

        if self.ui.button(&[id!(unshare_button)]).clicked(&actions) {
            if let Some(session_id) = &self.state.current_session_id {
                self.unshare_session(cx, session_id.clone());
            }
        }

        if self.ui.button(&[id!(copy_share_button)]).clicked(&actions) {
            if let Some(url) = self.state.current_share_url() {
                cx.copy_to_clipboard(&url);
            }
        }

        if self.ui.button(&[id!(summarize_button)]).clicked(&actions) {
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

        if self.ui.button(&[id!(hamburger_button)]).clicked(&actions) {
            self.toggle_sidebar(cx);
        }

        if self
            .ui
            .button(&[id!(side_panel), id!(sidebar_tabs), id!(projects_tab)])
            .clicked(&actions)
        {
            self.sidebar_mode = SidebarMode::Projects;
            self.update_sidebar_panel_visibility(cx);
        }

        if self
            .ui
            .button(&[id!(side_panel), id!(sidebar_tabs), id!(settings_tab)])
            .clicked(&actions)
        {
            self.sidebar_mode = SidebarMode::Settings;
            if !self.sidebar_open {
                self.toggle_sidebar(cx);
            }
            self.update_sidebar_panel_visibility(cx);
        }

        if self.ui.button(&[id!(send_button)]).clicked(&actions) {
            let text = self.ui.text_input(&[id!(input_box)]).text();
            if !text.is_empty() {
                let processed_text = self.process_pasted_content(cx, &text);
                self.send_message(cx, processed_text);
                self.ui.text_input(&[id!(input_box)]).set_text(cx, "");
            }
        }

        // Handle clear attachments button
        if self
            .ui
            .button(&[id!(clear_attachments_button)])
            .clicked(&actions)
        {
            self.state.attached_files.clear();
            self.update_attachments_ui(cx);
        }

        if self.ui.button(&[id!(clear_skill_button)]).clicked(&actions) {
            self.state.selected_skill_idx = None;
            self.ui
                .up_drop_down(&[id!(input_bar_toolbar), id!(skill_dropdown)])
                .set_selected_item(cx, 0);
            self.update_skill_ui(cx);
        }

        // Handle dropdown selections (main input bar only)
        // Provider selection changed - update model list
        if let Some(idx) = self
            .ui
            .up_drop_down(&[id!(input_bar_toolbar), id!(provider_dropdown)])
            .changed(&actions)
        {
            self.state.selected_provider_idx = idx;
            self.state.update_model_list_for_provider();
            self.ui
                .up_drop_down(&[id!(input_bar_toolbar), id!(model_dropdown)])
                .set_labels(cx, self.state.model_labels.clone());
            self.ui
                .up_drop_down(&[id!(input_bar_toolbar), id!(model_dropdown)])
                .set_selected_item(cx, 0);
        }

        // Model selection changed
        if let Some(idx) = self
            .ui
            .up_drop_down(&[id!(input_bar_toolbar), id!(model_dropdown)])
            .changed(&actions)
        {
            self.state.selected_model_idx = idx;
        }
        if let Some(idx) = self
            .ui
            .up_drop_down(&[id!(input_bar_toolbar), id!(agent_dropdown)])
            .changed(&actions)
        {
            self.state.selected_agent_idx = if idx > 0 { Some(idx - 1) } else { None };
        }
        if let Some(idx) = self
            .ui
            .up_drop_down(&[id!(input_bar_toolbar), id!(skill_dropdown)])
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
