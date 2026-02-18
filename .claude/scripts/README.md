# Delano Scripts

Utility scripts for the Delano PM system.

## Logging Scripts

### log-event.sh
Append events to the JSONL change log.

**Usage:**
```bash
# Log a file creation
./log-event.sh file_create code-mode --path ".claude/commands/pm/status.md" --size 1024

# Log a command run
./log-event.sh command_run user --command "pm:status" --exit 0

# Log with context
./log-event.sh file_modify agent --epic "setup" --task "1" --path "README.md" --added 10 --removed 2
```

### query-log.sh
Query the JSONL change log.

**Usage:**
```bash
# Show last 10 events
./query-log.sh --last 10

# Filter by type
./query-log.sh --type file_create

# Filter by agent and pretty print
./query-log.sh --agent user --pretty

# Events since a date
./query-log.sh --since "2024-12-24T00:00:00Z"
```
