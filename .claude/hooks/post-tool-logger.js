#!/usr/bin/env node
/**
 * post-tool-logger.js - Claude Code PostToolUse hook for JSONL change tracking
 *
 * This hook is called after Write/Edit/Skill tool operations complete.
 * It receives JSON via stdin and logs the operation to changes.jsonl.
 *
 * Input (via stdin):
 * {
 *   "tool_name": "Write" | "Edit" | "Skill",
 *   "tool_input": { "file_path": "/path/to/file.txt", ... },  // for Write/Edit
 *   "tool_input": { "skill": "pm:status", "args": "..." },    // for Skill
 *   "tool_response": { ... }
 * }
 */

const fs = require('fs');
const path = require('path');

// Get project directory from environment or use current working directory
const PROJECT_DIR = process.env.CLAUDE_PROJECT_DIR || process.cwd();

// Import log-event.js as a module
const logEventModule = require(path.join(PROJECT_DIR, '.claude', 'scripts', 'log-event.js'));

/**
 * Read all of stdin as a string
 */
async function readStdin() {
  return new Promise((resolve, reject) => {
    let data = '';

    // Set a timeout to prevent hanging if no input
    const timeout = setTimeout(() => {
      resolve(data || '{}');
    }, 1000);

    process.stdin.setEncoding('utf8');

    process.stdin.on('data', (chunk) => {
      data += chunk;
    });

    process.stdin.on('end', () => {
      clearTimeout(timeout);
      resolve(data);
    });

    process.stdin.on('error', (err) => {
      clearTimeout(timeout);
      reject(err);
    });

    // Resume stdin in case it was paused
    process.stdin.resume();
  });
}

/**
 * Get file size in bytes
 */
function getFileSize(filePath) {
  try {
    const stats = fs.statSync(filePath);
    return stats.size;
  } catch (err) {
    return 0;
  }
}

/**
 * Make path relative to project directory
 */
function makeRelativePath(filePath) {
  if (!filePath) return '';

  // Handle Windows paths
  const normalizedPath = filePath.replace(/\\/g, '/');
  const normalizedProject = PROJECT_DIR.replace(/\\/g, '/');

  if (normalizedPath.startsWith(normalizedProject)) {
    return normalizedPath.slice(normalizedProject.length).replace(/^\//, '');
  }

  return filePath;
}

/**
 * Log file event directly using log-event.js module
 */
function logFileEvent(type, filePath, data = {}) {
  try {
    const logPath = path.join(PROJECT_DIR, logEventModule.LOG_FILE);

    // Build event object
    const event = {
      timestamp: new Date().toISOString().replace(/\.\d{3}Z$/, 'Z'),
      type: type,
      agent: 'claude',
      context: {},
      data: {
        path: filePath,
        ...data
      }
    };

    // Append to log file
    logEventModule.appendEvent(event, logPath);
  } catch (err) {
    // Silently fail - don't block operations
    console.error(`Hook logging error: ${err.message}`);
  }
}

/**
 * Log command execution event
 */
function logCommandEvent(command, args, exitCode = 0) {
  try {
    const logPath = path.join(PROJECT_DIR, logEventModule.LOG_FILE);

    // Build event object
    const event = {
      timestamp: new Date().toISOString().replace(/\.\d{3}Z$/, 'Z'),
      type: 'command_run',
      agent: 'user',
      context: {},
      data: {
        command: command,
        args: args || '',
        exit_code: exitCode
      }
    };

    // Append to log file
    logEventModule.appendEvent(event, logPath);
  } catch (err) {
    // Silently fail - don't block operations
    console.error(`Hook logging error: ${err.message}`);
  }
}

/**
 * Main hook handler
 */
async function main() {
  try {
    // Read JSON input from stdin
    const input = await readStdin();

    if (!input || input.trim() === '') {
      process.exit(0);
    }

    let data;
    try {
      data = JSON.parse(input);
    } catch (parseErr) {
      // Invalid JSON, exit silently
      process.exit(0);
    }

    const toolName = data.tool_name;
    const toolInput = data.tool_input || {};
    const toolResponse = data.tool_response || {};

    // Handle Skill tool (PM commands)
    if (toolName === 'Skill') {
      const skillName = toolInput.skill || '';
      const skillArgs = toolInput.args || '';

      // Only log pm:* and context:* and testing:* commands
      if (skillName.startsWith('pm:') || skillName.startsWith('context:') || skillName.startsWith('testing:')) {
        // Determine exit code from response (assume success if no error)
        const exitCode = (toolResponse && typeof toolResponse === 'string' && toolResponse.includes('error')) ? 1 : 0;
        logCommandEvent(skillName, skillArgs, exitCode);
      }
      process.exit(0);
    }

    // Only process Write and Edit tools for file logging
    if (toolName !== 'Write' && toolName !== 'Edit') {
      process.exit(0);
    }

    // Check if operation was successful
    if (toolResponse.success === false) {
      process.exit(0);
    }

    // Get file path from tool input
    const filePath = toolInput.file_path || toolResponse.filePath;
    if (!filePath) {
      process.exit(0);
    }

    const relativePath = makeRelativePath(filePath);

    // Determine event type and log
    if (toolName === 'Write') {
      const size = getFileSize(filePath);
      logFileEvent('file_create', relativePath, { size: size });
    } else if (toolName === 'Edit') {
      // For edits, we don't have line counts from the hook input
      // Use placeholder values - could be enhanced later
      logFileEvent('file_modify', relativePath, { lines_added: 0, lines_removed: 0 });
    }

    // Exit successfully - don't block tool execution
    process.exit(0);

  } catch (err) {
    // Always exit 0 to not block Claude Code operations
    console.error(`Hook error: ${err.message}`);
    process.exit(0);
  }
}

main();
