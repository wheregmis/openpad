//! Sessions panel for IDE-style right sidebar using Makepad's FileTree widget.
//! Shows sessions grouped by project in a collapsible tree.

use crate::async_runtime;
use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::*;
use openpad_widgets::{SessionTree, SessionTreeAction};
use openpad_protocol::{Project, Session, SessionSummary};
use std::collections::{HashMap, HashSet};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.SessionsPanel = #(SessionsPanel::register_widget(vm)) {
        width: Fill
        height: Fill
        flow: Down

        file_tree: SessionTree {
            width: Fill
            height: Fill
            margin: Inset{ left: 10, right: 8, top: 8, bottom: 8 }
            node_height: 28.0
            draw_scroll_shadow: { shadow_size: 0.0 }

            file_node: <SessionTreeNode> {
                draw_bg +: {
                    color_1: theme.THEME_COLOR_TRANSPARENT
                    color_2: theme.THEME_COLOR_TRANSPARENT
                    color_active: theme.THEME_COLOR_HOVER_SUBTLE
                }
                draw_text +: {
                    color: theme.THEME_COLOR_TEXT_LIGHT
                    color_active: theme.THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular { font_size: 10.5 }
                }
            }

            folder_node: <SessionTreeNode> {
                draw_bg +: {
                    color_1: theme.THEME_COLOR_TRANSPARENT
                    color_2: theme.THEME_COLOR_TRANSPARENT
                    color_active: theme.THEME_COLOR_HOVER_SUBTLE
                }
                draw_text +: {
                    color: theme.THEME_COLOR_TEXT_PRIMARY
                    color_active: theme.THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_bold { font_size: 11.5 }
                }
                draw_icon +: {
                    color: theme.THEME_COLOR_TEXT_MUTED_LIGHTER
                    color_active: theme.THEME_COLOR_TEXT_MUTED_LIGHTER
                }
            }

            filler +: {
                color_1: theme.THEME_COLOR_TRANSPARENT
                color_2: theme.THEME_COLOR_TRANSPARENT
                color_active: theme.THEME_COLOR_TRANSPARENT
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct SessionsPanel {
    #[wrap]
    #[live]
    file_tree: SessionTree,

    #[rust]
    projects: Vec<Project>,
    #[rust]
    sessions: Vec<Session>,
    #[rust]
    selected_session_id: Option<String>,
    #[rust]
    working_by_session: HashMap<String, bool>,
    #[rust]
    session_node_to_id: HashMap<LiveId, String>,
    #[rust]
    project_node_to_id: HashMap<LiveId, Option<String>>,
}

impl SessionsPanel {
    fn derive_project_name(project: &Project) -> String {
        if let Some(name) = &project.name {
            if !name.trim().is_empty() {
                return name.clone();
            }
        }
        let worktree = if project.worktree == "." {
            std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| project.worktree.clone())
        } else {
            project.worktree.clone()
        };
        std::path::Path::new(&worktree)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|n| !n.is_empty())
            .unwrap_or(&worktree)
            .to_string()
    }

    fn project_node_id(project_id: &str) -> LiveId {
        LiveId::from_str(&format!("sessions_project:{}", project_id))
    }

    fn other_project_node_id() -> LiveId {
        LiveId::from_str("sessions_project:other")
    }

    fn session_node_id(session_id: &str) -> LiveId {
        LiveId::from_str(&format!("sessions_session:{}", session_id))
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

    fn session_display_label(&self, session: &Session) -> String {
        let mut title = async_runtime::get_session_title(session);
        if title.trim().is_empty() {
            title = "Untitled session".to_string();
        }

        let mut label = if title.chars().count() > 40 {
            format!("{}…", title.chars().take(40).collect::<String>())
        } else {
            title
        };

        if let Some(summary) = session.summary.as_ref() {
            if let Some((files, adds, dels)) = Self::session_diff_stats(summary) {
                label.push_str(&format!("   {}  {}  {}", files, adds, dels));
            }
        }

        if self
            .working_by_session
            .get(&session.id)
            .copied()
            .unwrap_or(false)
        {
            label.push_str("   ●");
        }

        if self
            .selected_session_id
            .as_ref()
            .map(|id| id == &session.id)
            .unwrap_or(false)
        {
            label.push_str("   (selected)");
        }

        label
    }

    fn draw_tree(&mut self, cx: &mut Cx2d) {
        self.session_node_to_id.clear();
        self.project_node_to_id.clear();

        let projects = self.projects.clone();
        let sessions = self.sessions.clone();

        let mut grouped: HashMap<String, Vec<Session>> = HashMap::new();
        for session in sessions {
            grouped
                .entry(session.project_id.clone())
                .or_default()
                .push(session);
        }

        let mut known_project_ids = HashSet::new();
        for project in &projects {
            if project.worktree == "/" || project.worktree.is_empty() {
                continue;
            }

            known_project_ids.insert(project.id.clone());

            let project_name = Self::derive_project_name(project);
            let project_node_id = Self::project_node_id(&project.id);
            self.project_node_to_id
                .insert(project_node_id, Some(project.id.clone()));

            if self
                .file_tree
                .begin_folder(cx, project_node_id, &project_name)
                .is_ok()
            {
                if let Some(project_sessions) = grouped.get(&project.id) {
                    for session in project_sessions {
                        let node_id = Self::session_node_id(&session.id);
                        let label = self.session_display_label(session);
                        self.session_node_to_id.insert(node_id, session.id.clone());
                        self.file_tree.file(cx, node_id, &label);
                    }
                }
                self.file_tree.end_folder();
            }
        }

        let ungrouped: Vec<Session> = grouped
            .into_iter()
            .filter(|(project_id, _)| !known_project_ids.contains(project_id))
            .flat_map(|(_, sessions)| sessions)
            .collect();

        if !ungrouped.is_empty()
            && self
                .file_tree
                .begin_folder(cx, Self::other_project_node_id(), "Other")
                .is_ok()
        {
            self.project_node_to_id
                .insert(Self::other_project_node_id(), None);
            for session in &ungrouped {
                let node_id = Self::session_node_id(&session.id);
                let label = self.session_display_label(session);
                self.session_node_to_id.insert(node_id, session.id.clone());
                self.file_tree.file(cx, node_id, &label);
            }
            self.file_tree.end_folder();
        }
    }
}

impl Widget for SessionsPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.file_tree.handle_event(cx, event, scope);
        });

        if let Some(item) = actions.find_widget_action(self.file_tree.widget_uid()) {
            if let SessionTreeAction::FileLeftClicked(node_id) = item.cast() {
                log!("SessionTree action: FileLeftClicked {:?}", node_id);
                if let Some(session_id) = self.session_node_to_id.get(&node_id).cloned() {
                    self.selected_session_id = Some(session_id.clone());
                    cx.action(ProjectsPanelAction::SelectSession(session_id.clone()));
                }
            } else if let SessionTreeAction::FileRightClicked(node_id) = item.cast() {
                log!("SessionTree action: FileRightClicked {:?}", node_id);
                if let Some(session_id) = self.session_node_to_id.get(&node_id).cloned() {
                    let working = self
                        .working_by_session
                        .get(&session_id)
                        .copied()
                        .unwrap_or(false);
                    cx.action(ProjectsPanelAction::OpenSessionContextMenu {
                        session_id,
                        x: 0.0,
                        y: 0.0,
                        working,
                    });
                }
            } else if let SessionTreeAction::FolderRightClicked(node_id) = item.cast() {
                log!("SessionTree action: FolderRightClicked {:?}", node_id);
                let project_id = self
                    .project_node_to_id
                    .get(&node_id)
                    .cloned()
                    .unwrap_or(None);
                cx.action(ProjectsPanelAction::OpenProjectContextMenu { project_id });
            } else if let SessionTreeAction::FolderLeftClicked(node_id) = item.cast() {
                log!("SessionTree action: FolderLeftClicked {:?}", node_id);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        while self.file_tree.draw_walk(cx, &mut Scope::empty(), walk).is_step() {
            self.draw_tree(cx);
        }
        DrawStep::done()
    }
}

impl SessionsPanelRef {
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

            let project_ids: Vec<String> = inner.projects.iter().map(|p| p.id.clone()).collect();
            for project_id in project_ids {
                inner.file_tree.set_folder_is_open(
                    cx,
                    SessionsPanel::project_node_id(&project_id),
                    true,
                    Animate::No,
                );
            }
            inner
                .file_tree
                .set_folder_is_open(cx, SessionsPanel::other_project_node_id(), true, Animate::No);

            inner.file_tree.redraw(cx);
        }
    }
}
