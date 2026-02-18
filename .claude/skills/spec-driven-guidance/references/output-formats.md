# Output Formats

Standard formats for guidance responses. Use these templates for consistent, actionable suggestions.

## PRD Guidance Format

Use when helping structure a new PRD:

```markdown
## PRD Suggestion: {Feature Name}

### Recommended Structure
1. **Overview** - What problem are we solving?
2. **Goals** - What does success look like?
3. **User Stories** - Who benefits and how?
4. **Requirements** - What must the solution do?
5. **Non-Goals** - What are we explicitly NOT doing?
6. **Technical Considerations** - Any constraints or dependencies?
7. **Success Metrics** - How will we measure success?

### Content Suggestions
- {Specific suggestion based on context}
- {Another suggestion}

### Next Step
Run `/pm:prd-new {feature-name}` to create this PRD
```

## Epic Breakdown Format

Use when suggesting how to split a PRD into epics:

```markdown
## Epic Breakdown Suggestion

Based on PRD: {prd-name}

### Proposed Epics
1. **{epic-name}** - {brief description}
   - Scope: {small/medium/large}
   - Dependencies: {none or list}

2. **{epic-name}** - {brief description}
   - Scope: {small/medium/large}
   - Dependencies: {epic-1 if applicable}

### Recommended Sequence
1. Start with: {epic-name} (no dependencies)
2. Then: {epic-name} (depends on #1)

### Next Step
Run `/pm:epic-start {first-epic-name}` to begin
```

## Task Decomposition Format

Use when suggesting task breakdown for an epic:

```markdown
## Task Breakdown Suggestion

For Epic: {epic-name}

### Proposed Tasks
1. **{task-title}**
   - Type: {feature/bug/chore/test}
   - Priority: {high/medium/low}
   - Parallel: {yes/no}
   - Description: {what needs to be done}

2. **{task-title}**
   - Type: {feature/bug/chore/test}
   - Priority: {high/medium/low}
   - Parallel: {yes/no}
   - Description: {what needs to be done}

### Dependencies
- Task 2 depends on Task 1
- Tasks 3, 4 can be parallelized

### Next Step
Run `/pm:epic-decompose {epic-name}` to create these tasks
```

## Progress Summary Format

Use for status overviews:

```markdown
## Workflow Status

### PRDs
| PRD | Status | Progress | Epics |
|-----|--------|----------|-------|
| {name} | {status} | {%} | {count} |

### Active Epics
| Epic | PRD | Status | Tasks | Progress |
|------|-----|--------|-------|----------|
| {name} | {prd} | {status} | {done}/{total} | {%} |

### Recommended Next Actions
1. {Specific action with command}
2. {Another action}
```

## Review Feedback Format

Use when reviewing existing documents:

```markdown
## Review: {document-name}

### Strengths
- {What's good about the document}

### Suggestions
- **{Area}**: {Specific improvement suggestion}
- **{Area}**: {Another suggestion}

### Missing Elements
- {Required section or content that's absent}

### Next Step
{Appropriate command or manual edit suggestion}
```

## Workflow State Response Format

Use when user asks "what's next?" or about workflow:

```markdown
## Current State: {state-name}

You are at: {description of current position in workflow}

### Completed
- {Previous step}

### Current
- {What should happen now}

### Up Next
- {What follows after current step}

### Action
Run `{command}` to proceed
```
