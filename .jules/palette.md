## 2025-02-04 – Empty State Pattern for PortalLists
**Learning:** `PortalList` widgets in Makepad draw nothing when their item range is zero, which can leave the screen looking "broken" or uninitialized. Using `flow: Overlay` on the parent container allows an empty state view to be positioned precisely in the same space as the list.
**Action:** Always provide a centered welcoming message or guidance view in major list components. Toggle its visibility in `draw_walk` based on whether the data vector is empty.

## 2025-02-05 – Accessible Labels for Custom Icon Buttons
**Learning:** For custom icon buttons (e.g., ellipses/dots), implementing the visual rendering in the `draw_bg` shader using SDFs while using the `text` property and `draw_text: { color: #0000 }` provides a descriptive, hidden accessibility label without breaking the visual design.
**Action:** Use this pattern to replace opaque symbols or empty strings in all icon-only controls.

## 2025-02-12 – Animation Drive Pattern for App-level Indicators
**Learning:** To drive UI animations in the main `App` (e.g., status spinners), increment frame indices within the `Event::NextFrame` block of `handle_event` and call `cx.new_next_frame()` while the animated state (e.g., `is_working`) is active.
**Action:** Use `frame_count % N == 0` to throttle update frequency to a reasonable rate (e.g., 10fps) to save CPU.

## 2025-02-12 – Interaction Feedback for Copy Actions
**Learning:** Users lack confidence when clicking "Copy" buttons that provide no visual confirmation. A simple label swap to "Copied!" for 2 seconds is high-impact and easy to implement using a transient `Instant` in the widget state.
**Action:** Always provide temporal feedback for clipboard interactions.
