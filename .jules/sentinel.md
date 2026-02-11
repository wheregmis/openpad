## 2025-02-05 – SSE Buffer Size Limit and Error Body Truncation
**Vulnerability:** Unbounded memory growth in SSE event stream and error response processing.
**Learning:** SSE streams can grow indefinitely if delimiters are missing, and error response bodies from external servers can be arbitrarily large. Truncating strings in Rust with `truncate()` requires checking `is_char_boundary()` to avoid panics on multi-byte characters.
**Prevention:** Always implement hard caps on buffers reading from external sources and use safe truncation methods for UTF-8 strings.
