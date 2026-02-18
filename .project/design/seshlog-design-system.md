# Seshlog Design System

> Design direction proposal for the Seshlog rebrand. This document establishes the visual language, component patterns, and implementation guidelines for a cohesive dark-theme UI.

**Created:** 2026-01-29
**Status:** Proposal
**Based on:** Analysis of current UI screenshots and inspiration references

---

## Table of Contents

1. [Brand Concept](#brand-concept)
2. [Design Critique Summary](#design-critique-summary)
3. [Color System](#color-system)
4. [Typography System](#typography-system)
5. [Spacing & Layout](#spacing--layout)
6. [Layout Architecture](#layout-architecture)
7. [Component Specifications](#component-specifications)
8. [Empty States](#empty-states)
9. [Implementation Priority](#implementation-priority)

---

## Brand Concept

### Name Etymology

**Seshlog** = Session + Log

The app captures moments in time and creates lasting records. The brand should feel:

| Attribute | Description |
|-----------|-------------|
| **Professional** | This is a work tool, not a toy |
| **Calm** | Meetings are stressful enough; the app shouldn't add to that |
| **Trustworthy** | Your data stays private, the UI should reinforce that |
| **Modern** | Comparable to tools like Linear, Notion, Raycast |

### Brand Personality

- Confident but not arrogant
- Minimal but not sparse
- Dark but not gloomy
- Intelligent but not complex

---

## Design Critique Summary

### Critical Issues Identified

| Priority | Issue | Description |
|----------|-------|-------------|
| **Critical** | Brand identity crisis | Meetily vs Seshlog inconsistency throughout |
| **Critical** | No cohesive color system | Yellow/orange doesn't convey professionalism |
| **High** | Empty states don't guide users | Minimal effort, no emotional connection |
| **High** | Summary output is visually weak | Looks like a homework assignment |
| **High** | Redundant recording controls | Floating button AND sidebar button |
| **Medium** | Typography lacks hierarchy | Flat, uninspiring type treatment |
| **Medium** | Button styles inconsistent | No clear system for filled vs outlined |
| **Medium** | Dark/light mode confusion | Dark sidebar with bright white content |
| **Medium** | Transcript format is noisy | Redundant speaker labels, poor grouping |
| **Low** | Accessibility gaps | Color contrast, touch targets, focus states |
| **Low** | Settings organization | Doesn't match user mental models |

### Core Problem

The UI was built feature-by-feature without a design system. Each component works in isolation, but nothing feels cohesive. The app doesn't have a personality.

---

## Color System

### Design Tokens (CSS Custom Properties)

```css
:root {
  /* Background Colors - Dark Theme */
  --bg-base: #0D0D0F;        /* App background, deepest layer */
  --bg-surface: #16161A;      /* Cards, panels, elevated surfaces */
  --bg-elevated: #1E1E24;     /* Hover states, nested cards */
  --bg-overlay: #26262E;      /* Modals, dropdowns, popovers */

  /* Accent Colors */
  --accent-primary: #8B5CF6;           /* Primary actions, active states, brand */
  --accent-primary-hover: #7C3AED;     /* Hover on primary elements */
  --accent-primary-muted: rgba(139, 92, 246, 0.12);  /* Backgrounds, subtle highlights */
  --accent-secondary: #06B6D4;         /* Links, secondary actions, live indicators */
  --accent-recording: #EF4444;         /* Recording state, destructive actions */
  --accent-success: #10B981;           /* Completed states, confirmations */
  --accent-warning: #F59E0B;           /* Warnings, attention needed */

  /* Text Colors */
  --text-primary: #F4F4F5;    /* Headlines, primary content */
  --text-secondary: #A1A1AA;  /* Body text, descriptions */
  --text-tertiary: #71717A;   /* Captions, timestamps, placeholders */
  --text-disabled: #52525B;   /* Disabled states */

  /* Border Colors */
  --border-subtle: #27272A;   /* Card borders, dividers */
  --border-default: #3F3F46;  /* Input borders, separators */
  --border-focus: #8B5CF6;    /* Focus rings */

  /* Semantic Colors */
  --color-error: #EF4444;
  --color-error-muted: rgba(239, 68, 68, 0.12);
  --color-success: #10B981;
  --color-success-muted: rgba(16, 185, 129, 0.12);
  --color-warning: #F59E0B;
  --color-warning-muted: rgba(245, 158, 11, 0.12);
  --color-info: #06B6D4;
  --color-info-muted: rgba(6, 182, 212, 0.12);
}
```

### Color Usage Guidelines

| Element | Token | Notes |
|---------|-------|-------|
| App background | `--bg-base` | Darkest, used for main window |
| Cards & panels | `--bg-surface` | Slight elevation from base |
| Hover states | `--bg-elevated` | Interactive feedback |
| Modals | `--bg-overlay` | Highest elevation |
| Primary buttons | `--accent-primary` | Main CTAs only |
| Links | `--accent-secondary` | Clickable text |
| Recording indicator | `--accent-recording` | Pulsing red dot |
| Success messages | `--accent-success` | Confirmations, completed |
| Body text | `--text-secondary` | Most readable text |
| Headlines | `--text-primary` | High contrast for titles |

### Accent Color by Section Type

| Section | Left Border Color | Token |
|---------|-------------------|-------|
| Agenda | Purple | `--accent-primary` |
| Notes | Gray | `--text-tertiary` |
| Action Items | Amber | `--accent-warning` |
| Goals | Green | `--accent-success` |
| Decisions | Cyan | `--accent-secondary` |

---

## Typography System

### Font Stack

```css
:root {
  --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
}
```

### Type Scale

| Token | Size | Weight | Line Height | Usage |
|-------|------|--------|-------------|-------|
| `--text-xs` | 11px | 400 | 1.4 | Badges, timestamps |
| `--text-sm` | 13px | 400 | 1.5 | Captions, labels |
| `--text-base` | 14px | 400 | 1.6 | Body text |
| `--text-md` | 15px | 500 | 1.5 | Emphasized body |
| `--text-lg` | 18px | 600 | 1.4 | Section headers |
| `--text-xl` | 24px | 600 | 1.3 | Page titles |
| `--text-2xl` | 32px | 700 | 1.2 | Hero text |

### CSS Implementation

```css
:root {
  /* Font Sizes */
  --text-xs: 0.6875rem;    /* 11px */
  --text-sm: 0.8125rem;    /* 13px */
  --text-base: 0.875rem;   /* 14px */
  --text-md: 0.9375rem;    /* 15px */
  --text-lg: 1.125rem;     /* 18px */
  --text-xl: 1.5rem;       /* 24px */
  --text-2xl: 2rem;        /* 32px */

  /* Font Weights */
  --font-normal: 400;
  --font-medium: 500;
  --font-semibold: 600;
  --font-bold: 700;

  /* Line Heights */
  --leading-tight: 1.2;
  --leading-snug: 1.3;
  --leading-normal: 1.4;
  --leading-relaxed: 1.5;
  --leading-loose: 1.6;
}
```

### Typography Patterns

#### Page Title
```css
.page-title {
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  line-height: var(--leading-snug);
  color: var(--text-primary);
}
```

#### Section Header
```css
.section-header {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  line-height: var(--leading-normal);
  color: var(--text-primary);
  letter-spacing: -0.01em;
}
```

#### Body Text
```css
.body-text {
  font-size: var(--text-base);
  font-weight: var(--font-normal);
  line-height: var(--leading-loose);
  color: var(--text-secondary);
}
```

#### Transcript Timestamp
```css
.timestamp {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--font-normal);
  color: var(--text-tertiary);
  font-feature-settings: 'tnum';  /* Tabular numbers for alignment */
}
```

#### Speaker Label
```css
.speaker-label {
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--accent-primary);
}
```

---

## Spacing & Layout

### Spacing Scale

```css
:root {
  --space-0: 0;
  --space-1: 0.25rem;   /* 4px */
  --space-2: 0.5rem;    /* 8px */
  --space-3: 0.75rem;   /* 12px */
  --space-4: 1rem;      /* 16px */
  --space-5: 1.25rem;   /* 20px */
  --space-6: 1.5rem;    /* 24px */
  --space-8: 2rem;      /* 32px */
  --space-10: 2.5rem;   /* 40px */
  --space-12: 3rem;     /* 48px */
  --space-16: 4rem;     /* 64px */
}
```

### Border Radius

```css
:root {
  --radius-none: 0;
  --radius-sm: 0.25rem;    /* 4px - Small buttons, badges */
  --radius-md: 0.5rem;     /* 8px - Buttons, inputs */
  --radius-lg: 0.75rem;    /* 12px - Cards, panels */
  --radius-xl: 1rem;       /* 16px - Modals, large containers */
  --radius-2xl: 1.5rem;    /* 24px - Large feature cards */
  --radius-full: 9999px;   /* Pills, avatars, toggles */
}
```

### Shadow System

```css
:root {
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.5);
  --shadow-xl: 0 20px 25px rgba(0, 0, 0, 0.6);

  /* Glow effects for accent elements */
  --glow-primary: 0 0 20px rgba(139, 92, 246, 0.3);
  --glow-recording: 0 0 20px rgba(239, 68, 68, 0.4);
}
```

### Z-Index Scale

```css
:root {
  --z-base: 0;
  --z-dropdown: 100;
  --z-sticky: 200;
  --z-overlay: 300;
  --z-modal: 400;
  --z-toast: 500;
  --z-tooltip: 600;
}
```

---

## Layout Architecture

### Overall Structure

```
┌─────────────────────────────────────────────────────────────────────┐
│ ▪ Seshlog                                    ● Recording  ─ □ ✕    │  <- Title bar (32px)
├────────┬────────────────────────────────────────────────────────────┤
│        │  ┌─────────────────────────────────────────────────────┐   │
│   S    │  │ Meeting Title                            [Actions] │   │  <- Header (48px)
│   I    │  └─────────────────────────────────────────────────────┘   │
│   D    │  ┌──────────────────────┬──────────────────────────────┐   │
│   E    │  │                      │                              │   │
│   B    │  │    PREP / INFO       │       TRANSCRIPT             │   │
│   A    │  │    (Collapsible)     │       (Primary focus)        │   │
│   R    │  │    280px - 320px     │       flex: 1                │   │
│        │  │                      │                              │   │
│  56px  │  │                      ├──────────────────────────────┤   │
│   or   │  │                      │       SUMMARY                │   │
│ 240px  │  │                      │       (Expandable drawer)    │   │
│        │  └──────────────────────┴──────────────────────────────┘   │
│        │  ┌─────────────────────────────────────────────────────┐   │
│        │  │ [Context input]                        ● REC 01:23  │   │  <- Control bar (56px)
│        │  └─────────────────────────────────────────────────────┘   │
└────────┴────────────────────────────────────────────────────────────┘
```

### Sidebar Specifications

#### Collapsed State (56px)

```
┌────────┐
│        │  8px padding top
│  ◈     │  Logo mark (32px)
│        │  16px gap
├────────┤
│   ⌂    │  Home
│   ◉    │  Record (pulsing when active)
│   ☰    │  Meetings
│        │
│        │  flex spacer
│        │
│   ⚙    │  Settings
│   ?    │  Help
│        │  8px padding bottom
└────────┘

Icon size: 20px
Icon button: 40px × 40px
Gap between icons: 4px
Active indicator: 3px left border, accent-primary
```

#### Expanded State (240px)

```
┌────────────────────────┐
│                        │  16px padding
│  ◈  Seshlog            │  Logo + wordmark
│                        │
├────────────────────────┤  Divider
│  🔍 Search meetings... │  Search input
├────────────────────────┤
│                        │
│  ⌂  Home               │  Nav item (44px height)
│  ◉  Record             │  Nav item (active: bg-elevated)
│  ☰  Meetings           │  Nav item (expandable)
│      └ Today           │  Sub-item (indent 24px)
│      └ Yesterday       │  Sub-item
│      └ This Week       │  Sub-item
│                        │
│        (spacer)        │
│                        │
│  ⚙  Settings           │
│  ?  Help               │
├────────────────────────┤
│  v0.3.0                │  Version (centered, text-tertiary)
└────────────────────────┘
```

### Content Panel Grid

```css
.content-layout {
  display: grid;
  grid-template-columns: minmax(280px, 320px) 1fr;
  grid-template-rows: auto 1fr auto;
  gap: 0;
  height: 100%;
}

.content-layout--prep-collapsed {
  grid-template-columns: 0 1fr;
}

.content-layout--summary-expanded {
  grid-template-rows: auto 1fr minmax(200px, 40%);
}
```

---

## Component Specifications

### Buttons

#### Button Hierarchy

| Level | Style | Usage |
|-------|-------|-------|
| Primary | Filled purple, white text | Main actions (Generate, Save, Start) |
| Secondary | Ghost with border | Alternative actions (Cancel, Back) |
| Tertiary | Ghost, no border | Inline actions (Edit, Copy, Delete) |
| Danger | Filled red | Destructive actions (Stop, Delete permanently) |

#### Button Sizes

| Size | Height | Padding | Font Size | Icon Size |
|------|--------|---------|-----------|-----------|
| `sm` | 28px | 8px 12px | 13px | 14px |
| `md` | 36px | 10px 16px | 14px | 16px |
| `lg` | 44px | 12px 24px | 15px | 18px |

#### Button States

```css
.btn-primary {
  background: var(--accent-primary);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  font-weight: var(--font-medium);
  transition: all 150ms ease;
}

.btn-primary:hover {
  background: var(--accent-primary-hover);
}

.btn-primary:active {
  transform: scale(0.98);
}

.btn-primary:focus-visible {
  outline: none;
  box-shadow: 0 0 0 2px var(--bg-base), 0 0 0 4px var(--accent-primary);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}
```

#### Button Variants CSS

```css
/* Secondary Button */
.btn-secondary {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
}

.btn-secondary:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
  border-color: var(--border-default);
}

/* Tertiary Button */
.btn-tertiary {
  background: transparent;
  color: var(--text-secondary);
  border: none;
}

.btn-tertiary:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
}

/* Danger Button */
.btn-danger {
  background: var(--accent-recording);
  color: white;
  border: none;
}

.btn-danger:hover {
  background: #DC2626;
}
```

### Cards

#### Base Card

```css
.card {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
}

.card--elevated {
  background: var(--bg-elevated);
}

.card--interactive {
  cursor: pointer;
  transition: all 150ms ease;
}

.card--interactive:hover {
  border-color: var(--border-default);
  background: var(--bg-elevated);
}
```

#### Prep Section Card

```css
.prep-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-left: 3px solid var(--accent-primary); /* Varies by section type */
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.prep-card__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  cursor: pointer;
}

.prep-card__header:hover {
  background: var(--bg-elevated);
}

.prep-card__title {
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.prep-card__count {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  background: var(--bg-elevated);
  padding: 2px 8px;
  border-radius: var(--radius-full);
}

.prep-card__content {
  padding: 0 var(--space-4) var(--space-4);
}

/* Section type variants */
.prep-card--agenda { border-left-color: var(--accent-primary); }
.prep-card--notes { border-left-color: var(--text-tertiary); }
.prep-card--actions { border-left-color: var(--accent-warning); }
.prep-card--goals { border-left-color: var(--accent-success); }
```

### Input Fields

```css
.input {
  width: 100%;
  height: 40px;
  padding: 0 var(--space-3);
  background: var(--bg-base);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: var(--text-base);
  transition: all 150ms ease;
}

.input::placeholder {
  color: var(--text-tertiary);
}

.input:hover {
  border-color: var(--text-tertiary);
}

.input:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px var(--accent-primary-muted);
}

.input--textarea {
  height: auto;
  min-height: 80px;
  padding: var(--space-3);
  resize: vertical;
  line-height: var(--leading-relaxed);
}
```

### Toggle Switch

```css
.toggle {
  position: relative;
  width: 44px;
  height: 24px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-full);
  cursor: pointer;
  transition: all 150ms ease;
}

.toggle::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  background: var(--text-tertiary);
  border-radius: 50%;
  transition: all 150ms ease;
}

.toggle--active {
  background: var(--accent-primary);
  border-color: var(--accent-primary);
}

.toggle--active::after {
  left: 22px;
  background: white;
}

.toggle:focus-visible {
  outline: none;
  box-shadow: 0 0 0 2px var(--bg-base), 0 0 0 4px var(--accent-primary);
}
```

### Recording Control Bar

#### Idle State

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│  [Add context for AI summary...]                     [ ● Start ]    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘

Height: 56px
Background: var(--bg-surface)
Border-top: 1px solid var(--border-subtle)
Padding: 0 var(--space-4)
```

#### Recording State

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│  ● RECORDING              ▁▂▃▅▂▁▃▅▂▁        00:12:34    [ ■ Stop ] │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘

Recording indicator: Pulsing red dot with "RECORDING" label
Audio meter: Animated bars showing input level
Duration: Monospace font, tabular numbers
Stop button: Danger variant
```

#### Post-Recording State

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│  [Add context to improve summary...]          [ ✦ Generate Summary ]│
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘

Generate button: Primary variant with sparkle icon
```

```css
.control-bar {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  height: 56px;
  padding: 0 var(--space-4);
  background: var(--bg-surface);
  border-top: 1px solid var(--border-subtle);
}

.control-bar__input {
  flex: 1;
}

.control-bar__status {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.recording-indicator {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--accent-recording);
  font-weight: var(--font-semibold);
}

.recording-indicator__dot {
  width: 8px;
  height: 8px;
  background: var(--accent-recording);
  border-radius: 50%;
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(1.1); }
}

.recording-duration {
  font-family: var(--font-mono);
  font-size: var(--text-base);
  color: var(--text-primary);
  font-feature-settings: 'tnum';
}
```

### Transcript Entry

#### Current Problem

```
[00:00] [Speaker 0]: Particular interest in the marriage mart,
[00:04] [Speaker 0]: This also remains uncertain.
[00:07] [Speaker 0]: For any return to society...
```

Every line repeats speaker labels. Visual noise, poor scannability.

#### Proposed Solution: Speaker Blocks

```
┌─────────────────────────────────────────────────────────────────┐
│ ●  Speaker 1                                            00:00   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Particular interest in the marriage mart. This also remains     │
│ uncertain.                                                      │
│                                                                 │
│ For any return to society, it rings with it a number of        │  00:07
│ compelling distractions. Miss Chardess, Miss Bridgerton,       │
│ and Miss Hyacinth.                                              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│ ●  Speaker 2                                            00:29   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Separate. So that was a way long ladies' name.                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

```css
.transcript-block {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  margin-bottom: var(--space-3);
  overflow: hidden;
}

.transcript-block__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  background: var(--bg-elevated);
  border-bottom: 1px solid var(--border-subtle);
}

.transcript-block__speaker {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.transcript-block__speaker-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--accent-primary);
}

/* Alternate speaker colors */
.transcript-block--speaker-1 .transcript-block__speaker-dot { background: #8B5CF6; }
.transcript-block--speaker-2 .transcript-block__speaker-dot { background: #06B6D4; }
.transcript-block--speaker-3 .transcript-block__speaker-dot { background: #10B981; }
.transcript-block--speaker-4 .transcript-block__speaker-dot { background: #F59E0B; }

.transcript-block__speaker-name {
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.transcript-block__timestamp {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.transcript-block__content {
  padding: var(--space-4);
}

.transcript-block__text {
  font-size: var(--text-base);
  line-height: var(--leading-loose);
  color: var(--text-secondary);
}

.transcript-block__inline-time {
  float: right;
  margin-left: var(--space-4);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}
```

### Summary Output

#### Structure

```
┌─────────────────────────────────────────────────────────────────────┐
│ ✦  AI Summary                                       [Copy] [Edit]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│ ┌─────────────────────────────────────────────────────────────────┐ │
│ │                                                                 │ │
│ │  TL;DR                                                          │ │
│ │                                                                 │ │
│ │  Discussion about returning to society with focus on the        │ │
│ │  marriage mart. Key figures mentioned: Miss Chardess,           │ │
│ │  Miss Bridgerton, Miss Hyacinth.                                │ │
│ │                                                                 │ │
│ └─────────────────────────────────────────────────────────────────┘ │
│                                                                     │
│ ACTION ITEMS ───────────────────────────────────────────────────    │
│                                                                     │
│ ┌─────────────────────────────────────────────────────────────────┐ │
│ │ ○  Review society calendar                                      │ │
│ │    Owner: Unassigned  ·  Due: Not set  ·  📍 01:23             │ │
│ └─────────────────────────────────────────────────────────────────┘ │
│                                                                     │
│ ┌─────────────────────────────────────────────────────────────────┐ │
│ │ ○  Follow up with Miss Bridgerton                               │ │
│ │    Owner: Unassigned  ·  Due: Not set  ·  📍 01:45             │ │
│ └─────────────────────────────────────────────────────────────────┘ │
│                                                                     │
│ KEY POINTS ─────────────────────────────────────────────────────    │
│                                                                     │
│   •  Marriage mart is of particular interest                        │
│   •  Return to society has compelling distractions                  │
│   •  Multiple eligible candidates identified                        │
│                                                                     │
│ DECISIONS ──────────────────────────────────────────────────────    │
│                                                                     │
│ ┌─────────────────────────────────────────────────────────────────┐ │
│ │  💡  No decisions were recorded in this meeting                 │ │
│ └─────────────────────────────────────────────────────────────────┘ │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

```css
.summary {
  padding: var(--space-6);
}

.summary__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-6);
}

.summary__title {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.summary__actions {
  display: flex;
  gap: var(--space-2);
}

/* TL;DR Card */
.summary__tldr {
  background: var(--accent-primary-muted);
  border: 1px solid var(--accent-primary);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  margin-bottom: var(--space-6);
}

.summary__tldr-label {
  font-size: var(--text-xs);
  font-weight: var(--font-semibold);
  color: var(--accent-primary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: var(--space-2);
}

.summary__tldr-content {
  font-size: var(--text-base);
  line-height: var(--leading-relaxed);
  color: var(--text-primary);
}

/* Section headers */
.summary__section-header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin: var(--space-6) 0 var(--space-4);
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.summary__section-header::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border-subtle);
}

/* Action item card */
.action-item {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
  margin-bottom: var(--space-2);
}

.action-item__checkbox {
  width: 18px;
  height: 18px;
  border: 2px solid var(--border-default);
  border-radius: var(--radius-sm);
  margin-right: var(--space-3);
}

.action-item__text {
  font-size: var(--text-base);
  color: var(--text-primary);
}

.action-item__meta {
  display: flex;
  gap: var(--space-3);
  margin-top: var(--space-2);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.action-item__timestamp {
  color: var(--accent-secondary);
  cursor: pointer;
}

.action-item__timestamp:hover {
  text-decoration: underline;
}

/* Empty section notice */
.summary__empty {
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
```

### Modal

```css
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: var(--z-modal);
}

.modal {
  width: 100%;
  max-width: 480px;
  max-height: 90vh;
  background: var(--bg-overlay);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  overflow: hidden;
}

.modal__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-6);
  border-bottom: 1px solid var(--border-subtle);
}

.modal__title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.modal__close {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-tertiary);
  border-radius: var(--radius-md);
  cursor: pointer;
}

.modal__close:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
}

.modal__content {
  padding: var(--space-6);
  overflow-y: auto;
}

.modal__footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding: var(--space-4) var(--space-6);
  border-top: 1px solid var(--border-subtle);
}
```

---

## Empty States

### Principles

1. **Illustrate the value** - Show what will appear, not just what's missing
2. **Guide to action** - Clear, prominent CTAs
3. **Add personality** - Use illustration or icon, not just text
4. **Be helpful** - Explain why this state exists

### No Meeting Loaded

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│                                                                     │
│                              ┌─────────┐                            │
│                              │         │                            │
│                              │   ◈     │                            │
│                              │         │                            │
│                              └─────────┘                            │
│                                                                     │
│                       Ready when you are                            │
│                                                                     │
│              Open a meeting prep file from Obsidian,                │
│                or start recording a new session.                    │
│                                                                     │
│                                                                     │
│            [ Open File ]            [ Start Recording ]             │
│                                                                     │
│                                                                     │
│   ─────────────────────────────────────────────────────────────     │
│                                                                     │
│   Recent meetings:                                                  │
│                                                                     │
│   📄  Team Standup - Jan 28                           Yesterday     │
│   📄  Product Review - Jan 27                         2 days ago    │
│   📄  Client Call - Jan 25                            4 days ago    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

```css
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-12);
  text-align: center;
}

.empty-state__icon {
  width: 80px;
  height: 80px;
  margin-bottom: var(--space-6);
  color: var(--text-tertiary);
}

.empty-state__title {
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  margin-bottom: var(--space-2);
}

.empty-state__description {
  font-size: var(--text-base);
  color: var(--text-secondary);
  max-width: 320px;
  margin-bottom: var(--space-6);
}

.empty-state__actions {
  display: flex;
  gap: var(--space-3);
}

.empty-state__recent {
  margin-top: var(--space-8);
  padding-top: var(--space-8);
  border-top: 1px solid var(--border-subtle);
  width: 100%;
  max-width: 400px;
}

.empty-state__recent-title {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--text-tertiary);
  margin-bottom: var(--space-4);
  text-align: left;
}
```

### No Summary Yet

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│                                                                     │
│                               ✦                                     │
│                                                                     │
│                    Transform your transcript                        │
│                                                                     │
│             Get key points, action items, and decisions             │
│                   extracted automatically by AI.                    │
│                                                                     │
│                                                                     │
│                     [ ✦ Generate Summary ]                          │
│                                                                     │
│                                                                     │
│              💡 Tip: Add context below to improve results           │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Recording in Progress (No Transcript Yet)

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│                                                                     │
│                         ◉ ◉ ◉                                       │
│                      (animated)                                     │
│                                                                     │
│                       Listening...                                  │
│                                                                     │
│               Transcript will appear as you speak.                  │
│                                                                     │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## About Modal Redesign

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                               ✕     │
│                                                                     │
│                               ◈                                     │
│                            Seshlog                                  │
│                             v0.3.0                                  │
│                                                                     │
│           Real-time meeting notes that stay on your machine         │
│                                                                     │
│                       [ Check for Updates ]                         │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   🔒  Privacy-first                    🤖  Use any model            │
│   Your data never leaves               Local or cloud,              │
│   your device                          your choice                  │
│                                                                     │
│   💰  Cost-smart                       🌍  Works everywhere         │
│   Run locally or pay only              Zoom, Meet, Teams,           │
│   when you want                        online or offline            │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   ✨ Coming soon: On-device AI agents for automated follow-ups      │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│              Docs  ·  GitHub  ·  Discord  ·  Twitter                │
│                                                                     │
│                     Made with ♥ by [Company]                        │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Settings Sidebar Pattern

Instead of horizontal tabs, use a left sidebar:

```
┌──────────────────────┬──────────────────────────────────────────────┐
│                      │                                              │
│  ←  Settings         │  General                                     │
│                      │                                              │
│  ▸ General           │  ┌────────────────────────────────────────┐  │
│    Recording         │  │                                        │  │
│    Transcription     │  │  Appearance                            │  │
│    AI & Summary      │  │                                        │  │
│    Storage           │  │  Theme                    [Dark ▾]     │  │
│    Privacy           │  │                                        │  │
│                      │  │  Sidebar default          [Expanded ▾] │  │
│                      │  │                                        │  │
│                      │  └────────────────────────────────────────┘  │
│                      │                                              │
│                      │  ┌────────────────────────────────────────┐  │
│                      │  │                                        │  │
│                      │  │  Notifications                         │  │
│                      │  │                                        │  │
│                      │  │  Meeting start           ────────●     │  │
│                      │  │                                        │  │
│                      │  │  Meeting end             ●────────     │  │
│                      │  │                                        │  │
│                      │  │  Summary ready           ────────●     │  │
│                      │  │                                        │  │
│                      │  └────────────────────────────────────────┘  │
│                      │                                              │
└──────────────────────┴──────────────────────────────────────────────┘

Settings sidebar width: 200px
Active item: bg-elevated, accent-primary left border
```

---

## Implementation Priority

### Phase 1: Foundation (Critical)

**Goal:** Establish design system basics, fix branding

| Task | Files | Effort |
|------|-------|--------|
| Create CSS custom properties | `globals.css` | Low |
| Remove Meetily references | Multiple | Low |
| Update logo/branding | `Logo.tsx`, assets | Medium |
| Switch to dark backgrounds | `globals.css`, components | Medium |
| Update button system | `button.tsx` | Medium |

### Phase 2: Layout (High)

**Goal:** Fix structural issues

| Task | Files | Effort |
|------|-------|--------|
| Remove floating record button | `RecordingControls.tsx` | Low |
| Create unified control bar | New component | Medium |
| Implement collapsible prep panel | Layout components | Medium |
| Update sidebar expanded/collapsed | `Sidebar/index.tsx` | Medium |

### Phase 3: Content (High)

**Goal:** Improve core content display

| Task | Files | Effort |
|------|-------|--------|
| Redesign transcript blocks | `TranscriptView.tsx` | High |
| Redesign summary output | `AISummary/` | High |
| Update prep cards with borders | `PrepPanel` components | Medium |
| Create better empty states | Multiple | Medium |

### Phase 4: Polish (Medium)

**Goal:** Complete the experience

| Task | Files | Effort |
|------|-------|--------|
| Settings sidebar pattern | `SettingsModal.tsx` | Medium |
| About modal redesign | `About.tsx` | Low |
| Add transitions/animations | CSS | Medium |
| Loading states | Multiple | Medium |
| Focus states for accessibility | CSS | Low |

---

## Quick Wins (Immediate)

These require minimal refactoring:

1. **Replace yellow/orange with purple** - CSS variable swap
2. **Darken content backgrounds** - `#FFFFFF` → `#16161A`
3. **Remove "Meetily" from About** - Content change
4. **Fix version inconsistency** - Centralize version constant
5. **Update empty state copy** - Content change
6. **Add left border to prep cards** - CSS addition
7. **Update button hover states** - CSS change

---

## File Structure for Design Tokens

```
frontend/src/
├── styles/
│   ├── tokens/
│   │   ├── colors.css
│   │   ├── typography.css
│   │   ├── spacing.css
│   │   └── index.css        # Imports all tokens
│   ├── components/
│   │   ├── buttons.css
│   │   ├── cards.css
│   │   ├── inputs.css
│   │   └── ...
│   └── globals.css          # Imports tokens + base styles
```

Or if using Tailwind, extend the config:

```js
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        bg: {
          base: '#0D0D0F',
          surface: '#16161A',
          elevated: '#1E1E24',
          overlay: '#26262E',
        },
        accent: {
          primary: '#8B5CF6',
          'primary-hover': '#7C3AED',
          secondary: '#06B6D4',
          recording: '#EF4444',
          success: '#10B981',
          warning: '#F59E0B',
        },
        // ... etc
      },
    },
  },
}
```

---

## References

- **Inspiration 1:** Dark purple note-taking app (Notion-like)
- **Inspiration 2:** Dark document editor with callouts
- **Design systems:** Linear, Raycast, Vercel Dashboard
- **Current screenshots:** `.temp/screenshots/`

---

*This document should be treated as a living specification. Update as implementation progresses and new patterns emerge.*
