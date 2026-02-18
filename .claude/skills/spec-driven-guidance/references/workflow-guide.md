# Spec-Driven Development Workflow

## Core Concept

Spec-driven development transforms requirements into code through a structured pipeline:

```
PRD → Epic(s) → Task(s) → Code → Review → Complete
```

## Document Hierarchy

### PRDs (Product Requirement Documents)
- **Location**: `.project/prds/{name}.md`
- **Purpose**: Capture the "what" and "why" of a feature
- **Scope**: One feature or project initiative
- **Contains**: Overview, goals, user stories, requirements, non-goals, success metrics

### Epics
- **Location**: `.project/epics/{name}/epic.md`
- **Purpose**: Technical implementation plan for a PRD component
- **Scope**: A coherent unit of work (1-4 weeks typical)
- **Contains**: Technical approach, acceptance criteria, task breakdown

### Tasks
- **Location**: `.project/epics/{name}/{id}.md` or after sync `{issue-id}.md`
- **Purpose**: Atomic unit of work
- **Scope**: Hours to a few days
- **Contains**: Specific implementation details, acceptance criteria

## Workflow States

### PRD States
| Status | Description |
|--------|-------------|
| `draft` | Being written, not ready for epic breakdown |
| `backlog` | Ready for epic creation, waiting to be scheduled |
| `in-progress` | Epic work underway |
| `review` | Awaiting stakeholder review/sign-off |
| `complete` | All epics delivered |

### Epic States
| Status | Description |
|--------|-------------|
| `draft` | Being planned, not ready for tasks |
| `backlog` | Ready to start, waiting to be scheduled |
| `in-progress` | Tasks being worked |
| `review` | Awaiting code review/QA validation |
| `complete` | All tasks done |

### Task States
| Status | Description |
|--------|-------------|
| `open` | Ready to be started |
| `in-progress` | Currently being worked |
| `review` | Awaiting PR review/testing |
| `closed` | Done, cancelled, or won't-fix |

## State Machine

```
[No PRD]
    ↓ User describes feature
[Draft PRD]
    ↓ Move to backlog
[Backlog PRD]
    ↓ /pm:prd-parse
[Epic Created]
    ↓ /pm:epic-decompose
[Tasks Created]
    ↓ /pm:issue-start
[Work In Progress]
    ↓ Work complete
[Review] (if review_required)
    ↓ Approved
[Task Closed]
    ↓ All tasks closed
[Epic Review] (if review_required)
    ↓ Approved
[Epic Complete]
    ↓ All epics complete
[PRD Review] (if review_required)
    ↓ Approved
[PRD Complete]
```

## Frontmatter Schema

### PRD Frontmatter
```yaml
name: Feature Name
status: draft | backlog | in-progress | review | complete
created: 2024-01-15T10:30:00Z
updated: 2024-01-15T10:30:00Z
priority: critical | high | medium | low
review_required: true | false      # Cascade to epics and tasks (AI enforcement only)
```

### Epic Frontmatter
```yaml
name: Epic Name
prd: feature-name
status: draft | backlog | in-progress | review | complete
created: 2024-01-15T10:30:00Z
updated: 2024-01-15T10:30:00Z
progress: 0-100
review_required: true | false      # Cascade to tasks (AI enforcement only)
```

### Task Frontmatter
```yaml
name: Task Name
status: open | in-progress | review | closed
created: 2024-01-15T10:30:00Z
updated: 2024-01-15T10:30:00Z
parallel: true | false
depends_on: [001, 002]
type: feature | bug | chore | test
priority: high | medium | low
effort_size: S | M | L | XL       # T-shirt sizing (S=0.5d, M=1d, L=2d, XL=3d)
effort_hours: 8                    # Estimated hours (takes precedence over size)
review_required: true | false      # AI enforcement only
```

**Effort Estimation:**
- `effort_size`: Quick T-shirt sizing for task duration
  - `S` = 0.5 days (4 hours) - Small fixes, simple changes
  - `M` = 1 day (8 hours) - Typical feature tasks
  - `L` = 2 days (16 hours) - Complex features
  - `XL` = 3 days (24 hours) - Major components
- `effort_hours`: Precise estimate in hours (overrides `effort_size` if both present)
- Default: 1 day if neither specified
- Used by Gantt chart to calculate task duration and timeline

## Best Practices

### Writing Good PRDs
1. **Clear Problem Statement**: What pain point are we solving?
2. **Measurable Goals**: How will we know it succeeded?
3. **User-Centric Stories**: Who benefits and how?
4. **Explicit Non-Goals**: What are we NOT doing?
5. **Technical Constraints**: Dependencies, limitations, requirements

### Breaking Down Epics
1. **Cohesive Scope**: Each epic should be a logical unit
2. **Independent When Possible**: Minimize cross-epic dependencies
3. **Size Appropriately**: 1-4 weeks of work is typical
4. **Clear Boundaries**: Know where one epic ends and another begins

### Decomposing Tasks
1. **Atomic**: Complete in hours to a few days
2. **Independent**: Can be worked in isolation when possible
3. **Testable**: Clear acceptance criteria
4. **Parallel-aware**: Mark which tasks can run concurrently
5. **Dependency-tracked**: Explicit `depends_on` when needed
