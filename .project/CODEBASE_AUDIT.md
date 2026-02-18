# Codebase Audit: Meetily → Meeting Companion

**Audit Date:** 2026-01-22
**PRD Reference:** `.project/prds/meeting-companion.md`

---

## Executive Summary

The Meetily codebase is **well-suited for the Meeting Companion fork**. Key findings:

| Area | Status | Effort to Adapt |
|------|--------|-----------------|
| Audio Capture (FR2) | ✅ Production-ready | None - use as-is |
| Transcription Abstraction (FR3/FR4) | ✅ Excellent | Low - add Deepgram provider |
| File I/O (FR1/FR5) | ⚠️ Partial | Medium - add Obsidian parser/writer |
| UI Components | ✅ Good foundation | Medium - new PrepPanel + layout |
| Backend Dependency | ⚠️ Can be removed | Medium - migrate to local SQLx |

**Estimated total effort:** 8-12 days (aligns with epic estimate)

---

## FR2: Audio Capture - ✅ READY

### What Exists
- **12,670 lines** of mature, modularized audio code
- Dual-path architecture: mic + system audio capture
- Professional mixing with RMS-based ducking
- Voice Activity Detection (VAD) filtering (~70% reduction in transcription load)
- Platform support: WASAPI (Windows), ScreenCaptureKit (macOS), ALSA/PulseAudio (Linux)

### Key Files
| File | Purpose |
|------|---------|
| `audio/recording_commands.rs` | Tauri commands (start/stop/pause) |
| `audio/recording_manager.rs` | Orchestration |
| `audio/pipeline.rs` | Mixing + VAD |
| `audio/devices/` | Device discovery (platform-specific) |

### Tauri Commands Available
- `start_recording()`, `start_recording_with_devices()`
- `stop_recording()`, `pause_recording()`, `resume_recording()`
- `is_recording()`, `get_recording_state()`
- `poll_audio_device_events()` (Bluetooth reconnection)

### Changes Needed
**None.** Audio capture works perfectly as-is. Mixed audio already routed to transcription channel.

---

## FR3/FR4: Transcription - ✅ EXCELLENT ABSTRACTION

### What Exists
- **Unified `TranscriptionProvider` trait** - all providers implement this
- Whisper provider (local, GPU-accelerated)
- Parakeet provider (local, ONNX-based)
- Generic `TranscriptionEngine` enum supports trait-based providers

### Provider Trait
```rust
#[async_trait]
pub trait TranscriptionProvider: Send + Sync {
    async fn transcribe(&self, audio: Vec<f32>, language: Option<String>)
        -> Result<TranscriptResult, TranscriptionError>;
    async fn is_model_loaded(&self) -> bool;
    fn provider_name(&self) -> &'static str;
}
```

### Key Files
| File | Purpose |
|------|---------|
| `audio/transcription/provider.rs` | Trait definition |
| `audio/transcription/engine.rs` | Provider initialization |
| `audio/transcription/worker.rs` | Async transcription worker |
| `audio/transcription/whisper_provider.rs` | Whisper implementation |
| `audio/transcription/parakeet_provider.rs` | Parakeet implementation |

### Changes Needed for Deepgram

1. **Create `deepgram_provider.rs`** (~200-300 lines)
   - Implement `TranscriptionProvider` trait
   - WebSocket connection to `wss://api.deepgram.com/v1/listen`
   - Handle interim vs final results
   - Parse speaker diarization labels

2. **Update `engine.rs`** (~20 lines)
   ```rust
   match config.provider.as_str() {
       "deepgram" => {
           let api_key = config.api_key.ok_or("API key required")?;
           Ok(TranscriptionEngine::Provider(Arc::new(DeepgramProvider::new(api_key))))
       }
   }
   ```

3. **Add dependency** - `tokio-tungstenite` for WebSocket

### Fallback Strategy
Architecture already supports wrapping:
```rust
pub struct DeepgramWithFallback {
    deepgram: DeepgramProvider,
    whisper: WhisperProvider,  // Used when Deepgram fails
}
```

**Effort:** 3-4 hours for basic Deepgram, +2 hours for fallback wrapper

---

## FR1/FR5: File I/O (Obsidian) - ⚠️ NEEDS WORK

### What Exists
- Tauri `dialog` plugin (file pickers) ✅
- Tauri `store` plugin (JSON preferences) ✅
- Basic `std::fs` file operations
- SQLite database for meetings
- **No markdown parsing** - only regex-based string manipulation

### What's Missing

| Feature | Status | What's Needed |
|---------|--------|---------------|
| YAML frontmatter parsing | ❌ | Add `serde_yaml` crate |
| Markdown section extraction | ❌ | Implement heading-based parser |
| Markdown file writing | ❌ | Atomic write + merge logic |
| Obsidian vault detection | ❌ | Directory picker + `.obsidian` check |

### Implementation Plan

1. **Add dependencies:**
   ```toml
   serde_yaml = "0.9"
   ```

2. **Create `obsidian/` module** (~400 lines total):
   ```
   src-tauri/src/obsidian/
   ├── mod.rs
   ├── parser.rs      # Frontmatter + section extraction
   ├── writer.rs      # Merge transcript + atomic save
   └── vault.rs       # Vault detection + file listing
   ```

3. **Tauri commands to add:**
   - `open_meeting_file(path)` → Parse and return MeetingPrep struct
   - `save_meeting_transcript(path, transcript)` → Merge and save

**Effort:** 4-6 hours

---

## UI Components - ✅ GOOD FOUNDATION

### What Exists
- **Two-panel layout pattern** already in meeting-details page
- `VirtualizedTranscriptView` - efficient transcript rendering
- `BlockNoteEditor` - rich text editor (can use for notes)
- Settings UI with tabs (General, Recordings, Transcription, Summary)
- Device selection components
- Recording controls (start/stop/timer)

### Current Home Page Layout
```
┌─────────────────────────────────────────┐
│ Sidebar │     TranscriptPanel           │
│ (meetings│     (full width)             │
│  list)   │                              │
└─────────────────────────────────────────┘
```

### Required Layout for MVP
```
┌─────────────────────────────────────────┐
│ [Open File]  meeting-prep.md            │
├─────────────┬───────────────────────────┤
│ PrepPanel   │ TranscriptPanel           │
│ (1/3 width) │ (2/3 width)               │
│             │                           │
│ - Goals     │ - Live transcript         │
│ - Agenda    │ - Speaker labels          │
│ - Context   │ - Timestamps              │
└─────────────┴───────────────────────────┘
│ [● Recording 00:01:15]  [■ Stop & Save] │
└─────────────────────────────────────────┘
```

### Changes Needed

1. **Create `PrepPanel.tsx`** (~150 lines)
   - Display parsed Goals, Agenda, Context
   - Read-only markdown rendering
   - Collapsible sections

2. **Modify `page.tsx`** (~50 lines changed)
   - Remove sidebar meeting list
   - Add file open button
   - Two-panel flex layout

3. **Add Settings fields** (~30 lines)
   - Deepgram API key input
   - Transcription provider toggle

**Effort:** 4-6 hours

---

## Backend Dependency - ⚠️ CAN BE REMOVED

### Current Architecture
```
Frontend → HTTP to localhost:5167 → FastAPI Backend → SQLite
```

### What Backend Provides
- Meeting CRUD (list, get, delete)
- Transcript search (full-text)
- Summary generation (LLM calls)
- Model config storage

### Key Finding
**Frontend already has native SQLx database layer!** The HTTP calls are redundant.

```
frontend/src-tauri/src/database/
├── manager.rs          # SQLite connection
├── repositories/
│   ├── meeting.rs      # Meeting CRUD
│   ├── transcript.rs   # Transcript storage
│   ├── summary.rs      # Summary storage
│   └── setting.rs      # Config storage
```

### Migration Plan

1. **Remove HTTP calls** in `frontend/src-tauri/src/api/api.rs`
2. **Replace with SQLx queries** using existing repositories
3. **Delete `backend/` directory** (not needed for MVP)

### What Can Be Deferred (Out of Scope)
- Summary generation (Phase 4)
- Full-text search (nice-to-have)
- Meeting history (not needed for Obsidian workflow)

**Effort:** 2-4 hours to remove backend dependency

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Deepgram API changes | Low | Medium | Pin API version, test thoroughly |
| Windows audio quirks | Medium | Low | Already handled in Meetily |
| Large file handling | Low | Medium | Test with 2hr meetings |
| Obsidian format edge cases | Medium | Low | Support standard format only |

---

## Recommended Task Order

Based on audit findings, optimal development order:

1. **Deepgram module** - Validate API works with existing audio pipeline
2. **Remove backend dependency** - Simplify architecture early
3. **Obsidian parser** - Enable file loading for testing
4. **UI layout change** - Two-panel with PrepPanel
5. **Obsidian writer** - Complete the workflow
6. **Settings & polish** - API key, provider toggle
7. **Integration testing** - Full end-to-end validation

---

## Files to Create

| File | Purpose | Est. Lines |
|------|---------|------------|
| `src-tauri/src/deepgram/mod.rs` | Deepgram WebSocket client | 250 |
| `src-tauri/src/deepgram/provider.rs` | TranscriptionProvider impl | 150 |
| `src-tauri/src/obsidian/mod.rs` | Module exports | 20 |
| `src-tauri/src/obsidian/parser.rs` | Frontmatter + section parser | 200 |
| `src-tauri/src/obsidian/writer.rs` | Transcript merger + atomic save | 150 |
| `src/app/_components/PrepPanel.tsx` | Meeting prep display | 150 |

**Total new code:** ~920 lines

## Files to Modify

| File | Change | Est. Lines |
|------|--------|------------|
| `src-tauri/src/audio/transcription/engine.rs` | Add Deepgram case | 20 |
| `src-tauri/src/audio/transcription/mod.rs` | Export Deepgram | 5 |
| `src-tauri/src/api/api.rs` | Remove HTTP calls | -100 |
| `src-tauri/src/lib.rs` | Register new commands | 10 |
| `src-tauri/Cargo.toml` | Add dependencies | 5 |
| `src/app/page.tsx` | Two-panel layout | 50 |
| `src/components/Settings/*` | Deepgram API key field | 30 |

**Net change:** ~-80 lines (removing more than adding!)

---

## Conclusion

The Meetily codebase is an **excellent foundation** for Meeting Companion:

- ✅ Audio capture is production-ready
- ✅ Transcription abstraction makes adding Deepgram trivial
- ✅ UI components can be reused with minimal changes
- ✅ Backend can be eliminated, simplifying architecture
- ⚠️ Only significant new work is Obsidian file I/O

**Confidence level:** High - this fork is very feasible within the 8-12 day estimate.
