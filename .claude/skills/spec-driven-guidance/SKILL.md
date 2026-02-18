---
name: spec-driven-guidance
description: Guides users through the spec-driven development workflow (PRD → Epic → Task). Activates when planning features, asking about workflow, or needing help with PRDs/epics/tasks. Advisory only - suggests but never creates.
allowed-tools: Read, Glob, Grep
---

# Spec-Driven Guidance

Guide users through the spec-driven development workflow from initial ideas to completed tasks.

## When to Activate

Activate this skill when the user:
- Wants to plan a new feature
- Asks "how do I..." for PRD/epic/task workflow
- Needs help structuring a PRD
- Asks about decomposing epics into tasks
- Is unsure what step comes next in the workflow

## Workflow Stages

### Stage 1: PRD Creation

When the user has a feature idea:

1. Help structure the idea into PRD sections
2. Suggest: "Use `/pm:prd-new <name>` to create the PRD"
3. Recommend sections: Overview, Goals, Requirements, Success Criteria

**Guidance Questions:**
- What is the feature's primary goal?
- Who are the target users?
- What are the success criteria?
- What are the technical requirements?

### Stage 2: PRD to Epic

When a PRD exists:

1. Review PRD content
2. Suggest epic breakdown approach
3. Recommend: "Use `/pm:prd-parse <name>` to create the epic"

**Guidance Questions:**
- What are the major components of this feature?
- Can this be broken into logical phases?
- Are there dependencies between components?

### Stage 3: Epic Decomposition

When an epic exists:

1. Analyze epic for task breakdown
2. Suggest task structure and dependencies
3. Recommend parallel vs sequential work

**Guidance Questions:**
- What are the atomic units of work?
- Which tasks can be done in parallel?
- What are the task dependencies?

### Stage 4: Task Execution

When tasks exist:

1. Identify next priority task
2. Suggest starting point
3. Recommend: "Use `/pm:next` to see priority task"

**Guidance Questions:**
- Which task is blocking others?
- What's the highest priority?
- What resources are needed?

## Output Format

Provide guidance in this structure:

```markdown
## Workflow Guidance

**Current Stage**: {stage name}
**Context**: {what exists, what's needed}

### Recommended Next Steps
1. {step with command suggestion}
2. {step}

### Tips
- {relevant tip for current stage}
- {another tip}

### Questions to Consider
- {question 1}
- {question 2}
```

## Boundaries

### CAN Do ✅
- Analyze existing PRDs, epics, tasks
- Suggest workflow steps
- Recommend commands to use
- Provide structural guidance
- Answer questions about the workflow

### CANNOT Do ❌
- Create PRDs, epics, or tasks
- Modify existing files
- Execute commands on user's behalf
- Make decisions about implementation details

## Related Commands

Useful commands for each stage:
- `/pm:prd-new <name>` - Create a new PRD
- `/pm:prd-parse <name>` - Parse PRD into epic
- `/pm:epic-status` - Check epic status
- `/pm:next` - Get next priority task
- `/pm:standup` - Get work status summary
