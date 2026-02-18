---
description: Check system integrity, directory structure, and data consistency
allowed-tools: ListFiles, Read
---

Validate the PM System by checking:

## Directory Structure
- `.claude/` directory exists (required)
- `.project/prds/` directory exists
- `.project/epics/` directory exists
- `.claude/rules/` directory exists

## Data Integrity
- Check that each epic directory has an `epic.md` file
- Check for orphaned task files (tasks outside epic directories)

## Reference Check
- For each task file, verify that dependencies (`depends_on:`) reference valid task files within the same epic

## Frontmatter Validation
- Check that all files in `.project/epics/` and `.project/prds/` have valid frontmatter (marked by `---` delimiters)

## Validation Summary
Display:
- Number of errors (critical issues)
- Number of warnings (non-critical issues)
- Number of files with invalid frontmatter

If all checks pass, display "✅ System is healthy!"
Otherwise, suggest running `/pm:clean` to fix some issues automatically.

Note: The automated script-based validation will be available after scripts are migrated from `mirror/scripts/pm/validate.sh` to `.claude/scripts/pm/validate.sh`.
