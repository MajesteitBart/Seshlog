---
name: change-tracking-guide
description: Helps query and understand JSONL change logs. Activates for activity queries, audit requests, session reviews, or questions about what changed. Read-only log analysis.
allowed-tools: Read, Bash, Grep
---

# Change Tracking Guide

Help users understand and query the JSONL change tracking logs.

## When to Activate

Activate this skill when the user:
- Asks "what changed recently?"
- Requests an audit or review
- Needs a session activity summary
- Asks about file history
- Wants to understand what work was done

## Log Location

Main change log: `.claude/logs/changes.jsonl`

Schema documentation: `.claude/logs/schema.md`

## Query Capabilities

### Recent Activity

```bash
# Last 10 events
tail -n 10 .claude/logs/changes.jsonl | jq '.'

# Last 20 events
tail -n 20 .claude/logs/changes.jsonl | jq '.'

# Events from today
grep "$(date +%Y-%m-%d)" .claude/logs/changes.jsonl | jq '.'

# Events from yesterday
grep "$(date -d 'yesterday' +%Y-%m-%d)" .claude/logs/changes.jsonl | jq '.'
```

### Filter by Type

```bash
# File creations only
jq -c 'select(.type == "file_create")' .claude/logs/changes.jsonl

# File modifications only
jq -c 'select(.type == "file_modify")' .claude/logs/changes.jsonl

# Command runs
jq -c 'select(.type == "command_run")' .claude/logs/changes.jsonl

# PRD operations
jq -c 'select(.type | startswith("prd_"))' .claude/logs/changes.jsonl

# Epic operations
jq -c 'select(.type | startswith("epic_"))' .claude/logs/changes.jsonl
```

### Filter by Context

```bash
# Events for specific epic
jq -c 'select(.context.epic == "epic-name")' .claude/logs/changes.jsonl

# Events for specific PRD
jq -c 'select(.context.prd == "prd-name")' .claude/logs/changes.jsonl

# Events by agent
jq -c 'select(.agent == "code-mode")' .claude/logs/changes.jsonl

# Events by mode
jq -c 'select(.context.mode == "architect")' .claude/logs/changes.jsonl
```

### Filter by File

```bash
# Events affecting specific file
jq -c 'select(.file == "path/to/file.md")' .claude/logs/changes.jsonl

# Events affecting files in directory
jq -c 'select(.file | startswith("src/"))' .claude/logs/changes.jsonl
```

### Count and Statistics

```bash
# Count events by type
jq -r '.type' .claude/logs/changes.jsonl | sort | uniq -c | sort -rn

# Count events today
grep "$(date +%Y-%m-%d)" .claude/logs/changes.jsonl | wc -l

# Count files modified
jq -c 'select(.type == "file_modify")' .claude/logs/changes.jsonl | jq -r '.file' | sort -u | wc -l
```

### Time-Based Queries

```bash
# Events in last hour (requires date parsing)
jq -c 'select(.timestamp | startswith("2025-12-24T1"))' .claude/logs/changes.jsonl

# Events in date range
jq -c 'select(.timestamp >= "2025-12-20" and .timestamp <= "2025-12-24")' .claude/logs/changes.jsonl
```

## Output Format

Provide activity reports in this structure:

```markdown
## Activity Report

### Summary
- **Period**: {time range}
- **Total Events**: {count}
- **Files Modified**: {count}

### Events by Type
| Type | Count |
|------|-------|
| file_create | {n} |
| file_modify | {n} |
| command_run | {n} |
| prd_create | {n} |
| epic_update | {n} |

### Recent Activity
| Time | Type | Description |
|------|------|-------------|
| {timestamp} | {type} | {summary} |
| {timestamp} | {type} | {summary} |

### Query Used
```bash
{the query command}
```

### Key Changes
- {significant change 1}
- {significant change 2}
```

## Schema Reference

Direct users to [`.claude/logs/schema.md`](.claude/logs/schema.md) for complete schema documentation including:
- Event types
- Field definitions
- Context structures
- Timestamp formats

## Common Use Cases

### Session Review

When reviewing what was done in a session:
1. Filter by timestamp range
2. Group by event type
3. Highlight key changes

### Audit Trail

When auditing specific changes:
1. Filter by file or epic
2. Show chronological sequence
3. Identify who made changes

### Progress Check

When checking progress on work:
1. Filter by epic or PRD
2. Count completed tasks
3. Identify blockers

## Boundaries

### CAN Do ✅
- Read and query log files
- Run bash commands for filtering (jq, grep, tail, wc)
- Explain schema format
- Summarize activity
- Provide query examples
- Help construct custom queries

### CANNOT Do ❌
- Write to log files
- Delete or modify log entries
- Create new log files
- Alter the logging system
- Execute commands that modify logs

## Related Commands

Useful commands for change tracking:
- `/pm:standup` - Get work status summary
- `/pm:in-progress` - Show in-progress work
- `/pm:epic-status` - Check epic status
- `/pm:prd-status <name>` - Check PRD status
