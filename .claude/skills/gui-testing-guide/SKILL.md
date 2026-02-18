---
name: gui-testing-guide
description: Provides guidance on GUI testing setup, configuration, and troubleshooting. Activates when user asks about visual testing, Chrome browser testing, screenshot verification, design validation, or GUI testing configuration.
allowed-tools: Read, Glob, Grep
---

# GUI Testing Guide

Advisory skill for GUI testing setup, configuration, and best practices in the Delano PM system.

## When to Activate

Activate this skill when the user:
- Asks about setting up visual or GUI testing
- Has questions about Chrome browser integration for testing
- Needs help configuring screenshot or design validation
- Encounters GUI testing failures or errors
- Asks about enforcement modes (mandatory vs advisory)
- Wants to understand how GUI testing integrates with task workflow
- Asks about the `/testing:gui` command

## Core Concepts

### What GUI Testing Does

The GUI testing system provides automated visual verification for web applications:

1. **Console Error Checking** - Monitors browser console for JavaScript errors
2. **Screenshot Capture** - Takes screenshots for visual verification
3. **Smoke Tests** - Verifies critical pages load correctly
4. **AI Design Validation** - Uses Claude's vision to compare screenshots against task descriptions

### Enforcement Modes

| Mode | Behavior | Use Case |
|------|----------|----------|
| `mandatory` | Blocks issue close if GUI tests fail | Production-critical UI, release branches |
| `advisory` | Warns but allows close on failure | Development, iteration, experimentation |
| `disabled` | Skips GUI testing entirely | Non-GUI projects, backend-only work |

**Recommendation:** Start with `advisory` mode during development, switch to `mandatory` for production-critical flows.

### Configuration Location

GUI testing is configured in `.project/context/gui-testing.md`.

## Configuration Guide

### Minimal Configuration

```yaml
---
created: 2026-01-02T10:00:00Z
updated: 2026-01-02T10:00:00Z
---
# GUI Testing Configuration

## Settings
type: web-application
enforcement: advisory
default_url: http://localhost:3000
```

### Full Configuration

```yaml
---
created: 2026-01-02T10:00:00Z
updated: 2026-01-02T10:00:00Z
---
# GUI Testing Configuration

## Settings
type: web-application
enforcement: advisory
default_url: http://localhost:3000

## Smoke Routes
smoke_routes:
  - path: /
    name: Home Page
  - path: /login
    name: Login Page
  - path: /dashboard
    name: Dashboard

## Console Filtering
console_filters:
  ignore_patterns:
    - "DevTools failed to load"
    - "ResizeObserver loop"
    - "[HMR]"
    - "Download the React DevTools"
  error_threshold: 0

## AI Design Validation
design_validation:
  enabled: true
  pass_threshold: 7

## Screenshots
screenshots:
  enabled: true
  directory: .project/screenshots
```

### Configuration Fields Explained

| Field | Required | Description |
|-------|----------|-------------|
| `type` | Yes | Project type: `web-application` or `none` |
| `enforcement` | Yes | `mandatory`, `advisory`, or `disabled` |
| `default_url` | Yes | Base URL for testing (usually localhost) |
| `smoke_routes` | No | List of routes to verify on smoke tests |
| `console_filters.ignore_patterns` | No | Console messages to ignore |
| `console_filters.error_threshold` | No | Max allowed errors (default: 0) |
| `design_validation.enabled` | No | Enable AI design comparison |
| `design_validation.pass_threshold` | No | Minimum score to pass (default: 7) |
| `screenshots.enabled` | No | Save screenshots |
| `screenshots.directory` | No | Where to save screenshots |

## Troubleshooting

### Chrome Not Connected

**Symptoms:**
- "Chrome extension not connected" error
- GUI tests fail to start

**Solutions:**
1. Ensure Chrome browser is open
2. Start Claude Code with Chrome flag: `claude --chrome`
3. Check that the Claude Code Chrome extension is installed
4. Refresh the browser tab if extension seems unresponsive

### Page Timeout

**Symptoms:**
- "Page load timeout" error
- Tests hang then fail

**Solutions:**
1. Verify development server is running
2. Check the `default_url` is correct in config
3. Ensure no firewall blocking localhost
4. Try accessing the URL manually in browser

### False Positive Console Errors

**Symptoms:**
- Tests fail due to expected warnings
- Third-party library warnings causing failures

**Solutions:**
1. Add patterns to `console_filters.ignore_patterns`:
   ```yaml
   ignore_patterns:
     - "DevTools failed to load"
     - "ResizeObserver loop"
     - "[HMR]"
     - "third-party-lib"
   ```
2. Increase `error_threshold` if some errors are acceptable

### Design Validation Too Strict

**Symptoms:**
- Valid implementations failing design validation
- Score consistently below threshold

**Solutions:**
1. Lower `pass_threshold` (e.g., from 7 to 6)
2. Make task descriptions more specific about UI expectations
3. Switch to `advisory` mode during development

### Screenshots Not Saving

**Symptoms:**
- Screenshots referenced but not found
- Directory empty after tests

**Solutions:**
1. Check `screenshots.directory` path is valid
2. Ensure directory exists or can be created
3. Verify Claude has write permissions

## Best Practices

### Writing Test-Friendly Task Descriptions

For better AI design validation, include in task descriptions:

**Good:**
```markdown
Add a login form with:
- Email input field with placeholder "Enter email"
- Password input field (type=password)
- Blue "Sign In" button below the fields
- "Forgot password?" link below the button
```

**Not as good:**
```markdown
Add login functionality to the page
```

### Configuring Smoke Routes

Focus on critical user flows:
```yaml
smoke_routes:
  - path: /
    name: Home Page
  - path: /login
    name: Login
  - path: /dashboard
    name: Main Dashboard
  - path: /settings
    name: User Settings
```

### When to Use Each Mode

| Scenario | Recommended Mode |
|----------|------------------|
| Active development | `advisory` |
| Pre-release testing | `mandatory` |
| Backend-only changes | `disabled` |
| Bug fixing UI | `mandatory` |
| Exploring/prototyping | `advisory` |

## Related Commands

| Command | Description |
|---------|-------------|
| `/testing:gui` | Run GUI tests on-demand |
| `/testing:gui --smoke` | Run smoke tests only |
| `/testing:gui --design "desc"` | Run with design validation |
| `/pm:init` | Configure GUI testing during project setup |
| `/pm:issue-close` | Runs GUI tests automatically (if configured) |

## Boundaries

### This Skill CAN:
- Explain configuration options and their effects
- Suggest troubleshooting steps for common issues
- Recommend best practices for GUI testing
- Analyze test failure messages
- Help interpret design validation scores

### This Skill CANNOT:
- Run tests (use `/testing:gui` command)
- Modify configuration files directly
- Connect to Chrome browser
- Take screenshots
- Execute any browser automation

For actions, use the `/testing:gui` command instead.
