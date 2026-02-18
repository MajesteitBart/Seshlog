# Meeting Companion App - High-Level Plan

## Vision

A desktop app that bridges your **meeting preparation** (Claude + Obsidian) with **live meeting execution** and **post-meeting notes** â€” all in one unified workflow.

---

## Core Concept: The Meeting Document Lifecycle

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    MEETING DOCUMENT LIFECYCLE                    â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                  â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”       â”‚
â”‚  â”‚   PREPARE    â”‚â”€â”€â”€â–¶â”‚   EXECUTE    â”‚â”€â”€â”€â–¶â”‚   PROCESS    â”‚       â”‚
â”‚  â”‚  (Obsidian)  â”‚    â”‚    (App)     â”‚    â”‚ (App + LLM)  â”‚       â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜       â”‚
â”‚                                                                  â”‚
â”‚  Claude generates     Live transcription   AI summarizes         â”‚
â”‚  meeting prep file    Tasks, notes added   Final notes saved     â”‚
â”‚  with agenda, goals   Timestamps marked    back to Obsidian      â”‚
â”‚                                                                  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## Feature Overview

### 1. Meeting Prep Import
- Open any `.md` file from your Obsidian vault
- Parse existing structure: agenda items, goals, attendees, context
- Display prep content in a side panel during meeting

### 2. Live Meeting Panel
- Real-time transcript (Deepgram streaming)
- Speaker diarization
- Click-to-timestamp: mark important moments
- Quick-add tasks inline (checkbox items)
- Manual notes field that syncs with transcript timestamps

### 3. Post-Meeting Processing
- AI summary generation (Claude API or local LLM)
- Extract action items from transcript
- Merge everything back into original Obsidian file
- Append transcript as collapsible section

---

## Architecture

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        TAURI APPLICATION                         â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                  â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚  â”‚                    NEXT.JS FRONTEND                      â”‚    â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤    â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”‚    â”‚
â”‚  â”‚  â”‚  Prep View  â”‚  â”‚ Live Panel  â”‚  â”‚ Summary Viewâ”‚      â”‚    â”‚
â”‚  â”‚  â”‚             â”‚  â”‚             â”‚  â”‚             â”‚      â”‚    â”‚
â”‚  â”‚  â”‚ - Agenda    â”‚  â”‚ - Transcriptâ”‚  â”‚ - AI Summaryâ”‚      â”‚    â”‚
â”‚  â”‚  â”‚ - Goals     â”‚  â”‚ - Timestampsâ”‚  â”‚ - Actions   â”‚      â”‚    â”‚
â”‚  â”‚  â”‚ - Context   â”‚  â”‚ - Tasks     â”‚  â”‚ - Export    â”‚      â”‚    â”‚
â”‚  â”‚  â”‚ - Attendees â”‚  â”‚ - Notes     â”‚  â”‚             â”‚      â”‚    â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜      â”‚    â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚                              â”‚                                   â”‚
â”‚                              â–¼                                   â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚  â”‚                    RUST BACKEND (TAURI)                  â”‚    â”‚
â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤    â”‚
â”‚  â”‚                                                          â”‚    â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”      â”‚    â”‚
â”‚  â”‚  â”‚Audio Captureâ”‚  â”‚  Deepgram   â”‚  â”‚  File I/O   â”‚      â”‚    â”‚
â”‚  â”‚  â”‚(from Meetilyâ”‚  â”‚  WebSocket  â”‚  â”‚  (Obsidian) â”‚      â”‚    â”‚
â”‚  â”‚  â”‚             â”‚  â”‚             â”‚  â”‚             â”‚      â”‚    â”‚
â”‚  â”‚  â”‚ - System    â”‚  â”‚ - Streaming â”‚  â”‚ - Read .md  â”‚      â”‚    â”‚
â”‚  â”‚  â”‚ - Mic       â”‚  â”‚ - Diarize   â”‚  â”‚ - Write .md â”‚      â”‚    â”‚
â”‚  â”‚  â”‚ - Mix       â”‚  â”‚ - Interim   â”‚  â”‚ - Watch dir â”‚      â”‚    â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜      â”‚    â”‚
â”‚  â”‚                                                          â”‚    â”‚
â”‚  â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                       â”‚    â”‚
â”‚  â”‚  â”‚  Local DB   â”‚  â”‚  LLM Client â”‚                       â”‚    â”‚
â”‚  â”‚  â”‚  (SQLite)   â”‚  â”‚  (Claude/   â”‚                       â”‚    â”‚
â”‚  â”‚  â”‚             â”‚  â”‚   Ollama)   â”‚                       â”‚    â”‚
â”‚  â”‚  â”‚ - Meetings  â”‚  â”‚             â”‚                       â”‚    â”‚
â”‚  â”‚  â”‚ - Segments  â”‚  â”‚ - Summarize â”‚                       â”‚    â”‚
â”‚  â”‚  â”‚ - Tasks     â”‚  â”‚ - Extract   â”‚                       â”‚    â”‚
â”‚  â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                       â”‚    â”‚
â”‚  â”‚                                                          â”‚    â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜    â”‚
â”‚                                                                  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## Meeting Document Structure

Your Obsidian meeting prep file becomes a living document:

```markdown
---
date: 2025-01-22
type: meeting
attendees: [Attendee A, Attendee B]
status: completed
---

# Meeting: Project Kickoff

## Prep (Pre-meeting)
### Goals
- [ ] Align on timeline
- [ ] Assign workstreams

### Agenda
1. Introductions (5 min)
2. Project scope review (15 min)
3. Timeline discussion (10 min)

### Context
Key background from previous conversations...

---

## Live Notes (During meeting)
### Tasks
- [ ] @AttendeeA: Send updated proposal by Friday [00:12:34]
- [ ] @AttendeeB: Schedule follow-up with legal [00:23:45]

### Timestamps
- [00:05:23] Important decision about budget
- [00:18:42] Sarah raised concern about timeline

### Manual Notes
- John mentioned they're flexible on Q2 deadline
- Budget approved for external contractor

---

## Summary (Post-meeting)
### Key Decisions
- Timeline pushed to March 15
- Budget increased to â‚¬50k

### Action Items
| Owner | Task | Due |
|-------|------|-----|
| John | Send proposal | Jan 24 |
| Sarah | Legal review | Jan 26 |

### Next Steps
Follow-up meeting scheduled for Jan 29

---

## Transcript
> [!collapsible]- Full Transcript
> [00:00:00] **Attendee A:** Thanks everyone for joining...
> [00:00:15] **Attendee B:** Happy to be here...
```

---

## Development Phases

### Phase 1: Foundation (Week 1-2)
**Goal:** Fork Meetily, get basic audio + transcription working

- [ ] Fork Meetily repository
- [ ] Strip out unused features (Whisper, Parakeet)
- [ ] Implement Deepgram WebSocket streaming
- [ ] Test dual audio capture (system + mic)
- [ ] Basic UI: start/stop recording, live transcript display

### Phase 2: Obsidian Integration (Week 3-4)
**Goal:** Read and write meeting files from/to Obsidian vault

- [ ] File picker: select Obsidian vault directory
- [ ] Recent meetings list (scan vault for meeting files)
- [ ] Parse meeting prep markdown (extract agenda, goals, context)
- [ ] Display prep content in side panel
- [ ] Write updated markdown back to vault

### Phase 3: Live Meeting Features (Week 5-6)
**Goal:** Full meeting execution experience

- [ ] Timestamp marking (hotkey + button)
- [ ] Inline task creation (converts to checkbox in markdown)
- [ ] Manual notes panel (timestamped)
- [ ] "Catch-up" scroll: jump back in transcript
- [ ] Agenda item tracking (mark as discussed)

### Phase 4: AI Processing (Week 7-8)
**Goal:** Post-meeting intelligence

- [ ] Claude API integration for summarization
- [ ] Prompt engineering for action item extraction
- [ ] Decision extraction from transcript
- [ ] Merge AI output with manual notes
- [ ] Final markdown generation

### Phase 5: Polish & Workflow (Week 9-10)
**Goal:** Smooth daily driver experience

- [ ] Hotkey for global start/stop
- [ ] System tray mode
- [ ] Meeting templates
- [ ] Calendar integration (optional: pull meeting info from gcal)
- [ ] Audio file export (backup)

---

## Key Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Base framework** | Tauri (from Meetily) | Lightweight, Rust audio capture works |
| **Transcription** | Deepgram Streaming | You know it, fast, good diarization |
| **Local storage** | SQLite | Already in Meetily, good for search |
| **File format** | Obsidian Markdown | Direct vault integration |
| **AI summarization** | Claude API (primary) | Best quality; fallback to Ollama for offline |
| **Frontend** | Next.js + Tailwind | Already in Meetily, fast iteration |

---

## Obsidian Integration Details

### Vault Configuration
```
Settings:
  - vault_path: "E:\Obsidian\Personal Vault"
  - meetings_folder: "Meetings"
  - template_path: "Templates/Meeting.md"
```

### File Watching
- Watch `meetings_folder` for new prep files
- Auto-detect meeting files by frontmatter `type: meeting`
- Show upcoming meetings based on `date` field

### Sync Strategy
- **Read once** at meeting start (load prep)
- **Write incrementally** during meeting (tasks, timestamps, notes)
- **Final write** after AI processing (summary, transcript)
- Preserve user formatting, only modify designated sections

---

## UI Wireframe Concept

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Meeting Companion                              [_][â–¡][X]   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚ PREP            â”‚  â”‚ LIVE TRANSCRIPT                 â”‚  â”‚
â”‚  â”‚                 â”‚  â”‚                                 â”‚  â”‚
â”‚  â”‚ ## Goals        â”‚  â”‚ [00:12:34] Attendee A: So the budget  â”‚  â”‚
â”‚  â”‚ - [ ] Timeline  â”‚  â”‚ is approved for 50k, which      â”‚  â”‚
â”‚  â”‚ - [x] Scope     â”‚  â”‚ gives us room for...            â”‚  â”‚
â”‚  â”‚                 â”‚  â”‚                                 â”‚  â”‚
â”‚  â”‚ ## Agenda       â”‚  â”‚ [00:12:52] Attendee B: That's great. â”‚  â”‚
â”‚  â”‚ 1. Intro âœ“      â”‚  â”‚ I'll loop in legal tomorrow.    â”‚  â”‚
â”‚  â”‚ 2. Scope âœ“      â”‚  â”‚                                 â”‚  â”‚
â”‚  â”‚ 3. Timeline     â”‚  â”‚ [â˜… Mark] [+ Task] [+ Note]      â”‚  â”‚
â”‚  â”‚                 â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤  â”‚
â”‚  â”‚ ## Context      â”‚  â”‚ TASKS                           â”‚  â”‚
â”‚  â”‚ Previous mtg    â”‚  â”‚ - [ ] Attendee A: proposal (Fri)      â”‚  â”‚
â”‚  â”‚ agreed on...    â”‚  â”‚ - [ ] Attendee B: legal review       â”‚  â”‚
â”‚  â”‚                 â”‚  â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤  â”‚
â”‚  â”‚                 â”‚  â”‚ NOTES                           â”‚  â”‚
â”‚  â”‚                 â”‚  â”‚ Budget confirmed at 50k         â”‚  â”‚
â”‚  â”‚                 â”‚  â”‚ Q2 deadline flexible            â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  [â— Recording 00:14:23]  [â¸ Pause]  [â–  Stop & Process]     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## Next Steps

1. **Clone Meetily** and get it running locally
2. **Audit the codebase** - identify what to keep vs. remove
3. **Set up Deepgram** - create streaming connection module
4. **Build Obsidian file parser** - extract prep sections from markdown

Want me to start with any of these? I could draft the Deepgram streaming integration code or the Obsidian markdown parser.

