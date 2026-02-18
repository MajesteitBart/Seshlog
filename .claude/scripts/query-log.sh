#!/bin/bash
# query-log.sh - Query the JSONL change log
#
# Usage: query-log.sh [options]
#
# Options:
#   --type <type>      - Filter by event type
#   --agent <agent>    - Filter by agent
#   --epic <name>      - Filter by epic
#   --since <date>     - Events since date (ISO format)
#   --last <n>         - Show last n events
#   --pretty           - Pretty print output

LOG_FILE=".claude/logs/changes.jsonl"

# Default values
FILTER=""
LAST=""
PRETTY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --type) FILTER+=" | select(.type == \"$2\")"; shift 2 ;;
        --agent) FILTER+=" | select(.agent == \"$2\")"; shift 2 ;;
        --epic) FILTER+=" | select(.context.epic == \"$2\")"; shift 2 ;;
        --since) FILTER+=" | select(.timestamp >= \"$2\")"; shift 2 ;;
        --last) LAST="$2"; shift 2 ;;
        --pretty) PRETTY=true; shift ;;
        *) shift ;;
    esac
done

# Build jq command
if [[ -n "$LAST" ]]; then
    CMD="tail -n $LAST \"$LOG_FILE\""
else
    CMD="cat \"$LOG_FILE\""
fi

if [[ -n "$FILTER" ]]; then
    if [ "$PRETTY" = true ]; then
        eval "$CMD" | jq -c ".$FILTER" 2>/dev/null | jq '.'
    else
        eval "$CMD" | jq -c ".$FILTER" 2>/dev/null
    fi
else
    if [ "$PRETTY" = true ]; then
        eval "$CMD" | jq '.' 2>/dev/null
    else
        eval "$CMD"
    fi
fi
