---
created: 2026-01-22T20:32:59Z
last_updated: 2026-01-22T21:13:34Z
version: 1.1
author: Claude Code PM System
---

# Technology Context

## Core Technology Stack

### Desktop Application (Tauri)

| Layer | Technology | Version |
|-------|------------|---------|
| Framework | Tauri | 2.6.2 |
| Backend | Rust | 1.77+ (2021 edition) |
| Frontend | Next.js | 14.x |
| UI Library | React | 18.x |
| Language | TypeScript | 5.7.x |

### Backend Server (Removed in Fork)

> **Note**: The FastAPI backend is being removed in Meeting Companion. Settings will use native SQLx instead.

## Transcription Providers

### Deepgram (NEW - Primary)

| Aspect | Details |
|--------|---------|
| API | WebSocket streaming |
| Endpoint | `wss://api.deepgram.com/v1/listen` |
| Model | Nova-2 |
| Features | Diarization, punctuation, interim results |
| Auth | API key in Authorization header |

**New Dependencies:**
```toml
tokio-tungstenite = "0.21"  # WebSocket client
```

### Whisper.cpp (Fallback)

| Library | Version | Purpose |
|---------|---------|---------|
| whisper-rs | 0.13.2 | Whisper.cpp bindings |

### Parakeet (Alternative Fallback)

| Library | Version | Purpose |
|---------|---------|---------|
| ort | 2.0 | ONNX Runtime for Parakeet |

## Audio Processing (Unchanged)

### Rust Audio Libraries

| Library | Version | Purpose |
|---------|---------|---------|
| cpal | 0.15.3 | Cross-platform audio capture |
| silero_rs | git rev | Voice Activity Detection (Silero VAD) |
| ebur128 | 0.1 | EBU R128 loudness normalization |
| nnnoiseless | 0.5 | RNNoise neural network noise reduction |
| rubato | 0.15.0 | Audio resampling |
| ringbuf | 0.4.8 | Ring buffer for audio streaming |

## File Format Dependencies (NEW)

### Obsidian Markdown Parsing

**New Dependencies:**
```toml
serde_yaml = "0.9"  # YAML frontmatter parsing
```

**Parsing Requirements:**
- YAML frontmatter between `---` markers
- Section extraction (## Prep, ### Goals, etc.)
- Preserve original formatting on write-back

## Frontend Dependencies

### Core UI

| Package | Version | Purpose |
|---------|---------|---------|
| @tauri-apps/api | 2.6.0 | Tauri frontend API |
| @tauri-apps/plugin-fs | 2.4.0 | File system access |
| @tauri-apps/plugin-store | 2.4.0 | Persistent storage |
| @tauri-apps/plugin-dialog | - | File picker (for Obsidian files) |

### UI Components

| Package | Version | Purpose |
|---------|---------|---------|
| @radix-ui/* | Various | Headless UI primitives |
| lucide-react | 0.469.0 | Icons |
| tailwindcss | 3.4.1 | CSS framework |

## GPU Acceleration

### Windows (Primary Platform)

| Backend | Use Case |
|---------|----------|
| Vulkan | Default (AMD/Intel) |
| CUDA | Optional (NVIDIA) |

### macOS / Linux (Deferred)

Supported by existing Meetily codebase but not primary target for MVP.

## Storage

### SQLx (Native - No HTTP)

| Repository | Purpose |
|------------|---------|
| `setting.rs` | API key storage, preferences |

**Key Change**: Bypass `localhost:5167` HTTP calls, use SQLx directly.

## Build Configuration

### Cargo Features

```toml
[features]
default = ["platform-default"]
metal = ["whisper-rs/metal"]       # macOS
coreml = ["whisper-rs/coreml"]     # macOS
cuda = ["whisper-rs/cuda"]         # NVIDIA
vulkan = ["whisper-rs/vulkan"]     # AMD/Intel (default on Windows)
```

### New Cargo Dependencies Summary

```toml
# Add to Cargo.toml
tokio-tungstenite = "0.21"  # Deepgram WebSocket
serde_yaml = "0.9"          # Obsidian YAML parsing
```

## Development Tools

| Tool | Purpose |
|------|---------|
| pnpm | Node.js package manager |
| cargo | Rust package manager |
| tauri-cli | Tauri development CLI |

## API Endpoints

### Removed
- `localhost:5167` - FastAPI backend (replaced by SQLx)
- `localhost:8178` - Whisper server (use local engine)

### Added
- `wss://api.deepgram.com/v1/listen` - Deepgram streaming

### Kept
- `localhost:3118` - Next.js dev server

## Environment Variables

```bash
# .env
DEEPGRAM_API_KEY=your_api_key_here
```

## Update History
- 2026-01-22: Updated for Meeting Companion fork - added Deepgram, noted backend removal
