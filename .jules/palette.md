## 2025-02-04 – Empty State Pattern for PortalLists
**Learning:** `PortalList` widgets in Makepad draw nothing when their item range is zero, which can leave the screen looking "broken" or uninitialized. Using `flow: Overlay` on the parent container allows an empty state view to be positioned precisely in the same space as the list.
**Action:** Always provide a centered welcoming message or guidance view in major list components. Toggle its visibility in `draw_walk` based on whether the data vector is empty.

## 2025-02-05 – Accessible Labels for Custom Icon Buttons
**Learning:** For custom icon buttons (e.g., ellipses/dots), implementing the visual rendering in the `draw_bg` shader using SDFs while using the `text` property and `draw_text: { color: #0000 }` provides a descriptive, hidden accessibility label without breaking the visual design.
**Action:** Use this pattern to replace opaque symbols or empty strings in all icon-only controls.

## 2026-02-24 – Animated Status Spinner
**Learning:** Using a rotating sequence of Unicode characters (e.g., ◐, ◓, ◑, ◒) in a `Label` text provides a lightweight, accessible, and high-impact way to indicate an active "Working" state without custom shaders. It integrates seamlessly with Makepad's frame-driven event loop.
**Action:** When adding long-running async states, implement this text-based spinner in the main header or status bar to provide immediate visual feedback.
