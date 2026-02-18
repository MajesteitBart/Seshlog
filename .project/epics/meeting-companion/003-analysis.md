---
issue: 003
analyzed: 2026-01-28T09:19:50Z
complexity: low
parallel_streams: 1
---

# Issue #003 Analysis: Create Obsidian file writer (Phase 1)

## Summary

This task adds markdown file generation when recordings stop. Phase 2 (merge into Obsidian prep file) is already complete. Phase 1 requires saving transcript + summary markdown to the recording folder.

## Scope

**Phase 1 only** - save markdown to recording folder when recording stops.

### Files to Modify/Create

1. `frontend/src-tauri/src/obsidian/writer.rs` - Add `save_meeting_markdown()` function
2. `frontend/src-tauri/src/obsidian/mod.rs` - Export new function
3. `frontend/src-tauri/src/lib.rs` - Add Tauri command for saving markdown
4. `frontend/src-tauri/src/audio/recording_saver.rs` - Call markdown generation in `stop_and_save()`

### Key Integration Points

1. **Recording Stop Hook**: `recording_saver.rs:stop_and_save()` already writes `transcripts.json` and `metadata.json` to the meeting folder. Add markdown generation here.

2. **Transcript Data**: Available via `self.transcript_segments` in `RecordingSaver` - already structured with timestamps and text.

3. **Summary Data**: Summaries are generated asynchronously AFTER recording stops (via frontend triggering LLM). The markdown file should be generated when:
   - Recording stops (transcript only)
   - Summary completes (update with summary)

4. **Meeting Folder Path**: Available as `self.meeting_folder` in `RecordingSaver` - e.g., `C:\Users\{user}\Music\meetily-recordings\Meeting 2026-01-28_10-30-00\`

## Work Stream

Since this is a small, focused change with clear dependencies (Phase 2 already done), a single stream is sufficient.

### Stream A: Implement Phase 1 Markdown Generation

**Scope**: Add `save_meeting_markdown()` to existing writer.rs and integrate into recording stop flow.

**Files**:
- `frontend/src-tauri/src/obsidian/writer.rs`
- `frontend/src-tauri/src/obsidian/mod.rs`
- `frontend/src-tauri/src/lib.rs`
- `frontend/src-tauri/src/audio/recording_saver.rs`

**Implementation Steps**:

1. Add `MeetingMarkdownData` struct to `writer.rs`:
   ```rust
   pub struct MeetingMarkdownData {
       pub date: String,
       pub time_start: String,
       pub duration: String,
       pub recording_filename: Option<String>,
       pub segments: Vec<TranscriptSegment>,
       pub summary: Option<String>,
   }
   ```

2. Add `generate_meeting_markdown()` function to format the content

3. Add `save_meeting_markdown()` async function with atomic write

4. Add Tauri command to update markdown with summary (called from frontend after LLM completes)

5. Call `save_meeting_markdown()` from `recording_saver.rs:stop_and_save()`

## Dependencies

- `writer.rs` already exists with atomic write helpers (reuse `atomic_write()`)
- `TranscriptSegment` struct exists in `recording_saver.rs` (needs import)
- Summary integration will be a separate Tauri command (frontend-driven)

## Risks

- **Low**: Well-understood codebase with existing patterns to follow
- **Low**: Atomic writes already implemented, just need to reuse

## Estimated Effort

- 2-3 hours implementation
- No parallelization needed (single small task)
