---
description: Show overall project status dashboard with PRDs, epics, and tasks
allowed-tools: ListFiles, Read
---

Display the overall project status including:

## Project Status
================

### PRDs:
Check `.project/prds/` directory and count total PRD files.

### Epics:
Check `.project/epics/` directory and count total epic directories.

### Tasks:
Check `.project/epics/` for task files (files matching `[0-9]*.md`) and count:
- Open tasks (status: open)
- Closed tasks (status: closed)
- Total tasks

Note: This command currently provides a manual overview. The script-based status report will be available after scripts are migrated from `mirror/scripts/pm/status.sh` to `.claude/scripts/pm/status.sh`.
