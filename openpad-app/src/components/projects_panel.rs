use crate::async_runtime;
use crate::constants::{COLOR_SESSION_NORMAL, COLOR_SESSION_SELECTED};
use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::*;
use openpad_protocol::{Project, Session, SessionSummary};
use std::collections::HashMap;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;
    use openpad_widgets::theme::*;

pub ProjectsPanel = {{ProjectsPanel}} {
        width: Fill, height: Fill
        list = <PortalList> {
            scroll_bar: <ScrollBar> {
                bar_size: 4.0
                bar_side_margin: 6.0
                smoothing: 0.15
            }

            ProjectHeader = <View> {
                width: Fill, height: Fit
                flow: Right, align: {y: 0.5}
                padding: { top: 12, bottom: 4, left: 8 }
                cursor: Hand

                chevron = <View> {
                    width: 16, height: 16
                    align: { x: 0.5, y: 0.5 }
                    show_bg: true
                    draw_bg: {
                        instance rotation: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let cx = self.rect_size.x * 0.5;
                            let cy = self.rect_size.y * 0.5;
                            let sz = 3.0;

                            // Draw a triangle chevron
                            sdf.rotate(self.rotation, cx, cy);
                            sdf.move_to(cx - sz, cy - sz * 0.6);
                            sdf.line_to(cx + sz, cy - sz * 0.6);
                            sdf.line_to(cx, cy + sz * 0.6);
                            sdf.close_path();
                            sdf.fill(#888);
                            return sdf.result;
                        }
                    }
                }

                project_name = <Label> {
                    width: Fill
                    margin: { left: 4 }
                    draw_text: { color: (THEME_COLOR_TEXT_MUTED), text_style: <THEME_FONT_BOLD> { font_size: 10 } }
                }

                project_working_dot = <View> {
                    visible: false
                    width: 6, height: 6
                    margin: { right: 4 }
                    show_bg: true
                    draw_bg: {
                        color: (THEME_COLOR_ACCENT_AMBER)
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let c = self.rect_size * 0.5;
                            let r = min(c.x, c.y) - 0.5;
                            sdf.circle(c.x, c.y, r);
                            sdf.fill(self.color);
                            return sdf.result;
                        }
                    }
                }

                new_session_header_button = <Button> {
                    width: Fit, height: 20
                    margin: { right: 4 }
                    text: "+"
                    draw_bg: {
                        color: (THEME_COLOR_TRANSPARENT)
                        color_hover: (THEME_COLOR_HOVER_MEDIUM)
                        border_radius: 4.0
                        border_size: 0.0
                    }
                    draw_text: { color: (THEME_COLOR_TEXT_MUTED_LIGHT), text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
                }
            }

            SessionRow = <View> {
                width: Fill, height: Fit
                flow: Overlay

                // Main row content
                <View> {
                    width: Fill, height: Fit
                    padding: { top: 1, bottom: 1, left: 16 }
                    flow: Right,
                    spacing: 2,
                    align: { y: 0.5 }

                    session_button = <Button> {
                        width: Fill, height: 24
                        margin: { right: 8 }
                        text: "Session"
                        draw_bg: {
                            color: (THEME_COLOR_TRANSPARENT)
                            color_hover: (THEME_COLOR_HOVER_SUBTLE)
                            border_radius: 4.0
                            border_size: 0.0
                        }
                        draw_text: { color: (THEME_COLOR_TEXT_NORMAL), text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                    }

                    summary_stats = <View> {
                        width: Fit, height: Fit
                        margin: { right: 6 }
                        flow: Right
                        spacing: 6
                        align: { y: 0.5 }

                        summary_files_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_TEXT_MUTED_DARK)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }

                        summary_add_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_DIFF_ADD_TEXT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }

                        summary_del_label = <Label> {
                            width: Fit, height: Fit
                            draw_text: {
                                color: (THEME_COLOR_DIFF_DEL_TEXT)
                                text_style: <THEME_FONT_REGULAR> { font_size: 8 }
                            }
                            text: ""
                        }
                    }

                    working_dot = <View> {
                        visible: false
                        width: 6, height: 6
                        show_bg: true
                        draw_bg: {
                            color: (THEME_COLOR_ACCENT_AMBER)
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let c = self.rect_size * 0.5;
                                let r = min(c.x, c.y) - 0.5;
                                sdf.circle(c.x, c.y, r);
                                sdf.fill(self.color);
                                return sdf.result;
                            }
                        }
                    }
                    menu_button = <Button> {
                        width: 28, height: 28
                        text: "⋯"
                        align: { x: 0.5, y: 0.5 }
                        draw_bg: {
                            color: (THEME_COLOR_TRANSPARENT)
                            color_hover: (THEME_COLOR_HOVER_MEDIUM)
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: {
                            color: (THEME_COLOR_TEXT_MUTED_LIGHT)
                            color_hover: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                            text_style: <THEME_FONT_BOLD> { font_size: 12 }
                        }
                    }
                }

                // Floating menu panel - overlays on top, right-aligned
                <View> {
                    width: Fill, height: Fit
                    padding: { top: 1, bottom: 1, right: 4 }
                    align: { x: 1.0, y: 0.5 }

                    menu_panel = <RoundedView> {
                        visible: false
                        width: Fit, height: Fit
                        flow: Right,
                        spacing: 2,
                        padding: { left: 6, right: 6, top: 2, bottom: 2 }
                        show_bg: true
                        draw_bg: {
                            color: #2a2a2a
                            border_radius: 6.0
                            border_size: 1.0
                            border_color: #444
                        }

                        menu_rename = <Button> {
                            width: Fit, height: 22
                            text: "Rename"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: { color: (THEME_COLOR_TEXT_NORMAL), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                        }

                        menu_branch = <Button> {
                            width: Fit, height: 22
                            text: "Branch"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_HOVER_MEDIUM)
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: { color: (THEME_COLOR_TEXT_NORMAL), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                        }

                        menu_abort = <Button> {
                            width: Fit, height: 22
                            text: "Abort"
                            visible: false
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_ACCENT_RED)
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: { color: (THEME_COLOR_TEXT_NORMAL), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                        }

                        menu_delete = <Button> {
                            width: Fit, height: 22
                            text: "Delete"
                            draw_bg: {
                                color: (THEME_COLOR_TRANSPARENT)
                                color_hover: (THEME_COLOR_ACCENT_RED)
                                border_radius: 4.0
                                border_size: 0.0
                            }
                            draw_text: { color: (THEME_COLOR_TEXT_NORMAL), text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                        }
                    }
                }
            }

            Spacer = <View> { width: Fill, height: 12 }
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

#[derive(Live, LiveHook, Widget)]
pub struct ProjectsPanel {
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
                .unwrap_or(true);

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
            let collapsed = self.collapsed_projects.get(&None).copied().unwrap_or(true);

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

        let list = self.view.portal_list(&[id!(list)]);
        for (item_id, widget) in list.items_with_actions(&actions) {
            if item_id >= self.items.len() {
                continue;
            }
            let panel_item = self.items[item_id].clone();
            match panel_item {
                PanelItemKind::ProjectHeader { project_id, .. } => {
                    if widget
                        .button(&[id!(new_session_header_button)])
                        .clicked(&actions)
                    {
                        cx.action(ProjectsPanelAction::CreateSession(project_id.clone()));
                    } else if widget.as_view().finger_up(&actions).is_some() {
                        let collapsed = self
                            .collapsed_projects
                            .get(&project_id)
                            .copied()
                            .unwrap_or(true);
                        self.collapsed_projects
                            .insert(project_id.clone(), !collapsed);
                        self.dirty = true;
                        self.redraw(cx);
                    }
                }
                PanelItemKind::SessionRow { session_id, .. } => {
                    if widget.button(&[id!(session_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::SelectSession(session_id.clone()));
                    }

                    if widget.button(&[id!(menu_button)]).clicked(&actions) {
                        let next = if self.open_menu_session_id.as_deref() == Some(&session_id) {
                            None
                        } else {
                            Some(session_id.clone())
                        };
                        self.open_menu_session_id = next;
                        self.redraw(cx);
                    }

                    if widget.button(&[id!(menu_delete)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::DeleteSession(session_id.clone()));
                        self.open_menu_session_id = None;
                        self.redraw(cx);
                    }
                    if widget.button(&[id!(menu_rename)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::RenameSession(session_id.clone()));
                        self.open_menu_session_id = None;
                        self.redraw(cx);
                    }
                    if widget.button(&[id!(menu_branch)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::BranchSession(session_id.clone()));
                        self.open_menu_session_id = None;
                        self.redraw(cx);
                    }
                    if widget.button(&[id!(menu_abort)]).clicked(&actions) {
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
                            item_widget.label(&[id!(project_name)]).set_text(cx, name);
                            let collapsed = self
                                .collapsed_projects
                                .get(project_id)
                                .copied()
                                .unwrap_or(true);
                            let rotation = if collapsed {
                                -std::f32::consts::FRAC_PI_2
                            } else {
                                0.0
                            };
                            item_widget.view(&[id!(chevron)]).apply_over(
                                cx,
                                live! {
                                    draw_bg: { rotation: (rotation) }
                                },
                            );
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
                                .view(&[id!(project_working_dot)])
                                .set_visible(cx, project_working);
                        }
                        PanelItemKind::SessionRow { session_id, title } => {
                            item_widget
                                .button(&[id!(session_button)])
                                .set_text(cx, title);
                            let selected = self
                                .selected_session_id
                                .as_ref()
                                .map(|id| id == session_id)
                                .unwrap_or(false);
                            let color = if selected {
                                COLOR_SESSION_SELECTED
                            } else {
                                COLOR_SESSION_NORMAL
                            };
                            item_widget.button(&[id!(session_button)]).apply_over(
                                cx,
                                live! {
                                    draw_bg: { color: (color) }
                                },
                            );
                            let working = self
                                .working_by_session
                                .get(session_id)
                                .copied()
                                .unwrap_or(false);
                            item_widget
                                .view(&[id!(working_dot)])
                                .set_visible(cx, working);
                            let menu_open =
                                self.open_menu_session_id.as_deref() == Some(session_id);
                            item_widget
                                .view(&[id!(menu_panel)])
                                .set_visible(cx, menu_open);
                            item_widget
                                .button(&[id!(menu_abort)])
                                .set_visible(cx, working);

                            let summary_text = self
                                .sessions
                                .iter()
                                .find(|s| &s.id == session_id)
                                .and_then(|s| s.summary.as_ref())
                                .and_then(Self::session_diff_stats);
                            let summary_files = item_widget.label(&[id!(summary_files_label)]);
                            let summary_add = item_widget.label(&[id!(summary_add_label)]);
                            let summary_del = item_widget.label(&[id!(summary_del_label)]);
                            if let Some((files, adds, dels)) = summary_text {
                                summary_files.set_text(cx, &files);
                                summary_add.set_text(cx, &adds);
                                summary_del.set_text(cx, &dels);
                                item_widget
                                    .view(&[id!(summary_stats)])
                                    .set_visible(cx, true);
                            } else {
                                summary_files.set_text(cx, "");
                                summary_add.set_text(cx, "");
                                summary_del.set_text(cx, "");
                                item_widget
                                    .view(&[id!(summary_stats)])
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
