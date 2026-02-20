## 2025-05-22 – Caching Computed Display Values in Makepad PortalList
**Learning:** In Makepad widgets like `PortalList`, the `draw_walk` function is called every time an item needs to be rendered, which can be every frame during animations (like "Thinking" indicators). Performing string allocations, joins, or complex detection (like `text.contains("```")`) inside this loop causes significant heap churn and performance degradation as the list grows.
**Action:** Memoize all computed display strings and flags (markdown detection, activity summaries, formatted timestamps) into the underlying data structures (e.g., `DisplayMessage`) when they are created or updated. Accessing precomputed fields in the render loop is significantly faster and reduces allocations by nearly 100% per frame.

## 2026-02-12 – Terminal Span Merging and Line Caching
**Learning:** Terminal output often arrives in small chunks (sometimes 1 character at a time), which can lead to thousands of `TerminalSpan` objects per line if not merged. This bloats memory and makes string concatenation in `draw_walk` extremely slow.
**Action:** Merge sequential spans with the same color in the backend and cache the final concatenated line text once per newline. This avoids redundant allocations in the render loop and reduces memory overhead for long-running terminal processes.

## 2026-05-15 – Throttling Animation Redraws and Caching Render Strings
**Learning:** Animations like "Thinking" spinners that call `redraw(cx)` every frame (~60fps) cause high CPU usage because they trigger the entire `draw_walk` tree, including expensive widget recycling and string operations. Additionally, formatting time-based labels (e.g., "1m 30s") in the render loop causes constant heap allocations.
**Action:** Throttle redraw frequency in `handle_event` for non-critical animations (e.g., every 6 frames for 10fps). Cache formatted strings in the widget state and only update them when the underlying data actually changes (e.g., when the current second changes). This combination significantly reduces CPU and memory overhead during active UI states.

## 2026-06-10 – Relocating Expensive Computation from Draw Loops and Optimizing DP Tables
**Learning:** Performing O(N*M) algorithms like LCS-based unified diffing inside a Makepad `draw_walk` loop is extremely expensive, especially since it runs every frame during animations or scrolling. Additionally, nested vectors (`Vec<Vec<T>>`) in DP tables cause significant heap churn.
**Action:** Move all non-UI computation (like diff generation) to background processing or once-per-update handlers (e.g., `MessageProcessor::refresh_message_caches`). Cache the final display strings in the underlying data structures. Optimize DP tables by using a single 1D vector with manual indexing to reduce allocations from O(N) to O(1) and improve cache locality.

## 2026-02-18 – Buffer Reuse and Offset-based Slicing in Multi-line Widgets
**Learning:** Widgets that render multi-line text (like `ColoredDiffText`) often allocate new `String` objects for every line during updates, causing significant heap churn in hot paths like streaming assistant responses. Even if the update is throttled, large diffs can trigger thousands of allocations.
**Action:** Implement a buffer-reuse strategy by storing line data as `start` and `end` offsets into a single persistent `String` buffer. Clear and reuse the buffer using `.clear()` and `.push_str()` to maintain capacity and reduce deallocations to O(1) amortized. Render lines using string slices in the `draw_walk` loop to eliminate per-frame allocations.

## 2026-06-25 – Eliminating Redundant Global State Caches and Massive Clones
**Learning:** Maintaining a redundant "active cache" field (like `messages_data`) that mirrors a subset of a primary collection (like `messages_by_session`) leads to massive performance degradation. In streaming applications, updating this cache on every token received triggers $O(N)$ heap clones of large data structures (e.g., entire chat histories).
**Action:** Remove redundant cache fields and implement a single-source-of-truth model. Perform targeted lookups or pass references/slices directly to UI components. This eliminates expensive redundant allocations in hot paths like SSE event processing and session switching.

## 2026-07-02 – Reducing Heap Churn and System Calls in Makepad Draw Loops
**Learning:** Performing $O(N)$ heap clones (e.g., `projects.clone()`) and calling system APIs (e.g., `std::env::current_dir()`) inside a Makepad `draw_walk` or `draw_tree` function causes significant performance degradation, especially during animations or frequent state updates. Furthermore, inefficient string operations like `chars().take(n).collect::<String>()` for UI label truncation cause constant temporary allocations.
**Action:** Use references and intermediate data structures that store `&T` instead of owned `T` objects in draw loops. Cache system-level information in persistent `#[rust]` fields. Optimize string handling by pre-allocating with `String::with_capacity` and using slice-based truncation with `is_char_boundary()` checks to eliminate unnecessary heap churn.
