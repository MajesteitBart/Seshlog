#!/usr/bin/env node
/**
 * log-event.js - Cross-platform logging utility for JSONL change tracking
 *
 * Usage: node log-event.js <type> <agent> [options]
 *
 * Arguments:
 *   type    - Event type: file_create, file_modify, file_delete, command_run, session_start, session_end
 *   agent   - Agent name or 'user'
 *
 * Options:
 *   --epic <name>      - Associated epic name
 *   --task <id>        - Associated task ID
 *   --path <path>      - File path (for file events)
 *   --size <bytes>     - File size (for file_create)
 *   --added <lines>    - Lines added (for file_modify)
 *   --removed <lines>  - Lines removed (for file_modify)
 *   --command <cmd>    - Command name (for command_run)
 *   --args <args>      - Command arguments (for command_run)
 *   --exit <code>      - Exit code (for command_run)
 *   --session <id>     - Session ID (for session events)
 *   --duration <secs>  - Duration in seconds (for session_end)
 *   --note <text>      - Optional note
 *   --json             - Output JSON to stdout (for hook consumption)
 *   --quiet            - Suppress confirmation message
 */

const fs = require('fs');
const path = require('path');

// Configuration
const LOG_FILE = '.claude/logs/changes.jsonl';

// Valid event types
const VALID_TYPES = [
  'file_create',
  'file_modify',
  'file_delete',
  'command_run',
  'session_start',
  'session_end'
];

// Parse command line arguments
function parseArgs(args) {
  const result = {
    type: null,
    agent: null,
    epic: null,
    task: null,
    data: {},
    json: false,
    quiet: false
  };

  // First two positional arguments
  if (args.length < 2) {
    console.error('Usage: node log-event.js <type> <agent> [options]');
    process.exit(1);
  }

  result.type = args[0];
  result.agent = args[1];

  // Parse options
  let i = 2;
  while (i < args.length) {
    const arg = args[i];

    switch (arg) {
      case '--epic':
        result.epic = args[++i];
        break;
      case '--task':
        result.task = args[++i];
        break;
      case '--path':
        result.data.path = args[++i];
        break;
      case '--size':
        result.data.size = parseInt(args[++i], 10);
        break;
      case '--added':
        result.data.lines_added = parseInt(args[++i], 10);
        break;
      case '--removed':
        result.data.lines_removed = parseInt(args[++i], 10);
        break;
      case '--command':
        result.data.command = args[++i];
        break;
      case '--args':
        result.data.args = args[++i];
        break;
      case '--exit':
        result.data.exit_code = parseInt(args[++i], 10);
        break;
      case '--session':
        result.data.session_id = args[++i];
        break;
      case '--duration':
        result.data.duration = parseInt(args[++i], 10);
        break;
      case '--note':
        result.data.note = args[++i];
        break;
      case '--json':
        result.json = true;
        break;
      case '--quiet':
        result.quiet = true;
        break;
      default:
        // Skip unknown arguments
        break;
    }
    i++;
  }

  return result;
}

// Validate event type
function validateType(type) {
  if (!VALID_TYPES.includes(type)) {
    console.error(`Invalid event type: ${type}`);
    console.error(`Valid types: ${VALID_TYPES.join(', ')}`);
    process.exit(1);
  }
}

// Build the event object
function buildEvent(parsed) {
  const timestamp = new Date().toISOString().replace(/\.\d{3}Z$/, 'Z');

  // Build context
  const context = {};
  if (parsed.epic) context.epic = parsed.epic;
  if (parsed.task) context.task = parsed.task;

  return {
    timestamp,
    type: parsed.type,
    agent: parsed.agent,
    context,
    data: parsed.data
  };
}

// Ensure log directory exists
function ensureLogDir(logPath) {
  const dir = path.dirname(logPath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

// Append event to log file (atomic operation)
function appendEvent(event, logPath) {
  ensureLogDir(logPath);

  const line = JSON.stringify(event) + '\n';

  // Use appendFileSync for atomic append
  // On most systems, appends under PIPE_BUF (4KB on Linux, 512B on POSIX) are atomic
  fs.appendFileSync(logPath, line, { encoding: 'utf8' });
}

// Main execution
function main() {
  const args = process.argv.slice(2);

  if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
    console.log(`
log-event.js - Cross-platform logging utility for JSONL change tracking

Usage: node log-event.js <type> <agent> [options]

Event Types:
  file_create   - New file created (requires: --path, --size)
  file_modify   - File modified (requires: --path, --added, --removed)
  file_delete   - File deleted (requires: --path)
  command_run   - Command executed (requires: --command, --args, --exit)
  session_start - Session began (requires: --session)
  session_end   - Session ended (requires: --session, --duration)

Options:
  --epic <name>      - Associated epic name
  --task <id>        - Associated task ID
  --path <path>      - File path (for file events)
  --size <bytes>     - File size (for file_create)
  --added <lines>    - Lines added (for file_modify)
  --removed <lines>  - Lines removed (for file_modify)
  --command <cmd>    - Command name (for command_run)
  --args <args>      - Command arguments (for command_run)
  --exit <code>      - Exit code (for command_run)
  --session <id>     - Session ID (for session events)
  --duration <secs>  - Duration in seconds (for session_end)
  --note <text>      - Optional note
  --json             - Output JSON to stdout (for hook consumption)
  --quiet            - Suppress confirmation message

Examples:
  node log-event.js file_create claude --path "src/app.js" --size 1024
  node log-event.js file_modify user --path "README.md" --added 10 --removed 5
  node log-event.js command_run user --command "npm test" --args "" --exit 0
  node log-event.js session_start user --session "abc123"
`);
    process.exit(0);
  }

  const parsed = parseArgs(args);
  validateType(parsed.type);

  const event = buildEvent(parsed);

  // Append to log file
  appendEvent(event, LOG_FILE);

  // Output JSON if requested (for hook consumption)
  if (parsed.json) {
    console.log(JSON.stringify(event));
  }

  // Confirmation message
  if (!parsed.quiet) {
    console.log(`Logged: ${parsed.type} by ${parsed.agent}`);
  }
}

// Run if called directly
if (require.main === module) {
  main();
}

// Export for use as module
module.exports = { buildEvent, appendEvent, parseArgs, VALID_TYPES, LOG_FILE };
