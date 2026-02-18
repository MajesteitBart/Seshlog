# PM Commands Reference

Quick reference for all project management commands. Always suggest these to users for file operations.

## PRD Commands

| Command | Purpose | When to Suggest |
|---------|---------|-----------------|
| `/pm:prd-new <name>` | Create new PRD through brainstorming | User wants to plan a new feature |
| `/pm:prd-parse <name>` | Extract structure from existing PRD | PRD exists but needs epic breakdown |
| `/pm:prd-list` | List all PRDs with status | User asks "what PRDs exist?" |
| `/pm:prd-status <name>` | Show detailed PRD status | User wants PRD progress |

## Epic Commands

| Command | Purpose | When to Suggest |
|---------|---------|-----------------|
| `/pm:epic-start <name>` | Create new epic | PRD approved, ready for implementation planning |
| `/pm:epic-decompose <name>` | Break epic into tasks | Epic defined, needs task breakdown |
| `/pm:epic-sync <name>` | Push tasks to GitHub Issues | Tasks ready for tracking |
| `/pm:epic-oneshot <name>` | Decompose + sync in one step | Quick workflow for simple epics |
| `/pm:epic-list` | List all epics with status | User asks "what epics exist?" |
| `/pm:epic-show <name>` | Show epic details and tasks | User wants epic progress |

## Task/Issue Commands

| Command | Purpose | When to Suggest |
|---------|---------|-----------------|
| `/pm:issue-start <id>` | Begin work on a task | User ready to implement |
| `/pm:issue-sync <id>` | Push progress to GitHub | Task has updates to share |
| `/pm:issue-close <id>` | Complete and close task | Task finished |
| `/pm:next` | Get recommended next task | User asks "what should I work on?" |

## Status Commands

| Command | Purpose | When to Suggest |
|---------|---------|-----------------|
| `/pm:status` | Full project dashboard | Overview request |
| `/pm:epic-show <name>` | Epic + task details | Epic-specific status |

## Command Flow Examples

### New Feature Flow
```
/pm:prd-new auth-system      # 1. Create PRD
/pm:epic-start auth-backend  # 2. Create first epic
/pm:epic-decompose auth-backend  # 3. Break into tasks
/pm:epic-sync auth-backend   # 4. Push to GitHub
/pm:issue-start 123          # 5. Start first task
```

### Quick Check Flow
```
/pm:status                   # What's the overall state?
/pm:next                     # What should I work on?
/pm:issue-start <id>         # Start that task
```

### Epic Completion Flow
```
/pm:epic-show my-epic        # Check remaining tasks
/pm:issue-close <last-id>    # Close final task
/pm:epic-close my-epic       # Close the epic
```

## Handoff Patterns

When providing guidance, always end with the appropriate command:

| Guidance Type | Suggested Command |
|---------------|-------------------|
| PRD content suggestions | `/pm:prd-new <name>` |
| Epic breakdown suggestions | `/pm:epic-start <name>` |
| Task decomposition suggestions | `/pm:epic-decompose <name>` |
| Next work item suggestion | `/pm:next` or `/pm:issue-start <id>` |
| Progress summary | `/pm:status` |
