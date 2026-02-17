## 2025-05-22 – Caching Computed Display Values in Makepad PortalList
**Learning:** In Makepad widgets like `PortalList`, the `draw_walk` function is called every time an item needs to be rendered, which can be every frame during animations (like "Thinking" indicators). Performing string allocations, joins, or complex detection (like `text.contains("```")`) inside this loop causes significant heap churn and performance degradation as the list grows.
**Action:** Memoize all computed display strings and flags (markdown detection, activity summaries, formatted timestamps) into the underlying data structures (e.g., `DisplayMessage`) when they are created or updated. Accessing precomputed fields in the render loop is significantly faster and reduces allocations by nearly 100% per frame.

## 2026-02-12 – Terminal Span Merging and Line Caching
**Learning:** Terminal output often arrives in small chunks (sometimes 1 character at a time), which can lead to thousands of `TerminalSpan` objects per line if not merged. This bloats memory and makes string concatenation in `draw_walk` extremely slow.
**Action:** Merge sequential spans with the same color in the backend and cache the final concatenated line text once per newline. This avoids redundant allocations in the render loop and reduces memory overhead for long-running terminal processes.

## 2026-05-15 – Throttling Animation Redraws and Caching Render Strings
**Learning:** Animations like "Thinking" spinners that call `redraw(cx)` every frame (~60fps) cause high CPU usage because they trigger the entire `draw_walk` tree, including expensive widget recycling and string operations. Additionally, formatting time-based labels (e.g., "1m 30s") in the render loop causes constant heap allocations.
**Action:** Throttle redraw frequency in `handle_event` for non-critical animations (e.g., every 6 frames for 10fps). Cache formatted strings in the widget state and only update them when the underlying data actually changes (e.g., when the current second changes). This combination significantly reduces CPU and memory overhead during active UI states.

## 2025-05-24 – Buffer Reuse in High-Throughput Terminal Streams
**Learning:** Terminal output processing often involves accumulating text character-by-character and normalizing ANSI escape sequences. Creating new `String` objects for each chunk or normalization pass causes excessive heap churn and performance degradation during fast streaming.
**Action:** Add pre-allocated, persistent buffers (e.g., `normalization_buffer`, `current_text_buffer`) to the backend state. Clear and reuse these buffers for all accumulation and normalization tasks instead of returning new owned strings. This pattern can reduce processing time by ~25-30% in character-at-a-time streaming scenarios.
