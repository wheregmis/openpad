## 2025-05-22 – Caching Computed Display Values in Makepad PortalList
**Learning:** In Makepad widgets like `PortalList`, the `draw_walk` function is called every time an item needs to be rendered, which can be every frame during animations (like "Thinking" indicators). Performing string allocations, joins, or complex detection (like `text.contains("```")`) inside this loop causes significant heap churn and performance degradation as the list grows.
**Action:** Memoize all computed display strings and flags (markdown detection, activity summaries, formatted timestamps) into the underlying data structures (e.g., `DisplayMessage`) when they are created or updated. Accessing precomputed fields in the render loop is significantly faster and reduces allocations by nearly 100% per frame.

## 2026-02-12 - [Terminal Span Merging and Line Caching]
**Learning:** Terminal output often arrives in small chunks (sometimes 1 character at a time), which can lead to thousands of  objects per line if not merged. This bloats memory and makes string concatenation in `draw_walk` extremely slow.
**Action:** Merge sequential spans with the same color in the backend and cache the final concatenated line text once per newline. This avoids redundant allocations in the render loop and reduces memory overhead for long-running terminal processes.

## 2026-02-12 - [Terminal Span Merging and Line Caching]
**Learning:** Terminal output often arrives in small chunks (sometimes 1 character at a time), which can lead to thousands of `TerminalSpan` objects per line if not merged. This bloats memory and makes string concatenation in `draw_walk` extremely slow.
**Action:** Merge sequential spans with the same color in the backend and cache the final concatenated line text once per newline. This avoids redundant allocations in the render loop and reduces memory overhead for long-running terminal processes.
