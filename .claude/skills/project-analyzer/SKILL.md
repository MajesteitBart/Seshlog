---
name: project-analyzer
description: Analyzes codebase structure, technology stack, and development patterns. Invokes when user asks about project structure, tech stack, onboarding, or needs agent recommendations. Read-only analysis - never modifies files.
allowed-tools: Read, Glob, Grep, Bash
---

# Project Analyzer

Analyze the current project's codebase to understand its structure, technology stack, and development patterns.

## When to Activate

Activate this skill when the user:
- Asks "what is this project?" or "analyze this project"
- Needs help understanding the codebase structure
- Asks about the technology stack or frameworks used
- Is onboarding to a new project
- Needs recommendations for agents or workflows

## Analysis Process

### 1. Detect Project Type
Check for these indicators:

**JavaScript/TypeScript:**
- `package.json` - Read for dependencies, scripts, name
- `tsconfig.json` - TypeScript configuration
- `node_modules/` presence

**Python:**
- `requirements.txt`, `pyproject.toml`, `setup.py`
- `venv/`, `.venv/` presence

**Rust:**
- `Cargo.toml` - Read for dependencies

**Go:**
- `go.mod` - Read for module info

**General:**
- `.git/` - Git repository
- `README.md` - Project documentation
- `.github/` - GitHub workflows
- `Dockerfile`, `docker-compose.yml` - Containerization

### 2. Analyze Structure
Use Glob and Read to understand:
- Source code directories (`src/`, `lib/`, `app/`)
- Test directories (`tests/`, `__tests__/`, `spec/`)
- Configuration files
- Documentation

### 3. Identify Patterns
Look for:
- Code organization (monorepo, multi-package)
- Testing framework indicators
- CI/CD configuration
- Development workflow files

## Output Format

Provide analysis in this structure:

```markdown
## Project Analysis: {project_name}

### Overview
- **Type**: {language/framework}
- **Structure**: {monorepo/single-package/library}
- **Primary Language**: {language}

### Technology Stack
| Category | Technology |
|----------|------------|
| Language | {lang} |
| Framework | {framework} |
| Testing | {test framework} |
| Build | {build tool} |

### Directory Structure
```
{key directories with descriptions}
```

### Key Files
- `{file}` - {purpose}

### Development Workflow
{detected workflow patterns}

### Recommended Agents
Based on this project, these agents would be useful:
1. **{agent-name}** - {purpose}
```

## Boundaries

### CAN Do ✅
- Read and analyze any file
- Detect languages, frameworks, patterns
- Suggest agents and workflows
- Provide structural overview

### CANNOT Do ❌
- Create, modify, or delete files
- Install dependencies
- Run build commands
- Make changes to the project

## Related Commands
After analysis, user may want to:
- `/pm:init` - Initialize PM system
- `/context:create` - Create context documentation
