#!/bin/bash
# log-event.sh - Append events to the JSONL change log
#
# This script wraps log-event.js for backward compatibility.
# For cross-platform support, use log-event.js directly.
#
# Usage: log-event.sh <type> <agent> [options]
#
# Arguments:
#   type    - Event type: file_create, file_modify, file_delete, command_run, session_start, session_end
#   agent   - Agent name or 'user'
#
# Options:
#   --epic <name>      - Associated epic name
#   --task <id>        - Associated task ID
#   --path <path>      - File path (for file events)
#   --size <bytes>     - File size (for file_create)
#   --added <lines>    - Lines added (for file_modify)
#   --removed <lines>  - Lines removed (for file_modify)
#   --command <cmd>    - Command name (for command_run)
#   --args <args>      - Command arguments (for command_run)
#   --exit <code>      - Exit code (for command_run)
#   --session <id>     - Session ID (for session events)
#   --duration <secs>  - Duration in seconds (for session_end)
#   --note <text>      - Optional note

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Try to use Node.js version first (cross-platform)
if command -v node &> /dev/null; then
    node "$SCRIPT_DIR/log-event.js" "$@"
    exit $?
fi

# Fallback to original bash implementation if Node.js is not available
echo "Warning: Node.js not found, using bash fallback" >&2

LOG_FILE=".claude/logs/changes.jsonl"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Parse arguments
TYPE=$1
AGENT=$2
shift 2

# Initialize optional fields
EPIC=""
TASK=""
declare -A DATA

while [[ $# -gt 0 ]]; do
    case $1 in
        --epic) EPIC="$2"; shift 2 ;;
        --task) TASK="$2"; shift 2 ;;
        --path) DATA[path]="$2"; shift 2 ;;
        --size) DATA[size]="$2"; shift 2 ;;
        --added) DATA[lines_added]="$2"; shift 2 ;;
        --removed) DATA[lines_removed]="$2"; shift 2 ;;
        --command) DATA[command]="$2"; shift 2 ;;
        --args) DATA[args]="$2"; shift 2 ;;
        --exit) DATA[exit_code]="$2"; shift 2 ;;
        --session) DATA[session_id]="$2"; shift 2 ;;
        --duration) DATA[duration]="$2"; shift 2 ;;
        --note) DATA[note]="$2"; shift 2 ;;
        *) shift ;;
    esac
done

# Build context JSON
CONTEXT="{}"
if [[ -n "$EPIC" && -n "$TASK" ]]; then
    CONTEXT="{\"epic\":\"$EPIC\",\"task\":\"$TASK\"}"
elif [[ -n "$EPIC" ]]; then
    CONTEXT="{\"epic\":\"$EPIC\"}"
elif [[ -n "$TASK" ]]; then
    CONTEXT="{\"task\":\"$TASK\"}"
fi

# Build data JSON
DATA_JSON="{"
FIRST=true
for key in "${!DATA[@]}"; do
    if [ "$FIRST" = true ]; then
        FIRST=false
    else
        DATA_JSON+=","
    fi
    # Handle numeric vs string values
    if [[ "${DATA[$key]}" =~ ^[0-9]+$ ]]; then
        DATA_JSON+="\"$key\":${DATA[$key]}"
    else
        DATA_JSON+="\"$key\":\"${DATA[$key]}\""
    fi
done
DATA_JSON+="}"

# Build and append the event
EVENT="{\"timestamp\":\"$TIMESTAMP\",\"type\":\"$TYPE\",\"agent\":\"$AGENT\",\"context\":$CONTEXT,\"data\":$DATA_JSON}"
echo "$EVENT" >> "$LOG_FILE"

echo "Logged: $TYPE by $AGENT"
