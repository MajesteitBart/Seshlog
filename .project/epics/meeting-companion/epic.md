---
name: meeting-companion
status: closed
created: 2026-01-22T20:54:10Z
updated: 2026-01-28T18:10:03Z
completed: 2026-01-28T18:10:03Z
progress: 100%
prd: .project/prds/meeting-companion.md
audit: .project/CODEBASE_AUDIT.md
github: pending-sync
---

# Epic: Meeting Companion MVP

## Overview

Fork Meetily into Meeting Companion - a desktop app that integrates Obsidian meeting prep files with live Deepgram transcription. The MVP establishes the core workflow: load prep → record with diarization → save transcript back to markdown.

**Key insight from audit:** Meetily's architecture is excellent - audio capture needs zero changes, transcription has a clean provider abstraction, and the backend can be eliminated entirely.

## Architecture Decisions

| Decision | Choice | Rationale | Audit Finding |
|----------|--------|-----------|---------------|
| Audio capture | Use as-is | 12,670 lines of production-ready code | ✅ No changes needed |
| Transcription | Add Deepgram provider | `TranscriptionProvider` trait exists | ✅ ~200 lines to add |
| Local fallback | Keep Whisper/Parakeet | Already works, offline capability | ✅ No changes needed |
| File format | Obsidian markdown | Direct vault integration | ⚠️ Need parser/writer |
| Backend server | Remove entirely | Native SQLx layer already exists | ✅ Simplifies architecture |
| UI layout | Adapt existing pattern | Two-panel layout exists in meeting-details | ✅ Copy pattern |

## Codebase Leverage

### What We Get for Free (No Changes)

| Component | Lines | Status |
|-----------|-------|--------|
| Audio capture pipeline | 12,670 | ✅ Production-ready |
| Whisper transcription | ~2,000 | ✅ Works as fallback |
| Parakeet transcription | ~1,500 | ✅ Works as fallback |
| VAD filtering | ~300 | ✅ 70% reduction in API calls |
| Device selection UI | ~400 | ✅ Reuse directly |
| Recording controls | ~200 | ✅ Reuse directly |
| VirtualizedTranscriptView | ~300 | ✅ Reuse directly |

### What We Add

| Component | Est. Lines | Purpose |
|-----------|------------|---------|
| `deepgram/provider.rs` | 200 | Implement TranscriptionProvider trait |
| `deepgram/websocket.rs` | 150 | WebSocket connection management |
| `obsidian/parser.rs` | 200 | YAML frontmatter + section extraction |
| `obsidian/writer.rs` | 150 | Merge transcript + atomic save |
| `PrepPanel.tsx` | 150 | Display Goals/Agenda/Context |
| Settings fields | 30 | Deepgram API key input |
| **Total new code** | **~880** | |

### What We Remove

| Component | Lines Removed | Reason |
|-----------|---------------|--------|
| HTTP calls to backend | ~100 | Use native SQLx instead |
| Backend FastAPI server | ~3,000 | Not needed |
| LLM integration UI | ~200 | Defer to Phase 4 |
| Sidebar meeting list | ~150 | Replace with file picker |
| **Net change** | **-2,570** | Architecture gets simpler |

## Technical Approach

### Rust Backend (Tauri)

**Keep unchanged:**
- `audio/` - Full audio capture pipeline (mic + system + mixing + VAD)
- `audio/transcription/` - Provider abstraction layer
- `whisper_engine/` - Local transcription (already implements trait)
- `parakeet_engine/` - Alternative local engine
- `database/` - Native SQLx repositories (meetings, transcripts, settings)

**Add new modules:**

```
src-tauri/src/
├── deepgram/
│   ├── mod.rs           # Module exports
│   ├── provider.rs      # TranscriptionProvider implementation
│   └── websocket.rs     # WebSocket connection + streaming
└── obsidian/
    ├── mod.rs           # Module exports
    ├── parser.rs        # Frontmatter + section extraction
    └── writer.rs        # Merge transcript + atomic save
```

**Modify:**
- `audio/transcription/engine.rs` - Add Deepgram case (~20 lines)
- `api/api.rs` - Remove HTTP calls, use SQLx directly
- `lib.rs` - Register new Tauri commands

**Remove/disable:**
- All `http://localhost:5167` calls
- Backend FastAPI dependency
- Summary generation UI (comment out, keep code)

### Frontend (Next.js)

**Keep:**
- Device selection components
- Recording controls (start/stop/timer)
- `VirtualizedTranscriptView` component
- Settings tab structure

**Modify:**
- `page.tsx` - Two-panel layout (copy pattern from meeting-details)
- Settings - Add Deepgram API key field, provider toggle

**Add:**
- `PrepPanel.tsx` - Display parsed meeting prep (read-only markdown)
- File open button (use existing Tauri dialog plugin)

**Remove:**
- Sidebar meeting list
- Summary panel (defer to Phase 4)

## Task Breakdown (Revised)

Based on audit findings, consolidated to 8 tasks:

| # | Task | Scope | Effort |
|---|------|-------|--------|
| 1 | Implement Deepgram WebSocket provider | Rust: `deepgram/` module, implement trait | 4 hrs |
| 2 | Create Obsidian file parser | Rust: `obsidian/parser.rs`, frontmatter + sections | 3 hrs |
| 3 | Create Obsidian file writer | Rust: `obsidian/writer.rs`, merge + atomic save | 3 hrs |
| 4 | Remove backend dependency | Rust: Replace HTTP with SQLx queries | 2 hrs |
| 5 | Build PrepPanel component | Frontend: Display Goals/Agenda/Context | 3 hrs |
| 6 | Implement two-panel layout | Frontend: Modify page.tsx, add file picker | 3 hrs |
| 7 | Add Deepgram settings | Frontend + Rust: API key storage, provider toggle | 2 hrs |
| 8 | End-to-end testing | Full workflow: load → record → save | 4 hrs |

**Total: ~24 hours (3-4 days focused work, 6-8 days with context switching)**

## Dependencies

### New Cargo Dependencies
```toml
tokio-tungstenite = "0.21"  # WebSocket client
serde_yaml = "0.9"          # YAML frontmatter parsing
```

### Existing (Keep)
- `whisper-rs` - Local transcription
- `cpal` - Audio capture
- `sqlx` - Database (already used)
- `tauri-plugin-dialog` - File picker (already initialized)
- `tauri-plugin-store` - Settings storage (already used)

## Implementation Order

```
Week 1:
├── Day 1-2: Task 1 (Deepgram provider) - validate API works
├── Day 2-3: Task 4 (Remove backend) - simplify early
└── Day 3-4: Task 2 (Obsidian parser) - enable file loading

Week 2:
├── Day 1-2: Task 5-6 (Frontend UI) - two-panel + PrepPanel
├── Day 2-3: Task 3 (Obsidian writer) - complete workflow
├── Day 3: Task 7 (Settings) - polish
└── Day 4: Task 8 (Testing) - end-to-end validation
```

## Success Criteria (Technical)

| Criteria | Target | Validation |
|----------|--------|------------|
| Deepgram connection | < 1s to first transcript | Manual test |
| Diarization accuracy | Speaker labels present | Manual test |
| File parse time | < 100ms | Benchmark |
| File save integrity | No data loss | Unit test |
| Whisper fallback | Works when offline | Manual test |
| Memory usage | < 500MB for 1hr meeting | Profiling |

## Risk Mitigation

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Deepgram API issues | Low | Test early, keep Whisper fallback |
| Obsidian format edge cases | Medium | Support standard format only for MVP |
| Audio routing changes | Low | Audit confirms no changes needed |
| Backend removal breaks something | Medium | Test incrementally |

## Tasks Created

- [x] 001.md - Implement Deepgram WebSocket provider (parallel: true) ✅ COMPLETE
- [x] 002.md - Create Obsidian file parser (parallel: true) ✅ Already implemented
- [ ] 003.md - Create Obsidian file writer (parallel: false, depends: 002) 🔄 REOPENED - Phase 1: save to recording folder
- [x] 004.md - Remove backend dependency (parallel: true) ✅ COMPLETE
- [x] 005.md - Build PrepPanel component (parallel: false, depends: 002) ✅ COMPLETE
- [x] 006.md - Implement two-panel layout (parallel: false, depends: 005) ✅ REVERTED - sidebar needed
- [x] 007.md - Add Deepgram settings (parallel: false, depends: 001) ✅ Done as part of 001
- [ ] 008.md - End-to-end testing (parallel: false, depends: all)

**Summary:**
- Total tasks: 8
- Parallel tasks: 3 (001, 002, 004 can start immediately)
- Sequential tasks: 5
- Estimated total effort: 24 hours (3-4 days focused, 6-8 days with context switching)

## Notes

- Audio capture is battle-tested (12k+ lines) - don't touch it
- `TranscriptionProvider` trait is the key abstraction - Deepgram just implements it
- Backend removal actually simplifies the codebase
- Keep all existing code commented rather than deleted
- Maintain ability to build/run Whisper-only mode for offline use
