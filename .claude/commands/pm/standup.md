---
description: Display daily standup report showing recent activity across PRDs, epics, and tasks
allowed-tools: ListFiles, Read
---

Display a daily standup report showing recent activity:

## Daily Standup Report
=====================

### Recent PRD Activity:
Check `.project/prds/` directory for recently modified PRD files.

### Recent Epic Activity:
Check `.project/epics/` directory for recently modified epic directories and task files.

### Recent Task Activity:
Look for task files (matching `[0-9]*.md`) with recent status changes:
- Tasks recently opened (status: open)
- Tasks recently closed (status: closed)
- Tasks in progress (status: in-progress)

### Summary:
Provide a brief summary of:
- Total PRDs
- Total Epics
- Open tasks count
- Closed tasks count
- In-progress tasks count

Note: This command currently provides a manual overview. The automated standup report script will be available after scripts are migrated from `mirror/scripts/pm/standup.sh` to `.claude/scripts/pm/standup.sh`.
