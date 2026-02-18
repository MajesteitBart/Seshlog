---
created: 2026-01-22T20:32:59Z
last_updated: 2026-01-22T20:32:59Z
version: 1.0
author: Claude Code PM System
---

# Project Structure

## Overview

Meetily is a monorepo containing a Tauri desktop application (frontend) and a FastAPI backend server.

```
meeting-minutes/
├── frontend/              # Tauri desktop application
│   ├── src/               # Next.js React application
│   └── src-tauri/         # Rust backend for Tauri
├── backend/               # FastAPI server
├── llama-helper/          # LLM helper utilities
├── docs/                  # Documentation and images
├── scripts/               # Build and utility scripts
└── .github/               # GitHub Actions workflows
```

## Frontend (Tauri Desktop App)

### Next.js UI (`frontend/src/`)

```
frontend/src/
├── app/                   # Next.js App Router pages
│   └── page.tsx           # Main recording interface
├── components/            # React components
│   ├── Sidebar/           # Sidebar with SidebarProvider.tsx
│   └── ...                # UI components
├── config/                # Configuration files
├── contexts/              # React context providers
├── hooks/                 # Custom React hooks
├── lib/                   # Utility libraries
├── services/              # API service layers
└── types/                 # TypeScript type definitions
```

### Tauri Rust Backend (`frontend/src-tauri/src/`)

```
frontend/src-tauri/src/
├── lib.rs                 # Main Tauri entry point, command registration
├── main.rs                # Application entry point
├── state.rs               # Global application state
├── tray.rs                # System tray functionality
├── onboarding.rs          # First-run onboarding
├── utils.rs               # Shared utilities
│
├── audio/                 # Audio capture and processing
│   ├── mod.rs             # Module exports
│   ├── pipeline.rs        # Audio mixing and VAD processing
│   ├── recording_manager.rs   # High-level recording coordination
│   ├── recording_commands.rs  # Tauri command interface
│   ├── recording_saver.rs     # Audio file writing
│   ├── recording_state.rs     # Recording state management
│   ├── device_detection.rs    # Device discovery
│   ├── stream.rs              # Audio streaming
│   ├── vad.rs                 # Voice Activity Detection
│   ├── devices/               # Device management
│   │   ├── discovery.rs       # list_audio_devices
│   │   ├── microphone.rs      # Microphone handling
│   │   ├── speakers.rs        # Speaker handling
│   │   ├── configuration.rs   # AudioDevice types
│   │   └── platform/          # Platform-specific code
│   │       ├── windows.rs     # WASAPI logic
│   │       ├── macos.rs       # ScreenCaptureKit
│   │       └── linux.rs       # ALSA/PulseAudio
│   ├── capture/               # Audio stream capture
│   │   ├── microphone.rs      # Mic capture stream
│   │   ├── system.rs          # System audio capture
│   │   └── core_audio.rs      # macOS CoreAudio
│   └── transcription/         # Transcription handling
│
├── whisper_engine/        # Whisper.cpp integration
│   ├── mod.rs             # Module exports
│   ├── whisper_engine.rs  # Model management and transcription
│   ├── commands.rs        # Tauri commands
│   ├── parallel_processor.rs  # Batch processing
│   └── system_monitor.rs  # Resource monitoring
│
├── parakeet_engine/       # Parakeet ONNX transcription
│
├── api/                   # Backend API integration
├── analytics/             # Usage analytics (PostHog)
├── console_utils/         # Console logging utilities
├── database/              # Local SQLite database
├── notifications/         # System notifications
├── ollama/                # Ollama LLM integration
├── openrouter/            # OpenRouter API integration
└── summary/               # Meeting summary generation
```

## Backend (FastAPI Server)

```
backend/
├── app/
│   ├── main.py            # FastAPI application, API endpoints
│   ├── db.py              # DatabaseManager, SQLite operations
│   ├── schema_validator.py    # Request/response validation
│   └── transcript_processor.py # Transcript processing
├── requirements.txt       # Python dependencies
├── docker-compose.yml     # Docker configuration
└── whisper-server-package/    # Whisper server (optional)
```

## Key Entry Points

| Purpose | File |
|---------|------|
| Frontend Main UI | `frontend/src/app/page.tsx` |
| Tauri Commands | `frontend/src-tauri/src/lib.rs` |
| Audio Recording | `frontend/src-tauri/src/audio/recording_commands.rs` |
| Whisper Engine | `frontend/src-tauri/src/whisper_engine/whisper_engine.rs` |
| Backend API | `backend/app/main.py` |
| Database | `backend/app/db.py` |

## Configuration Files

| File | Purpose |
|------|---------|
| `frontend/package.json` | Node.js dependencies |
| `frontend/src-tauri/Cargo.toml` | Rust dependencies |
| `frontend/src-tauri/tauri.conf.json` | Tauri configuration |
| `backend/requirements.txt` | Python dependencies |
| `Cargo.toml` (root) | Workspace configuration |

## Scripts

| Platform | Development | Production |
|----------|-------------|------------|
| macOS | `clean_run.sh` | `clean_build.sh` |
| Windows | `clean_run_windows.bat` | `clean_build_windows.bat` |
| Linux | `dev-gpu.sh` | `build-gpu.sh` |

## File Naming Patterns

- **Rust modules**: `snake_case.rs`
- **React components**: `PascalCase.tsx`
- **TypeScript types**: `types/` directory with `.ts` files
- **Configuration**: `*.json`, `*.toml`, `*.md`
