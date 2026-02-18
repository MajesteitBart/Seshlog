---
issue: 006
title: Implement two-panel layout
analyzed: 2026-01-28T07:55:38Z
estimated_hours: 3
parallelization_factor: 1.5
---

# Parallel Work Analysis: Issue #006

## Overview
Refactor the main page to use a two-panel layout with PrepPanel on the left (1/3 width) and TranscriptPanel on the right (2/3 width). The current implementation is already partially complete - the two-panel layout is in place, but the sidebar still renders and needs to be removed/simplified.

## Current State Assessment

Looking at the existing code:
- ✅ `page.tsx` already has two-panel layout implemented
- ✅ `PrepPanel` component exists and is functional
- ✅ File picker with `open_meeting_file` Tauri command integrated
- ✅ Header with "Open Meeting File" button exists
- ⚠️ Sidebar still renders (needs removal from layout)
- ⚠️ Mobile responsive view may need mobile drawer for PrepPanel
- ⚠️ Recording controls positioning depends on sidebar state

## Parallel Streams

### Stream A: Sidebar Removal & Layout Cleanup
**Scope**: Remove sidebar from main layout, update recording controls positioning
**Files**:
- `frontend/src/app/layout.tsx` - Remove Sidebar component
- `frontend/src/app/page.tsx` - Update recording controls positioning (remove sidebarCollapsed dependency)
**Agent Type**: frontend-specialist
**Can Start**: immediately
**Estimated Hours**: 1.5
**Dependencies**: none

### Stream B: Mobile Responsive Layout
**Scope**: Implement mobile drawer for PrepPanel, ensure single-column layout on mobile
**Files**:
- `frontend/src/app/page.tsx` - Add mobile drawer toggle
- `frontend/src/app/_components/PrepPanel.tsx` - Ensure mobile-friendly styles
- May need new `MobileDrawer.tsx` component
**Agent Type**: frontend-specialist
**Can Start**: immediately
**Estimated Hours**: 1.5
**Dependencies**: none

## Coordination Points

### Shared Files
- `frontend/src/app/page.tsx` - Both streams modify this file
  - Stream A: Recording controls positioning
  - Stream B: Mobile drawer integration
  - **Resolution**: Stream A handles main layout, Stream B adds mobile-specific wrapper

### Sequential Requirements
1. Stream A should handle the main layout refactor first
2. Stream B can add mobile enhancements in parallel but should merge carefully

## Conflict Risk Assessment
- **Medium Risk**: Both streams touch `page.tsx`, but different sections
- Stream A focuses on removing sidebar dependencies
- Stream B focuses on adding mobile drawer

## Parallelization Strategy

**Recommended Approach**: hybrid

Since the main two-panel layout is already implemented, the remaining work is:
1. Stream A: Remove sidebar, fix recording controls (can start immediately)
2. Stream B: Add mobile drawer (can start immediately, coordinate on page.tsx)

Given the overlap in `page.tsx`, a sequential approach may be safer:
- Complete Stream A first (sidebar removal, core cleanup)
- Then Stream B (mobile enhancements)

Alternatively, work on this as a single focused stream given the task size (3 hours).

## Expected Timeline

With parallel execution:
- Wall time: ~2 hours (streams overlap)
- Total work: 3 hours
- Efficiency gain: 33%

Without parallel execution:
- Wall time: 3 hours

## Recommendation

Given that:
1. The core two-panel layout is already implemented
2. The task is relatively small (3 hours)
3. There's significant file overlap

**Recommended approach**: Single focused stream to complete remaining work:
1. Remove sidebar from layout
2. Fix recording controls positioning
3. Add mobile drawer/responsive behavior
4. Final testing and cleanup

This avoids coordination overhead for a small task.

## Notes
- The PrepPanel, file picker, and two-panel structure are already working
- Main remaining work is sidebar removal and mobile responsiveness
- Consider keeping Sidebar component files for potential future use (just remove from layout)
