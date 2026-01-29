# Openpad AI coding guide

## Big picture
- Rust workspace with two crates: openpad-app (Makepad GUI) and openpad-protocol (async OpenCode HTTP/SSE client). See [Cargo.toml](Cargo.toml).
- UI is synchronous Makepad, while network work is async; the bridge is `Cx::post_action()` with `AppAction` messages in [openpad-app/src/main.rs](openpad-app/src/main.rs).
- OpenCode server must be running on http://localhost:4096; the client passes the current working directory as the `directory` query param in all requests in [openpad-protocol/src/client.rs](openpad-protocol/src/client.rs).

## Architecture & data flow
- Startup: `App::connect_to_opencode()` spawns a `tokio::runtime::Runtime`, creates `OpenCodeClient`, and begins SSE subscription in [openpad-app/src/main.rs](openpad-app/src/main.rs).
- SSE parsing lives in `parse_sse_event()` and maps `session.created`, `message.updated`, and `message.part.updated` into typed `Event` values in [openpad-protocol/src/client.rs](openpad-protocol/src/client.rs) and [openpad-protocol/src/types.rs](openpad-protocol/src/types.rs).
- UI state is stored in `messages: Vec<Message>` and updated via `AppAction::OpenCodeEvent`, `MessageUpdated`, and `PartUpdated` handlers in [openpad-app/src/main.rs](openpad-app/src/main.rs).

## Project-specific patterns
- Makepad UI is defined via `live_design!` and widget IDs like `status_label` and `input_box` in [openpad-app/src/main.rs](openpad-app/src/main.rs). Keep UI updates on the main thread; use `Cx::post_action()` from async tasks.
- Message parts are intentionally minimal: `Part` currently supports only `Text` and `Unknown` to stay robust to server changes in [openpad-protocol/src/types.rs](openpad-protocol/src/types.rs). Any new rendering should handle `Unknown` gracefully.
- Session creation is lazy: when sending a prompt, `send_message()` creates a session if none exists, then sends the prompt in [openpad-app/src/main.rs](openpad-app/src/main.rs).

## Developer workflows
- Run OpenCode server first (README calls the command `opencode`) and then run the app from the repo root with `cargo run --release`. See [README.md](README.md).
- The only binary is openpad-app’s `openpad` in [openpad-app/Cargo.toml](openpad-app/Cargo.toml).
- No tests or lint config are present in the repo at this time.

## Key references
- GUI entry point and event handling: [openpad-app/src/main.rs](openpad-app/src/main.rs).
- OpenCode client + SSE: [openpad-protocol/src/client.rs](openpad-protocol/src/client.rs).
- API data types and event enums: [openpad-protocol/src/types.rs](openpad-protocol/src/types.rs).
- Overview and prerequisites: [README.md](README.md).
