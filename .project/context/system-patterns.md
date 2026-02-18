---
created: 2026-01-22T20:32:59Z
last_updated: 2026-01-22T20:32:59Z
version: 1.0
author: Claude Code PM System
---

# System Patterns

## Architectural Style

Meetily follows a **three-tier architecture** with clear separation:

```
┌─────────────────────────────────────────────────────────────────┐
│                    Frontend (Tauri Desktop App)                  │
│  ┌──────────────────┐  ┌─────────────────┐  ┌────────────────┐ │
│  │   Next.js UI     │  │  Rust Backend   │  │ Whisper Engine │ │
│  │  (React/TS)      │←→│  (Audio + IPC)  │←→│  (Local STT)   │ │
│  └──────────────────┘  └─────────────────┘  └────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓ HTTP
┌─────────────────────────────────────────────────────────────────┐
│              Backend (FastAPI + SQLite + LLM)                   │
└─────────────────────────────────────────────────────────────────┘
```

## Audio Processing Pipeline

### Dual-Path Architecture

The audio system has two parallel processing paths:

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
    RecordingSaver.save()      WhisperEngine.transcribe()
```

**Key Insight**: Recording captures full mixed audio while transcription only receives VAD-filtered speech segments.

### Ring Buffer Mixing

- Mic and system audio arrive asynchronously at different rates
- Ring buffer (`VecDeque`) accumulates samples until 50ms windows align
- Professional mixing applies RMS-based ducking to prevent drowning

## Tauri Communication Patterns

### Command Pattern (Frontend → Rust)

```typescript
// Frontend invokes Rust command
await invoke('start_recording', {
  mic_device_name: "Built-in Microphone",
  system_device_name: "BlackHole 2ch",
  meeting_name: "Team Standup"
});
```

```rust
// Rust receives and processes
#[tauri::command]
async fn start_recording<R: Runtime>(
    app: AppHandle<R>,
    mic_device_name: Option<String>,
    system_device_name: Option<String>,
    meeting_name: Option<String>
) -> Result<(), String>
```

### Event Pattern (Rust → Frontend)

```rust
// Rust emits events
app.emit("transcript-update", TranscriptUpdate {
    text: "Hello world".to_string(),
    timestamp: chrono::Utc::now(),
})?;
```

```typescript
// Frontend listens
await listen<TranscriptUpdate>('transcript-update', (event) => {
  setTranscripts(prev => [...prev, event.payload]);
});
```

## Thread Safety Patterns

### Shared State with Arc

```rust
pub struct RecordingState {
    is_recording: Arc<AtomicBool>,
    audio_sender: Arc<RwLock<Option<mpsc::UnboundedSender<AudioChunk>>>>,
}
```

- `Arc<AtomicBool>` for simple flags (lock-free)
- `Arc<RwLock<T>>` for complex shared state across async tasks
- `mpsc::UnboundedSender` for async channel communication

### Async Boundaries

- Tokio runtime for async operations
- `CancellationToken` for graceful shutdown
- `async-trait` for trait abstractions with async methods

## State Management (Frontend)

### React Context Pattern

```typescript
// SidebarProvider manages global state
const SidebarContext = createContext<SidebarContextType>(...)

// Components consume via hooks
const { meetings, currentMeeting, isRecording } = useSidebar();
```

**Data Flow**:
Tauri commands → Rust state update → Emit events → Frontend listeners → Context update → Component re-render

## Error Handling

### Rust: Result + anyhow

```rust
use anyhow::{Result, Context};

fn process_audio() -> Result<()> {
    let samples = read_audio().context("Failed to read audio")?;
    Ok(())
}
```

### Frontend: Try-Catch with User Messages

```typescript
try {
    await invoke('start_recording', params);
} catch (error) {
    showToast("Failed to start recording", "error");
    console.error(error);
}
```

## Logging Patterns

### Performance-Aware Logging (Rust)

```rust
// Zero overhead in release builds
#[cfg(debug_assertions)]
macro_rules! perf_debug {
    ($($arg:tt)*) => { log::debug!($($arg)*) };
}

#[cfg(not(debug_assertions))]
macro_rules! perf_debug {
    ($($arg:tt)*) => {};
}
```

Use `perf_debug!()` and `perf_trace!()` for hot-path logging.

### Batched Metrics

```rust
// AudioMetricsBatcher aggregates metrics
// Prevents logging flood during audio processing
```

## Database Pattern (Backend)

### Async SQLite with DatabaseManager

```python
class DatabaseManager:
    async def create_meeting(self, title: str) -> Meeting:
        async with aiosqlite.connect(self.db_path) as db:
            cursor = await db.execute(...)
            await db.commit()
            return Meeting(...)
```

- All operations are async
- Connection pooling via aiosqlite
- Schema validation with Pydantic

## Module Organization

### Platform-Specific Code

```
audio/devices/platform/
├── windows.rs     # WASAPI implementation
├── macos.rs       # ScreenCaptureKit implementation
└── linux.rs       # ALSA/PulseAudio implementation
```

Uses `#[cfg(target_os = "...")]` for conditional compilation.

### Feature Flags

```toml
[features]
metal = ["whisper-rs/metal"]
cuda = ["whisper-rs/cuda"]
vulkan = ["whisper-rs/vulkan"]
```

## Buffer Management

### Pre-Allocated Buffer Pool

```rust
// buffer_pool.rs
pub struct AudioBufferPool {
    // Pre-allocated buffers to avoid allocation during audio processing
}
```

Prevents allocation overhead in the audio hot path.

## Design Decisions

1. **Local-First**: All transcription happens locally for privacy
2. **Modular Audio**: Audio system split into focused modules for maintainability
3. **GPU Acceleration**: Platform-optimal defaults with manual override options
4. **Graceful Degradation**: Falls back to CPU if GPU unavailable
5. **Async Everywhere**: Non-blocking operations throughout the stack
