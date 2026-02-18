<div align="center">
    <h1>
        <img src="docs/seshlog_icon.svg" width="80" />
        <br>
        Seshlog
    </h1>
    <p>
        <strong>Meeting transcription with Obsidian integration</strong>
    </p>
    <p>
        Record. Transcribe. Summarize. All on your device.
    </p>
</div>

---

## Overview

Seshlog is a desktop application that bridges Obsidian meeting preparation with live transcription and post-meeting processing. It is a fork of [Meetily](https://github.com/Zackriya-Solutions/meeting-minutes), streamlined to focus on the Obsidian integration workflow.

### Core Workflow

```
PREPARE (Obsidian) -> EXECUTE (Live) -> PROCESS (Save)
Load .md file          Record meeting      Save transcript back to .md
Parse prep             Deepgram STT        Update status
Show goals/context     Speaker labels      Keep prep sections intact
```

## Why This Fork Exists

- Obsidian-first meeting workflow: open existing prep notes and write outcomes back to the same file.
- Deepgram support for higher quality real-time transcription with speaker labeling.
- Streamlined branding and UX around quick meeting capture and processing.

## Main Changes vs Meetily

- Added Deepgram transcription provider and settings.
- Added Obsidian markdown parsing/writing flow for meeting prep + transcript output.
- Updated branding from Meetily to Seshlog.
- Kept local-first architecture and privacy-oriented workflow from upstream.

## Upstream Credit

This project is a fork of [Meetily](https://github.com/Zackriya-Solutions/meeting-minutes) by Zackriya Solutions.  
Their architecture and open-source foundation made this fork possible.

## Features

- **Privacy-First**: All processing happens locally on your device
- **Obsidian Integration**: Load meeting prep from markdown files, save transcripts back
- **Deepgram Transcription**: Cloud-based speech-to-text with speaker diarization
- **Local Fallback**: Whisper.cpp for offline transcription
- **Real-time Display**: Live transcript view during meetings

## Installation

### Windows

```bash
cd frontend
pnpm install
pnpm run tauri:build
```

The installer will be generated in `frontend/src-tauri/target/release/bundle/`.

### Development

```bash
cd frontend
pnpm install
pnpm run tauri:dev
```

## Configuration

1. **Deepgram API Key**: Set your API key in Settings to enable cloud transcription
2. **Obsidian Vault**: Point to your Obsidian vault to load meeting prep files
3. **Optional Analytics Key**: Set `SESHLOG_POSTHOG_API_KEY` in your environment to enable analytics

## Obsidian File Format

### Meeting Prep (Input)

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

### After Recording (Output)

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

## Tech Stack

- **Desktop**: Tauri 2.x (Rust) + Next.js 14 + React 18
- **Audio**: Rust (cpal, professional audio mixing, VAD)
- **Transcription**: Deepgram WebSocket + Whisper.cpp fallback
- **Storage**: Native SQLite via SQLx

## Acknowledgments

Seshlog is built on the excellent foundation of [Meetily](https://github.com/Zackriya-Solutions/meeting-minutes) by Zackriya Solutions. We're grateful for their open-source work on privacy-first meeting transcription.

## License

MIT License

