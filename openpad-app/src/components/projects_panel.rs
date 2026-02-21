use crate::async_runtime;
use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::*;
use openpad_protocol::{Project, Session, SessionSummary};
use std::collections::HashMap;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.ProjectsPanel = #(ProjectsPanel::register_widget(vm)) {
        width: Fill, height: Fill
        flow: Down
        padding: Inset{ left: 10, right: 8, top: 8, bottom: 8 }
        spacing: 0
        new_batch: true

        list := PortalList {
            width: Fill, height: Fill
            scroll_bar: ScrollBar {
                bar_size: 2.5
                bar_side_margin: 2.0
                smoothing: 0.15
            }

            ProjectHeader := View {
                width: Fill, height: 28
                flow: Right, align: Align{y: 0.5}
                padding: Inset{ top: 2, bottom: 2, left: 0, right: 2 }
                spacing: 2
                new_batch: true
                left_group := View {
                    width: Fill, height: Fill
                    flow: Right, align: Align{y: 0.5}
                    spacing: 2

                    chevron := Label {
                        width: 10, height: Fit
                        text: "v"
                        draw_text +: { color: theme.THEME_COLOR_TEXT_MUTED, text_style: theme.font_bold { font_size: 9.0 } }
                    }

                    folder_icon := View {
                        width: 17, height: Fill
                        padding: Inset{ left: 1, right: 3, top: 4, bottom: 0 }
                        align: Align{ y: 0.5 }

                        glyph := Icon {
                            width: 14, height: 14
                            icon_walk: Walk{ width: 14, height: 14 }
                            draw_icon +: {
                                svg: crate_resource("self://resources/icons/folder_sidebar.svg")
                                color: theme.THEME_COLOR_TEXT_MUTED_LIGHTER
                            }
                        }
                    }

                    project_toggle := Button {
                        width: Fill, height: Fill
                        margin: Inset{ left: 0, right: 2 }
                        padding: Inset{ left: 2, right: 2, top: 0, bottom: 0 }
                        align: Align{ x: 0.0, y: 0.5 }
                        text: "Project"
                        draw_bg +: {
                            color: theme.THEME_COLOR_TRANSPARENT
                            color_hover: theme.THEME_COLOR_HOVER_SUBTLE
                            color_active: theme.THEME_COLOR_HOVER_SUBTLE
                            border_radius: 8.0
                            border_size: 0.0
                        }
                        draw_text +: {
                            color: theme.THEME_COLOR_TEXT_PRIMARY
                            text_style: theme.font_bold { font_size: 11.5 }
                        }
                    }
                }

                project_working_dot := RoundedView {
                    visible: false
                    width: 5, height: 5
                    margin: Inset{ right: 5 }
                    show_bg: true
                    draw_bg +: {
                        color: theme.THEME_COLOR_ACCENT_AMBER
                        border_radius: 2.5
                    }
                }

                new_session_header_button := Button {
                    width: 20, height: 20
                    margin: Inset{ left: 2 }
                    text: "New session"
                    draw_text +: { color: #0000 }
                    draw_bg +: {
                        color: theme.THEME_COLOR_TRANSPARENT
                        color_hover: theme.THEME_COLOR_HOVER_SUBTLE
                        color_active: theme.THEME_COLOR_HOVER_SUBTLE
                        border_radius: 5.0
                        border_size: 0.0

                        pixel: fn() {
                            let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius)
                            sdf.fill(mix(self.color, self.color_hover, self.hover))

                            let cx = self.rect_size.x * 0.5
                            let cy = self.rect_size.y * 0.5
                            let thickness = 0.8
                            let size = 4.0
                            let plus_color = theme.THEME_COLOR_TEXT_MUTED
                            sdf.rect(cx - size, cy - thickness, size * 2.0, thickness * 2.0)
                            sdf.fill(plus_color)
                            sdf.rect(cx - thickness, cy - size, thickness * 2.0, size * 2.0)
                            sdf.fill(plus_color)

                            return sdf.result
                        }
                    }
                }
            }

            SessionRow := View {
                width: Fill, height: Fit
                flow: Overlay
                new_batch: true

                selected_pill := RoundedView {
                    visible: false
                    width: Fill, height: Fit
                    margin: Inset{ left: 22, right: 3, top: 1, bottom: 1 }
                    show_bg: true
                    draw_bg +: {
                        color: theme.THEME_COLOR_HOVER_SUBTLE
                        border_radius: 8.0
                    }
                }

                View {
                    width: Fill, height: Fit
                    padding: Inset{ top: 1, bottom: 1, left: 22, right: 28 }
                    flow: Right,
                    spacing: 3,
                    align: Align{ y: 0.5 }

                    session_button := Button {
                        width: Fill, height: 27
                        margin: Inset{ right: 2 }
                        text: "Session"
                        draw_bg +: {
                            color: theme.THEME_COLOR_TRANSPARENT
                            color_hover: theme.THEME_COLOR_HOVER_SUBTLE
                            color_active: theme.THEME_COLOR_TRANSPARENT
                            border_radius: 8.0
                            border_size: 0.0
                        }
                        draw_text +: {
                            color: theme.THEME_COLOR_TEXT_LIGHT,
                            text_style: theme.font_regular { font_size: 10.0 }}
                    }

                    summary_stats := View {
                        width: Fit, height: Fit
                        margin: Inset{ right: 2 }
                        flow: Right
                        spacing: 5
                        align: Align{ y: 0.5 }

                        summary_files_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: theme.THEME_COLOR_TEXT_MUTED
                                text_style: theme.font_regular { font_size: 8.0 }
                            }
                            text: ""
                        }

                        summary_add_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: theme.THEME_COLOR_DIFF_ADD_TEXT
                                text_style: theme.font_regular { font_size: 8.0 }
                            }
                            text: ""
                        }

                        summary_del_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: theme.THEME_COLOR_DIFF_DEL_TEXT
                                text_style: theme.font_regular { font_size: 8.0 }
                            }
                            text: ""
                        }
                    }

                    working_dot := RoundedView {
                        visible: false
                        width: 5, height: 5
                        margin: Inset{ right: 2 }
                        show_bg: true
                        draw_bg +: {
                            color: theme.THEME_COLOR_ACCENT_AMBER
                            border_radius: 2.5
                        }
                    }
                }

                View {
                    width: Fit, height: Fill
                    flow: Right
                    align: Align{ x: 1.0, y: 0.5 }
                    padding: Inset{ right: 3, top: 1, bottom: 1 }
                    menu_button := Button {
                        width: 24, height: 24
                        text: "Session options"
                        align: Align{ x: 0.5, y: 0.5 }
                        draw_text +: { color: #0000 }
                        draw_bg +: {
                            color: theme.THEME_COLOR_TRANSPARENT
                            color_hover: theme.THEME_COLOR_HOVER_SUBTLE
                            color_active: theme.THEME_COLOR_HOVER_SUBTLE
                            border_radius: 5.0
                            border_size: 0.0

                            pixel: fn() {
                                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius)
                                sdf.fill(mix(self.color, self.color_hover, self.hover))

                                let cx = self.rect_size.x * 0.5
                                let cy = self.rect_size.y * 0.5
                                let r = 1.0
                                let gap = 3.5
                                let dot_color = theme.THEME_COLOR_SHADE_8
                                sdf.circle(cx - gap, cy, r)
                                sdf.fill(dot_color)
                                sdf.circle(cx, cy, r)
                                sdf.fill(dot_color)
                                sdf.circle(cx + gap, cy, r)
                                sdf.fill(dot_color)

                                return sdf.result
                            }
                        }
                    }
                }

                View {
                    width: Fill, height: Fit
                    padding: Inset{ top: 1, bottom: 1, right: 3 }
                    align: Align{ x: 1.0, y: 0.5 }

                    menu_panel := RoundedView {
                        visible: false
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 2,
                        padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                        show_bg: true
                        draw_bg +: {
                            color: theme.THEME_COLOR_SHADE_5
                            border_radius: 7.0
                            border_size: 1.0
                            border_color: theme.THEME_COLOR_BORDER_MEDIUM
                        }

                        menu_collapse := Button {
                            width: 22, height: 22
                            text: "〉"
                            align: Align{ x: 0.5, y: 0.5 }
                            draw_bg +: {
                                color: theme.THEME_COLOR_TRANSPARENT
                                color_hover: theme.THEME_COLOR_HOVER_MEDIUM
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: theme.THEME_COLOR_SHADE_8, text_style: theme.font_bold { font_size: 10 } }
                        }

                        menu_rename := Button {
                            width: Fit, height: 22
                            text: "Rename"
                            draw_bg +: {
                                color: theme.THEME_COLOR_TRANSPARENT
                                color_hover: theme.THEME_COLOR_HOVER_MEDIUM
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: theme.THEME_COLOR_SHADE_10, text_style: theme.font_regular { font_size: 9 } }
                        }

                        menu_branch := Button {
                            width: Fit, height: 22
                            text: "Branch"
                            draw_bg +: {
                                color: theme.THEME_COLOR_TRANSPARENT
                                color_hover: theme.THEME_COLOR_HOVER_MEDIUM
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: theme.THEME_COLOR_SHADE_10, text_style: theme.font_regular { font_size: 9 } }
                        }

                        menu_abort := Button {
                            width: Fit, height: 22
                            text: "Abort"
                            visible: false
                            draw_bg +: {
                                color: theme.THEME_COLOR_TRANSPARENT
                                color_hover: theme.THEME_COLOR_ACCENT_RED
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: theme.THEME_COLOR_SHADE_10, text_style: theme.font_regular { font_size: 9 } }
                        }

                        menu_delete := Button {
                            width: Fit, height: 22
                            text: "Delete"
                            draw_bg +: {
                                color: theme.THEME_COLOR_TRANSPARENT
                                color_hover: theme.THEME_COLOR_ACCENT_RED
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: theme.THEME_COLOR_SHADE_10, text_style: theme.font_regular { font_size: 9 } }
                        }
                    }
                }
            }

            Spacer := View { width: Fill, height: 6 }

            EmptyState := View {
                width: Fill, height: Fit
                padding: Inset{ left: 12, right: 12, top: 24, bottom: 24 }
                flow: Down
                spacing: 8

                empty_label := Label {
                    width: Fill, height: Fit
                    text: "No projects yet"
                    draw_text +: {
                        color: theme.THEME_COLOR_TEXT_MUTED
                        text_style: theme.font_regular { font_size: 11 }
                    }
                }

                empty_hint := Label {
                    width: Fill, height: Fit
                    text: "Add a project to get started"
                    draw_text +: {
                        color: theme.THEME_COLOR_TEXT_MUTED
                        text_style: theme.font_regular { font_size: 9 }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum PanelItemKind {
    ProjectHeader {
        project_id: Option<String>,
        display_name: String,
        chevron: &'static str,
        project_working: bool,
    },
    SessionRow {
        session_id: String,
        display_title: String,
        selected: bool,
        working: bool,
        menu_open: bool,
        summary: Option<(String, String, String)>,
    },
    Spacer,
    EmptyState,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ProjectsPanel {
    #[source]
    source: ScriptObjectRef,

    #[deref]
    view: View,
    #[rust]
    projects: Vec<Project>,
    #[rust]
    sessions: Vec<Session>,
    #[rust]
    selected_session_id: Option<String>,
    #[rust]
    working_by_session: HashMap<String, bool>,
    #[rust]
    items: Vec<PanelItemKind>,
    #[rust]
    dirty: bool,
    #[rust]
    collapsed_projects: HashMap<Option<String>, bool>,
    #[rust]
    open_menu_session_id: Option<String>,
    /// When false, only project headers are shown (no sessions). Used for IDE-style left panel.
    #[rust]
    show_sessions: bool,
    #[rust]
    current_dir_name: String,
}

impl ProjectsPanel {
    fn derive_project_name(project: &Project, current_dir_name: &str) -> String {
        if let Some(name) = &project.name {
            if !name.is_empty() {
                return name.clone();
            }
        }
        // For "." worktree, resolve to actual current directory name
        let worktree = if project.worktree == "." {
            current_dir_name
        } else {
            &project.worktree
        };
        // Derive name from last component of worktree path
        std::path::Path::new(worktree)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|n| !n.is_empty())
            .unwrap_or(worktree)
            .to_string()
    }

    fn rebuild_items(&mut self) {
        // Optimization: avoid repeated system calls to get current directory.
        if self.current_dir_name.is_empty() {
            self.current_dir_name = std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| ".".to_string());
        }
        let current_dir_name = &self.current_dir_name;

        let mut grouped: HashMap<Option<String>, Vec<Session>> = HashMap::new();
        for session in &self.sessions {
            grouped
                .entry(Some(session.project_id.clone()))
                .or_default()
                .push(session.clone());
        }

        let mut items = Vec::new();
        for project in &self.projects {
            // Skip the global project (worktree "/") and empty worktrees
            if project.worktree == "/" || project.worktree.is_empty() {
                continue;
            }

            let project_id = Some(project.id.clone());
            let name = Self::derive_project_name(project, current_dir_name);
            let collapsed = self
                .collapsed_projects
                .get(&project_id)
                .copied()
                .unwrap_or(false);

            let display_name = if name.trim().is_empty() {
                "(project)".to_string()
            } else {
                name
            };
            let chevron = if collapsed { ">" } else { "v" };

            // Optimization: Pre-calculate project_working (O(N) search) during rebuild.
            let project_working = self.sessions.iter().any(|s| {
                let matches_project = match &project_id {
                    Some(pid) => &s.project_id == pid,
                    None => true,
                };
                matches_project && self.working_by_session.get(&s.id).copied().unwrap_or(false)
            });

            items.push(PanelItemKind::ProjectHeader {
                project_id: project_id.clone(),
                display_name,
                chevron,
                project_working,
            });

            if !collapsed && self.show_sessions {
                if let Some(sessions) = grouped.get(&project_id) {
                    for session in sessions {
                        let title = async_runtime::get_session_title(session);
                        let title = title.trim();

                        let display_title = if title.is_empty() {
                            "Untitled session".to_string()
                        } else {
                            let truncated: String = title.chars().take(45).collect();
                            if title.chars().count() > 45 {
                                format!("{}…", truncated)
                            } else {
                                truncated
                            }
                        };

                        let selected = self
                            .selected_session_id
                            .as_ref()
                            .map(|id| id == &session.id)
                            .unwrap_or(false);
                        let working = self
                            .working_by_session
                            .get(&session.id)
                            .copied()
                            .unwrap_or(false);
                        let menu_open = self.open_menu_session_id.as_deref() == Some(&session.id);
                        let summary = session.summary.as_ref().and_then(Self::session_diff_stats);

                        items.push(PanelItemKind::SessionRow {
                            session_id: session.id.clone(),
                            display_title,
                            selected,
                            working,
                            menu_open,
                            summary,
                        });
                    }
                }
            }
            items.push(PanelItemKind::Spacer);
        }

        // Collect ungrouped sessions (no matching project)
        let project_ids: std::collections::HashSet<String> =
            self.projects.iter().map(|p| p.id.clone()).collect();
        let ungrouped: Vec<&Session> = self
            .sessions
            .iter()
            .filter(|s| !project_ids.contains(&s.project_id))
            .collect();

        if !ungrouped.is_empty() {
            let collapsed = self.collapsed_projects.get(&None).copied().unwrap_or(false);
            let chevron = if collapsed { ">" } else { "v" };

            // Optimization: Pre-calculate project_working for "Other" project.
            let project_working = ungrouped
                .iter()
                .any(|s| self.working_by_session.get(&s.id).copied().unwrap_or(false));

            items.push(PanelItemKind::ProjectHeader {
                project_id: None,
                display_name: "Other".to_string(),
                chevron,
                project_working,
            });

            if !collapsed && self.show_sessions {
                for session in ungrouped {
                    let title = async_runtime::get_session_title(session);
                    let title = title.trim();

                    let display_title = if title.is_empty() {
                        "Untitled session".to_string()
                    } else {
                        let truncated: String = title.chars().take(45).collect();
                        if title.chars().count() > 45 {
                            format!("{}…", truncated)
                        } else {
                            truncated
                        }
                    };

                    let selected = self
                        .selected_session_id
                        .as_ref()
                        .map(|id| id == &session.id)
                        .unwrap_or(false);
                    let working = self
                        .working_by_session
                        .get(&session.id)
                        .copied()
                        .unwrap_or(false);
                    let menu_open = self.open_menu_session_id.as_deref() == Some(&session.id);
                    let summary = session.summary.as_ref().and_then(Self::session_diff_stats);

                    items.push(PanelItemKind::SessionRow {
                        session_id: session.id.clone(),
                        display_title,
                        selected,
                        working,
                        menu_open,
                        summary,
                    });
                }
            }
        }

        if items.is_empty() {
            items.push(PanelItemKind::EmptyState);
        }

        self.items = items;
        self.dirty = false;
    }

    fn session_diff_stats(summary: &SessionSummary) -> Option<(String, String, String)> {
        let (files, additions, deletions) = if !summary.diffs.is_empty() {
            let additions: i64 = summary.diffs.iter().map(|d| d.additions).sum();
            let deletions: i64 = summary.diffs.iter().map(|d| d.deletions).sum();
            (summary.diffs.len() as i64, additions, deletions)
        } else {
            (summary.files, summary.additions, summary.deletions)
        };

        if files <= 0 && additions == 0 && deletions == 0 {
            return None;
        }

        let file_label = if files == 1 { "file" } else { "files" };
        Some((
            format!("{} {}", files, file_label),
            format!("+{}", additions),
            format!("-{}", deletions),
        ))
    }
}

impl Widget for ProjectsPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        let list = self.view.portal_list(cx, &[id!(list)]);
        let mut menu_opened = false;
        for (item_id, widget) in list.items_with_actions(&actions) {
            if item_id >= self.items.len() {
                continue;
            }
            let panel_item = self.items[item_id].clone();
            match panel_item {
                PanelItemKind::ProjectHeader { project_id, .. } => {
                    if widget
                        .button(cx, &[id!(new_session_header_button)])
                        .clicked(&actions)
                    {
                        cx.action(ProjectsPanelAction::CreateSession(project_id.clone()));
                    } else if widget.button(cx, &[id!(project_toggle)]).clicked(&actions) {
                        let collapsed = self
                            .collapsed_projects
                            .get(&project_id)
                            .copied()
                            .unwrap_or(false);
                        self.collapsed_projects
                            .insert(project_id.clone(), !collapsed);
                        self.dirty = true;
                        self.redraw(cx);
                    }
                }
                PanelItemKind::SessionRow { session_id, .. } => {
                    if widget.button(cx, &[id!(session_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::SelectSession(session_id.clone()));
                    }

                    if widget.button(cx, &[id!(menu_button)]).clicked(&actions) {
                        menu_opened = true;
                        let (x, y) = (0.0, 0.0);
                        let working = self
                            .working_by_session
                            .get(&session_id)
                            .copied()
                            .unwrap_or(false);
                        cx.action(ProjectsPanelAction::OpenSessionContextMenu {
                            session_id: session_id.clone(),
                            x,
                            y,
                            working,
                        });
                    }

                    if widget.button(cx, &[id!(menu_delete)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::DeleteSession(session_id.clone()));
                        self.open_menu_session_id = None;
                        self.redraw(cx);
                    }
                    if widget.button(cx, &[id!(menu_rename)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::RenameSession(session_id.clone()));
                        self.open_menu_session_id = None;
                        self.redraw(cx);
                    }
                    if widget.button(cx, &[id!(menu_branch)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::BranchSession(session_id.clone()));
                        self.open_menu_session_id = None;
                        self.redraw(cx);
                    }
                    if widget.button(cx, &[id!(menu_abort)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::AbortSession(session_id.clone()));
                        self.open_menu_session_id = None;
                        self.redraw(cx);
                    }
                }
                _ => {}
            }
        }

        // Fallback: items_with_actions may omit items when the menu button's action
        // isn't associated with the list item. Check each SessionRow for menu button click.
        if !menu_opened {
            for (item_id, panel_item) in self.items.iter().enumerate() {
                if let PanelItemKind::SessionRow { session_id, .. } = panel_item {
                    let widget = list.item(cx, item_id, live_id!(SessionRow));
                    if widget.button(cx, &[id!(menu_button)]).clicked(&actions) {
                        let (x, y) = (0.0, 0.0);
                        let working = self
                            .working_by_session
                            .get(session_id)
                            .copied()
                            .unwrap_or(false);
                        cx.action(ProjectsPanelAction::OpenSessionContextMenu {
                            session_id: session_id.clone(),
                            x,
                            y,
                            working,
                        });
                        break;
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.dirty {
            self.rebuild_items();
        }

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if self.items.is_empty() {
                    list.set_item_range(cx, 0, 0);
                    continue;
                } else {
                    list.set_item_range(cx, 0, self.items.len());
                }
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id >= self.items.len() {
                        continue;
                    }

                    // Optimization: Use reference to avoid cloning the enum and its contained strings.
                    let panel_item = &self.items[item_id];
                    let template = match panel_item {
                        PanelItemKind::ProjectHeader { .. } => live_id!(ProjectHeader),
                        PanelItemKind::SessionRow { .. } => live_id!(SessionRow),
                        PanelItemKind::Spacer => live_id!(Spacer),
                        PanelItemKind::EmptyState => live_id!(EmptyState),
                    };
                    let item_widget = list.item(cx, item_id, template);

                    match panel_item {
                        PanelItemKind::ProjectHeader {
                            display_name,
                            chevron,
                            project_working,
                            ..
                        } => {
                            item_widget
                                .button(cx, &[id!(project_toggle)])
                                .set_text(cx, display_name);
                            item_widget.label(cx, &[id!(chevron)]).set_text(cx, chevron);
                            item_widget
                                .view(cx, &[id!(project_working_dot)])
                                .set_visible(cx, *project_working);
                        }
                        PanelItemKind::SessionRow {
                            session_id: _,
                            display_title,
                            selected,
                            working,
                            menu_open,
                            summary,
                        } => {
                            item_widget
                                .button(cx, &[id!(session_button)])
                                .set_text(cx, display_title);
                            item_widget
                                .view(cx, &[id!(selected_pill)])
                                .set_visible(cx, *selected);
                            item_widget
                                .view(cx, &[id!(working_dot)])
                                .set_visible(cx, *working);
                            item_widget
                                .view(cx, &[id!(menu_panel)])
                                .set_visible(cx, *menu_open);
                            item_widget
                                .button(cx, &[id!(menu_button)])
                                .set_visible(cx, !*menu_open);
                            item_widget
                                .button(cx, &[id!(menu_abort)])
                                .set_visible(cx, *working);

                            let summary_files = item_widget.label(cx, &[id!(summary_files_label)]);
                            let summary_add = item_widget.label(cx, &[id!(summary_add_label)]);
                            let summary_del = item_widget.label(cx, &[id!(summary_del_label)]);
                            if let Some((files, adds, dels)) = summary {
                                summary_files.set_text(cx, files);
                                summary_add.set_text(cx, adds);
                                summary_del.set_text(cx, dels);
                                item_widget
                                    .view(cx, &[id!(summary_stats)])
                                    .set_visible(cx, true);
                            } else {
                                summary_files.set_text(cx, "");
                                summary_add.set_text(cx, "");
                                summary_del.set_text(cx, "");
                                item_widget
                                    .view(cx, &[id!(summary_stats)])
                                    .set_visible(cx, false);
                            }
                        }
                        _ => {}
                    }

                    item_widget.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
}

impl ProjectsPanelRef {
    pub fn set_data(
        &self,
        cx: &mut Cx,
        projects: Vec<Project>,
        sessions: Vec<Session>,
        selected_session_id: Option<String>,
        working_by_session: HashMap<String, bool>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.projects = projects;
            inner.sessions = sessions;
            inner.selected_session_id = selected_session_id;
            inner.working_by_session = working_by_session;
            inner.dirty = true;
            inner.redraw(cx);
        }
    }

    /// When false, only project headers are shown (no sessions). For IDE-style left panel.
    pub fn set_show_sessions(&self, cx: &mut Cx, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            if inner.show_sessions != show {
                inner.show_sessions = show;
                inner.dirty = true;
                inner.redraw(cx);
            }
        }
    }
}
