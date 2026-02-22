//! Files panel for IDE-style left sidebar using Makepad's dedicated FileTree widget.

use crate::state::actions::ProjectsPanelAction;
use makepad_widgets::file_tree::{FileTree, FileTreeAction};
use makepad_widgets::*;
use openpad_protocol::Project;
use std::collections::HashMap;
use std::path::Path;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*
    use mod.theme.*

    mod.widgets.FilesPanel = #(FilesPanel::register_widget(vm)) {
        width: Fill
        height: Fill
        flow: Down

        file_tree: FileTree {
            width: Fill
            height: Fill
            margin: Inset{ left: 6, right: 6, top: 6, bottom: 6 }
            node_height: 28.0
            draw_scroll_shadow: { shadow_size: 0.0 }

            file_node: <FileTreeNode> {
                draw_bg +: {
                    color_1: theme.THEME_COLOR_TRANSPARENT
                    color_2: theme.THEME_COLOR_TRANSPARENT
                    color_active: theme.THEME_COLOR_HOVER_SUBTLE
                }
                draw_text +: {
                    color: theme.THEME_COLOR_TEXT_LIGHT
                    color_active: theme.THEME_COLOR_TEXT_PRIMARY
                    text_style: theme.font_regular { font_size: 11.0 }
                }
            }

            folder_node: <FileTreeNode> {
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

const IGNORED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
];

#[derive(Script, ScriptHook, Widget)]
pub struct FilesPanel {
    #[wrap]
    #[live]
    file_tree: FileTree,

    #[rust]
    projects: Vec<Project>,
    #[rust]
    file_node_to_path: HashMap<LiveId, (String, String)>,

    #[rust]
    current_dir_name: String,
    #[rust]
    dir_cache: HashMap<String, Vec<(String, bool)>>,
    #[rust]
    normalized_worktrees: HashMap<String, String>,
}

impl FilesPanel {
    fn derive_project_name(&self, project: &Project) -> String {
        if let Some(name) = &project.name {
            if !name.trim().is_empty() {
                return name.clone();
            }
        }
        let worktree = if project.worktree == "." {
            if !self.current_dir_name.is_empty() {
                self.current_dir_name.clone()
            } else {
                project.worktree.clone()
            }
        } else {
            project.worktree.clone()
        };
        Path::new(&worktree)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|n| !n.is_empty())
            .unwrap_or(&worktree)
            .to_string()
    }

    fn normalize_worktree(&mut self, worktree: &str) -> String {
        if let Some(cached) = self.normalized_worktrees.get(worktree) {
            return cached.clone();
        }

        let normalized = if worktree == "." {
            if let Ok(path) = std::env::current_dir() {
                path.to_string_lossy().to_string()
            } else {
                worktree.to_string()
            }
        } else {
            std::fs::canonicalize(worktree)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| worktree.to_string())
        };

        self.normalized_worktrees
            .insert(worktree.to_string(), normalized.clone());
        normalized
    }

    fn read_dir_entries(&mut self, path: &Path) -> Vec<(String, bool)> {
        let path_str = path.to_string_lossy().to_string();
        if let Some(cached) = self.dir_cache.get(&path_str) {
            return cached.clone();
        }

        let mut entries = Vec::new();
        if let Ok(rd) = std::fs::read_dir(path) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with('.') && name != ".git" {
                    continue;
                }
                let is_dir = e.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                if is_dir && IGNORED_DIRS.contains(&name.as_str()) {
                    continue;
                }
                entries.push((name, is_dir));
            }
            entries.sort_by(|a, b| match (a.1, b.1) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.0.to_lowercase().cmp(&b.0.to_lowercase()),
            });
        }

        self.dir_cache.insert(path_str, entries.clone());
        entries
    }

    fn node_id(project_id: &str, path: &Path) -> LiveId {
        let key = format!("{}:{}", project_id, path.to_string_lossy());
        LiveId::from_str(&key)
    }

    fn draw_dir_recursive(&mut self, cx: &mut Cx2d, project_id: &str, dir: &Path) {
        for (name, is_dir) in self.read_dir_entries(dir) {
            let full_path = dir.join(&name);
            let node_id = Self::node_id(project_id, &full_path);
            if is_dir {
                if self.file_tree.begin_folder(cx, node_id, &name).is_ok() {
                    self.draw_dir_recursive(cx, project_id, &full_path);
                    self.file_tree.end_folder();
                }
            } else {
                self.file_node_to_path.insert(
                    node_id,
                    (
                        project_id.to_string(),
                        full_path.to_string_lossy().to_string(),
                    ),
                );
                self.file_tree.file(cx, node_id, &name);
            }
        }
    }

    fn draw_tree(&mut self, cx: &mut Cx2d) {
        self.file_node_to_path.clear();

        // Optimization: avoid repeated system calls to get current directory.
        if self.current_dir_name.is_empty() {
            self.current_dir_name = std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| ".".to_string());
        }

        // Optimization: clone projects once per frame to avoid borrow checker issues
        // while still using mutable caches.
        let projects = self.projects.clone();
        for project in &projects {
            if project.worktree == "/" || project.worktree.is_empty() {
                continue;
            }

            let display_name = self.derive_project_name(project);
            let root = self.normalize_worktree(&project.worktree);
            let root_path = Path::new(&root);
            if !root_path.is_dir() {
                continue;
            }

            let project_node_id = LiveId::from_str(&format!("project:{}", project.id));
            if self
                .file_tree
                .begin_folder(cx, project_node_id, &display_name)
                .is_ok()
            {
                self.file_node_to_path
                    .insert(project_node_id, (project.id.clone(), root.clone()));
                self.draw_dir_recursive(cx, &project.id, root_path);
                self.file_tree.end_folder();
            }
        }
    }
}

impl Widget for FilesPanel {
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

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.file_tree.handle_event(cx, event, scope);
        });

        if let Some(item) = actions.find_widget_action(self.file_tree.widget_uid()) {
            if let FileTreeAction::FileClicked(node_id) = item.cast() {
                let Some((project_id, absolute_path)) = self.file_node_to_path.get(&node_id) else {
                    return;
                };
                if absolute_path.is_empty() {
                    return;
                }
                cx.action(ProjectsPanelAction::OpenFile {
                    project_id: project_id.clone(),
                    absolute_path: absolute_path.clone(),
                });
            }
        }
    }
}

impl FilesPanelRef {
    pub fn set_data(&self, cx: &mut Cx, projects: Vec<Project>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.projects = projects;

            // Clear caches when new data is set to ensure the file tree is refreshed.
            inner.dir_cache.clear();
            inner.normalized_worktrees.clear();

            let project_ids: Vec<String> = inner.projects.iter().map(|p| p.id.clone()).collect();
            for project_id in project_ids {
                let project_node_id = LiveId::from_str(&format!("project:{}", project_id));
                inner
                    .file_tree
                    .set_folder_is_open(cx, project_node_id, true, Animate::No);
            }
            inner.file_tree.redraw(cx);
        }
    }
}
