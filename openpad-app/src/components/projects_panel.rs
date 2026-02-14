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
        list := PortalList {
            scroll_bar: ScrollBar {
                bar_size: 4.0
                bar_side_margin: 6.0
                smoothing: 0.15
            }

            ProjectHeader := View {
                width: Fill, height: Fit
                flow: Right, align: Align{y: 0.5}
                padding: Inset{ top: 8, bottom: 2, left: 10, right: 6 }
                chevron := View {
                    width: 16, height: 16
                }

                project_toggle := Button {
                    width: Fill, height: 22
                    margin: Inset{ left: 0, right: 4 }
                    text: "> Project"
                    draw_bg +: {
                        color: THEME_COLOR_TRANSPARENT
                        color_hover: THEME_COLOR_HOVER_SUBTLE
                        color_active: THEME_COLOR_HOVER_SUBTLE
                        border_radius: 4.0
                        border_size: 0.0
                    }
                    draw_text +: {
                        color: THEME_COLOR_TEXT_LIGHT
                        text_style: theme.font_bold { font_size: 11 }
                    }
                }

                project_working_dot := RoundedView {
                    visible: false
                    width: 6, height: 6
                    margin: Inset{ right: 4 }
                    show_bg: true
                    draw_bg +: {
                        color: THEME_COLOR_ACCENT_AMBER
                        border_radius: 3.0
                    }
                }

                new_session_header_button := Button {
                    width: Fit, height: 20
                    margin: Inset{ left: 4 }
                    text: "+"
                    draw_bg +: {
                        color: THEME_COLOR_TRANSPARENT
                        color_hover: THEME_COLOR_HOVER_SUBTLE
                        color_active: THEME_COLOR_HOVER_SUBTLE
                        border_radius: 4.0
                        border_size: 0.0
                    }
                    draw_text +: { color: THEME_COLOR_TEXT_MUTED, text_style: theme.font_bold { font_size: 12 } }
                }
            }

            SessionRow := View {
                width: Fill, height: Fit
                flow: Overlay

                // Main row content
                View {
                    width: Fill, height: Fit
                    padding: Inset{ top: 0, bottom: 0, left: 22, right: 8 }
                    flow: Right,
                    spacing: 2,
                    align: Align{ y: 0.5 }

                    session_button := Button {
                        width: Fill, height: 22
                        margin: Inset{ right: 4 }
                        text: "Session"
                        draw_bg +: {
                            color: THEME_COLOR_TRANSPARENT
                            color_hover: THEME_COLOR_HOVER_SUBTLE
                            color_active: THEME_COLOR_HOVER_SUBTLE
                            border_radius: 4.0
                            border_size: 0.0
                        }
                        draw_text +: {
                            color: THEME_COLOR_TEXT_NORMAL,
                            text_style: theme.font_regular { font_size: 9 }}
                    }

                    summary_stats := View {
                        width: Fit, height: Fit
                        margin: Inset{ right: 4 }
                        flow: Right
                        spacing: 6
                        align: Align{ y: 0.5 }

                        summary_files_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: THEME_COLOR_TEXT_MUTED_DARK
                                text_style: theme.font_regular { font_size: 7.5 }
                            }
                            text: ""
                        }

                        summary_add_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: THEME_COLOR_DIFF_ADD_TEXT
                                text_style: theme.font_regular { font_size: 7.5 }
                            }
                            text: ""
                        }

                        summary_del_label := Label {
                            width: Fit, height: Fit
                            draw_text +: {
                                color: THEME_COLOR_DIFF_DEL_TEXT
                                text_style: theme.font_regular { font_size: 7.5 }
                            }
                            text: ""
                        }
                    }

                    working_dot := RoundedView {
                        visible: false
                        width: 6, height: 6
                        show_bg: true
                        draw_bg +: {
                            color: THEME_COLOR_ACCENT_AMBER
                            border_radius: 3.0
                        }
                    }
                    menu_button := Button {
                        width: 24, height: 22
                        text: "⋯"
                        align: Align{ x: 0.5, y: 0.5 }
                        draw_bg +: {
                            color: THEME_COLOR_TRANSPARENT
                            color_hover: THEME_COLOR_HOVER_SUBTLE
                            color_active: THEME_COLOR_HOVER_SUBTLE
                            border_radius: 4.0
                            border_size: 0.0
                        }
                        draw_text +: {
                            color: THEME_COLOR_TEXT_MUTED
                            color_hover: THEME_COLOR_TEXT_MUTED_LIGHTER
                            text_style: theme.font_bold { font_size: 10 }
                        }
                    }
                }

                // Floating menu panel - overlays on top, right-aligned
                View {
                    width: Fill, height: Fit
                    padding: Inset{ top: 1, bottom: 1, right: 4 }
                    align: Align{ x: 1.0, y: 0.5 }

                    menu_panel := RoundedView {
                        visible: false
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 2,
                        padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
                        show_bg: true
                        draw_bg +: {
                            color: #2a2a2a
                            border_radius: 6.0
                            border_size: 1.0
                            border_color: #444
                        }

                        menu_collapse := Button {
                            width: 22, height: 22
                            text: "〉"
                            align: Align{ x: 0.5, y: 0.5 }
                            draw_bg +: {
                                color: THEME_COLOR_TRANSPARENT
                                color_hover: THEME_COLOR_HOVER_MEDIUM
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: THEME_COLOR_TEXT_MUTED_LIGHT, text_style: theme.font_bold { font_size: 10 } }
                        }

                        menu_rename := Button {
                            width: Fit, height: 22
                            text: "Rename"
                            draw_bg +: {
                                color: THEME_COLOR_TRANSPARENT
                                color_hover: THEME_COLOR_HOVER_MEDIUM
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: THEME_COLOR_TEXT_NORMAL, text_style: theme.font_regular { font_size: 9 } }
                        }

                        menu_branch := Button {
                            width: Fit, height: 22
                            text: "Branch"
                            draw_bg +: {
                                color: THEME_COLOR_TRANSPARENT
                                color_hover: THEME_COLOR_HOVER_MEDIUM
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: THEME_COLOR_TEXT_NORMAL, text_style: theme.font_regular { font_size: 9 } }
                        }

                        menu_abort := Button {
                            width: Fit, height: 22
                            text: "Abort"
                            visible: false
                            draw_bg +: {
                                color: THEME_COLOR_TRANSPARENT
                                color_hover: THEME_COLOR_ACCENT_RED
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: THEME_COLOR_TEXT_NORMAL, text_style: theme.font_regular { font_size: 9 } }
                        }

                        menu_delete := Button {
                            width: Fit, height: 22
                            text: "Delete"
                            draw_bg +: {
                                color: THEME_COLOR_TRANSPARENT
                                color_hover: THEME_COLOR_ACCENT_RED
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text +: { color: THEME_COLOR_TEXT_NORMAL, text_style: theme.font_regular { font_size: 9 } }
                        }
                    }
                }
            }

            Spacer := View { width: Fill, height: 6 }
        }
    }
}

#[derive(Clone, Debug)]
pub enum PanelItemKind {
    ProjectHeader {
        project_id: Option<String>,
        name: String,
    },
    SessionRow {
        session_id: String,
        title: String,
    },
    Spacer,
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
}

impl ProjectsPanel {
    fn derive_project_name(project: &Project) -> String {
        if let Some(name) = &project.name {
            if !name.is_empty() {
                return name.clone();
            }
        }
        // For "." worktree, resolve to actual current directory name
        let worktree = if project.worktree == "." {
            std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| project.worktree.clone())
        } else {
            project.worktree.clone()
        };
        // Derive name from last component of worktree path
        std::path::Path::new(&worktree)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|n| !n.is_empty())
            .unwrap_or(&worktree)
            .to_string()
    }

    fn rebuild_items(&mut self) {
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
            let name = Self::derive_project_name(project);
            let collapsed = self
                .collapsed_projects
                .get(&project_id)
                .copied()
                .unwrap_or(false);

            items.push(PanelItemKind::ProjectHeader {
                project_id: project_id.clone(),
                name,
            });

            if !collapsed {
                if let Some(sessions) = grouped.get(&project_id) {
                    for session in sessions {
                        let title = async_runtime::get_session_title(session);
                        items.push(PanelItemKind::SessionRow {
                            session_id: session.id.clone(),
                            title,
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

            items.push(PanelItemKind::ProjectHeader {
                project_id: None,
                name: "Other".to_string(),
            });

            if !collapsed {
                for session in ungrouped {
                    let title = async_runtime::get_session_title(session);
                    items.push(PanelItemKind::SessionRow {
                        session_id: session.id.clone(),
                        title,
                    });
                }
            }
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

                    if widget.button(cx, &[id!(menu_button)]).clicked(&actions)
                        || widget.button(cx, &[id!(menu_collapse)]).clicked(&actions)
                    {
                        let next = if self.open_menu_session_id.as_deref() == Some(&session_id) {
                            None
                        } else {
                            Some(session_id.clone())
                        };
                        self.open_menu_session_id = next;
                        self.redraw(cx);
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
                    let panel_item = self.items[item_id].clone();
                    let template = match panel_item {
                        PanelItemKind::ProjectHeader { .. } => live_id!(ProjectHeader),
                        PanelItemKind::SessionRow { .. } => live_id!(SessionRow),
                        PanelItemKind::Spacer => live_id!(Spacer),
                    };
                    let item_widget = list.item(cx, item_id, template);

                    match &panel_item {
                        PanelItemKind::ProjectHeader {
                            name, project_id, ..
                        } => {
                            let collapsed = self
                                .collapsed_projects
                                .get(project_id)
                                .copied()
                                .unwrap_or(false);
                            let chevron = if collapsed { ">" } else { "v" };
                            let display_name = if name.trim().is_empty() {
                                "(project)"
                            } else {
                                name.as_str()
                            };
                            item_widget
                                .button(cx, &[id!(project_toggle)])
                                .set_text(cx, &format!("{chevron} {display_name}"));
                            // Show orange dot if any session in this project is working
                            let project_working = self.sessions.iter().any(|s| {
                                let matches_project = match project_id {
                                    Some(pid) => &s.project_id == pid,
                                    None => true,
                                };
                                matches_project
                                    && self.working_by_session.get(&s.id).copied().unwrap_or(false)
                            });
                            item_widget
                                .view(cx, &[id!(project_working_dot)])
                                .set_visible(cx, project_working);
                        }
                        PanelItemKind::SessionRow { session_id, title } => {
                            let display_title = if title.trim().is_empty() {
                                "Untitled session".to_string()
                            } else {
                                title.clone()
                            };
                            item_widget
                                .button(cx, &[id!(session_button)])
                                .set_text(cx, &display_title);
                            let selected = self
                                .selected_session_id
                                .as_ref()
                                .map(|id| id == session_id)
                                .unwrap_or(false);
                            let display_title = if selected {
                                format!("● {}", display_title)
                            } else {
                                display_title
                            };
                            item_widget
                                .button(cx, &[id!(session_button)])
                                .set_text(cx, &display_title);
                            let working = self
                                .working_by_session
                                .get(session_id)
                                .copied()
                                .unwrap_or(false);
                            item_widget
                                .view(cx, &[id!(working_dot)])
                                .set_visible(cx, working);
                            let menu_open =
                                self.open_menu_session_id.as_deref() == Some(session_id);
                            item_widget
                                .view(cx, &[id!(menu_panel)])
                                .set_visible(cx, menu_open);
                            item_widget
                                .button(cx, &[id!(menu_button)])
                                .set_visible(cx, !menu_open);
                            item_widget
                                .button(cx, &[id!(menu_abort)])
                                .set_visible(cx, working);

                            let summary_text = self
                                .sessions
                                .iter()
                                .find(|s| &s.id == session_id)
                                .and_then(|s| s.summary.as_ref())
                                .and_then(Self::session_diff_stats);
                            let summary_files = item_widget.label(cx, &[id!(summary_files_label)]);
                            let summary_add = item_widget.label(cx, &[id!(summary_add_label)]);
                            let summary_del = item_widget.label(cx, &[id!(summary_del_label)]);
                            if let Some((files, adds, dels)) = summary_text {
                                summary_files.set_text(cx, &files);
                                summary_add.set_text(cx, &adds);
                                summary_del.set_text(cx, &dels);
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
}
