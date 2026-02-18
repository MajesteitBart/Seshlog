---
description: List all work currently in progress
allowed-tools: ListFiles, Read
---

Display all work currently in progress:

## In Progress Tasks
===================

### Find In Progress Tasks:
1. Check `.project/epics/` directory for task files (matching `[0-9]*.md`)
2. Look for tasks with status: in-progress

### Display Format:
For each in-progress task, display:
- Task ID and title
- Associated Epic
- Status (in-progress)
- Priority level (if available)
- Brief description

### Summary:
Provide a count of:
- Total in-progress tasks
- In-progress tasks by priority (critical, high, medium, low)
- In-progress tasks by epic

### If No In Progress Tasks:
If no in-progress tasks are found, display a message indicating no tasks are currently in progress.

Note: This command currently provides a manual overview. The automated in-progress tasks finder will be available after scripts are migrated from `mirror/scripts/pm/in-progress.sh` to `.claude/scripts/pm/in-progress.sh`.
