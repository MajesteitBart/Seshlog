---
description: Display epic details and its associated tasks
argument-hint: <epic_name>
allowed-tools: ListFiles, Read
---

Display epic details and its associated tasks.

## Usage
```
/pm:epic-show <epic_name>
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
- **created**: Creation date

### 3. Display Epic Details
```
Epic: $ARGUMENTS
================

Status: {status}
Progress: {progress}
Created: {created}
PRD: {prd_path}
GitHub: {github_issue}

## Overview
{Epic overview section}

## Architecture Decisions
{Architecture decisions section}

## Technical Approach
{Technical approach sections}

## Implementation Strategy
{Implementation strategy section}

## Dependencies
{Dependencies section}

## Success Criteria (Technical)
{Success criteria section}

## Estimated Effort
{Estimated effort section}
```

### 4. List Tasks
Check `.project/epics/$ARGUMENTS/` directory for task files (files matching `[0-9]*.md`):
For each task, extract:
- **Task ID**: From filename
- **Title**: From task frontmatter
- **Status**: From task frontmatter

Display tasks in a table:
```
Tasks
=====
| ID  | Title                  | Status       |
|-----|------------------------|--------------|
| 001 | First task title       | in-progress  |
| 002 | Second task title      | open         |
| 003 | Third task title       | closed       |
```

### 5. Summary
At the end, show:
- Total number of tasks
- Tasks by status (open, in-progress, closed, blocked)

### 6. No Tasks Found
If no task files exist:
```
No tasks found for this epic.
Create tasks with: /pm:epic-decompose $ARGUMENTS
```

Note: This command currently provides a manual overview. The automated epic show script will be available after scripts are migrated from `mirror/scripts/pm/epic-show.sh` to `.claude/scripts/pm/epic-show.sh`.
