#!/usr/bin/env node
// UserPromptSubmit hook - logs user prompts to changes.jsonl

// You can add this hook to settings.local.json as follows:
//  "hooks": {
//  "UserPromptSubmit": [
//     {
//       "hooks": [
//         {
//           "type": "command",
//           "command": "node .claude/hooks/user-prompt-logger.js"
//         }
//       ]
//     }
//   ],
//  ... Other hooks ...
//  }

const fs = require('fs');
const path = require('path');

// Simple file locking using lockfile pattern
const acquireLock = (lockPath, timeout = 5000) => {
  const startTime = Date.now();
  const lockContent = `${process.pid}-${Date.now()}`;

  while (Date.now() - startTime < timeout) {
    try {
      // Attempt to create lock file exclusively
      fs.writeFileSync(lockPath, lockContent, { flag: 'wx' });
      return true;
    } catch (err) {
      if (err.code === 'EEXIST') {
        // Check if lock is stale (older than 30 seconds)
        try {
          const stat = fs.statSync(lockPath);
          if (Date.now() - stat.mtimeMs > 30000) {
            // Stale lock, remove it
            fs.unlinkSync(lockPath);
            continue;
          }
        } catch (statErr) {
          // Lock file was removed, try again
          continue;
        }
        // Wait a bit before retrying
        const waitTime = Math.min(50, timeout - (Date.now() - startTime));
        if (waitTime > 0) {
          Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, waitTime);
        }
      } else {
        throw err;
      }
    }
  }
  return false;
};

const releaseLock = (lockPath) => {
  try {
    fs.unlinkSync(lockPath);
  } catch (err) {
    // Ignore errors when releasing lock
  }
};

// Read JSON input from stdin
let input = '';
process.stdin.setEncoding('utf8');

process.stdin.on('data', (chunk) => {
  input += chunk;
});

process.stdin.on('end', () => {
  const projectDir = path.resolve(__dirname, '..', '..');
  const logFile = path.join(projectDir, '.claude', 'logs', 'changes.jsonl');
  const lockFile = logFile + '.lock';
  let lockAcquired = false;

  try {
    const data = JSON.parse(input);

    // Ensure log directory exists
    const logDir = path.dirname(logFile);
    if (!fs.existsSync(logDir)) {
      fs.mkdirSync(logDir, { recursive: true });
    }

    // Generate ISO 8601 timestamp
    const timestamp = new Date().toISOString().replace(/\.\d{3}Z$/, 'Z');

    // Create log entry
    const logEntry = {
      timestamp,
      type: 'user_prompt_submit',
      agent: 'user',
      context: {},
      data: {
        session_id: data.session_id || '',
        prompt: data.prompt || ''
      }
    };

    // Acquire lock before writing
    lockAcquired = acquireLock(lockFile);
    if (!lockAcquired) {
      // Log to stderr but don't block - write to fallback location
      console.error('Warning: Could not acquire lock for changes.jsonl, using fallback');
      const fallbackFile = path.join(logDir, `changes-${process.pid}.jsonl`);
      fs.appendFileSync(fallbackFile, JSON.stringify(logEntry) + '\n');
      process.exit(0);
    }

    // Check for disk space issues by checking if we can write
    try {
      fs.appendFileSync(logFile, JSON.stringify(logEntry) + '\n');
    } catch (writeErr) {
      if (writeErr.code === 'ENOSPC') {
        console.error('Error: Disk full, cannot write to changes.jsonl');
      } else {
        throw writeErr;
      }
    }

    process.exit(0);
  } catch (err) {
    // Provide more context about the error
    const errorContext = {
      message: err.message,
      code: err.code,
      phase: input ? 'processing' : 'reading_input'
    };
    console.error('Error in user-prompt-logger:', JSON.stringify(errorContext));

    // Exit 0 to not block Claude, but log failure for debugging
    process.exit(0);
  } finally {
    // Always release lock if acquired
    if (lockAcquired) {
      releaseLock(lockFile);
    }
  }
});
