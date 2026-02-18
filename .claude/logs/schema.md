# Change Log Schema

## Event Types

| Type | Description | Required Fields |
|------|-------------|-----------------|
| `file_create` | New file created | `path`, `size` |
| `file_modify` | File modified | `path`, `lines_added`, `lines_removed` |
| `file_delete` | File deleted | `path` |
| `command_run` | Command executed | `command`, `args`, `exit_code` |
| `session_start` | Session began | `session_id` |
| `session_end` | Session ended | `session_id`, `duration` |

## Base Event Structure

```json
{
  "timestamp": "ISO-8601 datetime",
  "type": "event_type",
  "agent": "agent name or 'user'",
  "context": {
    "epic": "epic-name (if applicable)",
    "task": "task-id (if applicable)"
  },
  "data": { ... type-specific fields ... }
}
```

## Example Events

```jsonl
{"timestamp":"2024-12-24T18:00:00Z","type":"session_start","agent":"user","context":{},"data":{"session_id":"abc123"}}
{"timestamp":"2024-12-24T18:01:00Z","type":"file_create","agent":"code-mode","context":{"epic":"setup"},"data":{"path":".claude/logs/schema.md","size":1024}}
```
