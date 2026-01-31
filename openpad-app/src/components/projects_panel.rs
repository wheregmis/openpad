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
                flow: Down,
                padding: { top: 6, bottom: 6 }
                project_name = <Label> {
                    draw_text: { color: #e6e9ee, text_style: <THEME_FONT_REGULAR> { font_size: 12 } }
                }
                project_path = <Label> {
                    draw_text: { color: #aab3bd, text_style: <THEME_FONT_REGULAR> { font_size: 10 } }
                }
            }

            NewSessionRow = <View> {
                width: Fill, height: Fit
                padding: { top: 6, bottom: 8 }
                new_session_button = <Button> {
                    width: Fill, height: 36
                    text: "+  New session"
                    draw_bg: {
                        color: #232830
                        color_hover: #2a313b
                        border_radius: 8.0
                        border_size: 1.0
                        border_color_1: #313842
                        border_color_2: #2a3039
                    }
                    draw_text: { color: #e6e9ee, text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
                }
            }

            SessionRow = <View> {
                width: Fill, height: Fit
                padding: { top: 2, bottom: 2 }
                flow: Right,
                spacing: 4,
                align: { y: 0.5 }

                session_button = <Button> {
                    width: Fill, height: 34
                    margin: { right: 8 }
                    text: "Session"
                  draw_bg: {
                        color: #1f2329
                        color_hover: #242a32
                        border_radius: 8.0
                        border_size: 0.0
                    }
                    draw_text: { color: #e6e9ee, text_style: <THEME_FONT_REGULAR> { font_size: 11 } }
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
                            color: #6b7b8c
                            color_hover: #ffffff
                            color_down: #ffffff
                        }
                        draw_bg: {
                            color: #1f2329
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
                            color: #6b7b8c
                            color_hover: #ffffff
                            color_down: #ffffff
                        }
                        draw_bg: {
                            color: #1f2329
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
                            color: #6b7b8c
                            color_hover: #ffffff
                            color_down: #ffffff
                        }
                        draw_bg: {
                            color: #1f2329
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
                            color: #6b7b8c
                            color_hover: #ffffff
                            color_down: #ffffff
                        }
                        draw_bg: {
                            color: #1f2329
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
        path: String,
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
            let path = if project.worktree == "." {
                std::env::current_dir()
                    .ok()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| project.worktree.clone())
            } else {
                project.worktree.clone()
            };

            items.push(PanelItemKind::ProjectHeader {
                project_id: project_id.clone(),
                name,
                path,
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
                path: "".to_string(),
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

        let list = self.view.portal_list(id!(list));
        for (item_id, widget) in list.items_with_actions(&actions) {
            if item_id >= self.items.len() {
                continue;
            }
            match &self.items[item_id] {
                PanelItemKind::NewSession { project_id } => {
                    if widget.button(id!(new_session_button)).clicked(&actions) {
                        cx.action(ProjectsPanelAction::CreateSession(project_id.clone()));
                    }
                }
                PanelItemKind::SessionRow { session_id, .. } => {
                    if widget.button(id!(session_button)).clicked(&actions) {
                        cx.action(ProjectsPanelAction::SelectSession(session_id.clone()));
                    }

                    if widget.button(id!(delete_button)).clicked(&actions) {
                        cx.action(ProjectsPanelAction::DeleteSession(session_id.clone()));
                    }
                    if widget.button(id!(rename_button)).clicked(&actions) {
                        cx.action(ProjectsPanelAction::RenameSession(session_id.clone()));
                    }
                    if widget.button(id!(branch_button)).clicked(&actions) {
                        cx.action(ProjectsPanelAction::BranchSession(session_id.clone()));
                    }
                    if widget.button(id!(abort_button)).clicked(&actions) {
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
                        PanelItemKind::ProjectHeader { name, path, .. } => {
                            item_widget.label(id!(project_name)).set_text(cx, name);
                            item_widget.label(id!(project_path)).set_text(cx, path);
                        }
                        PanelItemKind::SessionRow { session_id, title } => {
                            item_widget.button(id!(session_button)).set_text(cx, title);
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
                            item_widget.button(id!(session_button)).apply_over(
                                cx,
                                live! {
                                    draw_bg: { color: (color) }
                                },
                            );
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
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.projects = projects;
            inner.sessions = sessions;
            inner.selected_session_id = selected_session_id;
            inner.dirty = true;
            inner.redraw(cx);
        }
    }
}
