# Terminal Feature

## Overview
Openpad now includes an integrated terminal at the bottom of the application window, allowing you to run shell commands directly within the app while working on your projects.

## Features
- **Cross-platform PTY Support**: Uses `portable-pty` crate for cross-platform pseudo-terminal functionality
- **Shell Integration**: Automatically detects and uses your system's default shell ($SHELL on Unix, PowerShell on Windows)
- **Real-time Output**: Streams command output in real-time to the terminal view
- **Command History**: Type commands and press Enter to execute them
- **Clear Function**: Clear button to reset the terminal output

## Usage

### Basic Terminal Operations
1. **Running Commands**: Type your command in the input field at the bottom of the terminal and press Enter
2. **Viewing Output**: Command output appears in the scrollable terminal output area above the input field
3. **Clearing Output**: Click the "Clear" button in the terminal header to reset the output

### Terminal Layout
- **Header**: Shows "Terminal" label and a "Clear" button
- **Output Area**: Scrollable text view showing command output (supports word wrapping)
- **Input Field**: Text input for entering commands (placeholder: "Enter command...")

### Technical Details
- Terminal initializes automatically when the app starts
- Default size: 250px height (can be adjusted in the code)
- Uses monospace font for terminal output
- Command execution happens in a background thread with output posted back to the main thread via Makepad actions

## Implementation
The terminal is implemented as a custom Makepad widget in `openpad-app/src/components/terminal.rs`:
- `Terminal` struct: Main widget implementation with PTY management
- `TerminalRef`: Reference wrapper for accessing terminal from WidgetRef
- `TerminalAction`: Action enum for handling async terminal events

### Architecture
- PTY (Pseudo-Terminal) creates a real shell process
- Output is read line-by-line in a background thread
- Each line is sent to the main UI thread via `Cx::post_action()`
- Input commands are written to the PTY's master side
- The shell's output appears in real-time in the terminal view

## Limitations
- Currently shows simple text output (no ANSI color support yet)
- Fixed size (no resizable splitter yet)
- Cannot be hidden/shown dynamically (always visible)

## Future Enhancements
- ANSI color code support for colored terminal output
- Resizable terminal panel with drag handle
- Hide/show toggle button
- Command history navigation (up/down arrows)
- Multiple terminal tabs
- Terminal themes/customization
