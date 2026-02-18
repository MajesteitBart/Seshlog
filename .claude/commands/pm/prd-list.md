---
description: List all Product Requirements Documents with their status and metadata
allowed-tools: ListFiles, Read
---

List all Product Requirements Documents (PRDs) with their status and metadata.

## Instructions

### 1. List PRD Files
Check `.project/prds/` directory for all PRD files (files ending in `.md`).

### 2. Extract Metadata
For each PRD file, read and extract the frontmatter:
- **name**: The PRD identifier
- **description**: Brief description of the PRD
- **status**: Current status (draft, backlog, in-progress, complete)
- **created**: Creation date
- **updated**: Last update date (if present)

### 3. Display Format
Show the PRDs in a table format:

```
PRD List
========

| Name          | Description                     | Status      | Created    |
|---------------|---------------------------------|-------------|------------|
| user-auth     | User authentication system      | draft       | 2025-12-20 |
| payment-v2    | Payment gateway integration     | in-progress | 2025-12-22 |
| notifications | Real-time notification service  | complete    | 2025-12-15 |
```

### 4. Summary
At the end, show:
- Total number of PRDs
- Count by status (draft, backlog, in-progress, complete)

### 5. No PRDs Found
If `.project/prds/` directory is empty or doesn't exist:
```
No PRDs found.
Create a new PRD with: /pm:prd-new <feature_name>
```

Note: This command currently provides a manual overview. The automated PRD list script will be available after scripts are migrated from `mirror/scripts/pm/prd-list.sh` to `.claude/scripts/pm/prd-list.sh`.
