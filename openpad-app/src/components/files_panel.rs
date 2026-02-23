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

#[derive(Clone, Debug)]
struct CachedDirEntry {
    name: String,
    is_dir: bool,
    node_id: LiveId,
    full_path: String,
}

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
    cached_entries: HashMap<(String, String), Vec<CachedDirEntry>>,
    #[rust]
    normalized_worktrees: HashMap<String, String>,
    #[rust]
    derived_project_names: HashMap<String, String>,
    #[rust]
    project_node_ids: HashMap<String, LiveId>,
    #[rust]
    current_dir: String,
}

impl FilesPanel {
    fn get_project_name(
        &mut self,
        project_id: &str,
        project_name: Option<&String>,
        worktree: &str,
    ) -> String {
        if let Some(name) = self.derived_project_names.get(project_id) {
            return name.clone();
        }

        let name = if let Some(name) = project_name {
            if !name.trim().is_empty() {
                name.clone()
            } else {
                self.derive_from_worktree(worktree)
            }
        } else {
            self.derive_from_worktree(worktree)
        };

        self.derived_project_names
            .insert(project_id.to_string(), name.clone());
        name
    }

    fn derive_from_worktree(&mut self, worktree: &str) -> String {
        let actual_worktree = if worktree == "." {
            if self.current_dir.is_empty() {
                self.current_dir = std::env::current_dir()
                    .ok()
                    .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                    .unwrap_or_else(|| ".".to_string());
            }
            &self.current_dir
        } else {
            worktree
        };
        Path::new(actual_worktree)
            .file_name()
            .and_then(|n| n.to_str())
            .filter(|n| !n.is_empty())
            .unwrap_or(actual_worktree)
            .to_string()
    }

    fn get_normalized_worktree(&mut self, worktree: &str) -> String {
        if let Some(normalized) = self.normalized_worktrees.get(worktree) {
            return normalized.clone();
        }

        let normalized = if worktree == "." {
            if let Ok(path) = std::env::current_dir() {
                path.to_string_lossy().to_string()
            } else {
                ".".to_string()
            }
        } else {
            // Optimization: std::fs::canonicalize is slow, so we cache the result.
            std::fs::canonicalize(worktree)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| worktree.to_string())
        };

        self.normalized_worktrees
            .insert(worktree.to_string(), normalized.clone());
        normalized
    }

    fn get_dir_entries(&mut self, project_id: &str, dir_path: &Path) -> Vec<CachedDirEntry> {
        let path_str = dir_path.to_string_lossy().to_string();
        let cache_key = (project_id.to_string(), path_str.clone());
        if let Some(entries) = self.cached_entries.get(&cache_key) {
            return entries.clone();
        }

        let mut entries = Vec::new();
        // Optimization: avoid repeated filesystem scanning by caching directory contents.
        if let Ok(rd) = std::fs::read_dir(dir_path) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with('.') && name != ".git" {
                    continue;
                }
                let is_dir = e.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                if is_dir && IGNORED_DIRS.contains(&name.as_str()) {
                    continue;
                }

                let full_path = dir_path.join(&name);
                let full_path_str = full_path.to_string_lossy().to_string();
                let node_id = Self::node_id(project_id, &full_path);

                entries.push(CachedDirEntry {
                    name,
                    is_dir,
                    node_id,
                    full_path: full_path_str,
                });
            }
            // Sort: folders first, then alphabetical (case-insensitive)
            entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });
        }

        self.cached_entries.insert(cache_key, entries.clone());
        entries
    }

    fn node_id(project_id: &str, path: &Path) -> LiveId {
        let key = format!("{}:{}", project_id, path.to_string_lossy());
        LiveId::from_str(&key)
    }

    fn draw_dir_recursive(&mut self, cx: &mut Cx2d, project_id: &str, dir: &Path) {
        let entries = self.get_dir_entries(project_id, dir);
        for i in 0..entries.len() {
            // Optimization: avoid holding a borrow to self.cached_entries while recursing
            // by cloning the specific entry we are currently processing.
            let entry = entries[i].clone();
            if entry.is_dir {
                if self
                    .file_tree
                    .begin_folder(cx, entry.node_id, &entry.name)
                    .is_ok()
                {
                    self.draw_dir_recursive(cx, project_id, Path::new(&entry.full_path));
                    self.file_tree.end_folder();
                }
            } else {
                self.file_node_to_path.insert(
                    entry.node_id,
                    (project_id.to_string(), entry.full_path.clone()),
                );
                self.file_tree.file(cx, entry.node_id, &entry.name);
            }
        }
    }

    fn draw_tree(&mut self, cx: &mut Cx2d) {
        self.file_node_to_path.clear();

        // Optimization: avoid cloning self.projects every frame by using indexed access
        // and only cloning the minimum necessary strings to satisfy the borrow checker.
        for i in 0..self.projects.len() {
            let (project_id, worktree, project_name) = {
                let p = &self.projects[i];
                if p.worktree == "/" || p.worktree.is_empty() {
                    continue;
                }
                (p.id.clone(), p.worktree.clone(), p.name.clone())
            };

            let display_name = self.get_project_name(&project_id, project_name.as_ref(), &worktree);
            let root = self.get_normalized_worktree(&worktree);
            let root_path = Path::new(&root);
            if !root_path.is_dir() {
                continue;
            }

            // Optimization: cache project node IDs to avoid repeated formatting and hashing.
            let project_node_id = if let Some(id) = self.project_node_ids.get(&project_id) {
                *id
            } else {
                let id = LiveId::from_str(&format!("project:{}", project_id));
                self.project_node_ids.insert(project_id.clone(), id);
                id
            };

            if self
                .file_tree
                .begin_folder(cx, project_node_id, &display_name)
                .is_ok()
            {
                self.file_node_to_path
                    .insert(project_node_id, (project_id.clone(), root.clone()));
                self.draw_dir_recursive(cx, &project_id, root_path);
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

            // Optimization: invalidate all caches when new project data is set to ensure UI consistency.
            inner.cached_entries.clear();
            inner.normalized_worktrees.clear();
            inner.derived_project_names.clear();
            inner.project_node_ids.clear();
            inner.current_dir.clear();

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
