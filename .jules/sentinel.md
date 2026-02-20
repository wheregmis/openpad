## 2025-02-05 – SSE Buffer Size Limit and Error Body Truncation
**Vulnerability:** Unbounded memory growth in SSE event stream and error response processing.
**Learning:** SSE streams can grow indefinitely if delimiters are missing, and error response bodies from external servers can be arbitrarily large. Truncating strings in Rust with `truncate()` requires checking `is_char_boundary()` to avoid panics on multi-byte characters.
**Prevention:** Always implement hard caps on buffers reading from external sources and use safe truncation methods for UTF-8 strings.

## 2025-02-05 – Secret Masking and Request Timeouts
**Vulnerability:** Accidental leakage of API keys in debug logs and potential Denial of Service (DoS) from unvalidated network latency.
**Learning:** `derive(Debug)` on structs containing credentials can lead to accidental exposure. `reqwest::Client` has no default timeout, which can hang an application if a remote server is slow or malicious.
**Prevention:** Use a `SecretString` wrapper with a redacted `Debug` implementation for all credentials. Always set explicit timeouts on network requests (excluding long-running streams like SSE).

## 2025-02-05 – API Key Protection in Dialogs and Actions
**Vulnerability:** API keys were handled as plain `String` objects in dialog confirmation actions, potentially leaking them in debug logs due to `derive(Debug)` on core action enums.
**Learning:** Global dialog actions often carry sensitive data. Using a dedicated wrapper like `SecretString` for these fields ensures they are masked in developer logs while remaining accessible for functional use.
**Prevention:** Use `SecretString` for any action payload that might contain credentials, and ensure function signatures in the dispatch and runtime layers also use the protected type to maintain the security boundary.
## 2025-02-05 – Tool Input Redaction and Client Timeouts
**Vulnerability:** Leakage of sensitive credentials in tool input summaries and potential Denial of Service (DoS) from missing network timeouts.
**Learning:** Tool input summaries often serialize HashMaps directly to JSON for UI display. If these maps contain secrets (like API keys or tokens) that aren't in a preferred display list, they are leaked in full. Centralizing sensitive key detection logic ensures consistent redaction across both Debug logs and UI summaries.
**Prevention:** Always redact sensitive keys in data structures intended for UI display or logging. Use centralized heuristics for identifying credentials and ensure all network requests have explicit timeouts.

## 2025-02-05 – Enhanced Sensitive Key Detection and Error Security
**Vulnerability:** Potentially sensitive headers (e.g., Cookie, Authorization) and error metadata were not being fully protected in Debug logs or UI summaries.
**Learning:** Centralized key detection (`is_sensitive_key`) should cover a wide range of common sensitive keywords (auth, cookie, signature, credential) to be effective across different providers. Error response bodies and metadata from external APIs are high-risk areas for credential leakage.
**Prevention:** Expand centralized sensitive key heuristics and apply `SecretString` to all fields containing external API responses or headers that might be logged during error conditions.
