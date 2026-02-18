---
description: Show all blocked tasks with their blocking reasons
allowed-tools: ListFiles, Read
---

Display all blocked tasks with their blocking reasons:

## Blocked Tasks
===============

### Find Blocked Tasks:
1. Check `.project/epics/` directory for task files (matching `[0-9]*.md`)
2. Look for tasks that are blocked:
   - Have `blocked: true` field
   - Have `blocked-by` field with a reason or reference

### Display Format:
For each blocked task, display:
- Task ID and title
- Associated Epic
- Blocking reason (from blocked-by field)
- Status
- Priority level (if available)

### Summary:
Provide a count of:
- Total blocked tasks
- Blocked tasks by priority (critical, high, medium, low)

### If No Blocked Tasks:
If no blocked tasks are found, display a message indicating no tasks are currently blocked.

Note: This command currently provides a manual overview. The automated blocked tasks finder will be available after scripts are migrated from `mirror/scripts/pm/blocked.sh` to `.claude/scripts/pm/blocked.sh`.
