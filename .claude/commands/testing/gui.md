---
description: Execute Chrome-based GUI testing for web applications
argument-hint: '[url_or_route] [--smoke | --full | --design "description"]'
allowed-tools: Read, Write, Bash, Task, mcp__claude-in-chrome__*
---

# GUI Testing Command

Execute Chrome-based GUI testing with console error checking, screenshots, smoke tests, and AI-powered design validation.

## Usage

```
/testing:gui                           # Test default URL with full checks
/testing:gui /dashboard                # Test specific route
/testing:gui --smoke                   # Run smoke tests only
/testing:gui --design "Add login form" # Run with design validation
/testing:gui http://localhost:3000 --full  # Full test on specific URL
```

## Options

| Option | Description |
|--------|-------------|
| `--smoke` | Run smoke tests only (page load + console errors) |
| `--full` | Run all checks including design validation |
| `--design "desc"` | Run design validation with provided description |
| No options | Default to full mode |

## Preflight Checks

### 1. Configuration Check

```bash
# Check for GUI testing config
if [ ! -f ".project/context/gui-testing.md" ]; then
  echo "GUI testing not configured."
  echo "Run /pm:init to configure, or create .project/context/gui-testing.md manually."
  exit 1
fi
```

If not configured, display help and exit.

### 2. Read Configuration

Parse `.project/context/gui-testing.md` for:
- `enforcement` mode (mandatory | advisory | disabled)
- `default_url` (base URL for testing)
- `smoke_routes` (routes to test)
- `console_filters.ignore_patterns` (errors to ignore)
- `console_filters.error_threshold` (max allowed errors)
- `design_validation.enabled` (whether to run AI validation)
- `design_validation.pass_threshold` (minimum score to pass)
- `screenshots.enabled` and `screenshots.directory`

If `enforcement: disabled`, display message and exit:
```
GUI testing is disabled for this project.
To enable, edit .project/context/gui-testing.md and set enforcement to 'advisory' or 'mandatory'.
```

### 3. Chrome Connection Check

Use `mcp__claude-in-chrome__tabs_context_mcp` to verify Chrome is connected.

If Chrome not connected:
```
Chrome extension not connected.

To enable GUI testing:
1. Ensure Chrome browser is open
2. Start Claude Code with: claude --chrome
```

Exit without further action.

## Test Workflow

### Step 1: Browser Setup

1. Get browser context: `mcp__claude-in-chrome__tabs_context_mcp({ createIfEmpty: true })`
2. Create new tab: `mcp__claude-in-chrome__tabs_create_mcp()`
3. Store the `tabId` for all subsequent operations
4. Set viewport: `mcp__claude-in-chrome__resize_window({ tabId, width: 1280, height: 720 })`

### Step 2: Determine Target URL

**URL Validation and Resolution:**
```
# Validate URL to prevent SSRF and malformed URLs
function validateUrl(url):
  # Only allow http/https protocols
  if not url.startsWith('http://') and not url.startsWith('https://'):
    return false

  # Block internal/sensitive URLs
  blockedPatterns = ['localhost:22', '127.0.0.1:22', '0.0.0.0', '169.254.', '10.', '172.16.', '192.168.', 'file://', 'ftp://']
  for pattern in blockedPatterns:
    if url.contains(pattern) and not url.contains('localhost:3') and not url.contains('127.0.0.1:3'):
      # Allow common dev ports (3000-3999) on localhost but block other internal ranges
      return false

  # Validate URL format with proper hostname
  try:
    parsed = parseUrl(url)
    return parsed.hostname is not empty
  catch:
    return false

# URL Resolution with validation
If $ARGUMENTS contains a full URL (starts with http):
  if validateUrl($ARGUMENTS):
    targetUrl = $ARGUMENTS
  else:
    echo "❌ Invalid or unsafe URL provided. Only http/https URLs to allowed hosts are permitted."
    exit 1
Else if $ARGUMENTS contains a route (starts with /):
  # Validate route doesn't contain path traversal
  if $ARGUMENTS contains '..':
    echo "❌ Invalid route: path traversal not allowed"
    exit 1
  targetUrl = config.default_url + $ARGUMENTS
Else if $ARGUMENTS is empty:
  targetUrl = config.default_url
Else:
  # Validate argument doesn't contain dangerous characters
  if $ARGUMENTS matches /^[a-zA-Z0-9\-_\/]+$/:
    targetUrl = config.default_url + '/' + $ARGUMENTS
  else:
    echo "❌ Invalid route: only alphanumeric characters, dashes, underscores, and slashes allowed"
    exit 1
```

### Step 3: Navigate to Target

1. Navigate: `mcp__claude-in-chrome__navigate({ tabId, url: targetUrl })`
2. Wait for page load: `mcp__claude-in-chrome__computer({ action: 'wait', tabId, duration: 2 })`
3. Record navigation success/failure

### Step 4: Console Error Check

1. Read console messages:
   ```javascript
   mcp__claude-in-chrome__read_console_messages({
     tabId,
     onlyErrors: true,
     pattern: 'error|exception|failed'
   })
   ```

2. Filter out ignored patterns from config
3. Count remaining errors
4. Compare against `error_threshold`

**Result:**
- `consoleErrors`: count of errors found
- `consolePassed`: true if count <= threshold

### Step 5: Take Screenshot

```javascript
const screenshot = await mcp__claude-in-chrome__computer({
  action: 'screenshot',
  tabId
});
```

Store `screenshot.id` for potential design validation.

If `screenshots.enabled` and `screenshots.directory` configured:
- Note that screenshot was captured (MCP tool handles storage)

### Step 6: Smoke Tests (if --smoke or --full)

If `smoke_routes` configured in config, test each route:

For each route in `smoke_routes`:
1. Navigate to `config.default_url + route.path`
2. Wait for load
3. Verify page loaded: `mcp__claude-in-chrome__read_page({ tabId })`
4. Check console for errors
5. Record pass/fail for route

**Smoke Test Checks:**
- `page_loads`: read_page returns valid accessibility tree
- `no_console_errors`: console errors <= threshold
- `form_present`: find({ query: 'form or input' }) returns results
- `button_present`: find({ query: 'button' }) returns results

### Step 7: AI Design Validation (if --design or --full with context)

**Trigger conditions:**
- `--design "description"` provided as argument
- `--full` mode AND task context available
- Config has `design_validation.enabled: true`

**Validation Process:**

1. Ensure screenshot was captured in Step 5

2. Get design description:
   - From `--design` argument, OR
   - From task file content if called during issue-close

3. Present screenshot to Claude with validation prompt:

```markdown
Compare this screenshot to the following task requirements:

---
{design_description}
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
   - Do colors, fonts, spacing appear correct?
   - List any styling issues.

4. **Functionality Indicators** (Score: /2)
   - Are interactive elements visually actionable?
   - List any functional concerns.

## Summary
- **Total Score**: X/10
- **Verdict**: PASS (>=7) | WARN (4-6) | FAIL (<4)
- **Summary**: One sentence assessment
```

4. Parse response for score and verdict
5. Compare against `pass_threshold` from config

### Step 8: Compile Results

Build test result summary:

```markdown
## GUI Test Results

**URL:** {targetUrl}
**Timestamp:** {current_datetime}
**Mode:** {smoke|full|design}

### Console Errors
{passed ? "No errors" : "{count} errors found"}
{list of errors if any}

### Page Load
{passed ? "Page loaded successfully" : "Failed to load"}

### Smoke Tests
{if run: "{passed}/{total} routes passed" with details}

### Design Validation
{if run: "Score: {score}/10 - {verdict}" with details}

### Overall Result
{PASSED | FAILED | PASSED WITH WARNINGS}
```

### Step 9: Handle Result Based on Enforcement

**If enforcement = mandatory:**
- FAIL result: Display error, suggest fixes, return failure status
- PASS result: Continue normally

**If enforcement = advisory:**
- FAIL result: Display warning, note advisory mode, return success with warnings
- PASS result: Continue normally

### Step 10: Cleanup

- Close test tab if desired (or leave for user inspection on failure)
- Return test result for calling command to use

## Output Format

### Success Output
```
GUI Test Results

URL: http://localhost:3000/dashboard
Mode: full

Console Errors: None
Page Load: Passed
Smoke Tests: 3/3 passed
Design Validation: 8/10 - PASS

PASSED - All GUI tests passed
```

### Failure Output (Mandatory)
```
GUI Test Results

URL: http://localhost:3000/dashboard
Mode: full

Console Errors: 2 errors found
  - TypeError: Cannot read property 'map' of undefined (app.js:142)
  - Failed to load resource: 404 (api/users)

Page Load: Passed
Smoke Tests: 2/3 passed
  - /login: FAIL - form not found
Design Validation: 5/10 - WARN
  - Missing: Submit button not visible
  - Issue: Form fields not aligned

FAILED - GUI tests did not pass

Fix the issues above and retry.
```

### Warning Output (Advisory)
```
GUI Test Results

URL: http://localhost:3000/dashboard
Mode: full

Console Errors: 1 warning
  - [Deprecation] Feature will be removed in future version

PASSED WITH WARNINGS
Enforcement is advisory - proceeding despite warnings.
```

## Error Handling

| Error | Message |
|-------|---------|
| Config missing | "GUI testing not configured. Run /pm:init" |
| Chrome disconnected | "Chrome not connected. Run: claude --chrome" |
| Page timeout | "Page load timeout. Check if dev server is running at {url}" |
| Navigation failed | "Failed to navigate to {url}. Verify URL is correct." |
| Screenshot failed | "Failed to capture screenshot. Browser may have lost focus." |

## Integration with Issue Close

When called from `/pm:issue-close`:

1. Receive task description as design context
2. Extract URL hints from task (look for route patterns like `/page-name`)
3. Run full test suite with design validation
4. Return structured result for issue-close to handle enforcement

## Important Notes

- Follow `.claude/rules/gui-testing.md` for standard patterns
- Always create new tab for testing (never reuse user tabs)
- Wait for page stabilization before checking console
- Take screenshot before reporting failures (for debugging)
- Log results to `.claude/logs/changes.jsonl`
