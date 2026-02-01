# Inline Diff & Activity Indicators Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add inline diff display in assistant message bubbles, replace modal permission dialog with inline permission cards, and add a working status banner with elapsed time.

**Architecture:** Three independent UI features wired into the existing Makepad widget tree and SSE event handling. The message list PortalList gains two new item templates (PermissionCard, WorkingBanner). Diffs are rendered as collapsible text blocks inside assistant messages using session summary data already in the protocol. No protocol changes needed.

**Tech Stack:** Rust, Makepad (live_design DSL, Widget trait), openpad-protocol types

---

### Task 1: Add Diff Theme Colors

**Files:**
- Modify: `openpad-widgets/src/theme.rs:82` (after accent red)

**Step 1: Add diff colors to theme**

Add these colors after line 82 (`THEME_COLOR_ACCENT_RED`):

```rust
    // Diff colors
    pub THEME_COLOR_DIFF_ADD_BG = #1a2e1a
    pub THEME_COLOR_DIFF_ADD_TEXT = #4ec94e
    pub THEME_COLOR_DIFF_DEL_BG = #2e1a1a
    pub THEME_COLOR_DIFF_DEL_TEXT = #e06060
    pub THEME_COLOR_DIFF_CONTEXT_TEXT = #888888
    pub THEME_COLOR_DIFF_HEADER_BG = #1a1f2e
```

**Step 2: Verify it compiles**

Run: `cargo check -p openpad-widgets`

**Step 3: Commit**

```bash
git add openpad-widgets/src/theme.rs
git commit -m "feat: add diff theme colors"
```

---

### Task 2: Create Inline Diff View Component

**Files:**
- Create: `openpad-app/src/components/diff_view.rs`
- Modify: `openpad-app/src/components/mod.rs`

**Step 1: Create diff_view.rs with widget**

Create `openpad-app/src/components/diff_view.rs`:

```rust
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::theme::*;

    // Single diff line
    DiffLine = <View> {
        width: Fill, height: Fit
        padding: { left: 8, right: 8, top: 1, bottom: 1 }

        prefix_label = <Label> {
            width: 16, height: Fit
            draw_text: {
                text_style: <THEME_FONT_CODE> { font_size: 8 }
                color: (THEME_COLOR_TEXT_MUTED)
            }
            text: " "
        }
        line_label = <Label> {
            width: Fill, height: Fit
            draw_text: {
                text_style: <THEME_FONT_CODE> { font_size: 8 }
                color: (THEME_COLOR_TEXT_NORMAL)
            }
            text: ""
        }
    }

    // Collapsible file diff section
    pub DiffFileSection = <View> {
        width: Fill, height: Fit
        flow: Down

        header = <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 6
            padding: { left: 8, right: 8, top: 4, bottom: 4 }
            show_bg: true
            draw_bg: { color: (THEME_COLOR_DIFF_HEADER_BG) }
            cursor: Hand
            align: { y: 0.5 }

            arrow_label = <Label> {
                width: Fit, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_CODE> { font_size: 8 }
                    color: (THEME_COLOR_TEXT_MUTED)
                }
                text: "▶"
            }
            file_label = <Label> {
                width: Fit, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_CODE> { font_size: 8 }
                    color: (THEME_COLOR_TEXT_NORMAL)
                }
                text: ""
            }
            stats_label = <Label> {
                width: Fit, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_CODE> { font_size: 8 }
                    color: (THEME_COLOR_TEXT_MUTED)
                }
                text: ""
            }
        }

        // Content area (hidden by default, toggled by header click)
        content = <View> {
            width: Fill, height: Fit
            flow: Down
            visible: false
            padding: { left: 4, right: 4, top: 2, bottom: 2 }
            show_bg: true
            draw_bg: { color: (THEME_COLOR_BG_DARKER) }

            diff_text = <Label> {
                width: Fill, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_CODE> { font_size: 8, line_spacing: 1.3 }
                    color: (THEME_COLOR_TEXT_NORMAL)
                    wrap: Word
                }
                text: ""
            }
        }
    }

    // Main diff container shown at bottom of assistant bubble
    pub DiffView = {{DiffView}} {
        width: Fill, height: Fit
        flow: Down
        visible: false
        margin: { top: 8 }

        summary_header = <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 6
            padding: { left: 8, right: 8, top: 6, bottom: 6 }
            show_bg: true
            draw_bg: {
                color: (THEME_COLOR_SHADE_2)
                border_radius: 4.0
            }
            cursor: Hand
            align: { y: 0.5 }

            arrow_label = <Label> {
                width: Fit, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 8 }
                    color: (THEME_COLOR_TEXT_MUTED)
                }
                text: "▶"
            }
            summary_label = <Label> {
                width: Fit, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 8 }
                    color: (THEME_COLOR_TEXT_DIM)
                }
                text: "Files Changed"
            }
        }

        files_container = <View> {
            width: Fill, height: Fit
            flow: Down
            visible: false
            margin: { top: 2 }

            diff_content = <Label> {
                width: Fill, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_CODE> { font_size: 8, line_spacing: 1.3 }
                    color: (THEME_COLOR_TEXT_NORMAL)
                    wrap: Word
                }
                text: ""
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DiffView {
    #[deref]
    view: View,
    #[rust]
    expanded: bool,
    #[rust]
    diff_text: String,
    #[rust]
    summary_text: String,
}

impl Widget for DiffView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Toggle expand/collapse on summary header click
        if let Event::MouseUp(e) = event {
            if e.was_tap() {
                let summary_header = self.view.view(&[id!(summary_header)]);
                if summary_header.area().rect(cx).contains(e.abs) {
                    self.expanded = !self.expanded;
                    let arrow = if self.expanded { "▼" } else { "▶" };
                    self.view
                        .label(&[id!(summary_header), id!(arrow_label)])
                        .set_text(cx, arrow);
                    self.view
                        .view(&[id!(files_container)])
                        .set_visible(cx, self.expanded);
                    self.redraw(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

/// Compute a simple unified diff from before/after text.
/// Returns formatted text with +/- prefixes.
pub fn compute_unified_diff(before: &str, after: &str, context_lines: usize) -> String {
    let before_lines: Vec<&str> = before.lines().collect();
    let after_lines: Vec<&str> = after.lines().collect();

    // Simple LCS-based diff
    let lcs = lcs_table(&before_lines, &after_lines);
    let mut changes = Vec::new();
    build_diff(&lcs, &before_lines, &after_lines, before_lines.len(), after_lines.len(), &mut changes);
    changes.reverse();

    if changes.is_empty() {
        return String::new();
    }

    // Build output with context
    let mut output = String::new();
    let mut last_printed = None;

    for (i, change) in changes.iter().enumerate() {
        match change {
            DiffChange::Equal(line) => {
                // Check if this line is within context_lines of a changed line
                let near_change = changes[i.saturating_sub(context_lines)..=(i + context_lines).min(changes.len() - 1)]
                    .iter()
                    .any(|c| !matches!(c, DiffChange::Equal(_)));

                if near_change {
                    if let Some(last) = last_printed {
                        if i > last + 1 {
                            output.push_str("  ...\n");
                        }
                    }
                    output.push_str(&format!("  {}\n", line));
                    last_printed = Some(i);
                }
            }
            DiffChange::Add(line) => {
                if let Some(last) = last_printed {
                    if i > last + 1 {
                        output.push_str("  ...\n");
                    }
                }
                output.push_str(&format!("+ {}\n", line));
                last_printed = Some(i);
            }
            DiffChange::Remove(line) => {
                if let Some(last) = last_printed {
                    if i > last + 1 {
                        output.push_str("  ...\n");
                    }
                }
                output.push_str(&format!("- {}\n", line));
                last_printed = Some(i);
            }
        }
    }

    output
}

enum DiffChange<'a> {
    Equal(&'a str),
    Add(&'a str),
    Remove(&'a str),
}

fn lcs_table(a: &[&str], b: &[&str]) -> Vec<Vec<usize>> {
    let m = a.len();
    let n = b.len();
    let mut table = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = table[i - 1][j].max(table[i][j - 1]);
            }
        }
    }
    table
}

fn build_diff<'a>(
    table: &[Vec<usize>],
    a: &[&'a str],
    b: &[&'a str],
    i: usize,
    j: usize,
    result: &mut Vec<DiffChange<'a>>,
) {
    if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
        build_diff(table, a, b, i - 1, j - 1, result);
        result.push(DiffChange::Equal(a[i - 1]));
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        build_diff(table, a, b, i, j - 1, result);
        result.push(DiffChange::Add(b[j - 1]));
    } else if i > 0 {
        build_diff(table, a, b, i - 1, j, result);
        result.push(DiffChange::Remove(a[i - 1]));
    }
}

/// Format multiple FileDiffs into a single display string
pub fn format_diffs_for_display(diffs: &[openpad_protocol::FileDiff]) -> (String, String) {
    let total_adds: i64 = diffs.iter().map(|d| d.additions).sum();
    let total_dels: i64 = diffs.iter().map(|d| d.deletions).sum();
    let summary = format!(
        "{} file{} changed  +{}  -{}",
        diffs.len(),
        if diffs.len() == 1 { "" } else { "s" },
        total_adds,
        total_dels
    );

    let mut full_diff = String::new();
    for diff in diffs {
        full_diff.push_str(&format!(
            "── {} (+{} -{}) ──\n",
            diff.file, diff.additions, diff.deletions
        ));
        let unified = compute_unified_diff(&diff.before, &diff.after, 3);
        if unified.is_empty() {
            full_diff.push_str("  (no text changes)\n");
        } else {
            full_diff.push_str(&unified);
        }
        full_diff.push('\n');
    }

    (summary, full_diff)
}

// Ref extension methods
pub trait DiffViewWidgetRefExt {
    fn set_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]);
    fn clear_diffs(&self, cx: &mut Cx);
}

impl DiffViewWidgetRefExt for WidgetRef {
    fn set_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]) {
        if diffs.is_empty() {
            self.set_visible(cx, false);
            return;
        }
        let (summary, full_diff) = format_diffs_for_display(diffs);
        if let Some(mut inner) = self.borrow_mut::<DiffView>() {
            inner.summary_text = summary.clone();
            inner.diff_text = full_diff.clone();
            inner.expanded = false;
        }
        self.set_visible(cx, true);
        self.label(&[id!(summary_header), id!(summary_label)])
            .set_text(cx, &summary);
        self.label(&[id!(summary_header), id!(arrow_label)])
            .set_text(cx, "▶");
        self.view(&[id!(files_container)])
            .set_visible(cx, false);
        self.label(&[id!(files_container), id!(diff_content)])
            .set_text(cx, &full_diff);
    }

    fn clear_diffs(&self, cx: &mut Cx) {
        self.set_visible(cx, false);
    }
}
```

**Step 2: Register module in mod.rs**

In `openpad-app/src/components/mod.rs`, add:

```rust
pub mod diff_view;
```

And add the use statement:

```rust
pub use diff_view::{DiffView, DiffViewWidgetRefExt};
```

**Step 3: Register live_design in app.rs**

In `openpad-app/src/app.rs` inside the `live_design!` block (around line 74), add:

```rust
    use crate::components::diff_view::DiffView;
```

And in the `register_widget!` / `live_design` call section (around line 554), add:

```rust
        crate::components::diff_view::live_design(cx);
```

**Step 4: Verify it compiles**

Run: `cargo check -p openpad-app`

**Step 5: Commit**

```bash
git add openpad-app/src/components/diff_view.rs openpad-app/src/components/mod.rs openpad-app/src/app.rs
git commit -m "feat: add DiffView component with unified diff algorithm"
```

---

### Task 3: Integrate DiffView into Assistant Messages

**Files:**
- Modify: `openpad-app/src/components/message_list.rs`
- Modify: `openpad-app/src/state/handlers.rs`

**Step 1: Add DiffView to AssistantMsg template**

In `openpad-app/src/components/message_list.rs`, inside the `AssistantMsg` template in `live_design!`, add a `DiffView` between the `error_label` (line 217) and `stats_row` (line 219):

After `error_label` closing brace (line 217), before `stats_row`:

```rust
                    diff_view = <DiffView> {}
```

**Step 2: Add diff data to DisplayMessage**

In the `DisplayMessage` struct (around line 289), add a new field:

```rust
    pub diffs: Vec<openpad_protocol::FileDiff>,
```

**Step 3: Populate diffs in rebuild_from_parts**

In `rebuild_from_parts`, we don't have session data, so initialize diffs as empty. The diffs will be set separately. In the `DisplayMessage` construction (around line 371), add:

```rust
                diffs: Vec::new(),
```

**Step 4: Add set_diffs method to MessageListRef**

Add a new method to the `impl MessageListWidgetRefExt` block (after `set_working`):

```rust
    fn set_session_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]);
```

Implementation:

```rust
    fn set_session_diffs(&self, cx: &mut Cx, diffs: &[openpad_protocol::FileDiff]) {
        if let Some(mut inner) = self.borrow_mut() {
            // Apply diffs to the last assistant message
            if let Some(last_assistant) = inner.messages.iter_mut().rev().find(|m| m.role == "assistant") {
                last_assistant.diffs = diffs.to_vec();
            }
            inner.redraw(cx);
        }
    }
```

**Step 5: Render diffs in draw_walk**

In the `draw_walk` method, inside the assistant message rendering block (around line 473-523), after setting revert button visibility (line 521), add:

```rust
                            // Set diff view data
                            use crate::components::diff_view::DiffViewWidgetRefExt;
                            if msg.diffs.is_empty() {
                                item_widget
                                    .widget(&[id!(diff_view)])
                                    .clear_diffs(cx);
                            } else {
                                item_widget
                                    .widget(&[id!(diff_view)])
                                    .set_diffs(cx, &msg.diffs);
                            }
```

**Step 6: Wire up SessionDiffLoaded to message list**

In `openpad-app/src/state/handlers.rs`, in the `SessionDiffLoaded` handler (around line 370), after `state.update_session_meta_ui(ui, cx)`, add:

```rust
            // Also update inline diffs in message list
            ui.message_list(&[id!(message_list)])
                .set_session_diffs(cx, &diffs);
```

Also when a session update comes in with a summary that has diffs, pass them through. In `handle_session_updated` or the `SessionUpdated` event handler, after the session is stored, check if it has diffs and update the message list.

**Step 7: Verify it compiles**

Run: `cargo check -p openpad-app`

**Step 8: Commit**

```bash
git add openpad-app/src/components/message_list.rs openpad-app/src/state/handlers.rs
git commit -m "feat: integrate inline diff view into assistant message bubbles"
```

---

### Task 4: Create Inline Permission Card

**Files:**
- Create: `openpad-app/src/components/permission_card.rs`
- Modify: `openpad-app/src/components/mod.rs`

**Step 1: Create permission_card.rs**

Create `openpad-app/src/components/permission_card.rs`:

```rust
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use openpad_widgets::theme::*;

    pub PermissionCard = {{PermissionCard}} {
        width: Fill, height: Fit
        flow: Down
        padding: { left: 24, right: 100, top: 4, bottom: 4 }

        <RoundedView> {
            width: Fill, height: Fit
            flow: Down
            padding: { left: 14, right: 14, top: 10, bottom: 10 }
            spacing: 6,
            show_bg: true
            draw_bg: {
                color: (THEME_COLOR_SHADE_1)
                border_radius: 8.0
                border_size: 1.0
                border_color: (THEME_COLOR_ACCENT_AMBER)
            }

            // Header row
            <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 6
                align: { y: 0.5 }

                type_label = <Label> {
                    width: Fit, height: Fit
                    draw_text: {
                        text_style: <THEME_FONT_BOLD> { font_size: 9 }
                        color: (THEME_COLOR_ACCENT_AMBER)
                    }
                    text: "Permission Request"
                }

                status_label = <Label> {
                    width: Fit, height: Fit
                    visible: false
                    draw_text: {
                        text_style: <THEME_FONT_BOLD> { font_size: 8 }
                        color: (THEME_COLOR_TEXT_MUTED)
                    }
                    text: ""
                }
            }

            // Permission name
            permission_label = <Label> {
                width: Fill, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                    color: (THEME_COLOR_TEXT_NORMAL)
                    wrap: Word
                }
                text: ""
            }

            // Pattern/details
            pattern_label = <Label> {
                width: Fill, height: Fit
                draw_text: {
                    text_style: <THEME_FONT_CODE> { font_size: 8 }
                    color: (THEME_COLOR_TEXT_MUTED_LIGHTER)
                    wrap: Word
                }
                text: ""
            }

            // Action buttons row
            buttons_row = <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 8
                margin: { top: 4 }

                approve_button = <Button> {
                    width: Fit, height: 24
                    text: "Approve"
                    padding: { left: 12, right: 12 }
                    draw_bg: {
                        color: (THEME_COLOR_ACCENT_BLUE)
                        color_hover: (THEME_COLOR_ACCENT_BLUE_HOVER)
                        border_radius: 4.0
                    }
                    draw_text: {
                        color: (THEME_COLOR_TEXT_BRIGHT)
                        text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                    }
                }

                always_button = <Button> {
                    width: Fit, height: 24
                    text: "Always"
                    padding: { left: 12, right: 12 }
                    draw_bg: {
                        color: (THEME_COLOR_BG_BUTTON)
                        color_hover: (THEME_COLOR_BG_BUTTON_HOVER)
                        border_radius: 4.0
                    }
                    draw_text: {
                        color: (THEME_COLOR_TEXT_DIM)
                        text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                    }
                }

                reject_button = <Button> {
                    width: Fit, height: 24
                    text: "Deny"
                    padding: { left: 12, right: 12 }
                    draw_bg: {
                        color: (THEME_COLOR_TRANSPARENT)
                        color_hover: (THEME_COLOR_HOVER_MEDIUM)
                        border_radius: 4.0
                        border_size: 1.0
                        border_color: (THEME_COLOR_BORDER_LIGHT)
                    }
                    draw_text: {
                        color: (THEME_COLOR_TEXT_MUTED)
                        text_style: <THEME_FONT_REGULAR> { font_size: 9 }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum PermissionCardAction {
    None,
    Approved {
        session_id: String,
        request_id: String,
    },
    AlwaysApproved {
        session_id: String,
        request_id: String,
    },
    Rejected {
        session_id: String,
        request_id: String,
    },
}

#[derive(Live, LiveHook, Widget)]
pub struct PermissionCard {
    #[deref]
    view: View,
    #[rust]
    session_id: String,
    #[rust]
    request_id: String,
    #[rust]
    resolved: bool,
}

impl Widget for PermissionCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if self.resolved {
            return;
        }

        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(&[id!(approve_button)]).clicked(&actions) {
            cx.action(PermissionCardAction::Approved {
                session_id: self.session_id.clone(),
                request_id: self.request_id.clone(),
            });
            self.mark_resolved(cx, "Approved");
        }

        if self.view.button(&[id!(always_button)]).clicked(&actions) {
            cx.action(PermissionCardAction::AlwaysApproved {
                session_id: self.session_id.clone(),
                request_id: self.request_id.clone(),
            });
            self.mark_resolved(cx, "Always Approved");
        }

        if self.view.button(&[id!(reject_button)]).clicked(&actions) {
            cx.action(PermissionCardAction::Rejected {
                session_id: self.session_id.clone(),
                request_id: self.request_id.clone(),
            });
            self.mark_resolved(cx, "Denied");
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl PermissionCard {
    fn mark_resolved(&mut self, cx: &mut Cx, status: &str) {
        self.resolved = true;
        self.view.view(&[id!(buttons_row)]).set_visible(cx, false);
        self.view.label(&[id!(status_label)]).set_text(cx, status);
        self.view
            .label(&[id!(status_label)])
            .set_visible(cx, true);
        self.redraw(cx);
    }
}

pub trait PermissionCardWidgetRefExt {
    fn set_permission(
        &self,
        cx: &mut Cx,
        session_id: String,
        request_id: String,
        permission: &str,
        patterns: &[String],
    );
}

impl PermissionCardWidgetRefExt for WidgetRef {
    fn set_permission(
        &self,
        cx: &mut Cx,
        session_id: String,
        request_id: String,
        permission: &str,
        patterns: &[String],
    ) {
        if let Some(mut inner) = self.borrow_mut::<PermissionCard>() {
            inner.session_id = session_id;
            inner.request_id = request_id;
            inner.resolved = false;
        }
        self.label(&[id!(permission_label)])
            .set_text(cx, permission);
        self.label(&[id!(pattern_label)])
            .set_text(cx, &patterns.join("\n"));
        self.view(&[id!(buttons_row)]).set_visible(cx, true);
        self.label(&[id!(status_label)]).set_visible(cx, false);
    }
}
```

**Step 2: Register in mod.rs**

Add to `openpad-app/src/components/mod.rs`:

```rust
pub mod permission_card;
pub use permission_card::{PermissionCard, PermissionCardAction, PermissionCardWidgetRefExt};
```

**Step 3: Register live_design in app.rs**

In the live_design! block add the use, and in the register section add:

```rust
        crate::components::permission_card::live_design(cx);
```

**Step 4: Verify it compiles**

Run: `cargo check -p openpad-app`

**Step 5: Commit**

```bash
git add openpad-app/src/components/permission_card.rs openpad-app/src/components/mod.rs openpad-app/src/app.rs
git commit -m "feat: add inline PermissionCard widget"
```

---

### Task 5: Integrate Permission Cards into Message List

**Files:**
- Modify: `openpad-app/src/components/message_list.rs`
- Modify: `openpad-app/src/state/handlers.rs`
- Modify: `openpad-app/src/state/actions.rs`

**Step 1: Add PermissionCard template to PortalList**

In the `live_design!` of `message_list.rs`, add a third template to the PortalList (after `AssistantMsg`, before closing brace of PortalList):

```rust
            PermissionMsg = <PermissionCard> {}
```

Also add the use import at top of live_design:

```rust
    use crate::components::permission_card::PermissionCard;
```

**Step 2: Add display item enum**

Replace the simple `Vec<DisplayMessage>` with a mixed item list. Add a new enum:

```rust
#[derive(Clone, Debug)]
pub enum DisplayItem {
    Message(DisplayMessage),
    Permission {
        session_id: String,
        request_id: String,
        permission: String,
        patterns: Vec<String>,
        resolved: bool,
        resolution: Option<String>,
    },
}
```

Update the `MessageList` struct to use `Vec<DisplayItem>` instead of `Vec<DisplayMessage>` for `messages`.

Actually, simpler approach: keep `messages: Vec<DisplayMessage>` and add a separate `permissions: Vec<PendingPermissionDisplay>` list. Then interleave in draw_walk by rendering permissions after the last assistant message.

Even simpler: add permissions as items in the PortalList by computing a combined item list in draw_walk.

Let me keep it simple. Add a field to MessageList:

```rust
    #[rust]
    pending_permissions: Vec<PendingPermissionDisplay>,
```

Where:

```rust
#[derive(Clone, Debug)]
pub struct PendingPermissionDisplay {
    pub session_id: String,
    pub request_id: String,
    pub permission: String,
    pub patterns: Vec<String>,
}
```

**Step 3: Update draw_walk to render permission items**

In `draw_walk`, the total items becomes:

```rust
let total_items = self.messages.len()
    + self.pending_permissions.len()
    + if self.is_working { 1 } else { 0 };
```

When drawing item_id >= messages.len() and < messages.len() + pending_permissions.len(), use the `PermissionMsg` template.

**Step 4: Add MessageList methods for permissions**

```rust
    fn set_pending_permissions(&self, cx: &mut Cx, permissions: &[PendingPermissionDisplay]);
    fn remove_permission(&self, cx: &mut Cx, request_id: &str);
```

**Step 5: Wire handlers to push permissions into message list**

In `handlers.rs`, change `show_next_pending_permission` to push to the message list instead of showing the modal dialog:

```rust
fn show_next_pending_permission(state: &mut AppState, ui: &WidgetRef, cx: &mut Cx) {
    let Some(current_session_id) = &state.current_session_id else {
        return;
    };

    let displays: Vec<_> = state
        .pending_permissions
        .iter()
        .filter(|p| &p.session_id == current_session_id)
        .map(|p| PendingPermissionDisplay {
            session_id: p.session_id.clone(),
            request_id: p.id.clone(),
            permission: p.permission.clone(),
            patterns: p.patterns.clone(),
        })
        .collect();

    ui.message_list(&[id!(message_list)])
        .set_pending_permissions(cx, &displays);
}
```

**Step 6: Handle PermissionCardAction in app.rs**

In `handle_actions`, add handling for `PermissionCardAction`:

```rust
if let Some(action) = action.downcast_ref::<PermissionCardAction>() {
    match action {
        PermissionCardAction::Approved { session_id, request_id } => {
            // Same logic as current permission dialog approve
        }
        PermissionCardAction::AlwaysApproved { session_id, request_id } => {
            // Same logic as current permission dialog always
        }
        PermissionCardAction::Rejected { session_id, request_id } => {
            // Same logic as current permission dialog reject
        }
        _ => {}
    }
}
```

**Step 7: Verify it compiles**

Run: `cargo check -p openpad-app`

**Step 8: Commit**

```bash
git add openpad-app/src/components/message_list.rs openpad-app/src/state/handlers.rs openpad-app/src/app.rs
git commit -m "feat: integrate inline permission cards into message list"
```

---

### Task 6: Add Working Status Banner

**Files:**
- Modify: `openpad-app/src/components/message_list.rs`
- Modify: `openpad-app/src/state/handlers.rs`

**Step 1: Improve the "Thinking..." working indicator**

Currently the working indicator shows "Thinking..." as a fake assistant message. Replace it with a more informative status banner. In `draw_walk`, in the working indicator section (lines 425-443), change to:

```rust
                    if item_id >= self.messages.len() + self.pending_permissions.len() {
                        if !self.is_working {
                            continue;
                        }
                        // Working status banner
                        let item_widget = list.item(cx, item_id, live_id!(AssistantMsg));
                        let elapsed = self.working_since
                            .map(|t| t.elapsed().as_secs())
                            .unwrap_or(0);
                        let mins = elapsed / 60;
                        let secs = elapsed % 60;
                        let status_text = format!("Agent working... {}:{:02} elapsed", mins, secs);
                        item_widget
                            .widget(&[id!(msg_text)])
                            .set_text(cx, &status_text);
                        item_widget.label(&[id!(model_label)])
                            .set_text(cx, "");
                        item_widget.label(&[id!(timestamp_label)]).set_text(cx, "");
                        item_widget.label(&[id!(revert_label)]).set_visible(cx, false);
                        item_widget.label(&[id!(error_label)]).set_text(cx, "");
                        item_widget.view(&[id!(stats_row)]).set_visible(cx, false);
                        item_widget.view(&[id!(msg_actions)]).set_visible(cx, false);
                        item_widget.draw_all(cx, scope);
                    }
```

**Step 2: Add working_since field**

Add to `MessageList` struct:

```rust
    #[rust]
    working_since: Option<std::time::Instant>,
```

**Step 3: Update set_working to track time**

In the `set_working` method, track when work started:

```rust
    fn set_working(&self, cx: &mut Cx, working: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_working = working;
            if working && inner.working_since.is_none() {
                inner.working_since = Some(std::time::Instant::now());
            } else if !working {
                inner.working_since = None;
            }
            inner.redraw(cx);
        }
    }
```

**Step 4: Add timer for elapsed time updates**

To update the elapsed time display, we need periodic redraws while working. In `handle_event`, add a timer:

```rust
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        // ... existing code ...

        // Tick timer for working indicator
        if self.is_working {
            if let Event::NextFrame(_) = event {
                self.redraw(cx);
                cx.start_next_frame();
            }
            cx.start_next_frame();
        }
    }
```

**Step 5: Verify it compiles**

Run: `cargo check -p openpad-app`

**Step 6: Commit**

```bash
git add openpad-app/src/components/message_list.rs openpad-app/src/state/handlers.rs
git commit -m "feat: add working status banner with elapsed time"
```

---

### Task 7: Remove Modal Permission Dialog

**Files:**
- Modify: `openpad-app/src/app.rs` — remove `permission_dialog` from layout
- Modify: `openpad-app/src/state/handlers.rs` — remove all `permission_dialog` widget refs
- Keep: `openpad-app/src/components/permission_dialog.rs` — keep file for now but remove from layout, in case we need to reference it

**Step 1: Remove from app.rs layout**

In `openpad-app/src/app.rs`, remove line 187:

```rust
            permission_dialog = <PermissionDialog> { width: Fill }
```

**Step 2: Remove permission_dialog widget refs from handlers.rs**

In `handlers.rs`, remove all lines that reference `ui.permission_dialog(...)`. The permission flow is now handled entirely through the message list. Specifically:

- Remove the `use crate::components::permission_dialog::PermissionDialogWidgetRefExt;` import (line 3)
- In `PermissionReplied` handler (around line 480-489), remove the dialog hide logic
- In `show_next_pending_permission` (line 684-714), replace with the new message-list-based version from Task 5
- In `handle_permission_responded` (line 501-508), only call `remove_pending_permission` and `show_next_pending_permission`

**Step 3: Remove permission_dialog refs from app.rs handle_actions**

Remove lines that call `.permission_dialog(...)` in app.rs (around lines 858, 1309, 1394).

**Step 4: Verify it compiles**

Run: `cargo check -p openpad-app`

**Step 5: Commit**

```bash
git add openpad-app/src/app.rs openpad-app/src/state/handlers.rs
git commit -m "refactor: remove modal permission dialog in favor of inline cards"
```

---

### Task 8: Full Build and Manual Test

**Step 1: Full build**

Run: `cargo build -p openpad-app`

Fix any compilation errors.

**Step 2: Manual testing checklist**

- [ ] App launches without errors
- [ ] Sending a message shows streaming text appearing incrementally
- [ ] Working indicator shows elapsed time while agent works
- [ ] Permission requests appear as inline cards in the chat
- [ ] Approve/Deny/Always buttons on permission cards work correctly
- [ ] After agent completes, diff summary appears in last assistant message
- [ ] Clicking diff summary expands to show unified diff
- [ ] Session switching clears diffs and permissions properly

**Step 3: Commit any fixes**

```bash
git add -A
git commit -m "fix: resolve compilation and integration issues"
```
