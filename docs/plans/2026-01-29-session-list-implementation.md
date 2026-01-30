# Session List Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform the session list from displaying raw IDs to showing rich, meaningful information with timestamps, file changes, and visual hierarchy in a compact timeline view.

**Architecture:** Enhance the existing `ProjectsPanel` widget and `SessionRow` template to display metadata from the Session API. Track message counts client-side via SSE events. Implement smart title fallback logic and relative timestamp formatting. Use custom shaders for status indicators and interaction states.

**Tech Stack:** Makepad (Rust UI framework), live_design! DSL for UI, SSE for real-time updates, chrono for date/time formatting

---

## Task 1: Add Helper Functions for Formatting

**Files:**
- Modify: `openpad-app/src/app.rs`

**Step 1: Add chrono dependency**

Add to `openpad-app/Cargo.toml` dependencies:
```toml
chrono = "0.4"
```

**Step 2: Add time formatting helper functions**

Add near the top of `openpad-app/src/app.rs` after imports:

```rust
use chrono::{DateTime, Local, Utc};

/// Format millisecond timestamp to relative time string
fn format_relative_time(timestamp_ms: i64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(timestamp_ms / 1000, 0)
        .unwrap_or_else(|| Utc::now());
    let local: DateTime<Local> = datetime.into();
    let now = Local::now();
    let duration = now.signed_duration_since(local);

    if duration.num_minutes() < 1 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() < 1 {
        format!("Today at {}", local.format("%I:%M%p").to_string().to_lowercase())
    } else if duration.num_days() < 2 {
        format!("Yesterday at {}", local.format("%I:%M%p").to_string().to_lowercase())
    } else {
        format!("{}", local.format("%b %d at %I:%M%p").to_string().to_lowercase())
    }
}

/// Generate smart session title
fn generate_session_title(session: &Session) -> String {
    if !session.title.is_empty() {
        session.title.clone()
    } else if !session.slug.is_empty() {
        session.slug.clone()
    } else {
        let datetime = DateTime::<Utc>::from_timestamp(session.time.created / 1000, 0)
            .unwrap_or_else(|| Utc::now());
        let local: DateTime<Local> = datetime.into();
        format!("New session - {}", local.format("%b %d, %I:%M %p"))
    }
}
```

**Step 3: Build to verify compilation**

Run: `cargo build`
Expected: Success (warnings OK)

**Step 4: Commit**

```bash
git add openpad-app/Cargo.toml openpad-app/src/app.rs
git commit -m "feat: add time formatting and title generation helpers"
```

---

## Task 2: Extend Session Data Structure

**Files:**
- Modify: `openpad-app/src/app.rs` (around line 243-258)

**Step 1: Update PanelItemKind::SessionRow**

Replace the `SessionRow` variant in `PanelItemKind` enum (around line 253-256):

```rust
SessionRow {
    session_id: String,
    title: String,
    timestamp: String,
    is_archived: bool,
    is_shared: bool,
    is_forked: bool,
    file_changes: Option<(i64, i64, i64)>, // (additions, deletions, files)
    message_count: usize,
},
```

**Step 2: Add message count tracking to App struct**

Add to `App` struct (around line 457-486):

```rust
#[rust]
message_counts: std::collections::HashMap<String, usize>,
```

**Step 3: Add HashMap import**

Update imports at top of file to include:
```rust
use std::collections::HashMap;
```

**Step 4: Build to verify**

Run: `cargo build`
Expected: Compilation errors about `rebuild_items()` - expected, we'll fix next

**Step 5: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: extend session data structure with metadata fields"
```

---

## Task 3: Update rebuild_items() Logic

**Files:**
- Modify: `openpad-app/src/app.rs:279-351`

**Step 1: Update rebuild_items() method**

Replace the `rebuild_items()` method in `ProjectsPanel` impl:

```rust
fn rebuild_items(&mut self, message_counts: &HashMap<String, usize>) {
    let mut grouped: HashMap<Option<String>, Vec<Session>> = HashMap::new();
    for session in &self.sessions {
        grouped
            .entry(Some(session.project_id.clone()))
            .or_default()
            .push(session.clone());
    }

    let mut items = Vec::new();
    for project in &self.projects {
        let project_id = Some(project.id.clone());
        let name = project
            .name
            .clone()
            .unwrap_or_else(|| project.id.clone());
        let path = project.path.clone().unwrap_or_default();

        items.push(PanelItemKind::ProjectHeader {
            project_id: project_id.clone(),
            name,
            path,
        });
        items.push(PanelItemKind::NewSession {
            project_id: project_id.clone(),
        });

        if let Some(mut sessions) = grouped.get(&project_id).cloned() {
            // Sort by updated time, most recent first
            sessions.sort_by(|a, b| b.time.updated.cmp(&a.time.updated));

            for session in sessions {
                let title = generate_session_title(&session);
                let timestamp = format_relative_time(session.time.updated);
                let is_archived = session.time.archived.is_some();
                let is_shared = session.share.is_some();
                let is_forked = session.parent_id.is_some();
                let file_changes = session.summary.as_ref().map(|s| {
                    (s.additions, s.deletions, s.files)
                });
                let message_count = *message_counts.get(&session.id).unwrap_or(&0);

                items.push(PanelItemKind::SessionRow {
                    session_id: session.id.clone(),
                    title,
                    timestamp,
                    is_archived,
                    is_shared,
                    is_forked,
                    file_changes,
                    message_count,
                });
            }
        }
        items.push(PanelItemKind::Spacer);
    }

    if let Some(mut sessions) = grouped.get(&None).cloned() {
        if !sessions.is_empty() {
            sessions.sort_by(|a, b| b.time.updated.cmp(&a.time.updated));

            items.push(PanelItemKind::ProjectHeader {
                project_id: None,
                name: "Other".to_string(),
                path: "".to_string(),
            });
            items.push(PanelItemKind::NewSession { project_id: None });
            for session in sessions {
                let title = generate_session_title(&session);
                let timestamp = format_relative_time(session.time.updated);
                let is_archived = session.time.archived.is_some();
                let is_shared = session.share.is_some();
                let is_forked = session.parent_id.is_some();
                let file_changes = session.summary.as_ref().map(|s| {
                    (s.additions, s.deletions, s.files)
                });
                let message_count = *message_counts.get(&session.id).unwrap_or(&0);

                items.push(PanelItemKind::SessionRow {
                    session_id: session.id.clone(),
                    title,
                    timestamp,
                    is_archived,
                    is_shared,
                    is_forked,
                    file_changes,
                    message_count,
                });
            }
        }
    }

    self.items = items;
    self.dirty = false;
}
```

**Step 2: Build to verify**

Run: `cargo build`
Expected: More compilation errors - Widget::draw_walk needs updating

**Step 3: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: update rebuild_items to populate session metadata"
```

---

## Task 4: Update Widget Drawing Logic

**Files:**
- Modify: `openpad-app/src/app.rs:377-436`

**Step 1: Update draw_walk to pass message_counts**

In `ProjectsPanel` impl for `Widget`, update the `draw_walk` method (around line 377):

```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    if self.dirty {
        // Get message_counts from parent App - we'll pass it via scope later
        // For now, use empty HashMap
        let empty_counts = HashMap::new();
        self.rebuild_items(&empty_counts);
    }

    self.visible_items.clear();

    while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
        if let Some(mut list) = item.as_portal_list().borrow_mut() {
            if self.items.is_empty() {
                list.set_item_range(cx, 0, 0);
                continue;
            } else {
                list.set_item_range(cx, 0, self.items.len().saturating_sub(1));
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
                    PanelItemKind::SessionRow {
                        session_id,
                        title,
                        timestamp,
                        is_archived,
                        file_changes,
                        message_count,
                        ..
                    } => {
                        // Set title
                        item_widget.label(id!(session_title)).set_text(cx, title);

                        // Build metadata string
                        let mut metadata_parts = vec![timestamp.clone()];

                        if let Some((additions, deletions, files)) = file_changes {
                            metadata_parts.push(format!("+{} -{}", additions, deletions));
                            metadata_parts.push(format!("{} files", files));
                        }

                        if *message_count > 0 {
                            metadata_parts.push(format!("{} messages", message_count));
                        }

                        let metadata = metadata_parts.join(" • ");
                        item_widget.label(id!(session_metadata)).set_text(cx, &metadata);

                        // Set background color based on selection
                        let selected = self
                            .selected_session_id
                            .as_ref()
                            .map(|id| id == session_id)
                            .unwrap_or(false);
                        let bg_color = if selected {
                            vec4(0.14, 0.16, 0.20, 1.0) // #242a32
                        } else {
                            vec4(0.12, 0.14, 0.16, 1.0) // #1f2329
                        };

                        // Determine status color
                        let status_color = if *is_archived {
                            vec4(0.42, 0.48, 0.55, 1.0) // gray #6b7b8c
                        } else {
                            // Check if updated in last 24h (we'll refine this)
                            vec4(0.29, 0.56, 0.87, 1.0) // blue #4a90e2
                        };

                        item_widget.view(id!(session_row_bg)).apply_over(cx, live! {
                            draw_bg: {
                                color: (bg_color),
                                border_color: (status_color)
                            }
                        });
                    }
                    _ => {}
                }

                item_widget.draw_all(cx, scope);
                self.visible_items.push((panel_item, item_widget));
            }
        }
    }
    DrawStep::done()
}
```

**Step 2: Update set_data method signature**

Update `ProjectsPanelRef::set_data` (around line 440):

```rust
pub fn set_data(
    &self,
    cx: &mut Cx,
    projects: Vec<Project>,
    sessions: Vec<Session>,
    selected_session_id: Option<String>,
    message_counts: &HashMap<String, usize>,
) {
    if let Some(mut inner) = self.borrow_mut() {
        inner.projects = projects;
        inner.sessions = sessions;
        inner.selected_session_id = selected_session_id;
        inner.dirty = true;
        inner.redraw(cx);
    }
}
```

**Step 3: Build to verify**

Run: `cargo build`
Expected: Errors about missing UI elements (session_title, session_metadata, session_row_bg)

**Step 4: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: update draw_walk to render session metadata"
```

---

## Task 5: Design New SessionRow UI Template

**Files:**
- Modify: `openpad-app/src/app.rs:104-118` (live_design! section)

**Step 1: Replace SessionRow template**

Replace the `SessionRow` definition in the live_design! block:

```rust
SessionRow = <View> {
    width: Fill, height: Fit
    padding: 0

    session_row_bg = <View> {
        width: Fill, height: 52
        flow: Right
        show_bg: true
        draw_bg: {
            color: #1f2329
            uniform border_color: #4a90e2
            uniform border_size: 3.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Left status bar
                sdf.rect(0.0, 0.0, self.border_size, self.rect_size.y);
                sdf.fill(self.border_color);
                // Background
                sdf.rect(self.border_size, 0.0, self.rect_size.x - self.border_size, self.rect_size.y);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        // Main content area
        <View> {
            width: Fill, height: Fill
            flow: Down
            padding: { left: 12, right: 12, top: 8, bottom: 8 }
            spacing: 4

            // Line 1: Title + icons
            <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 6
                align: { y: 0.5 }

                session_title = <Label> {
                    width: Fill
                    draw_text: {
                        color: #e6e9ee
                        text_style: { font_size: 11 }
                    }
                }
            }

            // Line 2: Metadata
            session_metadata = <Label> {
                width: Fill
                draw_text: {
                    color: #7a8591
                    text_style: { font_size: 10 }
                }
            }
        }
    }
}
```

**Step 2: Build to verify**

Run: `cargo build`
Expected: Success

**Step 3: Test by running the app**

Run: `cargo run`
Expected: App launches, sessions show with titles and metadata (but still need to update all set_data call sites)

**Step 4: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: redesign SessionRow UI with status bar and metadata"
```

---

## Task 6: Update All set_data Call Sites

**Files:**
- Modify: `openpad-app/src/app.rs` (multiple locations in handle_actions)

**Step 1: Update handle_actions to pass message_counts**

Find all calls to `projects_panel(...).set_data()` and update them to pass `&self.message_counts`:

Around line 601-606:
```rust
AppAction::ProjectsLoaded(projects) => {
    self.projects = projects.clone();
    self.ui.projects_panel(id!(projects_panel)).set_data(
        cx,
        self.projects.clone(),
        self.sessions.clone(),
        self.selected_session_id.clone(),
        &self.message_counts,
    );
}
```

Around line 611-618:
```rust
AppAction::SessionsLoaded(sessions) => {
    self.sessions = sessions.clone();
    self.ui.projects_panel(id!(projects_panel)).set_data(
        cx,
        self.projects.clone(),
        self.sessions.clone(),
        self.selected_session_id.clone(),
        &self.message_counts,
    );
}
```

Around line 637-643:
```rust
ProjectsPanelAction::SelectSession(session_id) => {
    self.selected_session_id = Some(session_id.clone());
    self.ui.projects_panel(id!(projects_panel)).set_data(
        cx,
        self.projects.clone(),
        self.sessions.clone(),
        self.selected_session_id.clone(),
        &self.message_counts,
    );
}
```

Around line 661-666:
```rust
OcEvent::SessionCreated(session) => {
    if self.current_session_id.is_none() {
        self.current_session_id = Some(session.id.clone());
    }
    self.sessions.push(session.clone());
    self.ui.projects_panel(id!(projects_panel)).set_data(
        cx,
        self.projects.clone(),
        self.sessions.clone(),
        self.selected_session_id.clone(),
        &self.message_counts,
    );
}
```

**Step 2: Build to verify**

Run: `cargo build`
Expected: Success

**Step 3: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: update all set_data calls to pass message counts"
```

---

## Task 7: Implement Message Count Tracking

**Files:**
- Modify: `openpad-app/src/app.rs:654-683` (handle_opencode_event)

**Step 1: Track message counts from SSE events**

Update the `handle_opencode_event` method to track message counts:

```rust
fn handle_opencode_event(&mut self, cx: &mut Cx, event: &OcEvent) {
    match event {
        OcEvent::SessionCreated(session) => {
            if self.current_session_id.is_none() {
                self.current_session_id = Some(session.id.clone());
            }
            self.sessions.push(session.clone());
            self.ui.projects_panel(id!(projects_panel)).set_data(
                cx,
                self.projects.clone(),
                self.sessions.clone(),
                self.selected_session_id.clone(),
                &self.message_counts,
            );
        }
        OcEvent::MessageUpdated(message) => {
            // Track message count
            let session_id = message.session_id().to_string();
            *self.message_counts.entry(session_id.clone()).or_insert(0) += 1;

            // Find existing message or add new
            if let Some(existing) = self.messages.iter_mut().find(|m| m.id() == message.id()) {
                *existing = message.clone();
            } else {
                self.messages.push(message.clone());
            }

            // Update panel to show new count
            self.ui.projects_panel(id!(projects_panel)).set_data(
                cx,
                self.projects.clone(),
                self.sessions.clone(),
                self.selected_session_id.clone(),
                &self.message_counts,
            );

            cx.redraw_all();
        }
        OcEvent::PartUpdated { .. } => {
            // Current protocol does not include message id; ignore for now.
        }
        _ => {}
    }
}
```

**Step 2: Build and test**

Run: `cargo build && cargo run`
Expected: App runs, message counts increment when messages are received

**Step 3: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: track message counts via SSE events"
```

---

## Task 8: Add Hover States

**Files:**
- Modify: `openpad-app/src/app.rs` (live_design! SessionRow)

**Step 1: Add hover state to SessionRow background**

Update the `session_row_bg` View's `draw_bg`:

```rust
draw_bg: {
    instance hover: 0.0
    color: #1f2329
    uniform color_hover: #1f252c
    uniform border_color: #4a90e2
    uniform border_size: 3.0

    fn pixel(self) -> vec4 {
        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
        let bg = mix(self.color, self.color_hover, self.hover);

        // Left status bar
        sdf.rect(0.0, 0.0, self.border_size, self.rect_size.y);
        sdf.fill(self.border_color);

        // Background
        sdf.rect(self.border_size, 0.0, self.rect_size.x - self.border_size, self.rect_size.y);
        sdf.fill(bg);

        return sdf.result;
    }
}
```

**Step 2: Add animator for hover**

Add animator to `session_row_bg` View:

```rust
animator: {
    hover = {
        default: off
        off = {
            from: { all: Forward { duration: 0.15 } }
            apply: { draw_bg: { hover: 0.0 } }
        }
        on = {
            from: { all: Forward { duration: 0.15 } }
            apply: { draw_bg: { hover: 1.0 } }
        }
    }
}
```

**Step 3: Handle hover events in Widget impl**

Update `ProjectsPanel::handle_event` to track hover:

```rust
fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    let actions = cx.capture_actions(|cx| {
        self.view.handle_event(cx, event, scope);
    });

    for (item, widget) in &self.visible_items {
        match item {
            PanelItemKind::NewSession { project_id } => {
                if widget.button(id!(new_session_button)).clicked(&actions) {
                    cx.action(ProjectsPanelAction::CreateSession(project_id.clone()));
                }
            }
            PanelItemKind::SessionRow { session_id, .. } => {
                // Handle hover
                if widget.view(id!(session_row_bg)).finger_down(&actions).is_some() {
                    cx.action(ProjectsPanelAction::SelectSession(session_id.clone()));
                }

                // Animate hover
                match event.hits(cx, widget.view(id!(session_row_bg)).area()) {
                    Hit::FingerHoverIn(_) => {
                        widget.view(id!(session_row_bg)).animator_play(cx, id!(hover.on));
                    }
                    Hit::FingerHoverOut(_) => {
                        widget.view(id!(session_row_bg)).animator_play(cx, id!(hover.off));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
```

**Step 4: Build and test**

Run: `cargo build && cargo run`
Expected: Hovering over sessions shows subtle background change

**Step 5: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: add hover states to session rows"
```

---

## Task 9: Improve Status Bar Color Logic

**Files:**
- Modify: `openpad-app/src/app.rs` (draw_walk method)

**Step 1: Add helper to determine if session is recent**

Add helper function near other helpers:

```rust
/// Check if timestamp is within last 24 hours
fn is_recent(timestamp_ms: i64) -> bool {
    let now = Utc::now().timestamp_millis();
    let diff_ms = now - timestamp_ms;
    diff_ms < 24 * 60 * 60 * 1000 // 24 hours in milliseconds
}
```

**Step 2: Update status color logic in draw_walk**

Update the status color determination:

```rust
// Determine status color based on state
let status_color = if *is_archived {
    vec4(0.42, 0.48, 0.55, 1.0) // gray #6b7b8c
} else {
    // Check timestamp to see if recent
    // We need access to session.time.updated here
    // For now, default to blue, we'll refine in next step
    vec4(0.29, 0.56, 0.87, 1.0) // blue #4a90e2
};
```

**Step 3: Add timestamp to SessionRow data**

Update `PanelItemKind::SessionRow` to include raw timestamp:

```rust
SessionRow {
    session_id: String,
    title: String,
    timestamp: String,
    timestamp_ms: i64, // Add this
    is_archived: bool,
    is_shared: bool,
    is_forked: bool,
    file_changes: Option<(i64, i64, i64)>,
    message_count: usize,
},
```

**Step 4: Update rebuild_items to pass timestamp_ms**

In `rebuild_items`, add when creating `SessionRow`:

```rust
items.push(PanelItemKind::SessionRow {
    session_id: session.id.clone(),
    title,
    timestamp,
    timestamp_ms: session.time.updated, // Add this
    is_archived,
    is_shared,
    is_forked,
    file_changes,
    message_count,
});
```

**Step 5: Update draw_walk to use timestamp**

```rust
PanelItemKind::SessionRow {
    session_id,
    title,
    timestamp,
    timestamp_ms,
    is_archived,
    file_changes,
    message_count,
    ..
} => {
    // ... existing code ...

    // Determine status color based on state
    let status_color = if *is_archived {
        vec4(0.42, 0.48, 0.55, 1.0) // gray #6b7b8c
    } else if is_recent(*timestamp_ms) {
        vec4(0.23, 0.65, 0.36, 1.0) // green #3ba55d
    } else {
        vec4(0.29, 0.56, 0.87, 1.0) // blue #4a90e2
    };

    // ... rest of existing code ...
}
```

**Step 6: Build and test**

Run: `cargo build && cargo run`
Expected: Recent sessions (< 24h) show green bar, others blue, archived gray

**Step 7: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: dynamic status bar colors based on session state"
```

---

## Task 10: Add Status Icons Display

**Files:**
- Modify: `openpad-app/src/app.rs` (live_design! SessionRow)
- Create: `openpad-app/resources/icons/` (for SVG icons)

**Step 1: Create icon SVG files**

Note: For MVP, we'll use text symbols instead of SVG icons to keep it simple.

**Step 2: Update SessionRow to show status indicators**

Update the title row in SessionRow:

```rust
// Line 1: Title + icons
<View> {
    width: Fill, height: Fit
    flow: Right
    spacing: 6
    align: { y: 0.5 }

    session_title = <Label> {
        width: Fit
        draw_text: {
            color: #e6e9ee
            text_style: { font_size: 11 }
        }
    }

    // Status icons
    session_icons = <Label> {
        width: Fit
        draw_text: {
            color: #7a8591
            text_style: { font_size: 10 }
        }
    }

    <View> { width: Fill } // Spacer
}
```

**Step 3: Build icon string in draw_walk**

Update draw_walk to set icon text:

```rust
PanelItemKind::SessionRow {
    is_archived,
    is_shared,
    is_forked,
    ..
} => {
    // ... existing code ...

    // Build status icons string
    let mut icons = String::new();
    if *is_shared {
        icons.push_str("🔗 ");
    }
    if *is_forked {
        icons.push_str("↗ ");
    }
    if *is_archived {
        icons.push_str("📦");
    }

    item_widget.label(id!(session_icons)).set_text(cx, &icons);

    // ... rest of code ...
}
```

**Step 4: Build and test**

Run: `cargo build && cargo run`
Expected: Shared/forked/archived sessions show icons after title

**Step 5: Commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: display status icons for shared/forked/archived sessions"
```

---

## Task 11: Polish and Final Touches

**Files:**
- Modify: `openpad-app/src/app.rs`

**Step 1: Add separator line between sessions**

Update SessionRow to include a subtle bottom border:

```rust
SessionRow = <View> {
    width: Fill, height: Fit
    padding: 0
    show_bg: true
    draw_bg: {
        color: #0000
        uniform separator_color: #22262c
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            // Bottom separator line
            sdf.rect(0.0, self.rect_size.y - 1.0, self.rect_size.x, 1.0);
            sdf.fill(self.separator_color);
            return sdf.result;
        }
    }

    session_row_bg = <View> {
        // ... existing code ...
    }
}
```

**Step 2: Improve metadata formatting with color**

Update draw_walk to color file changes:

```rust
// Build metadata string with colored parts
let mut metadata_parts = vec![timestamp.clone()];

if let Some((additions, deletions, files)) = file_changes {
    // We can't color individual parts in a single Label easily
    // For MVP, keep plain text formatting
    metadata_parts.push(format!("+{} -{}", additions, deletions));
    metadata_parts.push(format!("{} files", files));
}

if *message_count > 0 {
    let msg_text = if *message_count == 1 {
        "1 message".to_string()
    } else {
        format!("{} messages", message_count)
    };
    metadata_parts.push(msg_text);
}

let metadata = metadata_parts.join(" • ");
item_widget.label(id!(session_metadata)).set_text(cx, &metadata);
```

**Step 3: Update selected state styling**

Improve selected state background color:

```rust
let bg_color = if selected {
    vec4(0.14, 0.16, 0.20, 1.0) // #242a32
} else {
    vec4(0.12, 0.14, 0.16, 1.0) // #1f2329
};

let title_color = if selected {
    vec4(1.0, 1.0, 1.0, 1.0) // #ffffff
} else {
    vec4(0.90, 0.91, 0.93, 1.0) // #e6e9ee
};

item_widget.label(id!(session_title)).apply_over(cx, live! {
    draw_text: { color: (title_color) }
});
```

**Step 4: Build and test full flow**

Run: `cargo build && cargo run`
Expected: Polished session list with all features working

**Step 5: Final commit**

```bash
git add openpad-app/src/app.rs
git commit -m "feat: polish session list UI with separators and improved styling"
```

---

## Task 12: Manual Testing & Verification

**Step 1: Test basic functionality**

1. Launch app: `cargo run`
2. Verify sessions display with meaningful titles
3. Check timestamps show relative time
4. Confirm file changes display when available
5. Test message count increments with new messages

**Step 2: Test interaction states**

1. Hover over sessions → background should lighten
2. Click session → should select and change background
3. Status bar should show correct color (green/blue/gray)

**Step 3: Test edge cases**

1. Session with no title/slug → shows "New session - {date}"
2. Session with no summary → metadata still shows timestamp
3. Empty message count → doesn't display in metadata
4. Multiple projects → proper grouping maintained

**Step 4: Document any issues**

Create issues for any bugs found.

**Step 5: Take screenshots**

Capture before/after screenshots for documentation.

---

## Success Criteria

- ✅ Sessions display meaningful titles instead of IDs
- ✅ Timestamps show relative time (e.g., "2m ago", "Yesterday")
- ✅ File change counts visible when summary exists
- ✅ Message counts tracked and displayed
- ✅ Status icons show for shared/forked/archived
- ✅ Hover states work smoothly
- ✅ Status bar colors indicate session state
- ✅ Selection state clearly visible
- ✅ Sessions sorted by update time
- ✅ Project grouping maintained

---

## Notes

- Message count tracking is client-side only - counts reset on app restart
- For production, consider persisting counts or fetching from API
- Future enhancement: fetch actual message counts from API on load
- Future enhancement: add hover action buttons (share, delete)
- Consider adding animations for row entrance (staggered fade-in)

---

## Rollback Plan

If issues arise:
```bash
git checkout master
git worktree remove .worktrees/session-list-redesign
```
