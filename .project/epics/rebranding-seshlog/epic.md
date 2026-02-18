---
name: rebranding-seshlog
status: in-progress
created: 2026-01-28T22:47:11Z
updated: 2026-01-29T15:49:51Z
progress: 90%
prd: .project/prds/rebranding-seshlog.md
github: [Will be updated when synced to GitHub]
---

# Epic: Rebranding to Seshlog

## Overview

Transform "Meeting Companion" (currently showing "meetily" in config) into **Seshlog** with the amber-based visual identity from the brandbook. This is a rebrand-only effort - no functional changes, no terminology changes (keep "meeting").

**Key Insight:** The codebase already uses CSS variables (HSL format) and Tailwind theming, making color/typography changes straightforward. The main work is icon generation and applying the brandbook values consistently.

## Architecture Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Font loading | Google Fonts CDN | Simpler than bundling; desktop app always online for Deepgram anyway |
| CSS variables | Keep HSL format | Already in place; convert brandbook hex to HSL |
| Icon generation | Manual + ImageMagick | One-time operation, no build pipeline needed |
| About dialog | Skip for MVP | Not blocking; can add in polish phase |

## Technical Approach

### Simplification Strategy

The PRD lists many requirements, but most collapse into a few targeted edits:

1. **Config files (3 edits)** - package.json, tauri.conf.json, Cargo.toml
2. **Theme files (2 edits)** - globals.css (variables), tailwind.config.ts (colors/fonts)
3. **Icons (1 script run)** - Generate from SVG using ImageMagick or Sharp
4. **Documentation (2 edits)** - README.md, CLAUDE.md

### Color Conversion (Hex to HSL)

Brandbook hex values converted to HSL for CSS variables:

| Token | Hex | HSL |
|-------|-----|-----|
| primary (light) | #F59E0B | 38 92% 50% |
| primary (dark) | #FBBF24 | 45 93% 56% |
| accent | #D97706 | 32 95% 44% |
| background (light) | #F9F8F6 | 40 23% 97% |
| background (dark) | #0A0A0F | 240 20% 5% |
| foreground (light) | #1A1917 | 45 10% 10% |
| foreground (dark) | #F0EFED | 40 11% 93% |
| muted (light) | #F0EFED | 40 11% 93% |
| muted (dark) | #1E1E24 | 240 12% 13% |
| muted-foreground (light) | #737069 | 42 7% 43% |
| muted-foreground (dark) | #8C8A84 | 45 5% 53% |
| border (light) | #E5E4E1 | 45 10% 89% |
| border (dark) | #2A2A32 | 240 10% 18% |
| destructive | #EF4444 | 0 84% 60% |

### Typography

Add Newsreader via Google Fonts import in globals.css:
```css
@import url('https://fonts.googleapis.com/css2?family=Newsreader:ital,opsz,wght@0,6..72,300..700;1,6..72,300..700&display=swap');
```

Update font-family in Tailwind config to use Newsreader as primary.

### Icon Generation

From `docs/seshlog_icon.svg`, generate:
- `icons/32x32.png`
- `icons/128x128.png`
- `icons/128x128@2x.png`
- `icons/icon.ico` (Windows)
- `icons/icon.icns` (macOS, for future)
- `public/favicon.ico`

Command (with ImageMagick):
```bash
convert -background none docs/seshlog_icon.svg -resize 32x32 icons/32x32.png
convert -background none docs/seshlog_icon.svg -resize 128x128 icons/128x128.png
convert -background none docs/seshlog_icon.svg -resize 256x256 icons/128x128@2x.png
convert docs/seshlog_icon.svg -define icon:auto-resize=256,128,64,48,32,16 icons/icon.ico
```

## Implementation Strategy

### Critical Path (Blocking)
1. Generate icons (needed for Tauri build)
2. Update tauri.conf.json (app name, identifier, icons)
3. Update globals.css (colors, fonts)

### Non-Blocking (Can be done in parallel)
- Update tailwind.config.ts
- Update package.json
- Update README.md
- Update CLAUDE.md

### Verification
- Build app and verify window title shows "Seshlog"
- Verify icon appears in taskbar
- Visual check that amber theme is applied

## Task Breakdown Preview

| # | Task | Files | Effort |
|---|------|-------|--------|
| 009 | Generate app icons from SVG | icons/*.png, icons/*.ico | Small |
| 010 | Update Tauri config (name, identifier, icons) | tauri.conf.json, Cargo.toml | Small |
| 011 | Apply Seshlog color palette to CSS variables | globals.css | Small |
| 012 | Add Newsreader font and update typography | globals.css, tailwind.config.ts | Small |
| 013 | Update package.json and window title references | package.json, layout.tsx | Small |
| 014 | Update documentation (README, CLAUDE.md) | README.md, CLAUDE.md | Small |
| 015 | Visual QA and final verification | - | Small |

**Total: 7 tasks** (down from 20+ requirements by grouping related changes)

## Dependencies

### Task Dependencies
```
009 (icons) ─┬─► 010 (tauri config)
             │
011 (colors) ─┴─► 015 (QA)
             │
012 (fonts) ──┤
             │
013 (pkg.json)┤
             │
014 (docs) ───┘
```

Tasks 009, 011, 012, 013, 014 can run in parallel.
Task 010 depends on 009 (needs icons).
Task 015 depends on all others.

### External Dependencies
- ImageMagick or Sharp for icon generation
- Google Fonts CDN for Newsreader font

## Success Criteria (Technical)

| Criteria | Verification |
|----------|--------------|
| Window title shows "Seshlog" | Visual check after build |
| App icon is Seshat logo | Taskbar icon check |
| Primary color is amber (#F59E0B) | DevTools computed styles |
| Font is Newsreader | DevTools computed styles |
| Dark mode uses correct palette | Toggle theme, visual check |
| Build completes without errors | `pnpm run tauri:build` succeeds |

## Estimated Effort

| Phase | Tasks | Estimate |
|-------|-------|----------|
| Icons + Config | 009, 010 | ~1 hour |
| Theme + Typography | 011, 012 | ~1 hour |
| Metadata + Docs | 013, 014 | ~30 min |
| QA + Polish | 015 | ~30 min |
| **Total** | | **~3 hours** |

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| SVG doesn't convert well at small sizes | Test 16x16 early; may need simplified version |
| HSL color conversion errors | Use online converter, verify with DevTools |
| Font loading delay | Test on fresh load; fallback to system serif |
| Tauri build breaks | Keep old config as backup; test build after each change |

## Files to Modify

### Configuration
- `frontend/src-tauri/tauri.conf.json` - productName, identifier, window title, icons
- `frontend/src-tauri/Cargo.toml` - package name
- `frontend/package.json` - name, description

### Styling
- `frontend/src/app/globals.css` - CSS variables, font import
- `frontend/tailwind.config.ts` - colors, fontFamily

### Assets
- `frontend/src-tauri/icons/*` - all icon sizes (create/replace)
- `frontend/public/favicon.ico` - web favicon

### Documentation
- `README.md` - project name, description
- `CLAUDE.md` - project overview section

## Tasks Created

### Phase 1
- [x] 009.md - Generate app icons from SVG (parallel: true)
- [x] 010.md - Update Tauri config for Seshlog identity (parallel: false, depends on 009)
- [x] 011.md - Apply Seshlog color palette to CSS variables (parallel: true)
- [x] 012.md - Add Newsreader font and update typography (parallel: true) **FIXED - layout.tsx was using Source Sans 3**
- [x] 013.md - Update package.json and app metadata (parallel: true)
- [x] 014.md - Update documentation for Seshlog rebrand (parallel: true)
- [x] 015.md - Visual QA and final verification (parallel: false, depends on all)

### Phase 2
- [x] 016.md - Replace public logo files with Seshat icon (parallel: true)
- [x] 017.md - Update component colors to use theme variables (parallel: true) **33 files updated**
- [ ] 018.md - Update version number and final visual QA (parallel: false, depends on 016, 017)

**Total tasks:** 10
**Completed:** 9
**Remaining:** 1
**Estimated remaining effort:** ~30 min
