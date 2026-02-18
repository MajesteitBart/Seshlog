---
created: 2026-01-22T20:32:59Z
last_updated: 2026-01-22T20:32:59Z
version: 1.0
author: Claude Code PM System
---

# Project Style Guide

## Code Style

### Rust (Tauri Backend)

#### Naming Conventions
- **Files**: `snake_case.rs` (e.g., `recording_manager.rs`)
- **Modules**: `snake_case` (e.g., `audio/devices/`)
- **Functions**: `snake_case` (e.g., `start_recording`)
- **Types/Structs**: `PascalCase` (e.g., `RecordingState`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_BUFFER_SIZE`)

#### Error Handling
```rust
use anyhow::{Result, Context};

fn process_audio() -> Result<()> {
    let samples = read_audio()
        .context("Failed to read audio")?;
    Ok(())
}
```

#### Logging
```rust
// Use performance-aware macros in hot paths
perf_debug!("Processing chunk: {:?}", chunk_id);
perf_trace!("Buffer size: {}", buffer.len());

// Use standard log macros elsewhere
log::info!("Recording started");
log::error!("Failed to initialize: {}", error);
```

#### Async Code
```rust
// Prefer async/await over blocking
async fn capture_audio(app: AppHandle) -> Result<()> {
    let stream = create_stream().await?;
    // ...
}

// Use Arc for shared state
let state = Arc::new(RwLock::new(RecordingState::default()));
```

### TypeScript/React (Frontend)

#### Naming Conventions
- **Files**: `PascalCase.tsx` for components, `camelCase.ts` for utilities
- **Components**: `PascalCase` (e.g., `SidebarProvider`)
- **Functions/Hooks**: `camelCase` (e.g., `useSidebar`)
- **Types/Interfaces**: `PascalCase` (e.g., `MeetingData`)
- **Constants**: `SCREAMING_SNAKE_CASE` or `camelCase`

#### Component Structure
```typescript
// Functional components with hooks
export function MeetingCard({ meeting }: MeetingCardProps) {
  const { isRecording } = useSidebar();

  return (
    <div className="...">
      {/* JSX */}
    </div>
  );
}
```

#### Tauri Integration
```typescript
// Use invoke for commands
import { invoke } from '@tauri-apps/api/core';

const startRecording = async () => {
  try {
    await invoke('start_recording', { params });
  } catch (error) {
    console.error('Failed to start recording:', error);
  }
};

// Use listen for events
import { listen } from '@tauri-apps/api/event';

useEffect(() => {
  const unlisten = listen('transcript-update', (event) => {
    setTranscript(event.payload);
  });
  return () => { unlisten.then(fn => fn()); };
}, []);
```

### Python (Backend)

#### Naming Conventions
- **Files**: `snake_case.py`
- **Functions**: `snake_case`
- **Classes**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`

#### FastAPI Endpoints
```python
@app.post("/api/meetings")
async def create_meeting(request: CreateMeetingRequest) -> MeetingResponse:
    """Create a new meeting."""
    db = DatabaseManager()
    meeting = await db.create_meeting(request.title)
    return MeetingResponse.from_model(meeting)
```

## Git Conventions

### Branch Naming
- **Main branch**: `main` (production)
- **Development**: `devtest`
- **Features**: `feature/description-here`
- **Fixes**: `fix/issue-description`
- **Releases**: Tags like `v0.2.0`

### Commit Messages
```
type(scope): brief description

Longer explanation if needed.

Generated with [Claude Code](https://claude.ai/code)
via [Happy](https://happy.engineering)

Co-Authored-By: Claude <redacted@example.com>
Co-Authored-By: Happy <redacted@example.com>
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Documentation

### Code Comments
- Explain "why", not "what"
- Keep comments concise and up-to-date
- Use `///` for Rust doc comments
- Use `/** */` for TypeScript JSDoc

### Markdown Files
- Use ATX-style headers (`#`, `##`, `###`)
- Include frontmatter where appropriate
- Use fenced code blocks with language identifiers

## File Organization

### Audio Module Pattern
```
audio/
â”œâ”€â”€ mod.rs              # Public API exports
â”œâ”€â”€ pipeline.rs         # Core processing logic
â”œâ”€â”€ devices/            # Device-specific code
â”‚   â”œâ”€â”€ mod.rs
â”‚   â”œâ”€â”€ discovery.rs
â”‚   â””â”€â”€ platform/
â”‚       â”œâ”€â”€ windows.rs
â”‚       â”œâ”€â”€ macos.rs
â”‚       â””â”€â”€ linux.rs
â””â”€â”€ capture/            # Stream capture
    â”œâ”€â”€ microphone.rs
    â””â”€â”€ system.rs
```

### Component Pattern
```
components/
â”œâ”€â”€ Sidebar/
â”‚   â”œâ”€â”€ index.tsx       # Main export
â”‚   â”œâ”€â”€ SidebarProvider.tsx
â”‚   â”œâ”€â”€ SidebarItem.tsx
â”‚   â””â”€â”€ types.ts
â””â”€â”€ Meeting/
    â”œâ”€â”€ MeetingCard.tsx
    â””â”€â”€ MeetingList.tsx
```

## Platform-Specific Code

### Conditional Compilation (Rust)
```rust
#[cfg(target_os = "macos")]
mod macos_impl;

#[cfg(target_os = "windows")]
mod windows_impl;

#[cfg(target_os = "linux")]
mod linux_impl;
```

### Feature Flags
```toml
[features]
metal = ["whisper-rs/metal"]
cuda = ["whisper-rs/cuda"]
vulkan = ["whisper-rs/vulkan"]
```

## Audio Naming Convention

Use consistent terminology:
- **Microphone** (not "input")
- **System audio** (not "output" or "loopback")
- **Sample rate**: Always 48kHz
- **Channels**: Mono for transcription, stereo for recording

## Performance Guidelines

- Use `perf_debug!()` for hot-path logging (zero cost in release)
- Pre-allocate buffers in audio processing
- Batch database operations
- Use async for I/O-bound operations
- Profile before optimizing

