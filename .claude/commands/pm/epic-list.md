---
description: List all epics with their status, progress, and linked PRD
allowed-tools: ListFiles, Read
---

List all epics with their status, progress, and linked PRD.

## Instructions

### 1. List Epic Directories
Check `.project/epics/` directory for all subdirectories (each represents an epic).

### 2. For Each Epic, Extract:
From `.project/epics/{epic_name}/epic.md`, read and extract the frontmatter:
- **name**: Epic identifier
- **status**: Current status (open, in-progress, closed)
- **progress**: Percentage complete
- **prd**: Path to linked PRD (if any)
- **github**: GitHub issue reference (if synced)
- **created**: Creation date

### 3. Display Format
Show the epics in a table format:

```
Epic List
=========

| Name          | Status      | Progress | PRD               | GitHub   |
|---------------|-------------|----------|-------------------|----------|
| user-auth     | open        | 0%       | .project/prds/...  | -        |
| payment-v2    | in-progress | 60%      | .project/prds/...  | #123     |
| notifications | closed      | 100%     | .project/prds/...  | #100     |
```

### 4. Summary
At the end, show:
- Total number of epics
- Count by status (open, in-progress, closed)
- Epics without linked PRDs
- Epics not synced to GitHub

### 5. No Epics Found
If `.project/epics/` directory is empty or doesn't exist:
```
No epics found.
Create an epic from a PRD with: /pm:prd-parse <feature_name>
```

Note: This command currently provides a manual overview. The automated epic list script will be available after scripts are migrated from `mirror/scripts/pm/epic-list.sh` to `.claude/scripts/pm/epic-list.sh`.
