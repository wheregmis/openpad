## 2025-02-05 – SSE Buffer Size Limit and Error Body Truncation
**Vulnerability:** Unbounded memory growth in SSE event stream and error response processing.
**Learning:** SSE streams can grow indefinitely if delimiters are missing, and error response bodies from external servers can be arbitrarily large. Truncating strings in Rust with `truncate()` requires checking `is_char_boundary()` to avoid panics on multi-byte characters.
**Prevention:** Always implement hard caps on buffers reading from external sources and use safe truncation methods for UTF-8 strings.

## 2025-02-05 – Secret Masking and Request Timeouts
**Vulnerability:** Accidental leakage of API keys in debug logs and potential Denial of Service (DoS) from unvalidated network latency.
**Learning:** `derive(Debug)` on structs containing credentials can lead to accidental exposure. `reqwest::Client` has no default timeout, which can hang an application if a remote server is slow or malicious.
**Prevention:** Use a `SecretString` wrapper with a redacted `Debug` implementation for all credentials. Always set explicit timeouts on network requests (excluding long-running streams like SSE).
