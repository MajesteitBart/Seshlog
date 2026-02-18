---
description: Mark an issue as complete and close it on GitHub
argument-hint: <issue_number> [completion_notes]
allowed-tools: ListFiles, Read, WriteFile, ExecuteCommand
---

# Issue Close

Mark an issue as complete and close it on GitHub.

## Usage
```
/pm:issue-close <issue_number> [completion_notes]
```

## Instructions

### 1. Find Local Task File

First check if `.project/epics/*/$ARGUMENTS.md` exists (new naming).
If not found, search for task file with `github:.*issues/$ARGUMENTS` in frontmatter (old naming).
If not found: "❌ No local task for issue #$ARGUMENTS"

### 2. Update Local Status

Get current datetime: `date -u +"%Y-%m-%dT%H:%M:%SZ"`

Update task file frontmatter:
```yaml
status: closed
updated: {current_datetime}
```

### 3. Update Progress File

If progress file exists at `.project/epics/{epic}/updates/$ARGUMENTS/progress.md`:
- Set completion: 100%
- Add completion note with timestamp
- Update last_sync with current datetime

### 4. Close on GitHub

Add completion comment and close:
```bash
# Validate issue number to prevent command injection
if ! echo "$ARGUMENTS" | grep -qE '^[0-9]+$'; then
  echo "❌ Invalid issue number. Must be numeric."
  exit 1
fi

# Add final comment
echo "✅ Task completed

$ARGUMENTS

---
Closed at: {timestamp}" | gh issue comment "$ARGUMENTS" --body-file -

# Close the issue
gh issue close "$ARGUMENTS"
```

### 5. Update Epic Task List on GitHub

Check the task checkbox in the epic issue:

```bash
# Get epic name from local task file path
epic_name={extract_from_path}

# Validate epic name to prevent path injection
if ! echo "$epic_name" | grep -qE '^[a-zA-Z0-9_-]+$'; then
  echo "❌ Invalid epic name format."
  exit 1
fi

# Get epic issue number from epic.md
epic_issue=$(grep 'github:' ".project/epics/$epic_name/epic.md" | grep -oE '[0-9]+$')

if [ -n "$epic_issue" ]; then
  # Get current epic body
  gh issue view "$epic_issue" --json body -q .body > /tmp/epic-body.md

  # Check off this task
  sed -i "s/- \[ \] #$ARGUMENTS/- [x] #$ARGUMENTS/" /tmp/epic-body.md

  # Update epic issue
  gh issue edit "$epic_issue" --body-file /tmp/epic-body.md

  echo "✓ Updated epic progress on GitHub"
fi
```

### 6. Update Epic Progress

- Count total tasks in epic
- Count closed tasks
- Calculate new progress percentage
- Update epic.md frontmatter progress field

### 6.5. GUI Testing Verification (if configured)

Check if GUI testing should run before completing the close:

```bash
# Check if GUI testing is configured
if [ -f ".project/context/gui-testing.md" ]; then
  # Parse enforcement mode using robust YAML extraction
  # Handles variations: "enforcement: value", "enforcement:value", "enforcement: 'value'", etc.
  enforcement=$(sed -n 's/^enforcement:[[:space:]]*["\x27]*\([^"\x27]*\)["\x27]*/\1/p' .project/context/gui-testing.md | head -1 | tr -d '[:space:]')

  # Fallback: try Python YAML parser if available and sed fails
  if [ -z "$enforcement" ] && command -v python3 &>/dev/null; then
    enforcement=$(python3 -c "
import yaml
import sys
try:
    with open('.project/context/gui-testing.md', 'r') as f:
        content = f.read()
        # Extract YAML frontmatter if present
        if content.startswith('---'):
            parts = content.split('---', 2)
            if len(parts) >= 3:
                data = yaml.safe_load(parts[1])
                print(data.get('enforcement', 'disabled'))
        else:
            data = yaml.safe_load(content)
            print(data.get('enforcement', 'disabled'))
except:
    print('disabled')
" 2>/dev/null)
  fi

  # Default to disabled if still empty
  enforcement=${enforcement:-disabled}

  if [ "$enforcement" != "disabled" ]; then
    echo "GUI_TESTING_ENABLED"
  fi
fi
```

**If GUI testing is enabled:**

1. **Read task description** from the local task file for design validation context

2. **Extract URL hints** from task description:
   - Look for route patterns like `/dashboard`, `/settings`
   - Look for URL mentions in the task
   - Fall back to default URL from config if no hints found

3. **Run GUI test:**
   Execute `/testing:gui {url} --design "{task_description}"`

   Pass the task description for AI design validation.

4. **Handle results based on enforcement mode:**

   **If enforcement = mandatory AND test FAILS:**
   ```
   ❌ GUI tests failed. Issue cannot be closed.

   {test_failure_details}

   Fix the issues above and retry /pm:issue-close $ARGUMENTS

   To bypass GUI testing, change enforcement to 'advisory' in:
   .project/context/gui-testing.md
   ```
   **STOP - do not proceed with close**

   **If enforcement = mandatory AND test PASSES:**
   Continue to step 7 (Output)

   **If enforcement = advisory AND test FAILS:**
   ```
   ⚠️ GUI tests failed (advisory mode)

   {test_failure_details}

   Close issue anyway? (yes/no)
   ```
   - If user says yes: Continue to step 7
   - If user says no: Stop and let user investigate

   **If enforcement = advisory AND test PASSES:**
   Continue to step 7 (Output)

5. **Include GUI test summary in GitHub close comment:**
   Add a section to the close comment with GUI test results.

### 7. Output

```
✅ Closed issue #$ARGUMENTS
  Local: Task marked complete
  GitHub: Issue closed & epic updated
  Epic progress: {new_progress}% ({closed}/{total} tasks complete)

Next: Run /pm:next for next priority task
```

## Important Notes

Follow `.claude/rules/frontmatter-operations.md` for updates.
Follow `.claude/rules/github-operations.md` for GitHub commands.
Always sync local state before GitHub.
