---
name: rebranding-seshlog
description: Rebrand Meeting Companion to Seshlog with full visual refresh and release preparation
status: draft
created: 2026-01-28T22:44:05Z
---

# PRD: Rebranding to Seshlog

## Executive Summary

Rebrand "Meeting Companion" to **Seshlog** - a name that evokes "session log" while paying homage to Seshat, the Egyptian goddess of writing and wisdom. This rebrand transforms the app's visual identity with an amber-based color scheme, Newsreader typography, and a distinctive hieroglyph-inspired logo, while preparing all assets for public release.

**Key Deliverables:**
- Complete visual refresh aligned with the Seshlog brandbook
- Updated app name, icons, and metadata across all platforms
- Installer and release branding (splash screens, about dialogs)
- Documentation updates (README, CLAUDE.md, help content)

**Note:** Meeting terminology is retained in the UI - only the app identity changes.

## Problem Statement

### The Problem
The current "Meeting Companion" branding is:
- **Generic** - Doesn't stand out in a crowded market of meeting tools
- **Forgettable** - No distinctive visual identity or personality
- **Inconsistent** - Mix of inherited Meetily styling and ad-hoc additions
- **Not release-ready** - Missing proper icons, installer branding, about dialogs

### Why Now
- Core functionality is stable (Deepgram + Obsidian integration complete)
- Brandbook has been created with comprehensive design system
- Need cohesive identity before any public release or sharing
- Time to establish distinctive market positioning

## User Stories

### Primary Persona: Current User Preparing for Broader Use

**User Story 1: Recognizable App Identity**
> As a user, I want the app to have a distinctive name and visual identity so I can easily identify it among my other applications.

Acceptance Criteria:
- [ ] App displays "Seshlog" in window title, taskbar, and about dialog
- [ ] App icon is the Seshat-inspired logo at all required sizes
- [ ] Splash screen shows Seshlog branding on launch
- [ ] Color scheme uses amber primary (#F59E0B) consistently

**User Story 2: Consistent Visual Experience**
> As a user, I want the UI to have a cohesive visual language so the app feels polished and professional.

Acceptance Criteria:
- [ ] Typography uses Newsreader for all text
- [ ] Buttons, badges, and UI elements follow brandbook styling
- [ ] Recording button states (idle/recording/processing/complete) use defined colors
- [ ] Light and dark modes both follow the brand palette

**User Story 3: Professional Installation Experience**
> As a user, I want the installation and first-run experience to reflect quality so I trust the app with my meeting recordings.

Acceptance Criteria:
- [ ] Windows installer shows Seshlog branding and icon
- [ ] Uninstaller displays correct app name and icon
- [ ] Start menu entry uses proper name and icon
- [ ] About dialog shows version, copyright, and credits

**User Story 4: Updated Documentation**
> As a user or contributor, I want documentation to reflect the new identity so there's no confusion about the app's name or purpose.

Acceptance Criteria:
- [ ] README.md introduces "Seshlog" with updated screenshots
- [ ] CLAUDE.md references Seshlog (not Meeting Companion)
- [ ] In-app help/settings use Seshlog branding
- [ ] Code comments updated where referencing the app name

## Requirements

### Functional Requirements

#### FR1: Application Identity
| ID | Requirement | Priority |
|----|-------------|----------|
| FR1.1 | Update window title to "Seshlog" | Must |
| FR1.2 | Update app name in package.json to "seshlog" | Must |
| FR1.3 | Update Tauri app identifier to "com.seshlog.app" | Must |
| FR1.4 | Update Tauri product name to "Seshlog" | Must |
| FR1.5 | Create About dialog with version/copyright info | Should |

#### FR2: Visual Assets
| ID | Requirement | Priority |
|----|-------------|----------|
| FR2.1 | Generate app icons from SVG at all required sizes (16, 32, 48, 64, 128, 256, 512, 1024) | Must |
| FR2.2 | Create Windows .ico file with all icon sizes | Must |
| FR2.3 | Create favicon.ico for web views | Must |
| FR2.4 | Create splash screen image (optional, for loading state) | Should |
| FR2.5 | Create installer banner/sidebar images for NSIS | Should |

#### FR3: Color Scheme
| ID | Requirement | Priority |
|----|-------------|----------|
| FR3.1 | Update CSS variables to amber palette (light mode) | Must |
| FR3.2 | Update CSS variables to amber palette (dark mode) | Must |
| FR3.3 | Update Tailwind config with brand colors | Must |
| FR3.4 | Apply status colors (recording: red, processing: purple, success: green) | Must |

#### FR4: Typography
| ID | Requirement | Priority |
|----|-------------|----------|
| FR4.1 | Add Newsreader font to project (Google Fonts or local) | Must |
| FR4.2 | Update font-family declarations to use Newsreader | Must |
| FR4.3 | Apply typography scale from brandbook | Should |
| FR4.4 | Style transcript text with italic speaker names | Should |

#### FR5: UI Components
| ID | Requirement | Priority |
|----|-------------|----------|
| FR5.1 | Update button styles to match brandbook | Must |
| FR5.2 | Update recording button states (idle/recording/processing/complete) | Must |
| FR5.3 | Update status badges to use brand colors | Must |
| FR5.4 | Update sidebar navigation styling | Should |
| FR5.5 | Update cards and form elements | Should |

#### FR6: Documentation
| ID | Requirement | Priority |
|----|-------------|----------|
| FR6.1 | Update README.md with new name and description | Must |
| FR6.2 | Update CLAUDE.md project overview section | Must |
| FR6.3 | Update any in-app help text | Should |
| FR6.4 | Update license file with correct app name | Should |

### Non-Functional Requirements

#### NFR1: Asset Quality
| ID | Requirement | Target |
|----|-------------|--------|
| NFR1.1 | Icons render crisply at all sizes | No visible aliasing |
| NFR1.2 | Logo maintains legibility at 16x16 | Recognizable silhouette |
| NFR1.3 | Colors meet WCAG contrast requirements | AA minimum |

#### NFR2: Performance
| ID | Requirement | Target |
|----|-------------|--------|
| NFR2.1 | Font loading doesn't delay app startup | < 100ms added |
| NFR2.2 | No layout shift from font swap | FOUT acceptable |

#### NFR3: Consistency
| ID | Requirement | Target |
|----|-------------|--------|
| NFR3.1 | All text uses Newsreader (no fallbacks visible) | 100% coverage |
| NFR3.2 | All primary buttons use amber styling | 100% coverage |
| NFR3.3 | Dark mode maintains brand identity | Colors from brandbook |

## Success Criteria

### Launch Criteria
- [ ] App displays "Seshlog" everywhere the name appears
- [ ] App icon is the Seshat logo at all sizes
- [ ] UI uses amber color scheme in both light and dark modes
- [ ] Typography uses Newsreader throughout
- [ ] Windows installer shows proper branding
- [ ] README and CLAUDE.md updated

### Quality Metrics
| Metric | Target |
|--------|--------|
| Brandbook compliance (color values) | 100% |
| Brandbook compliance (typography) | 100% |
| Icon sizes generated | All 8 required sizes |
| Documentation pages updated | All 4 key files |

## Constraints & Assumptions

### Constraints
- **Existing code patterns** - Must work within Tauri + Next.js architecture
- **Windows-first** - Icon formats and installer branding Windows-focused
- **No functional changes** - Rebrand only, no new features in scope
- **Meeting terminology kept** - Only app name changes, not internal terminology

### Assumptions
- Newsreader font available via Google Fonts CDN or bundled
- SVG logo can be converted to all required icon sizes with good quality
- Existing CSS architecture supports theming (CSS variables or Tailwind)
- No legal issues with "Seshlog" name

## Out of Scope

| Item | Reason | Future Phase |
|------|--------|--------------|
| macOS/Linux app icons | Windows-first MVP | Post-initial release |
| Animated logo/loading | Polish feature | v1.1 |
| Marketing website | Separate project | TBD |
| App Store listings | Not applicable for desktop | N/A |
| Video demo/screenshots | Marketing asset | Pre-release |
| Rename internal code (variables, functions) | Too invasive, low value | Never |

## Dependencies

### External Dependencies
| Dependency | Type | Risk |
|------------|------|------|
| Google Fonts (Newsreader) | CDN | Low - can bundle locally |
| ImageMagick/Sharp | Build tool for icons | Low - common tools |
| NSIS (Windows installer) | Build system | Low - already used by Tauri |

### Internal Dependencies
| Dependency | Description |
|------------|-------------|
| Brandbook (docs/seshlog-brandbook.html) | Source of truth for design |
| Logo SVG (docs/seshlog_icon.svg) | Source for icon generation |
| Existing theme system | Foundation for color updates |

## Technical Approach

### Icon Generation Pipeline
```bash
# From docs/seshlog_icon.svg, generate:
# - icons/16x16.png
# - icons/32x32.png
# - icons/icon.ico (Windows, multi-resolution)
# - public/favicon.ico
# Using Sharp or ImageMagick
```

### CSS Variable Updates
```css
/* globals.css updates */
:root {
  --primary: #F59E0B;
  --primary-foreground: #1A1917;
  --accent: #D97706;
  --background: #F9F8F6;
  /* ... full palette from brandbook */
}

.dark {
  --primary: #FBBF24;
  --background: #0A0A0F;
  /* ... dark mode palette */
}
```

### Font Integration
```css
/* Option 1: Google Fonts */
@import url('https://fonts.googleapis.com/css2?family=Newsreader:ital,wght@0,300..700;1,300..700&display=swap');

/* Option 2: Local bundle */
@font-face {
  font-family: 'Newsreader';
  src: url('/fonts/Newsreader-Variable.woff2') format('woff2');
}
```

### Tauri Config Updates
```json
// tauri.conf.json
{
  "productName": "Seshlog",
  "identifier": "com.seshlog.app",
  "bundle": {
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/icon.ico"],
    "windows": {
      "nsis": {
        "displayName": "Seshlog",
        "installerIcon": "icons/icon.ico"
      }
    }
  }
}
```

## Files to Modify

### Configuration Files
- `frontend/package.json` - name, description
- `frontend/src-tauri/tauri.conf.json` - productName, identifier, icons, NSIS config
- `frontend/src-tauri/Cargo.toml` - package name (if applicable)

### Styling Files
- `frontend/src/app/globals.css` - CSS variables, font imports
- `frontend/tailwind.config.ts` - theme colors
- Component-specific CSS as needed

### Documentation
- `README.md` - project overview
- `CLAUDE.md` - developer documentation
- Any other markdown files with app name

### Assets (New)
- `frontend/src-tauri/icons/` - all icon sizes
- `frontend/public/favicon.ico`
- `frontend/public/fonts/` - if bundling Newsreader locally

## Implementation Phases

### Phase 1: Core Identity (Critical Path)
1. Generate icons from SVG
2. Update Tauri config (name, identifier, icons)
3. Update package.json
4. Verify app launches with new name/icon

### Phase 2: Visual Refresh
1. Add Newsreader font
2. Update CSS variables to amber palette
3. Update Tailwind config
4. Apply typography throughout

### Phase 3: Component Updates
1. Update button styles
2. Update recording button states
3. Update badges and status indicators
4. Update sidebar and cards

### Phase 4: Polish & Documentation
1. Create/update About dialog
2. Update README.md
3. Update CLAUDE.md
4. Final visual QA pass

## Appendix: Brand Reference

### Color Palette (from brandbook)

**Light Mode:**
| Token | Value | Usage |
|-------|-------|-------|
| --primary | #F59E0B | Buttons, links, accents |
| --primary-foreground | #1A1917 | Text on primary |
| --accent | #D97706 | Hover states |
| --background | #F9F8F6 | Page background |
| --foreground | #1A1917 | Primary text |
| --muted | #F0EFED | Secondary backgrounds |
| --muted-foreground | #737069 | Secondary text |
| --border | #E5E4E1 | Borders, dividers |

**Dark Mode:**
| Token | Value | Usage |
|-------|-------|-------|
| --primary | #FBBF24 | Brighter amber for dark bg |
| --background | #0A0A0F | Deep dark background |
| --foreground | #F0EFED | Light text |
| --muted | #1E1E24 | Card backgrounds |
| --border | #2A2A32 | Subtle borders |

**Status Colors:**
| Status | Value |
|--------|-------|
| Recording | #EF4444 |
| Processing | #8B5CF6 |
| Success | #10B981 |

### Typography (from brandbook)

**Font Family:** Newsreader (Google Fonts)
**Weights:** 300 (Light), 400 (Regular), 500 (Medium), 600 (SemiBold), 700 (Bold)
**Italics:** Available for all weights

**Type Scale:**
| Style | Size | Weight | Usage |
|-------|------|--------|-------|
| Display | 2.5rem | 700 | Hero text |
| Heading 1 | 1.75rem | 600 | Page titles |
| Heading 2 | 1.25rem | 600 | Section titles |
| Body | 1.0625rem | 400 | Transcript text |
| UI Body | 1rem | 400 | Interface text |
| Small | 0.875rem | 400 | Secondary info |
| Caption | 0.75rem | 500 | Timestamps |
| Speaker | 0.9375rem | 500 italic | Speaker names |

### Logo

Source: `docs/seshlog_icon.svg`
- Seshat-inspired hieroglyph design
- Works at all sizes from 16px to 1024px
- Dark (#0A0A0F) on light backgrounds
- Inverted for dark mode if needed
