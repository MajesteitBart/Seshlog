---
name: context-management
description: Helps gather and maintain project context. Activates for context questions, session startup help, onboarding, or detecting stale context. Read-only analysis.
allowed-tools: Read, Glob, Grep
---

# Context Management

Help users understand and manage project context effectively.

## When to Activate

Activate this skill when the user:
- Starts a session needing context orientation
- Asks "what's the current context?"
- Is onboarding a new team member
- Needs to understand the project state
- Asks about what work is in progress

## Context Sources

### Primary Sources

Read and analyze these files:

- `.project/prds/*.md` - Product requirements documents
- `.project/epics/**/epic.md` - Implementation plans
- `.claude/logs/changes.jsonl` - Recent activity log
- `CLAUDE.md` - Project instructions
- `README.md` - Project overview
- `.claude/settings.json` - Project configuration

## Analysis Types

### Context Summary

Provide overview of:
- Active PRDs and their status
- In-progress epics
- Recent changes from logs
- Key project files
- Project configuration

### Gap Analysis

Identify:
- PRDs without corresponding epics
- Epics without tasks
- Stale files (not updated recently)
- Missing documentation
- Incomplete work items

### Onboarding Guide

For new users, provide:
1. Project overview
2. Current work in progress
3. Key files to read
4. Suggested starting points
5. Team workflow understanding

## Output Format

Provide context analysis in this structure:

```markdown
## Context Summary

### Active Work
| Item | Type | Status | Last Updated |
|------|------|--------|--------------|
| {prd/epic name} | PRD/Epic | {status} | {date} |

### Recent Activity
- {recent change from logs}
- {another recent change}

### Gaps Identified
- {missing item}
- {incomplete work}

### Recommended Reading
1. {file} - {why important}
2. {file} - {why important}

### Suggested Next Steps
1. {actionable step}
2. {actionable step}
```

## Detection Patterns

### Stale Context Detection

Look for indicators of stale context:
- PRDs older than 30 days without epic
- Epics without recent task activity
- Files not modified in extended periods
- Orphaned work items

### Missing Context Detection

Identify missing context:
- No PRDs for active features
- Epics without task breakdown
- Missing documentation for key areas
- Untracked work

## Boundaries

### CAN Do ✅
- Read and analyze context files
- Identify gaps and staleness
- Summarize current state
- Recommend reading order
- Provide onboarding guidance
- Answer context-related questions

### CANNOT Do ❌
- Create or modify context files
- Update staleness markers
- Write documentation
- Make changes to project files
- Execute commands to fix issues

## Related Commands

Useful commands for context management:
- `/pm:prd-list` - List all PRDs
- `/pm:prd-status <name>` - Check PRD status
- `/pm:epic-list` - List all epics
- `/pm:epic-status` - Check epic status
- `/pm:standup` - Get work status summary
- `/pm:in-progress` - Show in-progress work
