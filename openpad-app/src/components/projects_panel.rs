use crate::async_runtime;
use crate::constants::{COLOR_SESSION_NORMAL, COLOR_SESSION_SELECTED};
use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::*;
use openpad_protocol::{Project, Session};
use std::collections::HashMap;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::openpad::*;

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
                padding: { top: 12, bottom: 4, left: 16 }
                project_name = <Label> {
                    draw_text: { color: #888, text_style: <THEME_FONT_BOLD> { font_size: 10 } }
                }
            }

            NewSessionRow = <View> {
                width: Fill, height: Fit
                padding: { top: 4, bottom: 8, left: 16 }
                new_session_button = <Button> {
                    width: Fit, height: 24
                    text: "+ New session"
                    draw_bg: {
                        color: #0000
                        color_hover: #333
                        border_radius: 4.0
                        border_size: 0.0
                    }
                    draw_text: { color: #666, text_style: <THEME_FONT_REGULAR> { font_size: 9 } }
                }
            }

            SessionRow = <View> {
                width: Fill, height: Fit
                padding: { top: 1, bottom: 1, left: 16 }
                flow: Right,
                spacing: 4,
                align: { y: 0.5 }

                session_button = <Button> {
                    width: Fill, height: 24
                    margin: { right: 8 }
                    text: "Session"
                    draw_bg: {
                        color: #0000
                        color_hover: #2d2d2d
                        border_radius: 4.0
                        border_size: 0.0
                    }
                    draw_text: { color: #ccc, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                }

                working_dot = <View> {
                    visible: false
                    width: 6, height: 6
                    show_bg: true
                    draw_bg: {
                        color: #f59e0b
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

                // Action buttons container
                action_buttons = <View> {
                    width: Fit, height: Fit
                    flow: Right,
                    spacing: 4,
                    margin: { right: 4 },
                    align: { y: 0.5 }

                    abort_button = <Button> {
                        width: 28, height: 28
                        visible: false
                        text: ""
                        icon_walk: { width: 12, height: 12 }
                        label_walk: { width: 0, height: 0 }
                        align: { x: 0.5, y: 0.5 }
                        draw_icon: {
                            svg_file: dep("crate://self/resources/icons/stop.svg")
                            color: #444
                            color_hover: #666
                            color_down: #666
                        }
                        draw_bg: {
                            color: #0000
                            color_hover: #ef4444
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: { color: #0000 }
                    }

                    rename_button = <Button> {
                        width: 28, height: 28
                        text: ""
                        icon_walk: { width: 12, height: 12 }
                        label_walk: { width: 0, height: 0 }
                        align: { x: 0.5, y: 0.5 }
                        draw_icon: {
                            svg_file: dep("crate://self/resources/icons/pencil.svg")
                            color: #444
                            color_hover: #666
                            color_down: #666
                        }
                        draw_bg: {
                            color: #0000
                            color_hover: #3b82f6
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: { color: #0000 }
                    }

                    branch_button = <Button> {
                        width: 28, height: 28
                        text: ""
                        icon_walk: { width: 12, height: 12 }
                        label_walk: { width: 0, height: 0 }
                        align: { x: 0.5, y: 0.5 }
                        draw_icon: {
                            svg_file: dep("crate://self/resources/icons/branch.svg")
                            color: #444
                            color_hover: #666
                            color_down: #666
                        }
                        draw_bg: {
                            color: #0000
                            color_hover: #8b5cf6
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: { color: #0000 }
                    }

                    delete_button = <Button> {
                        width: 28, height: 28
                        text: ""
                        icon_walk: { width: 12, height: 12 }
                        label_walk: { width: 0, height: 0 }
                        align: { x: 0.5, y: 0.5 }
                        draw_icon: {
                            svg_file: dep("crate://self/resources/icons/trash.svg")
                            color: #444
                            color_hover: #666
                            color_down: #666
                        }
                        draw_bg: {
                            color: #0000
                            color_hover: #ef4444
                            border_radius: 6.0
                            border_size: 0.0
                        }
                        draw_text: { color: #0000 }
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
    NewSession {
        project_id: Option<String>,
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

            items.push(PanelItemKind::ProjectHeader {
                project_id: project_id.clone(),
                name,
            });

            if let Some(sessions) = grouped.get(&project_id) {
                for session in sessions {
                    let title = async_runtime::get_session_title(session);
                    items.push(PanelItemKind::SessionRow {
                        session_id: session.id.clone(),
                        title,
                    });
                }
            }

            items.push(PanelItemKind::NewSession {
                project_id: project_id.clone(),
            });
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
            items.push(PanelItemKind::ProjectHeader {
                project_id: None,
                name: "Other".to_string(),
            });
            for session in ungrouped {
                let title = async_runtime::get_session_title(session);
                items.push(PanelItemKind::SessionRow {
                    session_id: session.id.clone(),
                    title,
                });
            }
            items.push(PanelItemKind::NewSession { project_id: None });
        }

        self.items = items;
        self.dirty = false;
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
            match &self.items[item_id] {
                PanelItemKind::NewSession { project_id } => {
                    if widget.button(&[id!(new_session_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::CreateSession(project_id.clone()));
                    }
                }
                PanelItemKind::SessionRow { session_id, .. } => {
                    if widget.button(&[id!(session_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::SelectSession(session_id.clone()));
                    }

                    if widget.button(&[id!(delete_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::DeleteSession(session_id.clone()));
                    }
                    if widget.button(&[id!(rename_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::RenameSession(session_id.clone()));
                    }
                    if widget.button(&[id!(branch_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::BranchSession(session_id.clone()));
                    }
                    if widget.button(&[id!(abort_button)]).clicked(&actions) {
                        cx.action(ProjectsPanelAction::AbortSession(session_id.clone()));
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
                        PanelItemKind::NewSession { .. } => live_id!(NewSessionRow),
                        PanelItemKind::SessionRow { .. } => live_id!(SessionRow),
                        PanelItemKind::Spacer => live_id!(Spacer),
                    };
                    let item_widget = list.item(cx, item_id, template);

                    match &panel_item {
                        PanelItemKind::ProjectHeader { name, .. } => {
                            item_widget.label(&[id!(project_name)]).set_text(cx, name);
                        }
                        PanelItemKind::SessionRow { session_id, title } => {
                            item_widget.button(&[id!(session_button)]).set_text(cx, title);
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
