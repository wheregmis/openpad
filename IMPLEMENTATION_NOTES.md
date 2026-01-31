# Multiple Terminal Tabs - Implementation Notes

This document describes the implementation of multiple terminal tabs feature for Openpad.

## Overview

The terminal panel now supports multiple independent terminal instances, each running in its own tab. Users can:
- Create new terminals by clicking the "+" button
- Switch between terminals by clicking on tabs
- Each terminal maintains its own session, history, and output

## Implementation Approach

### Architecture Decision

Instead of trying to dynamically create tabs outside of Makepad's DSL system, we use a `PortalList` to render tabs, similar to how the ProjectsPanel renders session lists. This approach:
- Follows established Makepad patterns
- Leverages existing widget infrastructure
- Avoids complex dynamic widget creation

### Key Components

1. **TerminalInstance** - Encapsulates state for a single terminal
   - Unique ID for routing output
   - Shell name for tab label
   - Independent PTY and output buffer
   - Own reader thread

2. **Tab Bar** - 32px height bar at top of terminal panel
   - Horizontal PortalList for tabs
   - "+" button on the right
   - Active tab highlighted with blue border

3. **Output Routing** - Thread-safe output delivery
   - Each PTY reader includes terminal_id in actions
   - Terminal widget routes output to correct instance
   - No output mixing between terminals

## Code Structure

```
Terminal (Widget)
├── terminals: Vec<TerminalInstance>
│   ├── [0] id: 0, shell: "zsh", pty: ...
│   ├── [1] id: 1, shell: "zsh", pty: ...
│   └── [2] id: 2, shell: "zsh", pty: ...
├── active_terminal_index: usize
└── next_terminal_id: usize

UI Layout:
┌─────────────────────────┐
│ Tab Bar (PortalList)    │  ← tabs_list
├─────────────────────────┤
│ Terminal Output         │  ← output_list (for active terminal)
└─────────────────────────┘
```

## Event Flow

1. **Creating New Terminal**
   ```
   User clicks "+" → handle_event
   → create_new_terminal()
   → push new TerminalInstance
   → init_pty_for_terminal()
   → spawn PTY + reader thread
   → redraw UI
   ```

2. **Switching Terminals**
   ```
   User clicks tab → handle_event
   → items_with_actions() detects click
   → update active_terminal_index
   → redraw UI (shows new terminal's output)
   ```

3. **Receiving Output**
   ```
   PTY output → reader thread
   → Cx::post_action(OutputReceived { terminal_id, text })
   → handle_action finds terminal by ID
   → append_output_to_terminal()
   → redraw UI
   ```

## Design Decisions

### Why PortalList for Tabs?
- Consistent with other Makepad widgets
- Handles layout and scrolling automatically
- Event handling via items_with_actions()
- Works well with Makepad's rendering pipeline

### Why Terminal IDs?
- Thread-safe output routing
- Terminals can be reordered later without breaking output
- Survives terminal closes (future feature)
- Clean separation between index and identity

### Why Not Dynamic Widget Creation?
- Makepad's live_design! system prefers static templates
- PortalList provides dynamic behavior without dynamic widgets
- More maintainable and follows framework patterns

## Testing Strategy

The implementation was tested by:
1. Compilation check (`cargo check`) - PASSED
2. Code review against existing patterns
3. Manual verification of logic flow
4. Review of similar components (ProjectsPanel)

Cannot run GUI in CI environment (headless, no X11), but code follows established patterns that work elsewhere in the codebase.

## Future Enhancements

Not implemented (out of scope for this PR):
- Close button on tabs (needs close confirmation)
- Keyboard shortcuts (Ctrl+Tab, Ctrl+W)
- Custom tab names/labels
- Drag-and-drop tab reordering
- Tab context menu (rename, close, duplicate)

## Files Changed

- `openpad-app/src/components/terminal.rs`
  - +292 lines added
  - -89 lines removed
  - Net: +203 lines

## References

Similar patterns used in:
- `openpad-app/src/components/projects_panel.rs` - PortalList with items_with_actions
- `openpad-widgets/src/` - Custom widgets and UI patterns

## Maintainer Notes

When adding close button:
- Don't allow closing last terminal
- Adjust active_terminal_index if closing active tab
- Clean up PTY resources properly
- Consider confirmation dialog for terminals with history
