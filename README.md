# Openpad

Open-source Codex client for OpenCode (Claude Code server) built with Makepad.

## Overview

Openpad is a desktop Codex-style client that connects to the OpenCode server for chat and streaming responses. This is an MVP focused on core functionality: connecting to OpenCode, sending messages, and displaying streaming responses.

## Features

- ✅ Connects to OpenCode on startup
- ✅ Multi-project support with automatic project detection
- ✅ Multiple concurrent sessions per project
- ✅ Create sessions automatically or manually per project
- ✅ Send messages with proper project context
- ✅ Streaming responses in real time
- ✅ Rich text rendering (markdown, code blocks, syntax highlighting)
- ✅ Session management (create, delete, rename, branch, revert, share)
- ✅ Project-aware sidebar with sessions grouped by project
- ✅ Permission approvals inline
- ✅ Error handling with actionable guidance
- ✅ Token usage and cost tracking
- ✅ Diff visualization with colored rendering
- ✅ Terminal integration (PTY-based shell)

## Prerequisites

- Rust 1.70+
- OpenCode installed (Homebrew)
- OpenCode server running (default: localhost:4096)

## Installation

1. Install OpenCode:
   ```bash
   brew install opencode
   ```

2. Install Openpad:
   ```bash
   git clone <repository>
   cd openpad
   cargo build --release
   ```

## Usage

1. Start OpenCode server in your project directory:
   ```bash
   cd /path/to/your/project
   opencode serve --port 4096
   ```

2. Run Openpad:
   ```bash
   cargo run --release
   ```

3. The sidebar will show your projects and sessions. Click "+ New session" under any project to create a session for that project, or just start typing to create a session in the current/default project.

### Keyboard Shortcuts

- `Cmd/Ctrl + D`: Toggle left sidebar
- `Cmd/Ctrl + T`: Toggle terminal panel
- `Cmd/Ctrl + I`: Toggle right sessions panel

### Multi-Project Workflow

Openpad supports multiple projects simultaneously:

1. OpenCode server tracks all projects you work with
2. Sessions are automatically associated with their project directory
3. The sidebar groups sessions by project for easy navigation
4. Switching between sessions preserves the correct project context
5. All operations (messages, reverts, branches) use the session's project directory

## Architecture

Openpad consists of two crates:

- **openpad-protocol**: Complete async Rust client for OpenCode HTTP/SSE API
  - Full API coverage: Global, App, Project, Path, Config, Session, File/Find, TUI, Auth
  - Type-safe requests and responses
  - Real-time event subscription (SSE)
  - See [openpad-protocol/README.md](openpad-protocol/README.md) for full API documentation
- **openpad-app**: Makepad-based GUI application
  - Currently uses basic session operations (create, list, prompt, SSE)
  - Future versions will leverage the full protocol capabilities

The app bridges async operations to the sync UI using `Cx::post_action()` for thread-safe communication.

## Development

See [docs/plans/2026-01-29-openpad-mvp-design.md](docs/plans/2026-01-29-openpad-mvp-design.md) for architecture details.

## Future Enhancements

- ~~Session sidebar~~ ✅ Implemented
- ~~Permission approval UI~~ ✅ Implemented
- ~~Markdown rendering~~ ✅ Implemented
- ~~Syntax highlighting~~ ✅ Implemented
- ~~Terminal integration~~ ✅ Implemented
- ~~Code diff visualization~~ ✅ Implemented
- ~~Model/Agent selection UI~~ ✅ Implemented
- ~~API key management~~ ✅ Implemented
- ~~Configuration display~~ ✅ Implemented
- Session search and filtering
- Theme switching UI
- Additional keyboard shortcuts
- File navigation and search
- Terminal multi-tab support
- Command execution in chat
- Toast notifications

## License

MIT
