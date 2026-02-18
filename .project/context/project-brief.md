---
created: 2026-01-22T20:32:59Z
last_updated: 2026-01-22T20:32:59Z
version: 1.0
author: Claude Code PM System
---

# Project Brief

## What is Meetily?

Meetily is a privacy-first AI meeting assistant that runs entirely on your local machine. It captures meetings, transcribes them in real-time, and generates AI summaries—all without sending data to the cloud.

## Why Does It Exist?

### The Problem
Meeting AI tools create significant privacy and compliance risks:
- Sensitive business discussions end up on third-party servers
- Unclear data storage and retention policies
- Compliance violations (GDPR, HIPAA, ITAR)
- No control over who accesses meeting data

### The Solution
Meetily provides enterprise-grade meeting intelligence with complete data sovereignty:
- All processing happens locally on your device
- No data ever leaves your computer
- Works offline
- Fully customizable and self-hostable

## Project Scope

### In Scope
- Desktop application (macOS, Windows, Linux)
- Real-time audio capture (microphone + system audio)
- Local speech-to-text transcription
- AI-powered meeting summaries
- Meeting storage and search
- Multi-LLM provider support

### Out of Scope (Community Edition)
- Mobile applications
- Browser extensions
- Cloud-hosted deployments
- Real-time collaboration
- Video recording/analysis

## Success Criteria

### Technical Success
- [ ] Transcription accuracy comparable to cloud solutions
- [ ] Sub-second latency for real-time transcription
- [ ] GPU acceleration on all major platforms
- [ ] Stable audio capture across platforms

### User Success
- [ ] Simple one-click installation
- [ ] Works with any meeting platform
- [ ] Intuitive meeting management
- [ ] Useful AI summaries

### Project Success
- [ ] Active open-source community
- [ ] Regular feature updates
- [ ] Clear upgrade path to PRO/Enterprise

## Key Constraints

### Technical Constraints
- Must work entirely offline
- Must support GPU acceleration for reasonable performance
- Must handle long meetings (2+ hours)
- Must work with various audio configurations

### Business Constraints
- Community Edition remains free and open source
- PRO features justify commercial licensing
- Enterprise features require dedicated support

## Stakeholders

| Role | Interest |
|------|----------|
| End Users | Privacy, ease of use, accuracy |
| Contributors | Code quality, documentation, community |
| Enterprise Customers | Compliance, support, integration |
| Zackriya Solutions | Sustainable business, community growth |

## Timeline

### Completed
- v0.1.x: Initial release, unified architecture
- v0.2.0: Migration fixes, GitHub Actions, documentation

### Current
- Maintaining Community Edition stability
- PRO edition development

### Future
- Speaker identification
- Calendar integration
- Chat with meetings
- Enhanced export formats
