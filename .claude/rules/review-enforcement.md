# Review Enforcement Rule

AI agents must validate review requirements before completing PRDs, epics, or tasks.

## Review Required Inheritance

The `review_required` field cascades from parent to child:

```
PRD (review_required: true)
 └── Epic (inherits: true) ──► cannot skip review
      └── Task (inherits: true) ──► cannot skip review

PRD (review_required: false or unset)
 └── Epic (review_required: true)
      └── Task (inherits: true) ──► cannot skip review
 └── Epic (review_required: false or unset)
      └── Task (review_required: true) ──► cannot skip review
      └── Task (review_required: false or unset) ──► review optional
```

## Resolution Logic

When checking if review is required:

1. **Check PRD**: If `review_required: true`, return `true`
2. **Check Epic**: If `review_required: true`, return `true`
3. **Check Task**: If `review_required: true`, return `true`
4. **Default**: Return `false` (review is optional)

First `true` in the chain wins. Child settings cannot override parent enforcement.

## Validation Before Closing

Before transitioning to `complete` (PRD/Epic) or `closed` (Task):

```
IF review_required is true (inherited or direct):
  IF current status != 'review':
    WARN: "Review required. Move to 'review' status first."
    PROMPT: "Set status to 'review' now? [Y/n]"
```

## Commands Affected

These commands must check review enforcement:

- `/pm:issue-close` - Check task and parent epic/PRD
- `/pm:epic-close` - Check epic and parent PRD
- `/pm:prd-close` - Check PRD setting (if command exists)

## Example Frontmatter

**PRD with enforcement:**
```yaml
---
name: payment-system
status: in-progress
review_required: true    # All epics and tasks must go through review
---
```

**Epic with enforcement:**
```yaml
---
name: stripe-integration
status: in-progress
prd: payment-system
review_required: true    # All tasks in this epic must go through review
---
```

**Task with enforcement:**
```yaml
---
name: Add webhook handler
status: in-progress
review_required: true    # Just this task requires review
---
```

## Important Notes

- This enforcement is for AI agents only
- Dashboard UI always shows all status options (no restrictions)
- Human users can freely change status in the dashboard
- AI agents should prompt but not block if user explicitly requests skipping review
