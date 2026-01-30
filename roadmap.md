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

### Sidebar & Navigation
- [x] Animated side panel (hamburger menu toggle)
- [x] Projects listed with name derived from worktree path
- [x] Sessions grouped under their project
- [x] Selected session highlighting
- [x] "New session" button per project

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

---

## What's Remaining

### Phase 1: Core Chat Polish

#### Markdown & Rich Text Rendering
- [ ] Parse markdown in assistant messages (bold, italic, headers, lists)
- [ ] Code block rendering with syntax highlighting
- [ ] Inline code styling
- [ ] Link rendering

#### Permission System
- [ ] Wire `PermissionRequested` SSE events to show PermissionDialog
- [ ] PermissionDialog component exists — connect Accept/Reject to `respond_to_permission()` API
- [ ] Display permission type, pattern, and context

#### Error Handling
- [ ] Surface `AssistantError` details (ProviderAuthError, APIError, etc.)
- [ ] Show `SessionError` SSE events as in-chat error messages
- [ ] Display rate limit / auth errors with actionable guidance

#### Message Features
- [ ] Show token usage per assistant message (input/output/reasoning/cache)
- [ ] Show cost per message
- [ ] Show model name on assistant messages
- [ ] Message timestamps
- [ ] Copy message text to clipboard

### Phase 2: Session Features

#### Session Operations
- [ ] Delete session
- [ ] Rename session (update title)
- [ ] Abort ongoing session (cancel in-progress generation)
- [ ] Session branching (create child session from parent)

#### Revert / Undo
- [ ] Revert to a previous message state (`revert_message()`)
- [ ] Unrevert (`unrevert_session()`)
- [ ] Visual indication of revert points

#### Session Sharing
- [ ] Share session publicly (`share_session()`)
- [ ] Display share URL
- [ ] Unshare session

#### Session Summary
- [ ] Trigger session summarization (`summarize_session()`)
- [ ] Display file change summary (additions/deletions/files)
- [ ] Diff visualization for session changes

### Phase 3: Model & Provider Management

#### Model Selection
- [ ] Fetch available providers and models (`get_providers()`)
- [ ] Model picker UI (dropdown or dialog)
- [ ] Send prompts with specific `ModelSpec`
- [ ] Show current model in header or session context
- [ ] Model variant support (e.g., "extended" thinking)

#### Auth & Configuration
- [ ] API key management (`set_auth()`)
- [ ] Configuration display (`get_config()`)
- [ ] Provider status indicators

#### Agent Selection
- [ ] Fetch available agents (`agents()`)
- [ ] Agent picker for message sending
- [ ] Display agent used per message

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

#### Command Execution
- [ ] Send shell commands (`send_command()`, `send_shell()`)
- [ ] Display command output in chat
- [ ] Command palette UI

#### UI Enhancements
- [ ] Theme support (dark/light/custom)
- [ ] Keyboard shortcuts
- [ ] Multi-project workspace switching
- [ ] Session search/filter
- [ ] Notification toasts (`show_toast()`)
- [ ] Scroll-to-bottom on new messages
- [ ] Loading spinners during async operations

#### SSE Event Coverage
- [ ] Handle `SessionDeleted` events
- [ ] Handle `MessageRemoved` events
- [ ] Handle `PartRemoved` events
- [ ] Handle `SessionError` events
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
| `PATCH /session/:id` | ✅ | ❌ |
| `DELETE /session/:id` | ✅ | ❌ |
| `GET /session/:id/children` | ✅ | ❌ |
| `POST /session/:id/init` | ✅ | ❌ |
| `POST /session/:id/abort` | ✅ | ❌ |
| `POST /session/:id/share` | ✅ | ❌ |
| `POST /session/:id/unshare` | ✅ | ❌ |
| `POST /session/:id/summarize` | ✅ | ❌ |
| `GET /session/:id/message` | ✅ | ✅ |
| `GET /session/:id/message/:mid` | ✅ | ❌ |
| `POST /session/:id/message` | ✅ | ✅ |
| `POST /session/:id/command` | ✅ | ❌ |
| `POST /session/:id/shell` | ✅ | ❌ |
| `POST /session/:id/revert` | ✅ | ❌ |
| `POST /session/:id/unrevert` | ✅ | ❌ |
| `POST /session/:id/permissions/:pid` | ✅ | ❌ |
| `GET /project` | ✅ | ✅ |
| `GET /project/current` | ✅ | ✅ |
| `GET /config` | ✅ | ❌ |
| `GET /config/providers` | ✅ | ❌ |
| `POST /auth/:provider` | ✅ | ❌ |
| `GET /app/agents` | ✅ | ❌ |
| `POST /app/log` | ✅ | ❌ |
| `GET /find/text` | ✅ | ❌ |
| `GET /find/files` | ✅ | ❌ |
| `GET /find/symbols` | ✅ | ❌ |
| `GET /file/read` | ✅ | ❌ |
| `GET /file/status` | ✅ | ❌ |
| `GET /path` | ✅ | ❌ |
| TUI APIs (7 endpoints) | ✅ | ❌ |

**Protocol coverage:** 100% of OpenCode API  
**App integration:** ~25% of available API endpoints

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
│       └── permission_dialog.rs
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
