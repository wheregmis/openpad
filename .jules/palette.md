## 2025-02-04 – Empty State Pattern for PortalLists
**Learning:** `PortalList` widgets in Makepad draw nothing when their item range is zero, which can leave the screen looking "broken" or uninitialized. Using `flow: Overlay` on the parent container allows an empty state view to be positioned precisely in the same space as the list.
**Action:** Always provide a centered welcoming message or guidance view in major list components. Toggle its visibility in `draw_walk` based on whether the data vector is empty.

## 2025-02-13 – Accessible Labels for Icon-Only Buttons
**Learning:** Standard Makepad widgets do not support `aria_label`. A common pattern to provide descriptive labels for screen readers without affecting the visual design is to set the `text` property and use `draw_text: { color: #0000 }` to hide it.
**Action:** Use this pattern for all icon-only controls to improve accessibility.
