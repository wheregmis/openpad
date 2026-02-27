//! Sessions panel for IDE-style right sidebar using Makepad's FileTree widget.
//! Shows sessions grouped by project in a collapsible tree.

use crate::async_runtime;
use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::*;
use openpad_protocol::{Project, Session, SessionSummary};
use openpad_widgets::{SessionTree, SessionTreeAction};
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

#[derive(Clone, Debug)]
pub struct CachedSession {
    pub node_id: LiveId,
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct CachedProject {
    pub node_id: LiveId,
    pub id: Option<String>,
    pub name: String,
    pub sessions: Vec<CachedSession>,
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

    #[rust]
    current_dir_name: String,

    #[rust]
    dirty: bool,
    #[rust]
    cached_projects: Vec<CachedProject>,
    #[rust]
    cached_other_sessions: Vec<CachedSession>,
}

impl SessionsPanel {
    fn derive_project_name(project: &Project, current_dir_name: &str) -> String {
        if let Some(name) = &project.name {
            if !name.trim().is_empty() {
                return name.clone();
            }
        }
        let worktree = if project.worktree == "." {
            current_dir_name
        } else {
            &project.worktree
        };
        std::path::Path::new(worktree)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|n| !n.is_empty())
            .unwrap_or(worktree)
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

    fn rebuild_cache(&mut self) {
        self.session_node_to_id.clear();
        self.project_node_to_id.clear();
        self.cached_projects.clear();
        self.cached_other_sessions.clear();

        if self.current_dir_name.is_empty() {
            self.current_dir_name = std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| ".".to_string());
        }
        let current_dir_name = &self.current_dir_name;

        let mut grouped: HashMap<String, Vec<&Session>> = HashMap::new();
        for session in &self.sessions {
            grouped
                .entry(session.project_id.clone())
                .or_default()
                .push(session);
        }

        let mut known_project_ids = HashSet::new();
        for project in &self.projects {
            if project.worktree == "/" || project.worktree.is_empty() {
                continue;
            }

            known_project_ids.insert(project.id.clone());

            let project_name = Self::derive_project_name(project, current_dir_name);
            let project_node_id = Self::project_node_id(&project.id);
            self.project_node_to_id
                .insert(project_node_id, Some(project.id.clone()));

            let mut cached_sessions = Vec::new();
            if let Some(project_sessions) = grouped.get(&project.id) {
                for session in project_sessions {
                    let node_id = Self::session_node_id(&session.id);
                    let label = self.session_display_label(session);
                    self.session_node_to_id.insert(node_id, session.id.clone());
                    cached_sessions.push(CachedSession {
                        node_id,
                        id: session.id.clone(),
                        label,
                    });
                }
            }

            self.cached_projects.push(CachedProject {
                node_id: project_node_id,
                id: Some(project.id.clone()),
                name: project_name,
                sessions: cached_sessions,
            });
        }

        let ungrouped: Vec<&Session> = grouped
            .into_iter()
            .filter(|(project_id, _)| !known_project_ids.contains(project_id))
            .flat_map(|(_, sessions)| sessions)
            .collect();

        if !ungrouped.is_empty() {
            self.project_node_to_id
                .insert(Self::other_project_node_id(), None);
            for session in ungrouped {
                let node_id = Self::session_node_id(&session.id);
                let label = self.session_display_label(session);
                self.session_node_to_id.insert(node_id, session.id.clone());
                self.cached_other_sessions.push(CachedSession {
                    node_id,
                    id: session.id.clone(),
                    label,
                });
            }
        }

        self.dirty = false;
    }

    fn session_display_label(&self, session: &Session) -> String {
        let title = async_runtime::get_session_title(session);
        let title = title.trim();
        let display_title = if title.is_empty() {
            "Untitled session"
        } else {
            title
        };

        // Optimization: use String::with_capacity and more efficient truncation
        // to reduce allocations and heap churn in the draw loop.
        let mut label = String::with_capacity(128);

        if display_title.len() > 60 {
            // Fast estimate for truncation using byte indices
            let mut end = 40;
            while end > 0 && !display_title.is_char_boundary(end) {
                end -= 1;
            }
            label.push_str(&display_title[..end]);
            label.push('…');
        } else {
            label.push_str(display_title);
        }

        if let Some(summary) = session.summary.as_ref() {
            if let Some((files, adds, dels)) = Self::session_diff_stats(summary) {
                label.push_str("   ");
                label.push_str(&files);
                label.push_str("  ");
                label.push_str(&adds);
                label.push_str("  ");
                label.push_str(&dels);
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
        if self.dirty {
            self.rebuild_cache();
        }

        // Optimization: draw from pre-calculated cache to avoid $O(N)$ heap churn
        // during the high-frequency render loop.
        let projects = std::mem::take(&mut self.cached_projects);
        for project in &projects {
            if self
                .file_tree
                .begin_folder(cx, project.node_id, &project.name)
                .is_ok()
            {
                for session in &project.sessions {
                    self.file_tree.file(cx, session.node_id, &session.label);
                }
                self.file_tree.end_folder();
            }
        }
        self.cached_projects = projects;

        let other_sessions = std::mem::take(&mut self.cached_other_sessions);
        if !other_sessions.is_empty()
            && self
                .file_tree
                .begin_folder(cx, Self::other_project_node_id(), "Other")
                .is_ok()
        {
            for session in &other_sessions {
                self.file_tree.file(cx, session.node_id, &session.label);
            }
            self.file_tree.end_folder();
        }
        self.cached_other_sessions = other_sessions;
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
                    self.dirty = true;
                    self.redraw(cx);
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
        while self
            .file_tree
            .draw_walk(cx, &mut Scope::empty(), walk)
            .is_step()
        {
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
            inner.dirty = true;

            let project_ids: Vec<String> = inner.projects.iter().map(|p| p.id.clone()).collect();
            for project_id in project_ids {
                inner.file_tree.set_folder_is_open(
                    cx,
                    SessionsPanel::project_node_id(&project_id),
                    true,
                    Animate::No,
                );
            }
            inner.file_tree.set_folder_is_open(
                cx,
                SessionsPanel::other_project_node_id(),
                true,
                Animate::No,
            );

            inner.file_tree.redraw(cx);
        }
    }
}
