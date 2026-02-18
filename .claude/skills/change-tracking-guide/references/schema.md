# Change Tracking Schema Reference

## Event Structure

Every JSONL entry follows this structure:

```json
{
  "timestamp": "2024-12-24T10:30:00Z",
  "eventType": "file.modified",
  "sessionId": "session-abc123",
  "actor": "user",
  "data": { ... }
}
```

### Core Fields

| Field | Type | Description |
|-------|------|-------------|
| `timestamp` | ISO 8601 | When event occurred |
| `eventType` | string | Event category and action |
| `sessionId` | string | Claude Code session identifier |
| `actor` | string | Who triggered: `user`, `agent`, `skill`, `hook` |
| `data` | object | Event-specific payload |

## Event Types

### File Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `file.created` | New file written | `path`, `size`, `contentSummary` |
| `file.modified` | File updated | `path`, `changesSummary`, `linesChanged` |
| `file.deleted` | File removed | `path` |

### PRD Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `prd.created` | `/pm:prd-new` | `name`, `path` |
| `prd.updated` | PRD modified | `name`, `path`, `changesSummary` |
| `prd.status_changed` | Status update | `name`, `oldStatus`, `newStatus` |

### Epic Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `epic.created` | `/pm:prd-parse` | `name`, `path`, `prdSource` |
| `epic.decomposed` | `/pm:epic-decompose` | `name`, `taskCount` |
| `epic.status_changed` | Status update | `name`, `oldStatus`, `newStatus` |
| `epic.synced` | `/pm:epic-sync` | `name`, `issueNumber` |

### Task Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `task.created` | Epic decomposition | `epicName`, `taskId`, `name` |
| `task.started` | `/pm:issue-start` | `taskId`, `agentId` |
| `task.completed` | Task finished | `taskId`, `duration` |
| `task.status_changed` | Status update | `taskId`, `oldStatus`, `newStatus` |

### GitHub Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `github.issue_created` | Issue pushed | `issueNumber`, `title`, `entityType` |
| `github.issue_synced` | Issue updated | `issueNumber`, `direction` |
| `github.issue_closed` | Issue completed | `issueNumber` |

### Agent Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `agent.spawned` | Task tool | `agentId`, `agentType`, `taskDescription` |
| `agent.completed` | Agent finishes | `agentId`, `resultSummary` |

### Skill Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `skill.invoked` | Skill triggered | `skillName`, `trigger` |
| `skill.completed` | Skill finishes | `skillName`, `resultSummary` |

### Context Events

| Type | Trigger | Data Fields |
|------|---------|-------------|
| `context.loaded` | `/context:prime` | `files`, `tokenCount` |
| `context.updated` | `/context:update` | `files`, `changesSummary` |

## Query Patterns

### By Date Range
```bash
# Specific date
grep "2024-12-24" changes.jsonl

# Date range (requires jq)
jq 'select(.timestamp >= "2024-12-20" and .timestamp <= "2024-12-24")' changes.jsonl
```

### By Actor
```bash
grep '"actor":"agent"' changes.jsonl
grep '"actor":"user"' changes.jsonl
```

### By Entity
```bash
# All events for an epic
grep '"name":"my-epic"' changes.jsonl

# Events containing entity ID
grep 'entity-id-here' changes.jsonl
```

### Combined Filters
```bash
# Today's file modifications by user
grep "$(date +%Y-%m-%d)" changes.jsonl | grep '"eventType":"file.modified"' | grep '"actor":"user"'
```

## Status Values

### PRD Status
`draft` → `backlog` → `in-progress` → `review` → `complete`

### Epic Status
`draft` → `backlog` → `in-progress` → `review` → `complete`

### Task Status
`open` → `in-progress` → `review` → `closed`

**Note:** The `review` status is optional. When `review_required: true` is set, AI agents must transition through `review` before completing.
