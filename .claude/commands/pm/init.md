---
description: Install dependencies, configure GitHub, and initialize the PM system
allowed-tools: ExecuteCommand, ListFiles, WriteFile
---

Initialize the Claude Code PM System by:

1. **Check dependencies:**
   - Verify GitHub CLI (gh) is installed
   - If not, prompt user to install it from https://cli.github.com/

2. **Check GitHub authentication:**
   - Verify `gh auth status` shows authenticated state
   - If not authenticated, prompt user to run `gh auth login`

3. **Check gh extensions:**
   - Verify gh-sub-issue extension is installed
   - If not, prompt user to run `gh extension install yahsan2/gh-sub-issue`

4. **Create directory structure:**
   - `.project/prds/` - Product requirement documents
   - `.project/epics/` - Epic directories containing tasks
   - `.claude/rules/` - Project rules
   - `.claude/agents/` - Custom agents
   - `.claude/scripts/pm/` - PM scripts (will be migrated later)

5. **Check Git configuration:**
   - Verify this is a git repository
   - Check for remote origin
   - Warn if remote points to template repository

6. **GUI Project Detection:**

   Detect if this is a GUI/frontend project by checking for:
   ```bash
   # Check for frontend frameworks in package.json
   test -f package.json && grep -qE '"react"|"vue"|"angular"|"svelte"|"next"|"nuxt"|"vite"' package.json 2>/dev/null && echo "Frontend framework detected"

   # Check for frontend directories
   ls -d frontend/ web/ client/ app/ src/app/ src/pages/ 2>/dev/null && echo "Frontend directory detected"

   # Check for HTML/CSS files
   find . -maxdepth 3 \( -name "*.html" -o -name "*.css" -o -name "*.scss" -o -name "*.tsx" -o -name "*.jsx" \) 2>/dev/null | head -1 && echo "Frontend files detected"

   # Check for dev server scripts
   test -f package.json && grep -qE '"dev"|"start"|"serve"' package.json 2>/dev/null && echo "Dev server script detected"
   ```

   **If GUI project detected:**

   Ask user: "Detected a GUI/frontend project. Would you like to configure GUI testing? (yes/no)"

   If yes, ask:
   1. "What is your development server URL?" (default: http://localhost:3000)
   2. "Enforcement mode - mandatory (blocks task close) or advisory (warns only)?" (default: advisory)

   Then create `.project/context/gui-testing.md` with configuration:
   ```yaml
   ---
   created: {current_datetime}
   updated: {current_datetime}
   ---
   # GUI Testing Configuration

   ## Settings
   type: web-application
   enforcement: {user_choice}
   default_url: {user_url}

   ## Smoke Routes
   smoke_routes:
     - path: /
       name: Home Page

   ## Console Filtering
   console_filters:
     ignore_patterns:
       - "DevTools failed to load"
       - "ResizeObserver loop"
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

   Display: "GUI testing configured. Run /testing:gui to test your UI."

   **If not a GUI project or user declines:**
   Skip this step silently.

7. **Create GitHub labels (if applicable):**
   - Create "epic" label (color: 0E8A16)
   - Create "task" label (color: 1D76DB)

8. **Create CLAUDE.md** if it doesn't exist with basic template

9. **Display summary** with next steps:
   - Create first PRD: `/pm:prd-new <feature-name>`
   - View help: `/pm:help`
   - Check status: `/pm:status`

Note: The automated script-based initialization will be available after scripts are migrated from `mirror/scripts/pm/init.sh` to `.claude/scripts/pm/init.sh`.
