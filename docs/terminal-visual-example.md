# Terminal Feature - Visual Example

This mockup shows the terminal integrated into Openpad's UI:

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  ≡  Openpad                                                    ● Connected    ║
╠══════╦════════════════════════════════════════════════════════════════════════╣
║      ║  Session: Roadmap next-item                                 ↻ Unrevert ║
║      ╠════════════════════════════════════════════════════════════════════════╣
║      ║                                                                         ║
║ [📁] ║  You:                                                                   ║
║ make ║  Let's add a terminal to the bottom of the app                         ║
║ pad  ║                                                                         ║
║      ║  ┌─────────────────────────────────────────────────────────────────┐   ║
║ [📂] ║  │ Assistant:                                                      │   ║
║ neps ║  │ I'll help you add a terminal widget at the bottom of Openpad.  │   ║
║ e_bo ║  │ Let me create the terminal component...                        │   ║
║ t    ║  │                                                                 │   ║
║      ║  │ Created files:                                                  │   ║
║ [📂] ║  │ • openpad-app/src/components/terminal.rs                        │   ║
║ mico ║  │ • docs/terminal-feature.md                                      │   ║
║ ntro ║  │                                                                 │   ║
║ l_en ║  │ Next step: Build and test the terminal functionality           │   ║
║ gine ║  └─────────────────────────────────────────────────────────────────┘   ║
║      ║                                                                         ║
║ [📂] ║                                                                         ║
║ open ║                                                                         ║
║ pad  ║                                                                         ║
║      ║                                                                         ║
║ + Ne ║                                                                         ║
║ w    ║                                                                         ║
║ sess ║                                                                         ║
║ ion  ║                                                                         ║
║      ╠════════════════════════════════════════════════════════════════════════╣
║      ║  🔹 Ask anything...                                          [Send →]  ║
║      ╠════════════════════════════════════════════════════════════════════════╣
║      ║  Terminal                                                   [Clear]    ║
║      ║  ┌──────────────────────────────────────────────────────────────────┐ ║
║      ║  │ > ls -la                                                         │ ║
║      ║  │ total 48                                                         │ ║
║      ║  │ drwxr-xr-x  10 user  staff   320 Jan 31 03:49 .                 │ ║
║      ║  │ drwxr-xr-x   8 user  staff   256 Jan 30 15:23 ..                │ ║
║      ║  │ drwxr-xr-x  12 user  staff   384 Jan 31 03:49 .git              │ ║
║      ║  │ -rw-r--r--   1 user  staff   243 Jan 30 14:12 .gitignore        │ ║
║      ║  │ -rw-r--r--   1 user  staff  2843 Jan 31 03:40 Cargo.lock        │ ║
║      ║  │ -rw-r--r--   1 user  staff   213 Jan 30 14:12 Cargo.toml        │ ║
║      ║  │ -rw-r--r--   1 user  staff  1234 Jan 30 15:20 README.md         │ ║
║      ║  │ drwxr-xr-x   5 user  staff   160 Jan 31 03:49 docs              │ ║
║      ║  │ drwxr-xr-x   8 user  staff   256 Jan 31 03:40 openpad-app       │ ║
║      ║  │ drwxr-xr-x   6 user  staff   192 Jan 30 18:45 openpad-protocol  │ ║
║      ║  │ drwxr-xr-x   5 user  staff   160 Jan 30 17:30 openpad-widgets   │ ║
║      ║  │ drwxr-xr-x  12 user  staff   384 Jan 31 03:45 target            │ ║
║      ║  │ > cargo check                                                    │ ║
║      ║  │ Checking openpad-app v0.1.0                                      │ ║
║      ║  │ Finished dev profile in 0.60s                                    │ ║
║      ║  │ > pwd                                                            │ ║
║      ║  │ /Users/user/Documents/GitHub/openpad                             │ ║
║      ║  │ Terminal initialized with shell: /bin/zsh                        │ ║
║      ║  │                                                                  │ ║
║      ║  └──────────────────────────────────────────────────────────────────┘ ║
║      ║  🔸 Enter command...                                                  ║
╚══════╩════════════════════════════════════════════════════════════════════════╝
```

## Key Visual Elements

1. **Top Bar**: App title and connection status
2. **Left Sidebar**: Project/session list with hamburger menu toggle
3. **Main Chat Area**: Messages between user and Claude Code
4. **Input Bar**: "Ask anything..." field for chat messages
5. **Terminal Panel**: NEW! At the bottom with:
   - Header showing "Terminal" and [Clear] button
   - Scrollable output area showing command history and results
   - Input field at bottom for entering commands

## Terminal Visual Characteristics

- **Dark Theme**: Matches Openpad's color scheme
  - Terminal bg: `#1a1d23` (dark blue-gray)
  - Header: `#22262c` (slightly lighter)
  - Text: `#e6e9ee` (light gray)
  
- **Monospace Font**: Code-style font for terminal text
- **Scrollable**: Output area scrolls when content exceeds visible space
- **Fixed Height**: 250px by default
- **Full Width**: Spans entire main content area

## User Flow Example

1. Type `ls` in terminal input → Press Enter
2. Output appears: file listings
3. Type `cargo check` → Press Enter
4. See compilation output in real-time
5. Continue chatting with Claude Code above
6. Switch between terminal commands and AI chat seamlessly
