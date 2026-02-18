---
description: Check epic progress and task completion status
argument-hint: <epic_name>
allowed-tools: ListFiles, Read
---

Check epic progress and task completion status.

## Usage
```
/pm:epic-status <epic_name>
```

## Instructions

### 1. Validate Epic Exists
Check if `.project/epics/$ARGUMENTS/epic.md` exists:
- If not found, tell user: "❌ Epic not found: $ARGUMENTS"
- Suggest: "List available epics with: /pm:epic-list"
- Stop execution if epic doesn't exist

### 2. Read Epic Frontmatter
From `.project/epics/$ARGUMENTS/epic.md`, extract:
- **name**: Epic identifier
- **status**: Current status
- **progress**: Percentage complete
- **prd**: Path to linked PRD
- **github**: GitHub issue reference

### 3. List All Tasks
Check `.project/epics/$ARGUMENTS/` directory for task files (files matching `[0-9]*.md`):
For each task, extract:
- **Task ID**: From filename
- **Title**: From task frontmatter
- **Status**: From task frontmatter (open, in-progress, completed, blocked)

### 4. Calculate Progress
Count tasks by status:
- Total tasks
- Completed tasks
- In-progress tasks
- Open tasks
- Blocked tasks

Calculate actual completion percentage: `(completed / total) * 100`

### 5. Display Status Report
```
Epic Status Report: $ARGUMENTS
===============================

Epic Status: {status}
Recorded Progress: {progress}%
Actual Progress: {calculated_progress}%

Task Summary
------------
Total Tasks: {total}
Completed: {completed} ({percentage}%)
In Progress: {in_progress}
Open: {open}
Blocked: {blocked}

Task Details
-------------
| ID  | Title                  | Status       |
|-----|------------------------|--------------|
| 001 | First task title       | in-progress  |
| 002 | Second task title      | open         |
| 003 | Third task title       | completed    |

PRD Link
--------
{prd_path}

GitHub
------
{github_issue}
```

### 6. Progress Discrepancy Check
If recorded progress differs from actual progress by more than 5%:
```
⚠️ Progress discrepancy detected:
   Recorded: {progress}%
   Actual: {calculated_progress}%
   Update epic with: /pm:epic-edit $ARGUMENTS
```

### 7. No Tasks Found
If no task files exist:
```
No tasks found for this epic.
Create tasks with: /pm:epic-decompose $ARGUMENTS
```

### 8. Suggested Next Steps
Based on current status:
- If tasks are blocked: "Check blocked tasks with: /pm:blocked"
- If tasks in progress: "Continue working on in-progress tasks"
- If all open: "Start next task with: /pm:next"

Note: This command currently provides a manual overview. The automated epic status script will be available after scripts are migrated from `mirror/scripts/pm/epic-status.sh` to `.claude/scripts/pm/epic-status.sh`.
