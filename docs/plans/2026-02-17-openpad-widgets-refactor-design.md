# openpad-widgets Refactoring Design

**Date:** 2026-02-17
**Scope:** `openpad-widgets` crate
**Approach:** A — ID arrays + DisplayMessage::Default + DSL formatting + markdown enable

---

## Problem Statement

Four distinct code-quality issues in `openpad-widgets`:

1. **Repetitive indexed match arms** — `render.rs` and `events.rs` each contain ~80 lines of near-identical match arms for `step_row_0..9` (6-tuple per arm) and `tool_row_0..4` (4-tuple per arm). The same pattern is duplicated in both files.

2. **Dead `use_markdown` branch** — `render.rs:171` has `let use_markdown = false;`, making the entire Markdown rendering path dead code despite `cached_needs_markdown` being correctly computed per-message.

3. **Repeated `DisplayMessage` construction** — The struct is manually initialized ~8 times across `message_logic.rs` and `api.rs`, each time explicitly zeroing 15 `cached_*` fields (~20 lines each).

4. **Unreadable single-line DSL** — `message_list/mod.rs` lines 194–199 and 316–325 each pack multiple nested widgets into single megabyte-long lines.

---

## Design

### Section 1: Const ID Arrays (render.rs / events.rs / model.rs)

Define two `pub(super)` const arrays in `model.rs` (already imported by both render and events):

```rust
pub(super) const STEP_ROW: [(LiveId, LiveId, LiveId, LiveId, LiveId, LiveId); 10] = [
    (live_id!(step_row_0), live_id!(step_row_0_header), live_id!(step_row_0_body),
     live_id!(step_row_0_content), live_id!(step_row_0_dot), live_id!(step_row_0_line)),
    // ... rows 1–9
];

pub(super) const TOOL_ROW: [(LiveId, LiveId, LiveId, LiveId); 5] = [
    (live_id!(tool_row_0), live_id!(tool_icon_0), live_id!(tool_name_0), live_id!(tool_input_0)),
    // ... rows 1–4
];
```

All four `match step_id { 0 => ..., 1 => ..., ... }` blocks in `render.rs` and `events.rs` are replaced by:

```rust
let (row_id, header_id, body_id, content_id, dot_id, line_id) = STEP_ROW[step_id];
```

**Impact:** ~160 lines removed from render.rs + events.rs combined. Zero behavior change.

---

### Section 2: DisplayMessage Default + Constructor (message_logic.rs / api.rs)

Add `impl Default for DisplayMessage` that zeroes all `cached_*` fields and sets sensible defaults for optional fields:

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
            // ... all cached fields
        }
    }
}
```

Add a `DisplayMessage::new(role, text, message_id, timestamp, ...)` constructor for the semantic fields.

Each construction site in `message_logic.rs` and `api.rs` is replaced with:

```rust
let mut msg = DisplayMessage {
    role: role.to_string(),
    text,
    message_id: Some(message_id),
    ..DisplayMessage::default()
};
MessageProcessor::refresh_message_caches(&mut msg);
```

**Impact:** ~100 lines removed across message_logic.rs and api.rs. Zero behavior change.

---

### Section 3: DSL Formatting + Markdown Enable

**DSL formatting:** Reformat `tool_row_0..4` (lines 194–199) and `step_row_0..9` (lines 316–325) in `message_list/mod.rs` from single-line blobs to properly-indented multi-line DSL blocks. No behavior change.

**Markdown enable:** In `render.rs`:

```rust
// Before:
let use_markdown = false;

// After:
let use_markdown = msg.cached_needs_markdown;
```

This activates the already-written Markdown rendering path for messages containing markdown syntax (backticks, headers, blockquotes). `cached_needs_markdown` is already computed by `MessageProcessor::compute_needs_markdown` and cached on the message.

**Impact:** Assistant messages with markdown content will render with proper code blocks, bold, etc. Streaming animation path is already wired up and will activate for the streaming message.

---

## Files Changed

| File | Change |
|------|--------|
| `openpad-widgets/src/message_list/model.rs` | Add `STEP_ROW` and `TOOL_ROW` const arrays |
| `openpad-widgets/src/message_list/render.rs` | Replace match arms with array lookup; enable markdown |
| `openpad-widgets/src/message_list/events.rs` | Replace match arms with array lookup |
| `openpad-widgets/src/message_logic.rs` | Add `Default` impl; use `..Default::default()` in constructors |
| `openpad-widgets/src/message_list/api.rs` | Use `..DisplayMessage::default()` in constructors |
| `openpad-widgets/src/message_list/mod.rs` | Reformat DSL single-line blobs to multi-line |

---

## Success Criteria

- Code compiles and all existing tests pass (`cargo test`)
- `render.rs` and `events.rs` no longer contain any `match step_id` or `match idx` blocks
- `DisplayMessage` construction sites use `..DisplayMessage::default()`
- `use_markdown` is driven by `msg.cached_needs_markdown`
- DSL lines 194–199 and 316–325 in `mod.rs` are multi-line and readable
- No behavior regressions in the running app
