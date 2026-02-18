---
created: 2026-01-22T20:32:59Z
last_updated: 2026-01-22T21:13:34Z
version: 1.1
author: Claude Code PM System
---

# Project Overview

## Application Summary

**Meeting Companion** is a Tauri-based desktop application that bridges Obsidian meeting preparation with live Deepgram transcription and post-meeting note saving. Fork of Meetily v0.2.0.

> **Technical implementation**: See `.project/epics/meeting-companion/epic.md`

## Architecture (MVP)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Desktop App (Tauri)                          │
│  ┌──────────────────┐  ┌─────────────────┐  ┌────────────────┐ │
│  │   Next.js UI     │  │  Rust Backend   │  │  Transcription │ │
│  │  (React/TS)      │←→│  (Audio + IPC)  │←→│   Providers    │ │
│  │                  │  │                 │  │                │ │
│  │ - PrepPanel      │  │ - Audio capture │  │ - Deepgram     │ │
│  │ - TranscriptView │  │ - Obsidian I/O  │  │ - Whisper      │ │
│  │ - Settings       │  │ - Native SQLx   │  │ - Parakeet     │ │
│  └──────────────────┘  └─────────────────┘  └────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
          ↓                      ↓                    ↓
    Obsidian Vault         SQLite DB           Deepgram API
    (.md files)            (settings)          (WebSocket)
```

## Feature Status

### MVP Features (In Development)

| Feature | Task | Status |
|---------|------|--------|
| Deepgram WebSocket provider | 001 | Open |
| Obsidian file parser | 002 | Open |
| Obsidian file writer | 003 | Open |
| Backend removal | 004 | Open |
| PrepPanel component | 005 | Open |
| Two-panel layout | 006 | Open |
| Deepgram settings | 007 | Open |
| End-to-end testing | 008 | Open |

### Existing Features (From Meetily)

| Feature | Status | Notes |
|---------|--------|-------|
| Audio capture (mic + system) | ✅ Reuse | 12,670 lines, production-ready |
| Whisper transcription | ✅ Reuse | Fallback provider |
| Parakeet transcription | ✅ Reuse | Alternative fallback |
| VirtualizedTranscriptView | ✅ Reuse | No changes needed |
| SQLx database layer | ✅ Reuse | Settings storage |

## New Modules to Create

```
src-tauri/src/
├── deepgram/              # NEW
│   ├── mod.rs
│   ├── provider.rs        # TranscriptionProvider impl
│   └── websocket.rs       # WebSocket streaming
│
├── obsidian/              # NEW
│   ├── mod.rs
│   ├── parser.rs          # YAML + section extraction
│   ├── writer.rs          # Atomic file save
│   └── types.rs           # MeetingPrep structs
```

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Windows x64 | ✅ Primary | WASAPI audio, Vulkan/CUDA GPU |
| macOS | ⏸️ Deferred | Supported by Meetily codebase |
| Linux x64 | ⏸️ Deferred | Supported by Meetily codebase |

## Integration Points

### Transcription Providers
- **Deepgram** (new): `wss://api.deepgram.com/v1/listen`
- **Whisper** (existing): Local GPU-accelerated
- **Parakeet** (existing): Local ONNX-based

### File I/O
- **Input**: Obsidian `.md` files with YAML frontmatter
- **Output**: Same file with ## Transcript appended

### Data Storage
| Data | Location |
|------|----------|
| Settings | SQLite via SQLx (native) |
| API keys | SQLx settings repository |
| Audio files | Local downloads directory |
| Transcripts | Obsidian markdown files |

## Code Changes Summary

From `.project/CODEBASE_AUDIT.md`:

| Category | Lines |
|----------|-------|
| New code | ~880 |
| Removed code | ~2,570 |
| **Net change** | -1,690 (simpler) |

## Related Documentation

- **PRD**: `.project/prds/meeting-companion.md`
- **Epic**: `.project/epics/meeting-companion/epic.md`
- **Tasks**: `.project/epics/meeting-companion/001.md` - `008.md`
- **Audit**: `.project/CODEBASE_AUDIT.md`
- **CLAUDE.md**: Updated for fork

## Update History
- 2026-01-22: Updated for Meeting Companion fork, referenced epic and tasks
