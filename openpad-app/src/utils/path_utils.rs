/// Normalize a worktree path to an absolute directory.
/// This version uses canonicalize for filesystem-backed resolution.
pub fn normalize_worktree_canonical(worktree: &str) -> String {
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

/// Normalize a worktree path for display purposes.
/// This lighter version doesn't use canonicalize.
pub fn normalize_worktree(worktree: &str) -> String {
    if worktree == "." {
        if let Ok(current_dir) = std::env::current_dir() {
            return current_dir.to_string_lossy().to_string();
        }
    }
    worktree.to_string()
}
