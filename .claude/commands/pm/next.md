---
description: Show the next priority task to work on based on task status and priority
allowed-tools: ListFiles, Read
---

Display the next priority task to work on:

## Next Priority Task
===================

### Find the Next Task:
1. Check `.project/epics/` directory for task files (matching `[0-9]*.md`)
2. Look for tasks with the highest priority that are:
   - Open (status: open)
   - Not blocked (no blocked: true or blocked-by field)

### Task Priority Order:
- Priority: critical
- Priority: high
- Priority: medium
- Priority: low

### Display Format:
For the highest priority task found, display:
- Task ID and title
- Associated Epic
- Priority level
- Brief description
- Status

### If No Tasks Found:
If no open tasks are available, display a message indicating all tasks are completed or no tasks exist.

Note: This command currently provides a manual overview. The automated next task finder will be available after scripts are migrated from `mirror/scripts/pm/next.sh` to `.claude/scripts/pm/next.sh`.
