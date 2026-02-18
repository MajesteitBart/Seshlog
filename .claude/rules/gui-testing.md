# GUI Testing Operations Rule

Standard patterns for Chrome-based GUI testing operations using MCP browser automation tools.

## Chrome Connection

**Before any GUI test operation:**

Verify Chrome MCP is available by calling `mcp__claude-in-chrome__tabs_context_mcp`.
If the call fails or returns an error, Chrome is not connected.

**Connection failure response:**
```
Chrome extension not connected.
To enable GUI testing, start Claude Code with Chrome integration:
  claude --chrome
```

Do not attempt fallback or retry - inform user and exit.

## Tab Management

### Creating Test Tabs

Always create a new tab for testing:
```javascript
// Get or create browser context
const context = await mcp__claude-in-chrome__tabs_context_mcp({ createIfEmpty: true });

// Create new tab for testing
const tab = await mcp__claude-in-chrome__tabs_create_mcp();
// Store tabId for all subsequent operations
```

**Rules:**
- Never reuse existing user tabs for testing
- Store tab ID immediately after creation
- Use stored tab ID for all operations in the test session

### Viewport Setup

Set consistent viewport for testing:
```javascript
await mcp__claude-in-chrome__resize_window({
  tabId: tabId,
  width: 1280,   // Default, can be configured
  height: 720
});
```

### Cleanup

After testing completes (success or failure):
- Close test tabs if created
- Do not leave orphan tabs from test sessions

## Navigation

### URL Construction

```javascript
// If argument is full URL, use as-is
// If argument is a route (starts with /), prepend base URL from config
const targetUrl = argument.startsWith('http')
  ? argument
  : config.default_url + argument;
```

### Page Load Wait

After navigation, wait for page to stabilize:
```javascript
await mcp__claude-in-chrome__navigate({ tabId, url: targetUrl });

// Wait for initial render
await mcp__claude-in-chrome__computer({
  action: 'wait',
  tabId: tabId,
  duration: 2  // seconds
});
```

## Console Error Checking

### Standard Pattern

```javascript
const messages = await mcp__claude-in-chrome__read_console_messages({
  tabId: tabId,
  onlyErrors: true,
  pattern: 'error|exception|failed|warning'
});
```

### Filtering

Apply ignore patterns from configuration:
```javascript
const errors = messages.filter(msg =>
  !config.console_filters.ignore_patterns.some(pattern =>
    msg.toLowerCase().includes(pattern.toLowerCase())
  )
);
```

### Common Ignore Patterns

These patterns are typically noise and should be ignored by default:
- `DevTools failed to load`
- `ResizeObserver loop`
- `favicon.ico`
- `[HMR]` (Hot Module Replacement)
- `Download the React DevTools`

### Threshold Evaluation

```javascript
const passed = errors.length <= config.console_filters.error_threshold;
```

## Screenshot Capture

### Taking Screenshots

```javascript
const screenshot = await mcp__claude-in-chrome__computer({
  action: 'screenshot',
  tabId: tabId
});
// screenshot.id contains the image ID for later reference
```

### Storage

If configured to save screenshots:
- Directory: `.project/screenshots/` (create if not exists)
- Naming convention: `{route}-{ISO-timestamp}.png`
- Example: `dashboard-2026-01-02T10-30-00Z.png`

## Smoke Testing

### Page Load Verification

```javascript
const tree = await mcp__claude-in-chrome__read_page({ tabId });
if (!tree || tree.error) {
  // Page failed to load
}
```

### Element Finding

```javascript
const elements = await mcp__claude-in-chrome__find({
  tabId,
  query: 'submit button' // Natural language description
});
if (elements.length === 0) {
  // Expected element not found
}
```

### Common Smoke Checks

| Check | Implementation |
|-------|----------------|
| `page_loads` | `read_page` returns valid tree |
| `no_console_errors` | Console error count <= threshold |
| `form_present` | `find` locates form/input elements |
| `button_present` | `find` locates expected buttons |
| `heading_present` | `find` locates page heading |

## AI Design Validation

### Validation Prompt Template

When comparing a screenshot against a task description:

```markdown
Compare this screenshot to the following task requirements:

---
{task_description}
---

Analyze and score each category:

1. **Element Presence** (Score: /3)
   - Are all UI elements mentioned in the task visible?
   - List any missing elements.

2. **Layout & Positioning** (Score: /3)
   - Are elements positioned as expected?
   - Is the layout appropriate for the viewport?
   - List any positioning issues.

3. **Styling & Theme** (Score: /2)
   - Do colors, fonts, and spacing appear correct?
   - Is styling consistent with the design requirements?
   - List any styling issues.

4. **Functionality Indicators** (Score: /2)
   - Are interactive elements visually actionable?
   - Do visible states (enabled/disabled, selected) look correct?
   - List any functional concerns.

## Summary

- **Total Score**: X/10
- **Verdict**: PASS | WARN | FAIL
- **Summary**: One sentence assessment

Scoring:
- PASS: Score >= 7 with no critical missing elements
- WARN: Score 4-6 or minor issues present
- FAIL: Score < 4 or critical elements missing
```

### Threshold Configuration

Default pass threshold: 7/10
Configurable via `design_validation.pass_threshold` in config.

## Error Handling

### Network Errors

```
Page timeout after {seconds}s at {url}.
Check if the development server is running.
```

### Element Not Found

```
Expected element not found: {description}
Page may not have loaded correctly or element is missing.
```

### Chrome Disconnected

```
Lost connection to Chrome browser.
Restart Claude Code with: claude --chrome
```

## JSONL Event Logging

Log GUI test results to `.claude/logs/changes.jsonl`:

```json
{
  "timestamp": "2026-01-02T10:30:00Z",
  "eventType": "test.gui.completed",
  "sessionId": "{session-uuid}",
  "actor": "command:testing:gui",
  "data": {
    "url": "http://localhost:3000/dashboard",
    "mode": "full",
    "results": {
      "consoleErrors": 0,
      "pageLoad": true,
      "smokeTests": { "passed": 3, "total": 3 },
      "designValidation": { "score": 8, "verdict": "PASS" }
    },
    "verdict": "PASS",
    "duration_ms": 5234
  }
}
```

## Important Notes

- GUI tests are synchronous - wait for each step to complete
- Always include timestamps in results
- Take screenshot before reporting failure (for debugging)
- Clean up browser state even on test failure
- Respect configured timeouts from `.project/context/gui-testing.md`
