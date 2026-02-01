# Inline Diff Display & Live Activity Indicators

**Date**: 2026-02-01
**Status**: Design

## Problem

When an agent works on a task, the UI feels unresponsive:
1. No visibility into what the agent is doing (tool calls, file edits)
2. No inline view of file changes — users must check git diff externally
3. Permission requests interrupt flow with modal dialogs

## Design

### Feature A: Inline Diff Display

Show file diffs inline within assistant message bubbles after a response completes.

**Data source**: Session summary contains `Vec<FileDiff>` with `file`, `before`, `after`, `additions`, `deletions` fields (already in `openpad-protocol`).

**UI layout** (collapsed by default):
```
┌─ Files Changed ─────────────────────┐
│ ▶ src/main.rs  (+12 -3)            │
│ ▶ src/utils.rs (+5 -1)             │
└─────────────────────────────────────┘
```

Expanded shows unified diff per file:
```
┌─ src/main.rs ───────────────────────┐
│   fn main() {                       │
│ -     println!("old");              │
│ +     println!("new");              │
│   }                                 │
└─────────────────────────────────────┘
```

**Implementation**:
- New `DiffView` widget in `openpad-app/src/components/diff_view.rs`
- Line-based unified diff computed from `before`/`after` strings
- Green background for additions, red for deletions, neutral for context
- Collapsible per-file sections with +/- summary in header
- Rendered at bottom of assistant bubble, above metadata row
- Diff data fetched when session updates after assistant message completes

### Feature B: Live Activity Indicators

Three layers of feedback while the agent works:

#### B1: Streaming Typing Indicator
- Blinking cursor/caret at the end of assistant message text while `is_working` is true
- Disappears when streaming stops
- Implemented via Makepad animator on a small view at text end

#### B2: Inline Permission Cards (replaces modal dialog)
- When `PermissionAsked` arrives, render an inline card in the chat stream:
```
┌─ Agent Activity ────────────────┐
│ Requesting: Edit file           │
│ src/main.rs                     │
│ [Approve] [Deny]               │
└──────────────────────────────────┘
```
- Cards appear in message flow at the position where the permission was requested
- Approve/Deny buttons call existing `respond_to_permission()` API
- After response, card updates to show "Approved" or "Denied" status
- Replaces the current `PermissionDialog` modal

#### B3: Working Status Banner
- Subtle animated bar above the input area:
```
── Agent working · 0:42 elapsed ──
```
- Shows elapsed time since `is_working` became true
- Disappears when agent finishes
- Uses existing `is_working` state flag

## Files to Create/Modify

### New Files
- `openpad-app/src/components/diff_view.rs` — DiffView widget with unified diff rendering
- `openpad-app/src/components/permission_card.rs` — Inline permission card widget

### Modified Files
- `openpad-app/src/components/mod.rs` — Register new components
- `openpad-app/src/components/assistant_bubble.rs` — Add DiffView and typing indicator
- `openpad-app/src/components/message_list.rs` — Integrate permission cards into message flow; add DisplayMessage variant for permission requests
- `openpad-app/src/state/handlers.rs` — Route PermissionAsked to inline cards instead of modal; fetch diffs on session update
- `openpad-app/src/app.rs` — Add working status banner to chat panel layout; remove modal dialog references
- `openpad-app/src/state/actions.rs` — Add actions for diff loading and permission card interactions

### Possibly Remove
- `openpad-app/src/components/permission_dialog.rs` — Replaced by inline permission cards

## Diff Algorithm

Simple line-based approach:
1. Split `before` and `after` by newlines
2. Use a longest-common-subsequence (LCS) diff to identify added/removed/unchanged lines
3. Show 3 lines of context around changes (configurable)
4. For large diffs (>100 changed lines), show summary only with "Show full diff" expander

## Protocol Constraints

- No tool call events beyond permission requests — we cannot show arbitrary tool activity
- No thinking/reasoning content type — only token count available
- `PartUpdated` with `delta` already streams text incrementally
- `FileDiff` only available in session summary (post-completion)

## Out of Scope (Future)
- Extending protocol to expose tool calls and thinking
- Side-by-side diff view
- Syntax highlighting in diff view
- Inline file editing from diff view
