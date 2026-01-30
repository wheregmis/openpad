# Session List Redesign - Compact Timeline View

**Date:** 2026-01-29
**Status:** Approved
**Approach:** Option 1 - Compact Timeline View

## Overview

Redesign the session list UI to show meaningful information instead of session IDs, with better visual hierarchy, metadata display, and interaction states. The design focuses on a compact, timeline-oriented approach that maintains project grouping while adding rich metadata.

## Current Problems

1. Sessions display long hex IDs instead of meaningful titles
2. No visual hierarchy or distinction between sessions
3. Missing metadata (timestamps, file changes, message counts)
4. Poor visual design with generic styling
5. No status indicators for session state

## Design Goals

- Display meaningful session information at a glance
- Create visual hierarchy with project grouping
- Show relevant metadata (time, changes, status)
- Provide smooth interactions and feedback
- Maintain compact, scannable layout

---

## 1. Session Row Layout

### Structure (Left to Right)

1. **Status Indicator Bar** (3px wide)
   - Colored vertical bar on left edge
   - Shows session state at a glance
   - Rounded corners on left side

2. **Main Content Area** (two-line layout)
   - **Line 1:** Session title + status icons
   - **Line 2:** Metadata row

3. **Hover Actions** (right edge)
   - Share and delete icons
   - Appear on hover

### Dimensions

- Row height: 52px (increased from 34px)
- Padding: 8px vertical, 12px horizontal
- Gap between lines: 4px
- Status bar width: 3px

### Title Fallback Logic

```
if !title.is_empty() → use title
else if !slug.is_empty() → use slug
else → "New session - {formatted_date}"
```

### Typography

- **Title:** 11px, medium weight, #e6e9ee
- **Metadata:** 10px, regular weight, #7a8591
- Ellipsis truncation for long titles

---

## 2. Metadata Display & Icons

### Metadata Row Elements (Left to Right)

1. **Timestamp** - Relative time format
   - "2m ago", "1h ago" (< 1 hour)
   - "Today at 2:30pm" (today)
   - "Yesterday at 3:45pm" (yesterday)
   - "Jan 28 at 11:20am" (older)
   - Uses `time.updated`, falls back to `time.created`

2. **Dot Separator** - "•" between items (#5a6570)

3. **File Changes** (if summary exists)
   - Format: "+12 -3"
   - Green (#3ba55d) for additions
   - Red (#e85a5a) for deletions
   - Shows `summary.additions` and `summary.deletions`

4. **Files Count** (if summary exists)
   - Format: "5 files"
   - Gray text (#7a8591)
   - Shows `summary.files`

5. **Message Count** (tracked via events)
   - Format: "8 messages" or "8" with icon
   - Gray text (#7a8591)
   - Only shown if count > 0

### Status Icons (After Title, Line 1)

- 📌 **Pin** - Current/selected session
- 🔗 **Share** - If `share` field exists
- ↗ **Fork** - If `parent_id` exists
- 📦 **Archive** - If `time.archived` exists
- Size: 12px, Color: #7a8591
- 4px spacing between icons

---

## 3. Interaction States & Visual Feedback

### Selection State

- Background: #242a32
- Status bar: brighter, more saturated
- Subtle left border glow
- Title text: #ffffff (brighter)

### Hover State (Non-Selected)

- Background: #1f252c (subtle lift)
- Cursor: pointer
- Hover actions fade in (right edge)
- Transition: 150ms ease-out
- Optional: slight scale (1.005)

### Hover Actions (Right Edge)

- **Share button** (🔗) and **Delete button** (🗑️)
- Touch target: 24px × 24px
- Icon size: 14px
- Default color: #7a8591
- Hover color: #e6e9ee
- Delete hover: #e85a5a (red tint)
- Gap between icons: 8px
- Fade in on row hover

### Animation

- Rows fade in when loaded (stagger 30ms each)
- Status bar animates from left
- Hover transitions: smooth 150ms
- Selection: instant (no delay)
- Pressed state: darker background #1a1f25 (100ms)

### Micro-Interactions

- Icon buttons scale to 1.1 on hover
- Delete button: shake animation on click
- Ripple effect on selection (optional)

---

## 4. Grouping & Organization

### Project Grouping (Enhanced)

**Project Header:**
- Project name: 12px, semi-bold, #e6e9ee
- Project path: 10px, #7a8591
- Session count: "(3 sessions)" in gray
- Padding: 16px top, 8px bottom

**Session Sorting:**
- Sort by `time.updated` (most recent first)
- Archived sessions at bottom of project group
- Pinned/current session at top

**"New Session" Button:**
- Positioned under project header
- Dashed border instead of solid
- "+" icon with subtle hover animation

### Structure

```
┌─ "Today" (special group)
│  └─ Sessions updated today (all projects)
├─ Project 1
│  ├─ New session button
│  ├─ Session A (pinned)
│  ├─ Session B (recent)
│  └─ Session C (archived)
├─ Project 2
│  └─ ...
└─ "Other" (sessions without projects)
```

### Empty States

- **No sessions in project:** Show only "New session" button
- **No sessions at all:** Empty state illustration + helpful text
- **Loading:** Skeleton screens with shimmer animation

---

## 5. Data Handling & Message Tracking

### Message Count Tracking

**Client-Side Tracking:**
```rust
#[rust]
message_counts: HashMap<String, usize>
```

**Strategy:**
- Initial load: counts start at 0
- Real-time updates via SSE `Event::MessageUpdated`
- Increment count when new messages arrive
- Don't show count if 0 or unknown (keeps UI clean)

### Smart Title Generation

For empty title/slug:
```rust
format!("New session - {}", format_date(session.time.created))
// Example: "New session - Jan 29, 2:30 PM"
```

### Additional State

```rust
#[rust]
pinned_session: Option<String>,  // Currently active session
```

### Performance

- PortalList handles virtualization
- Only render visible rows
- Debounce rapid updates during streaming
- Efficient HashMap lookups for message counts

---

## 6. Colors, Polish & Visual Details

### Color Palette

**Status Indicator Colors:**
- Active (updated < 24h): `#3ba55d` (green)
- Normal: `#4a90e2` (blue)
- Archived: `#6b7b8c` (gray)
- Selected accent: `#5a9fd4` (lighter blue)

**Text Colors:**
- Title: `#e6e9ee`
- Title (selected): `#ffffff`
- Metadata: `#7a8591`
- Separator: `#5a6570`

**Background Colors:**
- Default: `#1f2329`
- Hover: `#1f252c`
- Selected: `#242a32`
- Pressed: `#1a1f25`

**Semantic Colors:**
- Additions (green): `#3ba55d`
- Deletions (red): `#e85a5a`
- Link/share (blue): `#4a90e2`

### Status Bar

- Width: 3px
- Border radius: left side only
- Glow effect for selected (optional)
- Color transitions: 200ms ease

### Borders & Shadows

- Session separator: 1px, `#22262c`
- No borders on rows by default
- Selected row: subtle inner shadow
- Hover actions: slight drop shadow

### Typography

- Consistent font weights throughout
- Tabular numbers for metadata alignment
- Text truncation with ellipsis for long titles

### Loading States

- Skeleton screens while loading
- Shimmer animation on placeholders
- Fade-in when data appears (200ms)
- Staggered entrance (30ms delay per row)

---

## Implementation Notes

### Files to Modify

1. **openpad-app/src/app.rs**
   - Update `PanelItemKind::SessionRow` to include metadata
   - Add message count tracking
   - Implement smart title generation
   - Add session sorting logic
   - Add hover state handling

2. **openpad-app/src/app.rs** (live_design section)
   - Redesign `SessionRow` widget template
   - Add status indicator bar
   - Update typography and spacing
   - Add hover action buttons
   - Implement new color scheme

3. **openpad-widgets/src/lib.rs** (if needed)
   - May need new reusable components
   - Status bar component
   - Metadata row component

### Data Flow

1. Sessions loaded from API → stored in `App.sessions`
2. Message events received → update `message_counts` HashMap
3. `rebuild_items()` called → generates display items with metadata
4. `draw_walk()` renders visible items with PortalList
5. User interaction → update selection state, post actions

### Testing Checklist

- [ ] Sessions display meaningful titles
- [ ] Timestamps format correctly (relative time)
- [ ] File change counts display when summary exists
- [ ] Message counts update via SSE events
- [ ] Status icons show for shared/forked/archived sessions
- [ ] Hover states work smoothly
- [ ] Selection state persists correctly
- [ ] Sorting by update time works
- [ ] Empty states display correctly
- [ ] Loading states animate properly
- [ ] Project grouping works correctly
- [ ] "Today" special group works
- [ ] Delete and share actions work

---

## Success Metrics

- Sessions are immediately identifiable by meaningful names
- Users can see session activity at a glance (time, changes)
- Visual hierarchy makes navigation intuitive
- Interactions feel smooth and responsive
- Information density is high but not overwhelming

---

## Future Enhancements (Out of Scope)

- Search/filter sessions
- Bulk operations (archive multiple, delete multiple)
- Session tags/labels
- Custom session colors
- Drag-and-drop to reorganize
- Session templates
