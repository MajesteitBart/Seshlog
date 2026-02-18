---
name: meeting-companion
description: Desktop app bridging Obsidian meeting prep with live transcription and post-meeting notes
status: complete
created: 2026-01-22T20:50:23Z
updated: 2026-01-28T18:10:03Z
---

# PRD: Meeting Companion MVP

## Executive Summary

Meeting Companion is a desktop application that bridges **meeting preparation** (Obsidian markdown files) with **live meeting execution** (real-time transcription) and **post-meeting processing** (AI summaries written back to Obsidian).

The MVP focuses on establishing the core workflow: load a meeting prep file from Obsidian, capture and transcribe the meeting with Deepgram (with speaker diarization), and save everything back to the original markdown file.

**Fork basis:** Meetily (Tauri + Next.js + Rust audio capture)

## Problem Statement

### The Problem
Meeting workflows are fragmented:
- **Preparation** happens in note-taking apps (Obsidian, Notion)
- **Recording/transcription** happens in separate tools (Otter, Fireflies)
- **Post-meeting notes** require manual copy-paste between systems
- Context is lost between these disconnected steps

### Why Now
- Deepgram provides fast, accurate streaming transcription with speaker diarization
- Meetily provides proven audio capture infrastructure (mic + system audio)
- Obsidian's plain-text markdown format enables easy programmatic integration
- Personal need: streamline meeting workflow without cloud lock-in

## User Stories

### Primary Persona: Knowledge Worker with Obsidian Workflow

**User Story 1: Load Meeting Prep**
> As a user, I want to open my Obsidian meeting prep file in the app so I can reference my agenda and goals during the meeting.

Acceptance Criteria:
- [ ] Can browse and select any `.md` file from filesystem
- [ ] App parses frontmatter (date, attendees, type)
- [ ] App displays prep sections (Goals, Agenda, Context) in side panel
- [ ] Prep content is read-only during meeting

**User Story 2: Record and Transcribe**
> As a user, I want to record my meeting with real-time transcription so I can follow along and catch what was said.

Acceptance Criteria:
- [ ] Can select microphone and system audio devices
- [ ] Start/stop recording with clear UI controls
- [ ] Live transcript appears with speaker labels (diarization)
- [ ] Timestamps shown for each utterance
- [ ] Recording timer visible

**User Story 3: Save to Obsidian**
> As a user, I want the transcript and any notes saved back to my original Obsidian file so everything stays in one place.

Acceptance Criteria:
- [ ] "Stop & Save" action writes to original markdown file
- [ ] Transcript appended in designated section
- [ ] Original prep content preserved
- [ ] File remains valid Obsidian markdown

**User Story 4: Fallback to Local Transcription**
> As a user, I want the option to use local transcription if I'm offline or prefer not to use cloud services.

Acceptance Criteria:
- [ ] Settings toggle between Deepgram (primary) and local (Whisper/Parakeet)
- [ ] Local transcription works without internet
- [ ] UI indicates which transcription mode is active

## Requirements

### Functional Requirements

#### FR1: Meeting File Management
| ID | Requirement | Priority |
|----|-------------|----------|
| FR1.1 | Open any markdown file from filesystem | Must |
| FR1.2 | Parse YAML frontmatter for meeting metadata | Must |
| FR1.3 | Extract and display prep sections (Goals, Agenda, Context) | Must |
| FR1.4 | Remember last used directory | Should |

#### FR2: Audio Capture
| ID | Requirement | Priority |
|----|-------------|----------|
| FR2.1 | Capture microphone audio | Must |
| FR2.2 | Capture system audio (meeting apps) | Must |
| FR2.3 | Mix mic + system audio for transcription | Must |
| FR2.4 | Device selection UI | Must |
| FR2.5 | Audio level indicators | Should |

#### FR3: Transcription (Deepgram Primary)
| ID | Requirement | Priority |
|----|-------------|----------|
| FR3.1 | Stream audio to Deepgram WebSocket API | Must |
| FR3.2 | Display live transcript with timestamps | Must |
| FR3.3 | Speaker diarization (identify who spoke) | Must |
| FR3.4 | Handle interim vs final results | Must |
| FR3.5 | Deepgram API key configuration in settings | Must |

#### FR4: Local Transcription (Fallback)
| ID | Requirement | Priority |
|----|-------------|----------|
| FR4.1 | Retain Whisper transcription engine | Must |
| FR4.2 | Retain Parakeet transcription engine | Should |
| FR4.3 | Settings toggle: Deepgram vs Local | Must |
| FR4.4 | Graceful fallback if Deepgram unavailable | Should |

#### FR5: File Output
| ID | Requirement | Priority |
|----|-------------|----------|
| FR5.1 | Write transcript to original markdown file | Must |
| FR5.2 | Preserve existing file content | Must |
| FR5.3 | Format transcript with timestamps and speakers | Must |
| FR5.4 | Update frontmatter status field | Should |

### Non-Functional Requirements

#### NFR1: Performance
| ID | Requirement | Target |
|----|-------------|--------|
| NFR1.1 | Transcription latency (Deepgram) | < 500ms |
| NFR1.2 | UI responsiveness during recording | No frame drops |
| NFR1.3 | Memory usage | < 500MB typical |

#### NFR2: Reliability
| ID | Requirement | Target |
|----|-------------|--------|
| NFR2.1 | Audio capture stability | No drops in 2hr meeting |
| NFR2.2 | Transcript data persistence | No data loss on crash |
| NFR2.3 | File write safety | Atomic writes, no corruption |

#### NFR3: Usability
| ID | Requirement | Target |
|----|-------------|--------|
| NFR3.1 | Time to start recording from app launch | < 30 seconds |
| NFR3.2 | Clear error messages | User understands what went wrong |

#### NFR4: Platform
| ID | Requirement | Target |
|----|-------------|--------|
| NFR4.1 | Primary platform | Windows 10/11 x64 |
| NFR4.2 | Secondary platforms | macOS, Linux (defer) |

## Success Criteria

### MVP Launch Criteria
- [ ] Can load an Obsidian meeting prep file and display its content
- [ ] Can record meeting with mic + system audio
- [ ] Live Deepgram transcription with speaker labels works reliably
- [ ] Transcript saves back to original markdown file correctly
- [ ] Local Whisper fallback works when Deepgram unavailable

### Key Metrics
| Metric | Target |
|--------|--------|
| Successful meeting recordings (no crashes) | 95% |
| Transcription accuracy (Deepgram) | > 90% WER |
| File save success rate | 100% |
| Time from "Stop" to file saved | < 5 seconds |

## Constraints & Assumptions

### Constraints
- **Deepgram API costs** - User provides own API key; no free tier bundled
- **Windows audio capture** - WASAPI has limitations with some audio configurations
- **File format** - Only standard Obsidian markdown supported (no plugins)

### Assumptions
- User has existing Obsidian vault with meeting files
- User has Deepgram API key (or willing to use local transcription)
- Meeting files follow a reasonable structure (has sections for prep/notes)
- Internet available for Deepgram (local fallback for offline)

## Out of Scope (MVP)

The following are explicitly **not** in MVP scope:

| Feature | Reason | Future Phase |
|---------|--------|--------------|
| AI summarization | Complexity; Phase 4 | Phase 4 |
| Action item extraction | Requires AI | Phase 4 |
| Timestamp marking (manual) | Nice-to-have | Phase 3 |
| Inline task creation | Nice-to-have | Phase 3 |
| Calendar integration | Complexity | Phase 5 |
| Meeting templates | Polish feature | Phase 5 |
| macOS/Linux builds | Windows-first | Post-MVP |
| System tray mode | Polish feature | Phase 5 |
| File watching (auto-detect meetings) | Complexity | Phase 2+ |

## Dependencies

### External Dependencies
| Dependency | Type | Risk |
|------------|------|------|
| Deepgram API | Cloud service | Medium - API changes, costs |
| Whisper.cpp | Local library | Low - already integrated |
| Tauri framework | Build system | Low - stable |

### Internal Dependencies
| Dependency | Description |
|------------|-------------|
| Meetily audio capture | Reuse existing mic + system audio code |
| Meetily Whisper engine | Retain as fallback |
| Meetily UI framework | Adapt existing Next.js components |

## Technical Approach

### What to Keep from Meetily
- Audio capture pipeline (`audio/` module)
- Whisper engine (`whisper_engine/`)
- Parakeet engine (`parakeet_engine/`)
- Tauri + Next.js framework
- SQLite database (for meeting metadata)
- Basic UI components

### What to Add
- Deepgram WebSocket streaming module (Rust)
- Obsidian file parser (extract frontmatter + sections)
- Obsidian file writer (merge transcript back)
- Prep panel UI component
- Transcription provider toggle in settings

### What to Remove/Simplify
- Backend FastAPI server (not needed for MVP)
- Complex meeting management UI
- Ollama/Claude/Groq integrations (defer to Phase 4)
- Summary generation features

## UI Concept (MVP)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Meeting Companion                              [_][â–¡][X]   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  [Open Meeting File]  meeting-prep-2026-01-22.md           â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ PREP                  â”‚ TRANSCRIPT                          â”‚
â”‚                       â”‚                                     â”‚
â”‚ ## Goals              â”‚ [00:00:12] Speaker 1: Welcome       â”‚
â”‚ - [ ] Align timeline  â”‚ everyone to the kickoff...          â”‚
â”‚ - [ ] Assign work     â”‚                                     â”‚
â”‚                       â”‚ [00:00:34] Speaker 2: Thanks for    â”‚
â”‚ ## Agenda             â”‚ having us. Let's start with...      â”‚
â”‚ 1. Introductions      â”‚                                     â”‚
â”‚ 2. Scope review       â”‚ [00:01:02] Speaker 1: Great idea.   â”‚
â”‚ 3. Timeline           â”‚ The first item on the agenda...     â”‚
â”‚                       â”‚                                     â”‚
â”‚ ## Context            â”‚                                     â”‚
â”‚ Previous meeting...   â”‚                                     â”‚
â”‚                       â”‚                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  [â— Recording 00:01:15]              [â–  Stop & Save]        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

## Appendix: Meeting File Format

### Expected Input Structure
```markdown
---
date: 2026-01-22
type: meeting
attendees: [Attendee A, Attendee B]
status: scheduled
---

# Meeting: Project Kickoff

## Prep
### Goals
- [ ] Align on timeline

### Agenda
1. Introductions

### Context
Background info...
```

### Expected Output Structure
```markdown
---
date: 2026-01-22
type: meeting
attendees: [Attendee A, Attendee B]
status: completed
---

# Meeting: Project Kickoff

## Prep
[... preserved ...]

## Transcript
[00:00:12] **Speaker 1:** Welcome everyone...
[00:00:34] **Speaker 2:** Thanks for having us...
```

