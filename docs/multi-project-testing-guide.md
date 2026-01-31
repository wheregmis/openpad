# Multi-Project, Multi-Session Testing Guide

This guide explains how to test the multi-project and multi-session features in Openpad.

## Overview

Openpad now properly supports multiple projects and multiple sessions per project, similar to openchamber. Each session is associated with a specific project directory, and all operations (message sending, loading, reverting, etc.) use the correct directory context.

## Testing Setup

### Prerequisites
1. OpenCode server running (default: localhost:4096)
2. Multiple project directories to test with
3. Openpad built and ready to run

### Recommended Test Projects

Create at least 2-3 test projects in different directories:

```bash
# Project 1: Simple Node.js project
mkdir -p ~/test-projects/nodejs-app
cd ~/test-projects/nodejs-app
echo "console.log('Hello from Node');" > index.js

# Project 2: Python project
mkdir -p ~/test-projects/python-app
cd ~/test-projects/python-app
echo "print('Hello from Python')" > main.py

# Project 3: Generic project
mkdir -p ~/test-projects/generic-project
cd ~/test-projects/generic-project
echo "# Test Project" > README.md
```

## Test Scenarios

### Test 1: Basic Multi-Project Session Creation

**Steps:**
1. Start OpenCode server in one of your test projects
   ```bash
   cd ~/test-projects/nodejs-app
   opencode
   ```

2. Start Openpad
   ```bash
   cd /path/to/openpad
   cargo run --release
   ```

3. Open another terminal and navigate to a different project, then use OpenCode to add it
   ```bash
   cd ~/test-projects/python-app
   opencode  # Or use the OpenCode API to add the project
   ```

4. In Openpad sidebar, verify you see both projects listed

5. Click "+ New session" under the Node.js project
   - **Expected**: A new session is created for the nodejs-app project
   - **Check logs**: Should show directory path for nodejs-app

6. Click "+ New session" under the Python project
   - **Expected**: A new session is created for the python-app project
   - **Check logs**: Should show directory path for python-app

**What to verify:**
- [ ] Sessions appear under the correct project in the sidebar
- [ ] Each session has the correct directory associated with it
- [ ] Logs show the correct normalized directory path for each session

### Test 2: Session Switching Between Projects

**Steps:**
1. Create a session in Project A
2. Send a message: "List all files in this directory"
3. Create a session in Project B
4. Send a message: "List all files in this directory"
5. Switch back to the session in Project A
6. Verify the messages/context are from Project A
7. Switch back to the session in Project B
8. Verify the messages/context are from Project B

**What to verify:**
- [ ] Messages are loaded correctly when switching sessions
- [ ] Each session maintains its own message history
- [ ] Operations in each session affect the correct project directory
- [ ] Logs show correct directory being used for each session

### Test 3: Message Sending with Correct Directory Context

**Steps:**
1. Create sessions in multiple projects
2. In Session A (Project 1): Send "Create a file named test-project1.txt"
3. In Session B (Project 2): Send "Create a file named test-project2.txt"
4. Verify files are created in the correct project directories
5. Check logs to confirm API calls used correct directory parameter

**What to verify:**
- [ ] Files/changes are made in the correct project directory
- [ ] No cross-project contamination
- [ ] Logs show different directory parameters for each session

### Test 4: Session Branching

**Steps:**
1. Create a session in Project A with some conversation history
2. Click the branch button on that session
3. Verify the new branched session is created under the same project
4. Send messages in the branched session
5. Verify both sessions remain independent but in the same project

**What to verify:**
- [ ] Branched session appears under the same project
- [ ] Branched session has the same directory as parent
- [ ] Logs confirm directory inheritance

### Test 5: Session Reverting

**Steps:**
1. Create a session and send multiple messages
2. Click revert on an earlier message
3. Verify messages after that point are removed
4. Send a new message
5. Verify the session continues from the revert point

**What to verify:**
- [ ] Revert operation uses correct directory
- [ ] Messages are properly loaded after revert
- [ ] Logs show directory-aware API calls

### Test 6: Creating Session Without Pre-selecting Project

**Steps:**
1. Open Openpad (no session selected)
2. Type a message and press Enter
3. Verify a session is created in the default/current project
4. Check logs for directory used

**What to verify:**
- [ ] Session is created with normalized directory
- [ ] If current_project exists, it uses that directory
- [ ] Session appears under the correct project in sidebar

## What to Look For in Logs

Enable logging by checking the terminal output when running Openpad. Look for:

### Session Creation Logs
```
Creating session for project: id=xxx, name=Some("project-name"), worktree=/path/to/project, normalized_directory=/full/path/to/project
```

### Message Sending Logs
```
Sending message to session: id=session-123, directory=/path/to/project, project_id=project-xxx
```
OR
```
No session - using current_project: id=project-xxx, worktree=/path/to/project, normalized_dir=/full/path/to/project
```

## Common Issues and Solutions

### Issue: Sessions not grouping by project
**Cause**: Sessions might have different project_id than expected
**Solution**: Check session.project_id matches one of the projects in state.projects

### Issue: Wrong directory being used
**Cause**: Directory not being passed to async operations
**Solution**: Verify logs show directory being passed for each operation

### Issue: "." directory not normalized
**Cause**: Directory normalization not applied consistently
**Solution**: Check that normalize_project_directory is used everywhere

## Validation Checklist

After running all tests, verify:

- [ ] All sessions are grouped correctly by project in sidebar
- [ ] Switching sessions loads correct messages and context
- [ ] Message operations affect correct project directory
- [ ] Session creation uses correct directory parameter
- [ ] Branching preserves parent's directory
- [ ] Reverting uses session's directory
- [ ] Unreverting uses session's directory
- [ ] Logs consistently show correct directory for all operations
- [ ] No cross-project contamination of messages or state
- [ ] UI correctly displays project names and session organization

## Debug Mode

To get more detailed logging, you can:

1. Check the terminal output where you ran `cargo run --release`
2. Look for log messages that include:
   - "Creating session for project"
   - "Sending message to session"
   - "No session - using current_project"

## Success Criteria

The multi-project, multi-session feature is working correctly if:

1. ✅ Sessions are created with the correct directory for their project
2. ✅ Switching between sessions from different projects works seamlessly
3. ✅ All session operations (send, load, revert, branch) use the correct directory
4. ✅ The sidebar correctly groups sessions by project
5. ✅ Logs consistently show the correct directory being used
6. ✅ No operations affect the wrong project directory
