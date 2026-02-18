---
description: Show PRD implementation status including linked epic and task progress
allowed-tools: ListFiles, Read
---

Show PRD implementation status including linked epic and task progress.

## Instructions

### 1. Find All PRDs
Check `.project/prds/` directory for all PRD files (files ending in `.md`).

### 2. For Each PRD, Extract:
- **name**: PRD identifier
- **description**: Brief description
- **status**: Current PRD status
- **created**: Creation date
- **updated**: Last update date (if present)

### 3. Check for Associated Epic
For each PRD, check if `.project/epics/{prd_name}/epic.md` exists:
- If epic exists, extract epic frontmatter:
  - **epic status**: open, in-progress, closed
  - **progress**: Percentage complete
  - **github**: GitHub issue reference (if synced)

### 4. Display Format
Show PRDs with their implementation status:

```
PRD Status Report
=================

PRD: user-auth
  Status: draft
  Created: 2025-12-20
  Epic: Not created yet
  → Create epic with: /pm:prd-parse user-auth

PRD: payment-v2
  Status: in-progress
  Created: 2025-12-22
  Epic: .project/epics/payment-v2/epic.md
    Epic Status: in-progress
    Progress: 60%
    GitHub: #123

PRD: notifications
  Status: completed
  Created: 2025-12-15
  Epic: .project/epics/notifications/epic.md
    Epic Status: completed
    Progress: 100%
    GitHub: #100
```

### 5. Summary
At the end, show:
- Total PRDs
- PRDs without epics (needs action)
- PRDs with epics in progress
- PRDs with completed epics

### 6. No PRDs Found
If `.project/prds/` directory is empty or doesn't exist:
```
No PRDs found.
Create a new PRD with: /pm:prd-new <feature_name>
```

Note: This command currently provides a manual overview. The automated PRD status script will be available after scripts are migrated from `mirror/scripts/pm/prd-status.sh` to `.claude/scripts/pm/prd-status.sh`.
