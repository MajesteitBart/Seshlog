---
created: 2026-01-22T20:43:45Z
updated: 2026-01-22T20:43:45Z
---
# GUI Testing Configuration

Configuration for Chrome-based GUI testing in the Meeting Companion app.

## Settings

```yaml
type: web-application
enforcement: advisory
default_url: http://localhost:3118
```

- **type**: Project type. Set to `none` to disable GUI testing detection.
- **enforcement**:
  - `mandatory` - Blocks issue close if GUI tests fail
  - `advisory` - Warns but allows close on failure
  - `disabled` - Skips GUI testing entirely
- **default_url**: Base URL for testing (Tauri dev server on port 3118)

## Smoke Routes

Routes to test for basic functionality during smoke tests:

```yaml
smoke_routes:
  - path: /
    name: Home Page (Recording Interface)
```

Add routes for critical user flows in your application.

## Console Filtering

Configure which console messages to ignore and the error threshold:

```yaml
console_filters:
  ignore_patterns:
    - "DevTools failed to load"
    - "ResizeObserver loop"
    - "[HMR]"
    - "Download the React DevTools"
  error_threshold: 0
```

- **ignore_patterns**: Console messages containing these strings are ignored
- **error_threshold**: Maximum allowed errors (0 = no errors allowed)

## AI Design Validation

Configure how AI compares screenshots against task descriptions:

```yaml
design_validation:
  enabled: true
  pass_threshold: 7
```

- **enabled**: Set to `false` to skip design validation
- **pass_threshold**: Minimum score (out of 10) to pass

## Screenshots

Configure screenshot capture:

```yaml
screenshots:
  enabled: true
  directory: .project/screenshots
```

## Usage

Run GUI tests on-demand:
```
/testing:gui                    # Test default URL
/testing:gui /dashboard         # Test specific route
/testing:gui --smoke            # Run smoke tests only
/testing:gui --design "desc"    # Run with design validation
```

GUI tests run automatically on `/pm:issue-close` when enforcement is not `disabled`.
