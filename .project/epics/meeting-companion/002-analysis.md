---
issue: 002
title: Create Obsidian file parser
analyzed: 2026-01-23T11:19:08Z
estimated_hours: 3
parallelization_factor: 1.5
---

# Parallel Work Analysis: Issue #002

## Overview
Create a Rust module to parse Obsidian markdown meeting files. This involves creating data structures, implementing parsing logic for YAML frontmatter and markdown sections, and exposing via Tauri command.

## Parallel Streams

### Stream A: Types & Module Structure
**Scope**: Create the module skeleton and data structures
**Files**:
- `frontend/src-tauri/src/obsidian/mod.rs`
- `frontend/src-tauri/src/obsidian/types.rs`
**Agent Type**: backend-specialist
**Can Start**: immediately
**Estimated Hours**: 0.5
**Dependencies**: none

**Tasks**:
1. Create `obsidian/` module directory
2. Define `MeetingPrep` struct
3. Define `MeetingFrontmatter` struct
4. Set up module exports

### Stream B: Parser Implementation
**Scope**: Implement parsing logic for frontmatter and markdown sections
**Files**:
- `frontend/src-tauri/src/obsidian/parser.rs`
**Agent Type**: backend-specialist
**Can Start**: after Stream A (needs types defined)
**Estimated Hours**: 1.5
**Dependencies**: Stream A

**Tasks**:
1. Implement frontmatter extraction between `---` markers
2. Implement YAML parsing with `serde_yaml`
3. Implement section extraction (Goals, Agenda, Context)
4. Implement title extraction
5. Handle edge cases and malformed input

### Stream C: Tauri Integration & Tests
**Scope**: Create Tauri command and unit tests
**Files**:
- `frontend/src-tauri/src/obsidian/mod.rs` (command registration)
- `frontend/src-tauri/src/lib.rs` (command export)
- `frontend/src-tauri/src/obsidian/parser.rs` (tests module)
**Agent Type**: backend-specialist
**Can Start**: after Stream B
**Estimated Hours**: 1.0
**Dependencies**: Stream B

**Tasks**:
1. Create `open_meeting_file` Tauri command
2. Register command in `lib.rs`
3. Write unit tests for frontmatter parsing
4. Write unit tests for section extraction
5. Test with sample Obsidian file

## Coordination Points

### Shared Files
- `frontend/src-tauri/src/obsidian/mod.rs` - Streams A & C (A creates, C adds command)
- `frontend/src-tauri/Cargo.toml` - Stream A (add serde_yaml dependency)

### Sequential Requirements
1. Types must be defined before parser can use them
2. Parser must work before Tauri command can call it
3. All code must exist before tests can run

## Conflict Risk Assessment
- **Low Risk**: Streams work on sequential phases of the same module
- This is fundamentally a sequential task with minimal parallelization opportunity

## Parallelization Strategy

**Recommended Approach**: sequential

This task is small (3 hours estimated) and has tight internal dependencies. The three "streams" are really phases of a single piece of work:
1. Define types
2. Implement parser
3. Integrate and test

Attempting to parallelize would add coordination overhead without meaningful time savings.

**Alternative**: If you want parallelism, work on Task 004 (Remove backend dependency) simultaneously, as it's truly independent.

## Expected Timeline

With sequential execution (recommended):
- Wall time: 3 hours
- Total work: 3 hours

This task is best done by a single agent in one focused session.

## Notes
- `serde_yaml` dependency needs to be added to Cargo.toml
- Task 003 (Obsidian file writer) conflicts with this task - same module
- Consider completing both 002 and 003 together to avoid module coordination issues
- Sample test file should be created for manual testing
