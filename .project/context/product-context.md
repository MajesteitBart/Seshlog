---
created: 2026-01-22T20:32:59Z
last_updated: 2026-01-22T21:13:34Z
version: 1.1
author: Claude Code PM System
---

# Product Context

## Product Definition

**Meeting Companion** is a desktop application that bridges Obsidian meeting preparation with live transcription and post-meeting processing. It is a fork of Meetily, streamlined for the Obsidian workflow.

> **Full requirements**: See `.project/prds/meeting-companion.md`

## Core Workflow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   PREPARE   │ ──► │   EXECUTE   │ ──► │   PROCESS   │
│  (Obsidian) │     │   (Live)    │     │   (Save)    │
└─────────────┘     └─────────────┘     └─────────────┘
Load .md file       Record meeting      Save transcript
Parse prep          Deepgram STT        back to .md file
Show Goals/         Speaker labels      Update status
Agenda/Context      Live transcript
```

## Target User

**Primary Persona**: Knowledge worker who uses Obsidian for meeting preparation and wants seamless transcription integration.

| Need | Solution |
|------|----------|
| Prepare meetings in Obsidian | Load .md file with Goals/Agenda/Context |
| Live transcription with speaker ID | Deepgram streaming with diarization |
| Transcript back to Obsidian | Write ## Transcript section to source file |
| No cloud dependency for notes | All files stay in local vault |

## Key Features (MVP)

### FR1: Meeting File Import
- Open any `.md` file from Obsidian vault
- Parse YAML frontmatter (date, attendees, status)
- Extract ## Prep section (Goals, Agenda, Context)
- Display in PrepPanel during meeting

### FR2: Live Transcription
- Real-time Deepgram streaming
- Speaker diarization (Speaker 1, Speaker 2, etc.)
- Timestamped entries
- VirtualizedTranscriptView for performance

### FR3: Post-Meeting Save
- Append ## Transcript section to source file
- Update frontmatter `status: completed`
- Atomic file writes (temp + rename)
- Preserve all original content

## Functional Requirements Reference

See PRD for complete requirements:
- **FR1**: Meeting File Import (`.project/prds/meeting-companion.md`)
- **FR2**: Audio Capture (existing Meetily code)
- **FR3**: Cloud Transcription (Deepgram)
- **FR4**: Local Transcription Fallback (Whisper)
- **FR5**: Meeting File Save

## Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| Transcription latency | < 500ms (Deepgram interim) |
| File save reliability | Atomic writes, no data loss |
| Memory usage | < 500MB during recording |
| Platform | Windows primary, macOS/Linux deferred |

## Out of Scope (MVP)

- AI summarization (Phase 4)
- Calendar integration (Phase 5)
- Timestamp marking hotkeys (Phase 3)
- Inline task creation (Phase 3)
- System tray mode (Phase 5)

## Key Differentiators from Meetily

| Meetily | Meeting Companion |
|---------|-------------------|
| Standalone meeting management | Obsidian-native workflow |
| Local-only transcription | Cloud-first (Deepgram) + local fallback |
| Requires FastAPI backend | No backend (native SQLx) |
| No speaker identification | Diarization built-in |

## Update History
- 2026-01-22: Updated for Meeting Companion fork, referenced PRD
