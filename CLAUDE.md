# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Seshlog** is a desktop application that bridges Obsidian meeting preparation with live transcription and post-meeting processing. It is a fork of Meetily, streamlined to focus on the Obsidian integration workflow.

### Core Workflow
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

### Key Differences from Meetily
| Aspect | Meetily | Seshlog |
|--------|---------|-------------------|
| Transcription | Local Whisper only | **Deepgram primary** + Whisper fallback |
| File Source | App-managed meetings | **Obsidian markdown files** |
| Backend | FastAPI server required | **No backend needed** (native SQLx) |
| Diarization | None | **Speaker identification** |
| Output | Separate transcript files | **Write back to source .md** |

### Key Technology Stack
- **Desktop App**: Tauri 2.x (Rust) + Next.js 14 + React 18
- **Audio Processing**: Rust (cpal, professional audio mixing, VAD)
- **Transcription**: Deepgram WebSocket (cloud) + Whisper.cpp (local fallback)
- **File Format**: Obsidian markdown with YAML frontmatter
- **Storage**: Native SQLite via SQLx (no HTTP backend)

## Essential Development Commands

### Desktop App Development

**Location**: `/frontend`

```bash
# Windows Development (Primary Platform)
clean_run_windows.bat       # Clean build and run
clean_build_windows.bat     # Production build

# Windows (PowerShell) with logging
$env:RUST_LOG="debug"; ./clean_run_windows.bat

# Manual Commands
pnpm install                # Install dependencies
pnpm run dev                # Next.js dev server (port 3118)
pnpm run tauri:dev          # Full Tauri development mode
pnpm run tauri:build        # Production build

# GPU-Specific Builds
pnpm run tauri:dev:cuda     # NVIDIA CUDA
pnpm run tauri:dev:vulkan   # AMD/Intel Vulkan
pnpm run tauri:dev:cpu      # CPU-only (no GPU)
```

### Service Endpoints
- **Frontend Dev**: http://localhost:3118
- **Deepgram API**: wss://api.deepgram.com/v1/listen (WebSocket)

### Environment Variables
```bash
# .env file (already configured)
DEEPGRAM_API_KEY=your_api_key_here
```

## High-Level Architecture

### Simplified Two-Tier Architecture (No Backend)

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

### UI Layout (MVP)

```
┌─────────────────────────────────────────────────────────────┐
│  Seshlog                              [_][□][X]   │
├─────────────────────────────────────────────────────────────┤
│  [Open Meeting File]  meeting-prep-2026-01-22.md           │
├───────────────────────┬─────────────────────────────────────┤
│ PREP (1/3)            │ TRANSCRIPT (2/3)                    │
│                       │                                     │
│ ## Goals              │ [00:00:12] Speaker 1: Welcome       │
│ - [ ] Align timeline  │ everyone to the kickoff...          │
│ - [ ] Assign work     │                                     │
│                       │ [00:00:34] Speaker 2: Thanks for    │
│ ## Agenda             │ having us. Let's start with...      │
│ 1. Introductions      │                                     │
│ 2. Scope review       │ [00:01:02] Speaker 1: Great idea.   │
│                       │                                     │
│ ## Context            │                                     │
│ Previous meeting...   │                                     │
└───────────────────────┴─────────────────────────────────────┤
│  [● Recording 00:01:15]  ☁️ Deepgram   [■ Stop & Save]      │
└─────────────────────────────────────────────────────────────┘
```

### Audio Processing Pipeline (Unchanged from Meetily)

The audio system is **production-ready** (12,670 lines) and requires no changes:

```
Raw Audio (Mic + System)
         ↓
┌────────────────────────────────────────────────────────────┐
│              Audio Pipeline Manager                         │
│  (frontend/src-tauri/src/audio/pipeline.rs)                │
└─────────────┬──────────────────────────┬───────────────────┘
              ↓                          ↓
    ┌─────────────────┐        ┌─────────────────────┐
    │ Recording Path  │        │ Transcription Path  │
    │ (Pre-mixed)     │        │ (VAD-filtered)      │
    └─────────────────┘        └─────────────────────┘
              ↓                          ↓
    RecordingSaver.save()      TranscriptionProvider.transcribe()
                                   (Deepgram or Whisper)
```

### Transcription Provider Architecture

The codebase has an excellent **`TranscriptionProvider` trait** that makes adding Deepgram trivial:

```rust
#[async_trait]
pub trait TranscriptionProvider: Send + Sync {
    async fn transcribe(&self, audio: Vec<f32>, language: Option<String>)
        -> Result<TranscriptResult, TranscriptionError>;
    async fn is_model_loaded(&self) -> bool;
    fn provider_name(&self) -> &'static str;
}
```

**Existing Providers** (keep as fallback):
- `WhisperProvider` - Local, GPU-accelerated
- `ParakeetProvider` - Local, ONNX-based

**New Provider** (to implement):
- `DeepgramProvider` - Cloud WebSocket streaming with diarization

## Module Structure

### New Modules to Create

```
src-tauri/src/
├── deepgram/                  # NEW: Deepgram integration
│   ├── mod.rs                # Module exports
│   ├── provider.rs           # TranscriptionProvider implementation
│   └── websocket.rs          # WebSocket connection + streaming
│
├── obsidian/                  # NEW: Obsidian file handling
│   ├── mod.rs                # Module exports
│   ├── parser.rs             # YAML frontmatter + section extraction
│   ├── writer.rs             # Merge transcript + atomic save
│   └── types.rs              # MeetingPrep, MeetingFile structs
```

### Existing Modules (No Changes Needed)

```
src-tauri/src/
├── audio/                     # ✅ Production-ready, don't modify
│   ├── devices/              # Device discovery
│   ├── capture/              # Audio streams
│   ├── pipeline.rs           # Mixing + VAD
│   ├── recording_manager.rs  # Orchestration
│   └── transcription/        # Provider abstraction
│       ├── provider.rs       # TranscriptionProvider trait
│       ├── whisper_provider.rs
│       └── parakeet_provider.rs
│
├── database/                  # ✅ Use for settings storage
│   └── repositories/
│       └── setting.rs        # API key storage
```

### Frontend Components

```
src/
├── app/
│   └── page.tsx              # MODIFY: Two-panel layout, file picker
│
├── components/
│   ├── PrepPanel/            # NEW: Display meeting prep
│   │   └── PrepPanel.tsx
│   ├── Settings/
│   │   └── TranscriptSettings.tsx  # MODIFY: Add Deepgram fields
│   └── Transcript/
│       └── VirtualizedTranscriptView.tsx  # ✅ Reuse as-is
```

## Obsidian File Format

### Input Format (Meeting Prep)
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

### Output Format (After Recording)
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

## Development Tasks

See `.project/epics/meeting-companion/` for detailed task breakdown:

| # | Task | Status | Parallel |
|---|------|--------|----------|
| 001 | Implement Deepgram WebSocket provider | Open | ✅ |
| 002 | Create Obsidian file parser | Open | ✅ |
| 003 | Create Obsidian file writer | Open | ❌ (needs 002) |
| 004 | Remove backend dependency | Open | ✅ |
| 005 | Build PrepPanel component | Open | ❌ (needs 002) |
| 006 | Implement two-panel layout | Open | ❌ (needs 005) |
| 007 | Add Deepgram settings | Open | ❌ (needs 001) |
| 008 | End-to-end testing | Open | ❌ (needs all) |

**Start with:** Tasks 001, 002, and 004 can run in parallel.

## Critical Development Patterns

### 1. Implementing Deepgram Provider

```rust
// deepgram/provider.rs
pub struct DeepgramProvider {
    api_key: String,
    websocket: Option<WebSocketConnection>,
}

impl TranscriptionProvider for DeepgramProvider {
    async fn transcribe(&self, audio: Vec<f32>, language: Option<String>)
        -> Result<TranscriptResult, TranscriptionError>
    {
        // Stream audio via WebSocket
        // Parse JSON responses with speaker labels
        // Return TranscriptResult with diarization
    }
}
```

**Deepgram WebSocket URL:**
```
wss://api.deepgram.com/v1/listen?model=nova-2&diarize=true&punctuate=true
```

### 2. Obsidian File Parsing

```rust
// obsidian/parser.rs
pub struct MeetingPrep {
    pub frontmatter: MeetingFrontmatter,
    pub title: String,
    pub goals: Vec<String>,
    pub agenda: Vec<String>,
    pub context: String,
    pub raw_content: String,  // Preserve for writing back
}

pub fn parse_meeting_file(path: &Path) -> Result<MeetingPrep> {
    // 1. Extract YAML between --- markers
    // 2. Find ## Prep section
    // 3. Extract ### Goals, ### Agenda, ### Context
}
```

### 3. Atomic File Writing

```rust
// obsidian/writer.rs
pub fn save_transcript(path: &Path, prep: &MeetingPrep, transcript: &[TranscriptEntry])
    -> Result<()>
{
    // 1. Build new content (prep + transcript)
    // 2. Write to temp file first
    // 3. Atomic rename to target path
    // 4. Update frontmatter status to "completed"
}
```

### 4. Frontend State (Simplified)

```typescript
// No more backend API calls - use Tauri commands directly
const [meetingPrep, setMeetingPrep] = useState<MeetingPrep | null>(null);
const [transcripts, setTranscripts] = useState<TranscriptEntry[]>([]);

// Load meeting file
const loadMeeting = async () => {
  const path = await open({ filters: [{ name: 'Markdown', extensions: ['md'] }] });
  const prep = await invoke<MeetingPrep>('parse_meeting_file', { path });
  setMeetingPrep(prep);
};

// Save transcript
const saveMeeting = async () => {
  await invoke('save_meeting_transcript', {
    path: meetingPrep.path,
    transcript: transcripts
  });
};
```

## Testing and Debugging

### Enable Rust Logging
```bash
# Windows (PowerShell)
$env:RUST_LOG="debug"; ./clean_run_windows.bat

# Specific module logging
$env:RUST_LOG="app_lib::deepgram=debug,app_lib::obsidian=debug"; ./clean_run_windows.bat
```

### Test Deepgram Connection
```rust
// Quick test in main or test module
let provider = DeepgramProvider::new(api_key);
let result = provider.transcribe(test_audio, Some("en".to_string())).await;
assert!(result.is_ok());
```

### Test File Parsing
Create test fixtures in `tests/fixtures/`:
```
tests/fixtures/
├── valid-meeting.md          # Full valid meeting prep
├── minimal-meeting.md        # Only frontmatter + title
├── no-frontmatter.md         # Missing YAML
└── malformed.md              # Invalid YAML
```

## Platform-Specific Notes

### Windows (Primary Platform)
- **Audio Capture**: Uses WASAPI (Windows Audio Session API)
- **GPU**: CUDA (NVIDIA) or Vulkan (AMD/Intel) via Cargo features
- **Build Tools**: Requires Visual Studio Build Tools with C++ workload
- **System Audio**: Uses WASAPI loopback for system capture

### macOS / Linux (Deferred)
These platforms are out of scope for MVP but the codebase supports them.

## Dependencies

### New Cargo Dependencies
```toml
# Cargo.toml additions
tokio-tungstenite = "0.21"  # WebSocket client for Deepgram
serde_yaml = "0.9"          # YAML frontmatter parsing
```

### Existing (Keep)
- `whisper-rs` - Local transcription fallback
- `cpal` - Audio capture
- `sqlx` - Native database (no backend needed)
- `tauri-plugin-dialog` - File picker
- `tauri-plugin-store` - Settings storage

## Important Constraints

1. **Don't Touch Audio Code**: The 12,670 lines of audio capture code is battle-tested. Use it as-is.

2. **Implement the Trait**: Deepgram just needs to implement `TranscriptionProvider`. Don't modify the trait.

3. **Atomic Writes**: Always use temp file + rename for saving to prevent data loss.

4. **Preserve Original Content**: When writing transcript back, preserve all original meeting prep content.

5. **API Key Security**: Store Deepgram API key in native SQLx database, not plain text config.

6. **Graceful Fallback**: If Deepgram fails, fall back to local Whisper automatically.

## Key Files Reference

**Core (New)**:
- `src-tauri/src/deepgram/` - Deepgram WebSocket integration (to create)
- `src-tauri/src/obsidian/` - Obsidian file I/O (to create)
- `src/components/PrepPanel/` - Meeting prep display (to create)

**Core (Modify)**:
- `src-tauri/src/audio/transcription/engine.rs` - Add Deepgram case
- `src-tauri/src/lib.rs` - Register new Tauri commands
- `src/app/page.tsx` - Two-panel layout
- `src/components/Settings/TranscriptSettings.tsx` - Deepgram API key

**Core (Reuse)**:
- `src-tauri/src/audio/` - Full audio pipeline (no changes)
- `src-tauri/src/whisper_engine/` - Local transcription fallback
- `src/components/Transcript/VirtualizedTranscriptView.tsx` - Transcript display

## Project Documentation

- **PRD**: `.project/prds/meeting-companion.md`
- **Epic**: `.project/epics/meeting-companion/epic.md`
- **Tasks**: `.project/epics/meeting-companion/001.md` through `008.md`
- **Codebase Audit**: `.project/CODEBASE_AUDIT.md`
- **Fork Plan**: `FORK_PLAN.md`



