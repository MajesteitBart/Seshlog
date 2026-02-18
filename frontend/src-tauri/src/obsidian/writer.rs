//! Writer for Obsidian markdown meeting files
//!
//! Handles two writing modes:
//! 1. **Phase 1**: Save standalone meeting markdown to recording folder (transcript + summary)
//! 2. **Phase 2**: Merge transcript data back into existing Obsidian meeting prep file
//!
//! Key features:
//! - Preserves existing content when merging
//! - Atomic writes to prevent data loss
//! - Formats transcripts with timestamps and speaker labels
//! - Generates frontmatter with meeting metadata

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A single transcript segment with timestamp, speaker, and text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    /// Timestamp in "HH:MM:SS" or "MM:SS" format
    pub timestamp: String,
    /// Speaker identifier (e.g., "Speaker 1") or None for unspeakered
    pub speaker: Option<String>,
    /// The transcribed text
    pub text: String,
}

/// Request to save transcript to a meeting file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveTranscriptRequest {
    /// Path to the meeting markdown file
    pub file_path: String,
    /// Transcript segments to save
    pub segments: Vec<TranscriptSegment>,
    /// Whether to update frontmatter status to "completed"
    #[serde(default = "default_update_status")]
    pub update_status: bool,
}

fn default_update_status() -> bool {
    true
}

// ============================================================================
// PHASE 1: Standalone Meeting Markdown Generation
// ============================================================================

/// Simplified transcript entry for markdown generation
/// Used when converting from audio recording's TranscriptSegment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownTranscriptEntry {
    /// Display timestamp like "[00:02:15]" or "[02:15]"
    pub display_time: String,
    /// The transcribed text
    pub text: String,
    /// Optional speaker label (e.g., "Speaker 1")
    pub speaker: Option<String>,
}

/// Data for generating a standalone meeting markdown file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingMarkdownData {
    /// Date in YYYY-MM-DD format
    pub date: String,
    /// Start time in HH:MM:SS format
    pub time_start: String,
    /// Duration in MM:SS or HH:MM:SS format
    pub duration: String,
    /// Recording filename (just the filename, not full path)
    pub recording_filename: Option<String>,
    /// Transcript entries for markdown
    pub entries: Vec<MarkdownTranscriptEntry>,
    /// Optional AI-generated summary markdown
    pub summary: Option<String>,
}

/// Generate markdown content for a meeting file
///
/// Creates a complete markdown document with:
/// - YAML frontmatter with metadata
/// - Summary section (if provided)
/// - Transcript section with formatted entries
pub fn generate_meeting_markdown(data: &MeetingMarkdownData) -> String {
    let mut content = String::new();

    // Frontmatter
    content.push_str("---\n");
    content.push_str(&format!("date: {}\n", data.date));
    content.push_str(&format!("time_start: {}\n", data.time_start));
    content.push_str(&format!("duration: {}\n", data.duration));
    if let Some(ref recording) = data.recording_filename {
        content.push_str(&format!("recording: {}\n", recording));
    }
    content.push_str("status: completed\n");
    content.push_str("---\n\n");

    // Title
    content.push_str(&format!("# Meeting Notes - {}\n\n", data.date));

    // Summary section (if available)
    if let Some(ref summary) = data.summary {
        content.push_str("## Summary\n\n");
        content.push_str(summary);
        content.push_str("\n\n");
    }

    // Transcript section
    content.push_str(&format_markdown_transcript(&data.entries));

    content
}

/// Format transcript entries into markdown
fn format_markdown_transcript(entries: &[MarkdownTranscriptEntry]) -> String {
    if entries.is_empty() {
        return "## Transcript\n\n_No transcript recorded._\n".to_string();
    }

    let mut lines = vec!["## Transcript".to_string(), String::new()];

    for entry in entries {
        let line = match &entry.speaker {
            Some(speaker) => {
                format!("{} **{}:** {}", entry.display_time, speaker, entry.text)
            }
            None => {
                format!("{} {}", entry.display_time, entry.text)
            }
        };
        lines.push(line);
        lines.push(String::new()); // Blank line between entries
    }

    lines.join("\n")
}

/// Save meeting markdown to a file alongside the recording
///
/// # Arguments
/// * `recording_path` - Path to the audio recording file
/// * `data` - Meeting data to write
///
/// # Returns
/// * `Ok(PathBuf)` - Path to the created markdown file
/// * `Err(WriteError)` - If writing fails
pub async fn save_meeting_markdown(
    recording_path: &Path,
    data: &MeetingMarkdownData,
) -> Result<PathBuf, WriteError> {
    // Generate markdown path from recording path
    let md_path = recording_path.with_extension("md");

    // Generate content
    let content = generate_meeting_markdown(data);

    // Atomic write
    atomic_write(&md_path, &content).await?;

    log::info!("Saved meeting markdown to: {}", md_path.display());
    Ok(md_path)
}

/// Save meeting markdown to a specific folder
///
/// # Arguments
/// * `folder` - Folder to save the markdown file in
/// * `filename` - Name of the markdown file (without extension)
/// * `data` - Meeting data to write
///
/// # Returns
/// * `Ok(PathBuf)` - Path to the created markdown file
/// * `Err(WriteError)` - If writing fails
pub async fn save_meeting_markdown_to_folder(
    folder: &Path,
    filename: &str,
    data: &MeetingMarkdownData,
) -> Result<PathBuf, WriteError> {
    let md_path = folder.join(format!("{}.md", filename));

    // Generate content
    let content = generate_meeting_markdown(data);

    // Atomic write
    atomic_write(&md_path, &content).await?;

    log::info!("Saved meeting markdown to: {}", md_path.display());
    Ok(md_path)
}

/// Update an existing meeting markdown file with summary content
///
/// # Arguments
/// * `md_path` - Path to the existing markdown file
/// * `summary` - Summary markdown to insert
///
/// # Returns
/// * `Ok(())` - Success
/// * `Err(WriteError)` - If update fails
pub async fn update_meeting_markdown_with_summary(
    md_path: &Path,
    summary: &str,
) -> Result<(), WriteError> {
    // Read existing content
    let content = tokio::fs::read_to_string(md_path)
        .await
        .map_err(|e| WriteError::ReadError(format!("Failed to read file: {}", e)))?;

    // Insert or replace summary section
    let updated_content = insert_summary_section(&content, summary);

    // Atomic write
    atomic_write(md_path, &updated_content).await?;

    log::info!("Updated meeting markdown with summary: {}", md_path.display());
    Ok(())
}

/// Insert or replace the Summary section in markdown content
fn insert_summary_section(content: &str, summary: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result_lines: Vec<String> = Vec::new();
    let mut in_summary_section = false;
    let mut summary_inserted = false;

    for line in &lines {
        let trimmed = line.trim();

        // Check if we're entering the Summary section
        if !in_summary_section && trimmed.eq_ignore_ascii_case("## Summary") {
            in_summary_section = true;
            // Insert new summary section
            result_lines.push("## Summary".to_string());
            result_lines.push(String::new());
            result_lines.push(summary.to_string());
            result_lines.push(String::new());
            summary_inserted = true;
            continue;
        }

        // Check if we're exiting the Summary section (hit another ## heading)
        if in_summary_section && trimmed.starts_with("## ") && !trimmed.eq_ignore_ascii_case("## Summary") {
            in_summary_section = false;
            result_lines.push(line.to_string());
            continue;
        }

        // Skip lines while in old summary section
        if in_summary_section {
            continue;
        }

        result_lines.push(line.to_string());
    }

    // If no summary section existed, insert before transcript section
    if !summary_inserted {
        let mut final_lines: Vec<String> = Vec::new();
        let mut inserted = false;

        for line in &result_lines {
            // Insert summary before ## Transcript if found
            if !inserted && line.trim().eq_ignore_ascii_case("## Transcript") {
                final_lines.push("## Summary".to_string());
                final_lines.push(String::new());
                final_lines.push(summary.to_string());
                final_lines.push(String::new());
                inserted = true;
            }
            final_lines.push(line.clone());
        }

        // If no transcript section either, append summary at end after title
        if !inserted {
            let mut new_result: Vec<String> = Vec::new();
            let mut after_title = false;

            for line in &result_lines {
                new_result.push(line.clone());
                // Insert after the first # heading
                if !after_title && line.starts_with("# ") {
                    new_result.push(String::new());
                    new_result.push("## Summary".to_string());
                    new_result.push(String::new());
                    new_result.push(summary.to_string());
                    after_title = true;
                }
            }

            return new_result.join("\n");
        }

        return final_lines.join("\n");
    }

    result_lines.join("\n")
}

// ============================================================================
// PHASE 2: Merge into Existing Obsidian File (existing implementation)
// ============================================================================

/// Error type for write operations
#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("Failed to read file: {0}")]
    ReadError(String),
    #[error("Failed to write file: {0}")]
    WriteError(String),
    #[error("Failed to rename temp file: {0}")]
    RenameError(String),
    #[error("Invalid file format: {0}")]
    FormatError(String),
}

/// Merge transcript segments into the original file content
pub fn merge_transcript(
    original_content: &str,
    segments: &[TranscriptSegment],
    update_status: bool,
) -> Result<String, WriteError> {
    let mut content = original_content.to_string();

    // Update frontmatter status if requested
    if update_status {
        content = update_frontmatter_status(&content, "completed");
    }

    // Format the transcript section
    let transcript_section = format_transcript_section(segments);

    // Find or create the Transcript section
    content = insert_transcript_section(&content, &transcript_section);

    Ok(content)
}

/// Update the frontmatter status field
fn update_frontmatter_status(content: &str, new_status: &str) -> String {
    let content = content.trim_start();

    // Check if frontmatter exists
    if !content.starts_with("---") {
        return content.to_string();
    }

    // Find the closing marker
    let after_first_marker = &content[3..];
    let Some(end_pos) = after_first_marker.find("\n---") else {
        return content.to_string();
    };

    let frontmatter = &after_first_marker[..end_pos];
    let rest_of_file = &after_first_marker[end_pos + 4..];

    // Update or add status field in frontmatter
    let updated_frontmatter = if frontmatter.contains("status:") {
        // Replace existing status
        let lines: Vec<String> = frontmatter
            .lines()
            .map(|line| {
                if line.trim_start().starts_with("status:") {
                    format!("status: {}", new_status)
                } else {
                    line.to_string()
                }
            })
            .collect();
        lines.join("\n")
    } else {
        // Add status field
        format!("{}\nstatus: {}", frontmatter.trim_end(), new_status)
    };

    format!("---\n{}\n---{}", updated_frontmatter, rest_of_file)
}

/// Format transcript segments into markdown
fn format_transcript_section(segments: &[TranscriptSegment]) -> String {
    if segments.is_empty() {
        return "## Transcript\n\n_No transcript recorded._\n".to_string();
    }

    let mut lines = vec!["## Transcript".to_string(), String::new()];

    for segment in segments {
        let line = match &segment.speaker {
            Some(speaker) => {
                format!("[{}] **{}:** {}", segment.timestamp, speaker, segment.text)
            }
            None => {
                format!("[{}] {}", segment.timestamp, segment.text)
            }
        };
        lines.push(line);
        lines.push(String::new()); // Blank line between segments
    }

    lines.join("\n")
}

/// Insert or replace the Transcript section in the content
fn insert_transcript_section(content: &str, transcript_section: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result_lines: Vec<String> = Vec::new();
    let mut in_transcript_section = false;
    let mut transcript_inserted = false;
    let mut transcript_section_level = 0;

    for line in &lines {
        let trimmed = line.trim();

        // Check if we're entering the Transcript section
        if !in_transcript_section
            && (trimmed.eq_ignore_ascii_case("## Transcript")
                || trimmed.eq_ignore_ascii_case("## transcript"))
        {
            in_transcript_section = true;
            transcript_section_level = count_heading_level(trimmed);
            // Don't add the old heading, we'll add the new section
            continue;
        }

        // Check if we're exiting the Transcript section (hit another heading at same/higher level)
        if in_transcript_section && is_heading(trimmed) {
            let level = count_heading_level(trimmed);
            if level <= transcript_section_level {
                // We've exited the transcript section
                // Insert new transcript before this heading
                if !transcript_inserted {
                    result_lines.push(transcript_section.to_string());
                    result_lines.push(String::new());
                    transcript_inserted = true;
                }
                in_transcript_section = false;
            }
        }

        // Skip lines while in old transcript section
        if in_transcript_section {
            continue;
        }

        result_lines.push(line.to_string());
    }

    // If we never inserted the transcript (no existing section or section was at end)
    if !transcript_inserted {
        // Add a blank line if needed
        if !result_lines.is_empty() && !result_lines.last().unwrap().is_empty() {
            result_lines.push(String::new());
        }
        result_lines.push(transcript_section.to_string());
    }

    result_lines.join("\n")
}

/// Count the heading level (number of # characters)
fn count_heading_level(line: &str) -> usize {
    line.chars().take_while(|c| *c == '#').count()
}

/// Check if a line is a markdown heading
fn is_heading(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') && trimmed.chars().skip_while(|c| *c == '#').next() == Some(' ')
}

/// Perform an atomic write to the file (write to temp, then rename)
pub async fn atomic_write(path: &Path, content: &str) -> Result<(), WriteError> {
    let temp_path = path.with_extension("md.tmp");

    // Write to temp file
    tokio::fs::write(&temp_path, content)
        .await
        .map_err(|e| WriteError::WriteError(format!("Failed to write temp file: {}", e)))?;

    // Atomic rename
    tokio::fs::rename(&temp_path, path)
        .await
        .map_err(|e| WriteError::RenameError(format!("Failed to rename temp file: {}", e)))?;

    Ok(())
}

/// Save transcript to a meeting file
pub async fn save_transcript(request: &SaveTranscriptRequest) -> Result<(), WriteError> {
    let path = Path::new(&request.file_path);

    // Read original content
    let original_content = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| WriteError::ReadError(format!("Failed to read file: {}", e)))?;

    // Merge transcript
    let merged_content = merge_transcript(&original_content, &request.segments, request.update_status)?;

    // Atomic write
    atomic_write(path, &merged_content).await?;

    log::info!("Saved transcript to: {}", request.file_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_FILE: &str = r#"---
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
"#;

    #[test]
    fn test_format_transcript_section() {
        let segments = vec![
            TranscriptSegment {
                timestamp: "00:00:12".to_string(),
                speaker: Some("Speaker 1".to_string()),
                text: "Hello everyone.".to_string(),
            },
            TranscriptSegment {
                timestamp: "00:00:20".to_string(),
                speaker: Some("Speaker 2".to_string()),
                text: "Hi there!".to_string(),
            },
        ];

        let result = format_transcript_section(&segments);
        assert!(result.contains("## Transcript"));
        assert!(result.contains("[00:00:12] **Speaker 1:** Hello everyone."));
        assert!(result.contains("[00:00:20] **Speaker 2:** Hi there!"));
    }

    #[test]
    fn test_format_transcript_no_speaker() {
        let segments = vec![TranscriptSegment {
            timestamp: "00:01:00".to_string(),
            speaker: None,
            text: "Unattributed speech.".to_string(),
        }];

        let result = format_transcript_section(&segments);
        assert!(result.contains("[00:01:00] Unattributed speech."));
        assert!(!result.contains("**"));
    }

    #[test]
    fn test_format_transcript_empty() {
        let result = format_transcript_section(&[]);
        assert!(result.contains("## Transcript"));
        assert!(result.contains("No transcript recorded"));
    }

    #[test]
    fn test_update_frontmatter_status() {
        let result = update_frontmatter_status(SAMPLE_FILE, "completed");
        assert!(result.contains("status: completed"));
        assert!(!result.contains("status: scheduled"));
    }

    #[test]
    fn test_update_frontmatter_no_existing_status() {
        let content = r#"---
date: 2026-01-22
type: meeting
---

# Meeting
"#;
        let result = update_frontmatter_status(content, "completed");
        assert!(result.contains("status: completed"));
    }

    #[test]
    fn test_update_frontmatter_no_frontmatter() {
        let content = "# Just a title\nSome content";
        let result = update_frontmatter_status(content, "completed");
        // Should return unchanged
        assert_eq!(result, content);
    }

    #[test]
    fn test_merge_transcript() {
        let segments = vec![
            TranscriptSegment {
                timestamp: "00:00:12".to_string(),
                speaker: Some("Speaker 1".to_string()),
                text: "Welcome everyone.".to_string(),
            },
        ];

        let result = merge_transcript(SAMPLE_FILE, &segments, true).unwrap();

        // Status should be updated
        assert!(result.contains("status: completed"));

        // Original content preserved
        assert!(result.contains("# Meeting: Project Kickoff"));
        assert!(result.contains("## Prep"));
        assert!(result.contains("### Goals"));
        assert!(result.contains("Align on timeline"));

        // Transcript section added
        assert!(result.contains("## Transcript"));
        assert!(result.contains("[00:00:12] **Speaker 1:** Welcome everyone."));
    }

    #[test]
    fn test_merge_transcript_replace_existing() {
        let content_with_transcript = r#"---
date: 2026-01-22
status: scheduled
---

# Meeting

## Prep
Some prep content.

## Transcript

[00:00:00] Old transcript content.

## Notes
Some notes.
"#;

        let segments = vec![TranscriptSegment {
            timestamp: "00:01:00".to_string(),
            speaker: Some("Speaker 1".to_string()),
            text: "New transcript.".to_string(),
        }];

        let result = merge_transcript(content_with_transcript, &segments, false).unwrap();

        // Old transcript should be replaced
        assert!(!result.contains("Old transcript content"));
        assert!(result.contains("New transcript."));

        // Other sections preserved
        assert!(result.contains("## Prep"));
        assert!(result.contains("Some prep content."));
        assert!(result.contains("## Notes"));
        assert!(result.contains("Some notes."));
    }

    #[test]
    fn test_insert_transcript_at_end() {
        let content = r#"# Meeting

## Prep
Content here.
"#;

        let transcript_section = "## Transcript\n\n[00:00:00] Test.";
        let result = insert_transcript_section(content, transcript_section);

        assert!(result.contains("## Prep"));
        assert!(result.contains("## Transcript"));
        assert!(result.ends_with("[00:00:00] Test."));
    }

    // =========================================================================
    // Phase 1 Tests: Standalone Meeting Markdown Generation
    // =========================================================================

    #[test]
    fn test_generate_meeting_markdown_basic() {
        let data = MeetingMarkdownData {
            date: "2026-01-28".to_string(),
            time_start: "10:30:00".to_string(),
            duration: "45:12".to_string(),
            recording_filename: Some("meeting-2026-01-28_10-30-00.mp4".to_string()),
            entries: vec![
                MarkdownTranscriptEntry {
                    display_time: "[00:00:12]".to_string(),
                    text: "Welcome everyone.".to_string(),
                    speaker: Some("Speaker 1".to_string()),
                },
                MarkdownTranscriptEntry {
                    display_time: "[00:00:34]".to_string(),
                    text: "Thanks for having us.".to_string(),
                    speaker: Some("Speaker 2".to_string()),
                },
            ],
            summary: None,
        };

        let result = generate_meeting_markdown(&data);

        // Check frontmatter
        assert!(result.contains("date: 2026-01-28"));
        assert!(result.contains("time_start: 10:30:00"));
        assert!(result.contains("duration: 45:12"));
        assert!(result.contains("recording: meeting-2026-01-28_10-30-00.mp4"));
        assert!(result.contains("status: completed"));

        // Check title
        assert!(result.contains("# Meeting Notes - 2026-01-28"));

        // Check transcript
        assert!(result.contains("## Transcript"));
        assert!(result.contains("[00:00:12] **Speaker 1:** Welcome everyone."));
        assert!(result.contains("[00:00:34] **Speaker 2:** Thanks for having us."));
    }

    #[test]
    fn test_generate_meeting_markdown_with_summary() {
        let data = MeetingMarkdownData {
            date: "2026-01-28".to_string(),
            time_start: "14:00:00".to_string(),
            duration: "30:00".to_string(),
            recording_filename: None,
            entries: vec![
                MarkdownTranscriptEntry {
                    display_time: "[00:01:00]".to_string(),
                    text: "Let's discuss the roadmap.".to_string(),
                    speaker: None,
                },
            ],
            summary: Some("## Key Points\n- Discussed roadmap\n- Assigned tasks".to_string()),
        };

        let result = generate_meeting_markdown(&data);

        // Check summary is present
        assert!(result.contains("## Summary"));
        assert!(result.contains("## Key Points"));
        assert!(result.contains("Discussed roadmap"));

        // Check transcript follows summary
        assert!(result.contains("## Transcript"));
        assert!(result.contains("[00:01:00] Let's discuss the roadmap."));
    }

    #[test]
    fn test_generate_meeting_markdown_empty_transcript() {
        let data = MeetingMarkdownData {
            date: "2026-01-28".to_string(),
            time_start: "09:00:00".to_string(),
            duration: "00:00".to_string(),
            recording_filename: None,
            entries: vec![],
            summary: None,
        };

        let result = generate_meeting_markdown(&data);

        assert!(result.contains("## Transcript"));
        assert!(result.contains("No transcript recorded"));
    }

    #[test]
    fn test_insert_summary_section_new() {
        let content = r#"---
date: 2026-01-28
status: completed
---

# Meeting Notes - 2026-01-28

## Transcript

[00:00:12] Hello.
"#;

        let result = insert_summary_section(content, "This is the summary.");

        // Summary should be inserted before transcript
        assert!(result.contains("## Summary"));
        assert!(result.contains("This is the summary."));
        assert!(result.contains("## Transcript"));

        // Verify order: Summary comes before Transcript
        let summary_pos = result.find("## Summary").unwrap();
        let transcript_pos = result.find("## Transcript").unwrap();
        assert!(summary_pos < transcript_pos);
    }

    #[test]
    fn test_insert_summary_section_replace() {
        let content = r#"---
date: 2026-01-28
---

# Meeting Notes

## Summary

Old summary content.

## Transcript

[00:00:00] Test.
"#;

        let result = insert_summary_section(content, "New summary content.");

        // Old summary should be replaced
        assert!(!result.contains("Old summary content"));
        assert!(result.contains("New summary content."));
        assert!(result.contains("## Transcript"));
    }

    #[test]
    fn test_format_markdown_transcript_with_speakers() {
        let entries = vec![
            MarkdownTranscriptEntry {
                display_time: "[00:00:05]".to_string(),
                text: "First line.".to_string(),
                speaker: Some("Alice".to_string()),
            },
            MarkdownTranscriptEntry {
                display_time: "[00:00:15]".to_string(),
                text: "Second line.".to_string(),
                speaker: Some("Bob".to_string()),
            },
        ];

        let result = format_markdown_transcript(&entries);

        assert!(result.contains("[00:00:05] **Alice:** First line."));
        assert!(result.contains("[00:00:15] **Bob:** Second line."));
    }

    #[test]
    fn test_format_markdown_transcript_no_speakers() {
        let entries = vec![
            MarkdownTranscriptEntry {
                display_time: "[00:01:00]".to_string(),
                text: "Unattributed text.".to_string(),
                speaker: None,
            },
        ];

        let result = format_markdown_transcript(&entries);

        assert!(result.contains("[00:01:00] Unattributed text."));
        assert!(!result.contains("**")); // No bold formatting without speaker
    }
}

