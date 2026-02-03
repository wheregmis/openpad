# Openpad Roadmap

Native GUI client for OpenCode (Claude Code server) built with Makepad + Rust.

## What's Implemented

### Core Infrastructure
- [x] HTTP/SSE client for full OpenCode API (`openpad-protocol`)
- [x] Makepad GUI application shell (`openpad-app`)
- [x] Reusable widget library (`openpad-widgets`)
- [x] Tokio async runtime for network operations
- [x] Broadcast channel SSE event distribution

### Connection & Health
- [x] Auto-connect to OpenCode server on startup with retry loop
- [x] Health polling every 5 seconds
- [x] Status indicator (Connected / Disconnected) with colored dot

### Session Management
- [x] List sessions grouped by project
- [x] Create new sessions
- [x] Select sessions to load messages
- [x] SSE events for session creation and updates
- [x] Session title display (updates live via SSE)

### Messaging
- [x] Send text prompts via input bar (Enter key or Send button)
- [x] Receive messages via SSE `message.updated` events
- [x] Stream assistant responses via SSE `message.part.updated` events
- [x] User/Assistant message bubbles with distinct styling
- [x] Message list with PortalList virtualization
- [x] Inline permission prompt embedded in chat panel

### Sidebar & Navigation
- [x] Animated side panel (hamburger menu toggle)
- [x] Projects listed with name derived from worktree path
- [x] Sessions grouped under their project
- [x] Selected session highlighting
- [x] "New session" button per project
- [x] Settings tab in sidebar (toggle between Projects and Settings panels)

### UI Components
- [x] HeaderBar with title and status
- [x] HamburgerButton with animated open/close
- [x] StatusDot (connection indicator)
- [x] SidePanel with ExpDecay easing animation
- [x] ProjectsPanel (PortalList with project/session hierarchy)
- [x] MessageList (PortalList with user/assistant templates)
- [x] UserBubble / AssistantBubble
- [x] InputBar, InputField, SendButton
- [x] AppBg (gradient background)
- [x] SettingsDialog (Settings panel: provider selection, API key, config display; theme-styled)

---

## What's Remaining

### Phase 1: Core Chat Polish ✅ COMPLETED

#### Markdown & Rich Text Rendering
- [x] Parse markdown in assistant messages (bold, italic, headers, lists)
- [x] Code block rendering with syntax highlighting
- [x] Inline code styling
- [x] Link rendering

#### Permission System
- [x] Wire `PermissionRequested` SSE events to show PermissionDialog
- [x] PermissionDialog actions wired to permission reply API
- [x] Display permission type, pattern, and context
- [x] Inline permission UI (non-modal, embedded in chat)

#### Error Handling
- [x] Surface `AssistantError` details (ProviderAuthError, APIError, etc.)
- [x] Show `SessionError` SSE events as in-chat error messages
- [x] Display rate limit / auth errors with actionable guidance

#### Message Features
- [x] Show token usage per assistant message (input/output/reasoning/cache)
- [x] Show cost per message
- [x] Show model name on assistant messages
- [x] Message timestamps
- [x] Copy message text to clipboard

### Phase 2: Session Features ✅ COMPLETED

#### Session Operations
- [x] Delete session
- [x] Rename session (update title)
- [x] Abort ongoing session (cancel in-progress generation)
- [x] Session branching (create child session from parent)

#### Revert / Undo
- [x] Revert to a previous message state (`revert_message()`)
- [x] Unrevert (`unrevert_session()`)
- [x] Visual indication of revert points

#### Session Sharing
- [x] Share session publicly (`share_session()`)
- [x] Display share URL (with copy button)
- [x] Unshare session

#### Session Summary
- [x] Trigger session summarization (`summarize_session()`)
- [x] Display file change summary (additions/deletions/files)
- [x] Diff visualization for session changes (with colored rendering)

### Phase 3: Model & Provider Management

#### Model Selection
- [x] Fetch available providers and models (`get_providers()`)
- [x] Model picker UI (dropdown or dialog)
- [x] Send prompts with specific `ModelSpec`
- [x] Show current model in header or session context
- [ ] Model variant support (e.g., "extended" thinking)

#### Auth & Configuration
- [x] API key management (`set_auth()`) — Settings panel: provider dropdown, API key input, Update Key button
- [x] Configuration display (`get_config()`) — Current Configuration section in Settings panel
- [x] Provider status indicators

#### Agent Selection
- [x] Fetch available agents (`agents()`)
- [x] Agent picker for message sending
- [x] Display agent used per message

### Phase 4: File & Code Integration

#### File Operations
- [ ] File search (`search_files()`) with results panel
- [ ] Text/pattern search (`search_text()`) with line-level results
- [ ] Symbol search (`search_symbols()`)
- [ ] File reader/viewer (`read_file()`)
- [ ] Git status display (`get_file_status()`)

#### Code Context
- [ ] Attach files to prompts
- [ ] Display file diffs from assistant tool calls
- [ ] Navigate to file locations from search results

### Phase 5: Advanced Features

#### Embedded Terminal (Partially Implemented)
- [x] PTY-based terminal with shell integration (portable-pty)
- [x] ANSI escape sequence parsing (colors, SGR, CSI private modes)
- [x] Shell command input via TextInput inside PortalList
- [x] Output rendering with PortalList virtualization
- [x] Prompt detection and filtering
- [x] Backspace handling in shell echo
- [x] Terminal component integrated in app
- [x] Clear terminal output (Ctrl+L or clear command)
- [x] Terminal resize support (dynamic rows/cols)
- [x] Command history (up/down arrow keys)
- [ ] Multiple terminal tabs
- [ ] Proper thread cleanup on widget drop

#### Command Execution
- [ ] Send shell commands (`send_command()`, `send_shell()`)
- [ ] Display command output in chat
- [ ] Command palette UI

#### UI Enhancements
- [x] Theme support (theme color constants defined)
- [ ] Theme switching UI (dark/light selector)
- [ ] Keyboard shortcuts
- [x] Multi-project workspace support (projects listed, sessions grouped)
- [ ] Session search/filter
- [ ] Notification toasts (`show_toast()`)
- [ ] Scroll-to-bottom on new messages
- [ ] Loading spinners during async operations

#### SSE Event Coverage
- [x] Handle `SessionDeleted` events
- [x] Handle `SessionCreated` events
- [x] Handle `SessionUpdated` events
- [x] Handle `MessageUpdated` events
- [x] Handle `PartUpdated` events
- [x] Handle `SessionError` events
- [x] Handle `PermissionRequested` events
- [x] Handle `PermissionResponded` events
- [x] Handle `PermissionDismissed` events
- [x] Handle `MessageRemoved` events
- [x] Handle `PartRemoved` events
- [ ] Reconnect SSE on stream disconnect

---

## API Coverage

| API Endpoint | Protocol | App |
|---|---|---|
| `GET /global/health` | ✅ | ✅ |
| `GET /global/event` (SSE) | ✅ | ✅ |
| `GET /session` | ✅ | ✅ |
| `POST /session` | ✅ | ✅ |
| `GET /session/:id` | ✅ | ❌ |
| `PATCH /session/:id` | ✅ | ✅ |
| `DELETE /session/:id` | ✅ | ✅ |
| `GET /session/:id/children` | ✅ | ❌ |
| `POST /session/:id/init` | ✅ | ❌ |
| `POST /session/:id/abort` | ✅ | ✅ |
| `POST /session/:id/share` | ✅ | ✅ |
| `POST /session/:id/unshare` | ✅ | ✅ |
| `POST /session/:id/summarize` | ✅ | ✅ |
| `GET /session/:id/message` | ✅ | ✅ |
| `GET /session/:id/message/:mid` | ✅ | ❌ |
| `POST /session/:id/message` | ✅ | ✅ |
| `POST /session/:id/command` | ✅ | ❌ |
| `POST /session/:id/shell` | ✅ | ❌ |
| `POST /session/:id/revert` | ✅ | ✅ |
| `POST /session/:id/unrevert` | ✅ | ✅ |
| `POST /session/:id/permissions/:pid` | ✅ | ✅ |
| `POST /permission/:pid/reply` | ✅ | ✅ |
| `GET /project` | ✅ | ✅ |
| `GET /project/current` | ✅ | ✅ |
| `GET /config` | ✅ | ✅ |
| `GET /config/providers` | ✅ | ✅ |
| `POST /auth/:provider` | ✅ | ✅ |
| `GET /app/agents` | ✅ | ✅ |
| `POST /app/log` | ✅ | ❌ |
| `GET /find/text` | ✅ | ❌ |
| `GET /find/files` | ✅ | ❌ |
| `GET /find/symbols` | ✅ | ❌ |
| `GET /file/read` | ✅ | ❌ |
| `GET /file/status` | ✅ | ❌ |
| `GET /path` | ✅ | ❌ |
| TUI APIs (7 endpoints) | ✅ | ❌ |

**Protocol coverage:** 100% of OpenCode API  
**App integration:** ~55% of available API endpoints (up from ~52%)

---

## Architecture

```
openpad/
├── openpad-protocol/     # Async HTTP/SSE client
│   ├── src/client.rs     # OpenCodeClient with all API methods
│   ├── src/types.rs      # Request/response types, SSE events
│   └── src/lib.rs        # Error types, re-exports
├── openpad-app/          # Makepad GUI application
│   ├── src/app.rs        # Main app (AppMain, event handling, state)
│   ├── src/actions.rs    # AppAction, ProjectsPanelAction enums
│   └── src/components/   # Widget components
│       ├── app_bg.rs
│       ├── user_bubble.rs
│       ├── assistant_bubble.rs
│       ├── projects_panel.rs
│       ├── message_list.rs
│       ├── permission_dialog.rs
│       ├── settings_dialog.rs
│       └── ...
└── openpad-widgets/      # Reusable Makepad widgets
    └── src/widgets/
        ├── header_bar.rs
        ├── hamburger_button.rs
        ├── status_dot.rs
        ├── side_panel.rs
        ├── input_bar.rs
        ├── input_field.rs
        └── send_button.rs
```
