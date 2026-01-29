# Openpad

A native GUI client for OpenCode (Claude Code server) built with Makepad.

## Overview

Openpad provides a clean chat interface for interacting with Claude Code through the OpenCode server. This is an MVP focused on core functionality: connecting to OpenCode, sending messages, and displaying streaming responses.

## Features

- ✅ Connect to OpenCode server on startup
- ✅ Create chat sessions automatically
- ✅ Send text messages
- ✅ Display streaming responses in real-time
- ✅ Plain text message rendering

## Prerequisites

- Rust 1.70+
- OpenCode server running (default: localhost:4096)

## Installation

```bash
git clone <repository>
cd openpad
cargo build --release
```

## Usage

1. Start OpenCode server:
   ```bash
   opencode
   ```

2. Run Openpad:
   ```bash
   cargo run --release
   ```

3. Type messages in the input box and press Enter

## Architecture

Openpad consists of two crates:

- **openpad-protocol**: Async Rust client for OpenCode HTTP/SSE API
- **openpad-app**: Makepad-based GUI application

The app bridges async operations to the sync UI using `Cx::post_action()` for thread-safe communication.

## Development

See [docs/plans/2026-01-29-openpad-mvp-design.md](docs/plans/2026-01-29-openpad-mvp-design.md) for architecture details.

## Future Enhancements

- Session sidebar
- Permission approval UI
- Markdown rendering
- Syntax highlighting
- Terminal integration
- Code diff visualization

## License

MIT
