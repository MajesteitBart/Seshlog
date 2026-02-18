#!/usr/bin/env node
/**
 * session-tracker.js - Session tracking hook for Claude Code
 *
 * Tracks session start and end events, logging them to JSONL.
 *
 * Usage:
 *   node session-tracker.js start   - Log session start (called on first tool use)
 *   node session-tracker.js stop    - Log session end (called by Stop hook)
 *
 * Session state is persisted in a temp file to maintain consistency
 * across multiple hook invocations within the same session.
 */

const fs = require('fs');
const path = require('path');
const os = require('os');
const crypto = require('crypto');

// Import log-event utilities
const projectDir = process.env.CLAUDE_PROJECT_DIR || process.cwd();
const logEventPath = path.join(projectDir, '.claude', 'scripts', 'log-event.js');
const { buildEvent, appendEvent, LOG_FILE } = require(logEventPath);

// Session file location (temp directory, unique per project)
const projectHash = crypto.createHash('md5').update(projectDir).digest('hex').slice(0, 8);
const SESSION_FILE = path.join(os.tmpdir(), `claude-session-${projectHash}.json`);

/**
 * Generate a unique session ID
 */
function generateSessionId() {
  const timestamp = Date.now().toString(36);
  const random = crypto.randomBytes(4).toString('hex');
  return `${timestamp}-${random}`;
}

/**
 * Read session state from temp file
 */
function readSessionState() {
  try {
    if (fs.existsSync(SESSION_FILE)) {
      const data = fs.readFileSync(SESSION_FILE, 'utf8');
      return JSON.parse(data);
    }
  } catch (err) {
    // Ignore read errors, will create new session
  }
  return null;
}

/**
 * Write session state to temp file
 */
function writeSessionState(state) {
  try {
    fs.writeFileSync(SESSION_FILE, JSON.stringify(state), 'utf8');
  } catch (err) {
    console.error('Failed to write session state:', err.message);
  }
}

/**
 * Clear session state
 */
function clearSessionState() {
  try {
    if (fs.existsSync(SESSION_FILE)) {
      fs.unlinkSync(SESSION_FILE);
    }
  } catch (err) {
    // Ignore delete errors
  }
}

/**
 * Handle session start
 * Called on PreToolUse - only logs once per session
 */
function handleStart() {
  const existing = readSessionState();

  // If session already exists and is recent (within 2 hours), don't log again
  if (existing && existing.startTime) {
    const age = Date.now() - existing.startTime;
    const twoHours = 2 * 60 * 60 * 1000;
    if (age < twoHours) {
      // Session already started, skip
      process.exit(0);
    }
  }

  // Create new session
  const sessionId = generateSessionId();
  const startTime = Date.now();

  writeSessionState({
    sessionId,
    startTime,
    projectDir
  });

  // Log session_start event
  const event = {
    timestamp: new Date().toISOString().replace(/\.\d{3}Z$/, 'Z'),
    type: 'session_start',
    agent: 'user',
    context: {},
    data: {
      session_id: sessionId
    }
  };

  try {
    const logPath = path.join(projectDir, LOG_FILE);
    appendEvent(event, logPath);
  } catch (err) {
    // Fail silently to not block Claude Code
    console.error('Failed to log session start:', err.message);
  }
}

/**
 * Handle session stop
 * Called by Stop hook
 */
function handleStop() {
  const state = readSessionState();

  if (!state || !state.sessionId) {
    // No session to end
    process.exit(0);
  }

  const duration = Math.round((Date.now() - state.startTime) / 1000);

  // Log session_end event
  const event = {
    timestamp: new Date().toISOString().replace(/\.\d{3}Z$/, 'Z'),
    type: 'session_end',
    agent: 'user',
    context: {},
    data: {
      session_id: state.sessionId,
      duration
    }
  };

  try {
    const logPath = path.join(projectDir, LOG_FILE);
    appendEvent(event, logPath);
  } catch (err) {
    console.error('Failed to log session end:', err.message);
  }

  // Clear session state
  clearSessionState();
}

/**
 * Main entry point
 */
function main() {
  const action = process.argv[2];

  switch (action) {
    case 'start':
      handleStart();
      break;
    case 'stop':
      handleStop();
      break;
    default:
      console.log('Usage: node session-tracker.js <start|stop>');
      process.exit(1);
  }
}

// Run
main();
