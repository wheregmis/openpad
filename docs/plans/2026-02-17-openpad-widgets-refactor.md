# openpad-widgets Refactoring Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Eliminate four code-quality issues in `openpad-widgets`: repetitive indexed match arms, dead markdown branch, duplicated `DisplayMessage` construction, and unreadable single-line DSL.

**Architecture:** Four independent tasks in dependency order — const ID arrays first (used by render/events refactors), then DisplayMessage::Default, then markdown enable, then DSL formatting last (purely cosmetic). Each task is independently compilable and committable.

**Tech Stack:** Rust, Makepad (live_id! macro generates const-compatible LiveId(u64) literals)

---

## Task 1: Add const ID arrays to model.rs

**Files:**
- Modify: `openpad-widgets/src/message_list/model.rs`

### Step 1: Add STEP_ROW and TOOL_ROW const arrays

Open `openpad-widgets/src/message_list/model.rs` and add the following after line 28 (after the `pub(super) const MAX_STEP_ROWS: usize = 10;` line):

```rust
/// Pre-computed LiveId tuples for step rows. Eliminates match arms in render/events.
/// Order: (row, header, body, content, dot, line)
pub(super) const STEP_ROW: [(LiveId, LiveId, LiveId, LiveId, LiveId, LiveId); 10] = [
    (live_id!(step_row_0), live_id!(step_row_0_header), live_id!(step_row_0_body), live_id!(step_row_0_content), live_id!(step_row_0_dot), live_id!(step_row_0_line)),
    (live_id!(step_row_1), live_id!(step_row_1_header), live_id!(step_row_1_body), live_id!(step_row_1_content), live_id!(step_row_1_dot), live_id!(step_row_1_line)),
    (live_id!(step_row_2), live_id!(step_row_2_header), live_id!(step_row_2_body), live_id!(step_row_2_content), live_id!(step_row_2_dot), live_id!(step_row_2_line)),
    (live_id!(step_row_3), live_id!(step_row_3_header), live_id!(step_row_3_body), live_id!(step_row_3_content), live_id!(step_row_3_dot), live_id!(step_row_3_line)),
    (live_id!(step_row_4), live_id!(step_row_4_header), live_id!(step_row_4_body), live_id!(step_row_4_content), live_id!(step_row_4_dot), live_id!(step_row_4_line)),
    (live_id!(step_row_5), live_id!(step_row_5_header), live_id!(step_row_5_body), live_id!(step_row_5_content), live_id!(step_row_5_dot), live_id!(step_row_5_line)),
    (live_id!(step_row_6), live_id!(step_row_6_header), live_id!(step_row_6_body), live_id!(step_row_6_content), live_id!(step_row_6_dot), live_id!(step_row_6_line)),
    (live_id!(step_row_7), live_id!(step_row_7_header), live_id!(step_row_7_body), live_id!(step_row_7_content), live_id!(step_row_7_dot), live_id!(step_row_7_line)),
    (live_id!(step_row_8), live_id!(step_row_8_header), live_id!(step_row_8_body), live_id!(step_row_8_content), live_id!(step_row_8_dot), live_id!(step_row_8_line)),
    (live_id!(step_row_9), live_id!(step_row_9_header), live_id!(step_row_9_body), live_id!(step_row_9_content), live_id!(step_row_9_dot), live_id!(step_row_9_line)),
];

/// Pre-computed LiveId tuples for tool rows. Eliminates match arms in render.
/// Order: (row, icon, name, input)
pub(super) const TOOL_ROW: [(LiveId, LiveId, LiveId, LiveId); 5] = [
    (live_id!(tool_row_0), live_id!(tool_icon_0), live_id!(tool_name_0), live_id!(tool_input_0)),
    (live_id!(tool_row_1), live_id!(tool_icon_1), live_id!(tool_name_1), live_id!(tool_input_1)),
    (live_id!(tool_row_2), live_id!(tool_icon_2), live_id!(tool_name_2), live_id!(tool_input_2)),
    (live_id!(tool_row_3), live_id!(tool_icon_3), live_id!(tool_name_3), live_id!(tool_input_3)),
    (live_id!(tool_row_4), live_id!(tool_icon_4), live_id!(tool_name_4), live_id!(tool_input_4)),
];
```

> **Note on const:** `live_id!(foo)` expands to `LiveId(hash)` where `hash` is a `u64` literal — const-compatible. If the compiler rejects `const`, change `const` to `static`.

### Step 2: Verify it compiles

```bash
cargo build -p openpad-widgets 2>&1 | head -30
```

Expected: no errors. If you see `error[E0015]: cannot call non-const fn`, change `const` to `static` for both arrays.

### Step 3: Run existing tests

```bash
cargo test -p openpad-widgets 2>&1
```

Expected: all tests pass (the only test is `tail_mode_transitions_*`).

### Step 4: Commit

```bash
git add openpad-widgets/src/message_list/model.rs
git commit -m "refactor: add const STEP_ROW and TOOL_ROW ID arrays to model.rs"
```

---

## Task 2: Replace match arms in render.rs

**Files:**
- Modify: `openpad-widgets/src/message_list/render.rs`

### Step 1: Replace TOOL_ROW match block

Find this block in `render.rs` (around line 93–125):

```rust
let (row_id, icon_id, name_id, input_id) = match idx {
    0 => (
        live_id!(tool_row_0),
        live_id!(tool_icon_0),
        live_id!(tool_name_0),
        live_id!(tool_input_0),
    ),
    1 => (
        live_id!(tool_row_1),
        ...
    ),
    // ... through 4
    _ => continue,
};
```

Replace the entire `match idx { ... }` block with:

```rust
let Some(&(row_id, icon_id, name_id, input_id)) = TOOL_ROW.get(idx) else {
    continue;
};
```

Also replace the "hide unused tool rows" block (around line 132–145):

```rust
for idx in running_tools.map(|t| t.len()).unwrap_or(0)..5 {
    let row_id = match idx {
        0 => live_id!(tool_row_0),
        1 => live_id!(tool_row_1),
        2 => live_id!(tool_row_2),
        3 => live_id!(tool_row_3),
        4 => live_id!(tool_row_4),
        _ => continue,
    };
    item_widget
        .view(cx, &[id!(thinking_tools)])
        .view(cx, &[row_id])
        .set_visible(cx, false);
}
```

Replace with:

```rust
let shown = running_tools.map(|t| t.len()).unwrap_or(0);
for &(row_id, _, _, _) in TOOL_ROW.iter().skip(shown) {
    item_widget
        .view(cx, &[id!(thinking_tools)])
        .view(cx, &[row_id])
        .set_visible(cx, false);
}
```

### Step 2: Replace STEP_ROW match block in render.rs

Find the `match step_id` block inside `if has_steps && msg.show_steps` (around line 291–373):

```rust
let (row_id, header_id, body_id, content_id, dot_id, line_id) =
    match step_id {
        0 => (
            live_id!(step_row_0),
            live_id!(step_row_0_header),
            ...
        ),
        // ... through 9
        _ => continue,
    };
```

Replace the `match step_id { ... }` block with:

```rust
let Some(&(row_id, header_id, body_id, content_id, dot_id, line_id)) = STEP_ROW.get(step_id) else {
    continue;
};
```

### Step 3: Verify it compiles

```bash
cargo build -p openpad-widgets 2>&1 | head -30
```

Expected: no errors. Check that `STEP_ROW` and `TOOL_ROW` are in scope (they come from `super::*` via `use super::*` at the top of render.rs).

### Step 4: Run tests

```bash
cargo test -p openpad-widgets 2>&1
```

Expected: all pass.

### Step 5: Commit

```bash
git add openpad-widgets/src/message_list/render.rs
git commit -m "refactor: replace match arms with TOOL_ROW/STEP_ROW array lookup in render.rs"
```

---

## Task 3: Replace match arms in events.rs

**Files:**
- Modify: `openpad-widgets/src/message_list/events.rs`

### Step 1: Replace STEP_ROW match block

Find this block in `events.rs` (around line 80–91):

```rust
let (row_id, header_id) = match step_id {
    0 => (live_id!(step_row_0), live_id!(step_row_0_header)),
    1 => (live_id!(step_row_1), live_id!(step_row_1_header)),
    2 => (live_id!(step_row_2), live_id!(step_row_2_header)),
    3 => (live_id!(step_row_3), live_id!(step_row_3_header)),
    4 => (live_id!(step_row_4), live_id!(step_row_4_header)),
    5 => (live_id!(step_row_5), live_id!(step_row_5_header)),
    6 => (live_id!(step_row_6), live_id!(step_row_6_header)),
    7 => (live_id!(step_row_7), live_id!(step_row_7_header)),
    8 => (live_id!(step_row_8), live_id!(step_row_8_header)),
    9 => (live_id!(step_row_9), live_id!(step_row_9_header)),
    _ => continue,
};
```

Replace with (note: we only need the first two elements of the tuple):

```rust
let Some(&(row_id, header_id, _, _, _, _)) = STEP_ROW.get(step_id) else {
    continue;
};
```

### Step 2: Verify it compiles and tests pass

```bash
cargo build -p openpad-widgets 2>&1 | head -30
cargo test -p openpad-widgets 2>&1
```

Expected: clean build, all tests pass.

### Step 3: Commit

```bash
git add openpad-widgets/src/message_list/events.rs
git commit -m "refactor: replace match arms with STEP_ROW array lookup in events.rs"
```

---

## Task 4: Add DisplayMessage::Default and use it everywhere

**Files:**
- Modify: `openpad-widgets/src/message_logic.rs`
- Modify: `openpad-widgets/src/message_list/api.rs`

### Step 1: Add Default impl to message_logic.rs

Open `openpad-widgets/src/message_logic.rs`. After the `DisplayMessage` struct definition (after line 130), add:

```rust
impl Default for DisplayMessage {
    fn default() -> Self {
        Self {
            role: String::new(),
            text: String::new(),
            message_id: None,
            timestamp: None,
            model_id: None,
            tokens: None,
            cost: None,
            error_text: None,
            is_error: false,
            diffs: Vec::new(),
            show_diffs: false,
            steps: Vec::new(),
            show_steps: false,
            duration_ms: None,
            cached_steps_summary: String::new(),
            cached_grouped_summary: String::new(),
            cached_tool_groups: Vec::new(),
            cached_needs_markdown: false,
            cached_thinking_activity: String::new(),
            cached_running_tools: Vec::new(),
            cached_timestamp: String::new(),
            cached_token_usage: String::new(),
            cached_cost: String::new(),
            cached_full_diff: String::new(),
            cached_diff_files: String::new(),
            cached_diff_add: String::new(),
            cached_diff_del: String::new(),
        }
    }
}
```

### Step 2: Verify it compiles

```bash
cargo build -p openpad-widgets 2>&1 | head -20
```

### Step 3: Update construction sites in message_logic.rs

There are approximately 4 construction sites in `message_logic.rs` (inside `rebuild_from_parts`). Each looks like:

```rust
let mut msg = DisplayMessage {
    role: role.to_string(),
    text: String::new(),       // ← these are the "default" fields
    message_id: Some(message_id),
    timestamp,
    model_id,
    tokens,
    cost,
    error_text: None,
    is_error: false,
    diffs: Vec::new(),
    show_diffs: false,
    steps,
    show_steps: true,
    duration_ms,
    cached_steps_summary: String::new(),
    cached_grouped_summary: String::new(),
    cached_tool_groups: Vec::new(),
    cached_needs_markdown: false,
    cached_thinking_activity: String::new(),
    cached_running_tools: Vec::new(),
    cached_timestamp: String::new(),
    cached_token_usage: String::new(),
    cached_cost: String::new(),
    cached_full_diff: String::new(),
    cached_diff_files: String::new(),
    cached_diff_add: String::new(),
    cached_diff_del: String::new(),
};
```

Replace each with only the non-default fields, plus `..DisplayMessage::default()`. For example, the `pending_steps_only` construction becomes:

```rust
let mut msg = DisplayMessage {
    role: role.to_string(),
    message_id: Some(message_id),
    timestamp,
    model_id,
    tokens,
    cost,
    steps,
    show_steps: true,
    duration_ms,
    ..DisplayMessage::default()
};
```

Repeat for all construction sites. Fields that match the default value (`None`, `Vec::new()`, `String::new()`, `false`) can be dropped. Fields with non-default values must stay explicit.

### Step 4: Update construction sites in api.rs

Same pattern for the ~3 construction sites in `openpad-widgets/src/message_list/api.rs`. Each `DisplayMessage { ... }` literal with all the `cached_*` fields becomes a compact struct literal with `..DisplayMessage::default()`.

For example, in `append_text_for_message`:

```rust
let mut msg = DisplayMessage {
    role: role.to_string(),
    text: text.to_string(),
    message_id: Some(message_id.to_string()),
    ..DisplayMessage::default()
};
```

### Step 5: Verify compile and tests

```bash
cargo build -p openpad-widgets 2>&1 | head -20
cargo test -p openpad-widgets 2>&1
```

Expected: clean build, all tests pass.

### Step 6: Commit

```bash
git add openpad-widgets/src/message_logic.rs openpad-widgets/src/message_list/api.rs
git commit -m "refactor: add DisplayMessage::Default, use ..default() at all construction sites"
```

---

## Task 5: Enable Markdown rendering in render.rs

**Files:**
- Modify: `openpad-widgets/src/message_list/render.rs`

### Step 1: Write a test for compute_needs_markdown

The test should live in `openpad-widgets/src/message_logic.rs` in a `#[cfg(test)]` block at the bottom of the file. Add:

```rust
#[cfg(test)]
mod tests {
    use super::MessageProcessor;

    #[test]
    fn needs_markdown_detects_code_blocks() {
        assert!(MessageProcessor::compute_needs_markdown("hello ```rust\ncode\n``` world"));
    }

    #[test]
    fn needs_markdown_detects_inline_code() {
        assert!(MessageProcessor::compute_needs_markdown("use `cargo build`"));
    }

    #[test]
    fn needs_markdown_detects_heading() {
        assert!(MessageProcessor::compute_needs_markdown("# Title\nsome text"));
    }

    #[test]
    fn needs_markdown_false_for_plain_text() {
        assert!(!MessageProcessor::compute_needs_markdown("hello world, no markdown here"));
    }
}
```

### Step 2: Run the tests to verify they pass

```bash
cargo test -p openpad-widgets -- message_logic 2>&1
```

Expected: all 4 pass (the function already exists and is correct).

### Step 3: Enable the markdown branch

In `openpad-widgets/src/message_list/render.rs`, find line 171:

```rust
let use_markdown = false;
```

Replace with:

```rust
let use_markdown = msg.cached_needs_markdown;
```

### Step 4: Verify compile

```bash
cargo build -p openpad-widgets 2>&1 | head -20
```

Expected: clean build.

### Step 5: Run all tests

```bash
cargo test -p openpad-widgets 2>&1
```

Expected: all tests pass.

### Step 6: Commit

```bash
git add openpad-widgets/src/message_list/render.rs openpad-widgets/src/message_logic.rs
git commit -m "feat: enable markdown rendering for messages with markdown syntax"
```

---

## Task 6: Reformat DSL single-line blobs in mod.rs

**Files:**
- Modify: `openpad-widgets/src/message_list/mod.rs`

This is a pure formatting task — no logic changes. The goal is to break the monster single-line widget definitions into readable multi-line blocks.

### Step 1: Reformat tool_row_0..4 (lines ~194–199)

Each current line looks like:
```
tool_row_0 := View { visible: false, width: Fill, height: Fit, flow: Right, spacing: 6, align: Align{ y: 0.5 }, tool_icon_0 := Label { ... }, tool_name_0 := Label { ... }, tool_input_0 := Label { ... } }
```

Reformat to:

```
tool_row_0 := View {
    visible: false
    width: Fill, height: Fit
    flow: Right, spacing: 6
    align: Align{ y: 0.5 }

    tool_icon_0 := Label {
        width: Fit, height: Fit
        draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
        text: ""
    }
    tool_name_0 := Label {
        width: Fit, height: Fit
        draw_text +: { color: #d2dae8, text_style: theme.font_bold { font_size: 9 } }
        text: ""
    }
    tool_input_0 := Label {
        width: Fill, height: Fit
        draw_text +: { color: #b8c2d3, text_style: theme.font_regular { font_size: 9 } }
        text: ""
    }
}
```

Do this for `tool_row_0` through `tool_row_4`.

### Step 2: Reformat step_row_0..9 (lines ~316–325)

Each current line looks like:
```
step_row_0 := View { width: Fill, height: Fit, flow: Down, spacing: 2, step_row_0_header_row := View { ... step_row_0_rail := View { ... step_row_0_dot := RoundedView { ... }, step_row_0_line := View { ... } }, step_row_0_header := Button { ... } }, step_row_0_body := View { ... step_row_0_content := Label { ... } } }
```

Reformat to proper indented nesting:

```
step_row_0 := View {
    width: Fill, height: Fit
    flow: Down, spacing: 2

    step_row_0_header_row := View {
        width: Fill, height: Fit
        flow: Right, spacing: 6
        align: Align{ y: 0.0 }

        step_row_0_rail := View {
            width: 10, height: Fill
            flow: Down
            align: Align{ x: 0.5 }

            step_row_0_dot := RoundedView {
                width: 6, height: 6
                margin: Inset{ top: 4 }
                show_bg: true
                draw_bg +: { color: #444, border_radius: 3.0 }
            }
            step_row_0_line := View {
                width: 2, height: Fill
                margin: Inset{ top: 4 }
                show_bg: true
                draw_bg +: { color: #333 }
            }
        }

        step_row_0_header := Button {
            width: Fill, height: Fit
            padding: Inset{ left: 4, right: 6, top: 2, bottom: 2 }
            align: Align{ x: 0.0 }
            draw_bg +: { color: #0000, color_hover: #333, border_radius: 4.0, border_size: 0.0 }
            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9 } }
            text: ""
        }
    }

    step_row_0_body := View {
        visible: true
        width: Fill, height: Fit
        flow: Down
        padding: Inset{ left: 18, top: 2, bottom: 4 }

        step_row_0_content := Label {
            width: Fill, height: Fit
            draw_text +: { color: #d2dae8, text_style: theme.font_regular { font_size: 9, line_spacing: 1.3 } }
            text: ""
        }
    }
}
```

Do this for `step_row_0` through `step_row_9`, incrementing all IDs.

### Step 3: Verify it compiles

```bash
cargo build -p openpad-widgets 2>&1 | head -30
```

Expected: clean build. DSL parse errors will show line/column info — fix indentation or syntax if needed.

### Step 4: Run all tests

```bash
cargo test -p openpad-widgets 2>&1
```

Expected: all tests pass.

### Step 5: Commit

```bash
git add openpad-widgets/src/message_list/mod.rs
git commit -m "refactor: reformat DSL single-line tool_row and step_row blobs to multi-line"
```

---

## Final Verification

```bash
cargo test -p openpad-widgets 2>&1
cargo build -p openpad-widgets 2>&1
```

Check success criteria:
- [ ] No `match step_id` or `match idx` blocks remain in `render.rs` or `events.rs`
- [ ] `DisplayMessage` construction sites all use `..DisplayMessage::default()`
- [ ] `use_markdown = msg.cached_needs_markdown` in `render.rs`
- [ ] `mod.rs` DSL blocks are multi-line and readable
- [ ] All tests pass
