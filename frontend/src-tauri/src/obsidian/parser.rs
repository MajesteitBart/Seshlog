//! Parser for Obsidian markdown meeting files
//!
//! Handles extraction of:
//! - YAML frontmatter between `---` markers
//! - Meeting title (first `# ` heading or filename)
//! - Configurable sections via section_labels.yaml
//!
//! Section labels are loaded from an embedded YAML config file,
//! supporting multiple languages and alternative labels.

use crate::obsidian::types::{MeetingFrontmatter, MeetingPrep};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

// Embed the section labels config at compile time
const SECTION_LABELS_YAML: &str = include_str!("section_labels.yaml");

/// Error type for parsing operations
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid frontmatter: {0}")]
    FrontmatterError(String),
    #[error("Missing required content")]
    MissingContent,
    #[error("Config error: {0}")]
    ConfigError(String),
}

// ============================================================================
// Configuration Types
// ============================================================================

/// Root configuration structure
#[derive(Debug, Deserialize)]
struct SectionConfig {
    sections: Vec<SectionDefinition>,
    #[serde(default)]
    frontmatter_aliases: HashMap<String, Vec<String>>,
}

/// Definition of a single section type
#[derive(Debug, Deserialize)]
struct SectionDefinition {
    field: String,
    levels: Vec<usize>,
    content_type: ContentType,
    labels: HashMap<String, Vec<String>>,
}

/// How to parse section content
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ContentType {
    Bullet,
    Numbered,
    Checkbox,
    Raw,
}

/// Compiled section matcher with all labels flattened
struct SectionMatcher {
    field: String,
    levels: Vec<usize>,
    content_type: ContentType,
    /// All labels as "## Label" or "### Label" patterns (lowercase for matching)
    patterns: Vec<String>,
}

/// Lazy-loaded configuration
static CONFIG: Lazy<Result<Vec<SectionMatcher>, String>> = Lazy::new(|| {
    load_and_compile_config()
});

fn load_and_compile_config() -> Result<Vec<SectionMatcher>, String> {
    let config: SectionConfig = serde_yaml::from_str(SECTION_LABELS_YAML)
        .map_err(|e| format!("Failed to parse section_labels.yaml: {}", e))?;

    let mut matchers = Vec::new();

    for section in config.sections {
        let mut patterns = Vec::new();

        // Generate all heading patterns for all labels at all levels
        for (_lang, labels) in &section.labels {
            for label in labels {
                for level in &section.levels {
                    let prefix = "#".repeat(*level);
                    // Store lowercase for case-insensitive matching
                    patterns.push(format!("{} {}", prefix, label.to_lowercase()));
                }
            }
        }

        matchers.push(SectionMatcher {
            field: section.field,
            levels: section.levels,
            content_type: section.content_type,
            patterns,
        });
    }

    Ok(matchers)
}

/// Get the section matchers, panicking if config failed to load
fn get_matchers() -> &'static Vec<SectionMatcher> {
    CONFIG.as_ref().expect("Failed to load section config")
}

/// Find a matcher by field name
fn get_matcher_for_field(field: &str) -> Option<&'static SectionMatcher> {
    get_matchers().iter().find(|m| m.field == field)
}

// ============================================================================
// Main Parser
// ============================================================================

/// Parse a meeting file from its path
pub fn parse_meeting_file(path: &Path, content: &str) -> Result<MeetingPrep, ParseError> {
    let mut prep = MeetingPrep::new(path.to_path_buf(), content.to_string());

    // Parse frontmatter
    prep.frontmatter = parse_frontmatter(content)?;

    // Parse title (first # heading after frontmatter, or derive from filename)
    prep.title = extract_title(content);
    if prep.title.is_empty() {
        // Derive title from filename (without extension)
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            prep.title = stem.replace('-', " ").replace('_', " ");
        }
    }

    // Parse sections using config
    let body = get_body_after_frontmatter(content);

    // Extract each configured section
    if let Some(matcher) = get_matcher_for_field("agenda") {
        prep.agenda = extract_section_items(&body, matcher);
    }
    if let Some(matcher) = get_matcher_for_field("notes") {
        prep.notes = extract_section_items(&body, matcher);
    }
    if let Some(matcher) = get_matcher_for_field("action_items") {
        prep.action_items = extract_section_items(&body, matcher);
    }
    if let Some(matcher) = get_matcher_for_field("goals") {
        prep.goals = extract_section_items(&body, matcher);
    }
    if let Some(matcher) = get_matcher_for_field("context") {
        let items = extract_section_items(&body, matcher);
        prep.context = items.join("\n");
    }

    Ok(prep)
}

/// Extract items from a section based on its matcher configuration
fn extract_section_items(body: &str, matcher: &SectionMatcher) -> Vec<String> {
    let section_content = match find_section_by_matcher(body, matcher) {
        Some(content) => content,
        None => return Vec::new(),
    };

    let mut items = Vec::new();

    for line in section_content.lines() {
        let trimmed = line.trim();

        match matcher.content_type {
            ContentType::Bullet => {
                // Try bullet first, then numbered
                if let Some(text) = extract_bullet_item(trimmed) {
                    if !text.is_empty() {
                        items.push(text);
                    }
                } else if let Some(text) = extract_numbered_item(trimmed) {
                    if !text.is_empty() {
                        items.push(text);
                    }
                }
            }
            ContentType::Numbered => {
                if let Some(text) = extract_numbered_item(trimmed) {
                    if !text.is_empty() {
                        items.push(text);
                    }
                }
            }
            ContentType::Checkbox => {
                if let Some(text) = extract_checkbox_item(trimmed) {
                    if !text.is_empty() {
                        // Preserve checkbox state
                        if trimmed.contains("[x]") || trimmed.contains("[X]") {
                            items.push(format!("[x] {}", text));
                        } else {
                            items.push(format!("[ ] {}", text));
                        }
                    }
                }
            }
            ContentType::Raw => {
                // For raw content, just collect all non-empty lines
                if !trimmed.is_empty() {
                    items.push(line.to_string());
                }
            }
        }
    }

    items
}

/// Find a section by checking against all patterns in the matcher
fn find_section_by_matcher(content: &str, matcher: &SectionMatcher) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_section = false;
    let mut section_level = 0;
    let mut section_lines: Vec<&str> = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        let trimmed_lower = trimmed.to_lowercase();

        // Check if this line starts the target section
        if !in_section {
            for pattern in &matcher.patterns {
                if trimmed_lower == *pattern {
                    in_section = true;
                    section_level = count_heading_level(trimmed);
                    break;
                }
            }
            continue;
        }

        // We're in the section - check for section end
        if is_heading(trimmed) {
            let level = count_heading_level(trimmed);
            // End if we hit a heading at the same or higher level
            if level <= section_level {
                break;
            }
        }

        section_lines.push(line);
    }

    if section_lines.is_empty() && !in_section {
        None
    } else {
        Some(section_lines.join("\n"))
    }
}

// ============================================================================
// Frontmatter Parsing
// ============================================================================

/// Extract YAML frontmatter from content between `---` markers
fn parse_frontmatter(content: &str) -> Result<MeetingFrontmatter, ParseError> {
    // Check if content starts with frontmatter marker
    let content = content.trim_start();
    if !content.starts_with("---") {
        // No frontmatter present, return defaults
        return Ok(MeetingFrontmatter::default());
    }

    // Find the closing marker
    let after_first_marker = &content[3..];
    let end_pos = after_first_marker.find("\n---");

    match end_pos {
        Some(pos) => {
            let yaml_content = after_first_marker[..pos].trim();
            if yaml_content.is_empty() {
                return Ok(MeetingFrontmatter::default());
            }

            // Parse YAML
            serde_yaml::from_str(yaml_content).map_err(|e| {
                ParseError::FrontmatterError(format!("YAML parse error: {}", e))
            })
        }
        None => {
            // No closing marker found, return defaults
            Ok(MeetingFrontmatter::default())
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Extract the meeting title from the first `# ` heading
fn extract_title(content: &str) -> String {
    // Skip frontmatter if present
    let body = get_body_after_frontmatter(content);

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return trimmed[2..].trim().to_string();
        }
    }

    String::new()
}

/// Get the content after the frontmatter
fn get_body_after_frontmatter(content: &str) -> &str {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return content;
    }

    // Find closing marker
    let after_first_marker = &content[3..];
    if let Some(pos) = after_first_marker.find("\n---") {
        // Return content after the closing marker and newline
        let after_closing = &after_first_marker[pos + 4..];
        after_closing.trim_start_matches('\n').trim_start_matches('\r')
    } else {
        content
    }
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

/// Extract text from a checkbox item
/// Matches: `- [ ] text`, `- [x] text`, `- [X] text`
fn extract_checkbox_item(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // Check for checkbox pattern
    if trimmed.starts_with("- [ ] ") {
        return Some(trimmed[6..].trim().to_string());
    }
    if trimmed.starts_with("- [x] ") || trimmed.starts_with("- [X] ") {
        return Some(trimmed[6..].trim().to_string());
    }
    // Also handle without space after checkbox
    if trimmed.starts_with("- [ ]") && trimmed.len() > 5 {
        return Some(trimmed[5..].trim().to_string());
    }
    if (trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]")) && trimmed.len() > 5 {
        return Some(trimmed[5..].trim().to_string());
    }

    None
}

/// Extract text from a bullet list item
/// Matches: `- text` (but not checkbox items)
fn extract_bullet_item(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // Must start with "- " but not be a checkbox
    if trimmed.starts_with("- ") && !trimmed.starts_with("- [") {
        return Some(trimmed[2..].trim().to_string());
    }

    None
}

/// Extract text from a numbered list item
/// Matches: `1. text`, `2. text`, etc.
fn extract_numbered_item(line: &str) -> Option<String> {
    let trimmed = line.trim();

    // Check for numbered list pattern: digits followed by . and space
    let mut chars = trimmed.chars().peekable();

    // Must start with a digit
    if !chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        return None;
    }

    // Consume all digits
    let mut pos = 0;
    while chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        chars.next();
        pos += 1;
    }

    // Must have a period
    if chars.next() != Some('.') {
        return None;
    }
    pos += 1;

    // Get the rest of the text (skip optional space)
    let rest = &trimmed[pos..];
    Some(rest.trim().to_string())
}

// ============================================================================
// Public Utility Functions
// ============================================================================

/// Get all supported labels for a field (useful for documentation/UI)
pub fn get_labels_for_field(field: &str) -> Vec<String> {
    if let Some(matcher) = get_matcher_for_field(field) {
        // Extract unique labels from patterns (remove heading prefixes)
        let mut labels: Vec<String> = matcher.patterns
            .iter()
            .map(|p| {
                // Remove "## " or "### " prefix
                p.trim_start_matches('#').trim().to_string()
            })
            .collect();
        labels.sort();
        labels.dedup();
        labels
    } else {
        Vec::new()
    }
}

/// Get all configured field names
pub fn get_configured_fields() -> Vec<String> {
    get_matchers().iter().map(|m| m.field.clone()).collect()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// New format sample (user's Obsidian template)
    const SAMPLE_FILE_NEW: &str = r#"---
tags: meeting
date: 2026-01-23
time_start: 2026-01-23T10:00:00
time_end:
attendees:
  - Bart van der Meeren
  - Attendee A
---

# Weekly Standup

## Agenda
- Review last week's progress
- Discuss blockers
- Plan for next week

## Notes
- Team is on track for deadline
- Need to coordinate with design team

## Action Items
- [ ] Send follow-up email
- [x] Update project board
- [ ] Schedule next meeting
"#;

    /// Legacy format sample (original Meetily format)
    const SAMPLE_FILE_LEGACY: &str = r#"---
date: 2026-01-22
type: meeting
attendees: [Attendee A, Attendee B]
status: scheduled
---

# Meeting: Project Kickoff

## Prep
### Goals
- [ ] Align on timeline
- [x] Review budget
- [ ] Assign workstreams

### Agenda
1. Introductions (5 min)
2. Scope review (15 min)
3. Q&A (10 min)

### Context
Background from previous conversations...
This is multiline context.
"#;

    /// Dutch format sample
    const SAMPLE_FILE_DUTCH: &str = r#"---
date: 2026-01-23
attendees:
  - Attendee B
---

# Weekelijkse Vergadering

## Agendapunten
- Voortgang bespreken
- Planning volgende week

## Notities
- Alles op schema
- Meer resources nodig

## Actiepunten
- [ ] Email versturen
- [x] Document bijwerken
"#;

    // ===== Config Loading Tests =====

    #[test]
    fn test_config_loads_successfully() {
        let matchers = get_matchers();
        assert!(!matchers.is_empty(), "Should have loaded section matchers");
    }

    #[test]
    fn test_config_has_expected_fields() {
        let fields = get_configured_fields();
        assert!(fields.contains(&"agenda".to_string()));
        assert!(fields.contains(&"notes".to_string()));
        assert!(fields.contains(&"action_items".to_string()));
        assert!(fields.contains(&"goals".to_string()));
        assert!(fields.contains(&"context".to_string()));
    }

    #[test]
    fn test_get_labels_for_field() {
        let labels = get_labels_for_field("agenda");
        assert!(!labels.is_empty());
        // Should contain various language labels
        assert!(labels.iter().any(|l| l.contains("agenda")));
    }

    // ===== New Format Tests =====

    #[test]
    fn test_parse_frontmatter_new_format() {
        let fm = parse_frontmatter(SAMPLE_FILE_NEW).unwrap();
        assert_eq!(fm.date, Some("2026-01-23".to_string()));
        assert_eq!(fm.time_start, Some("2026-01-23T10:00:00".to_string()));
        assert!(fm.time_end.is_none());
        assert_eq!(fm.attendees, Some(vec!["Bart van der Meeren".to_string(), "Attendee A".to_string()]));
    }

    #[test]
    fn test_extract_title_new_format() {
        let title = extract_title(SAMPLE_FILE_NEW);
        assert_eq!(title, "Weekly Standup");
    }

    #[test]
    fn test_extract_agenda_new_format() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_NEW).unwrap();
        assert_eq!(prep.agenda.len(), 3);
        assert_eq!(prep.agenda[0], "Review last week's progress");
        assert_eq!(prep.agenda[1], "Discuss blockers");
        assert_eq!(prep.agenda[2], "Plan for next week");
    }

    #[test]
    fn test_extract_notes_new_format() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_NEW).unwrap();
        assert_eq!(prep.notes.len(), 2);
        assert_eq!(prep.notes[0], "Team is on track for deadline");
        assert_eq!(prep.notes[1], "Need to coordinate with design team");
    }

    #[test]
    fn test_extract_action_items_new_format() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_NEW).unwrap();
        assert_eq!(prep.action_items.len(), 3);
        assert_eq!(prep.action_items[0], "[ ] Send follow-up email");
        assert_eq!(prep.action_items[1], "[x] Update project board");
        assert_eq!(prep.action_items[2], "[ ] Schedule next meeting");
    }

    // ===== Legacy Format Tests =====

    #[test]
    fn test_parse_frontmatter_legacy() {
        let fm = parse_frontmatter(SAMPLE_FILE_LEGACY).unwrap();
        assert_eq!(fm.date, Some("2026-01-22".to_string()));
        assert_eq!(fm.meeting_type, Some("meeting".to_string()));
        assert_eq!(fm.attendees, Some(vec!["John".to_string(), "Sarah".to_string()]));
        assert_eq!(fm.status, Some("scheduled".to_string()));
    }

    #[test]
    fn test_extract_goals_legacy() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_LEGACY).unwrap();
        assert_eq!(prep.goals.len(), 3);
        assert!(prep.goals[0].contains("Align on timeline"));
        assert!(prep.goals[1].contains("Review budget"));
        assert!(prep.goals[2].contains("Assign workstreams"));
    }

    #[test]
    fn test_extract_context_legacy() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_LEGACY).unwrap();
        assert!(prep.context.contains("Background from previous conversations"));
        assert!(prep.context.contains("multiline context"));
    }

    // ===== Dutch Format Tests =====

    #[test]
    fn test_parse_dutch_agenda() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_DUTCH).unwrap();
        assert_eq!(prep.agenda.len(), 2);
        assert_eq!(prep.agenda[0], "Voortgang bespreken");
        assert_eq!(prep.agenda[1], "Planning volgende week");
    }

    #[test]
    fn test_parse_dutch_notes() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_DUTCH).unwrap();
        assert_eq!(prep.notes.len(), 2);
        assert_eq!(prep.notes[0], "Alles op schema");
        assert_eq!(prep.notes[1], "Meer resources nodig");
    }

    #[test]
    fn test_parse_dutch_action_items() {
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, SAMPLE_FILE_DUTCH).unwrap();
        assert_eq!(prep.action_items.len(), 2);
        assert!(prep.action_items[0].contains("Email versturen"));
        assert!(prep.action_items[1].contains("Document bijwerken"));
    }

    // ===== Edge Case Tests =====

    #[test]
    fn test_parse_frontmatter_missing() {
        let content = "# Just a title\nSome content";
        let fm = parse_frontmatter(content).unwrap();
        assert!(fm.date.is_none());
        assert!(fm.meeting_type.is_none());
    }

    #[test]
    fn test_parse_frontmatter_empty() {
        let content = "---\n---\n# Title";
        let fm = parse_frontmatter(content).unwrap();
        assert!(fm.date.is_none());
    }

    #[test]
    fn test_title_from_filename() {
        let content = "---\ndate: 2026-01-22\n---\n";
        let path = Path::new("weekly-standup-notes.md");
        let prep = parse_meeting_file(path, content).unwrap();
        assert_eq!(prep.title, "weekly standup notes");
    }

    #[test]
    fn test_case_insensitive_matching() {
        // Test that "## AGENDA" matches the same as "## Agenda"
        let content = r#"---
date: 2026-01-23
---

# Test Meeting

## AGENDA
- Item 1
- Item 2
"#;
        let path = Path::new("test.md");
        let prep = parse_meeting_file(path, content).unwrap();
        assert_eq!(prep.agenda.len(), 2);
    }

    #[test]
    fn test_minimal_file() {
        let content = r#"---
date: 2026-01-22
---

# Quick Meeting
"#;
        let path = Path::new("minimal.md");
        let prep = parse_meeting_file(path, content).unwrap();

        assert_eq!(prep.title, "Quick Meeting");
        assert!(prep.goals.is_empty());
        assert!(prep.agenda.is_empty());
        assert!(prep.notes.is_empty());
        assert!(prep.action_items.is_empty());
        assert!(prep.context.is_empty());
        assert_eq!(prep.frontmatter.date, Some("2026-01-22".to_string()));
    }

    // ===== Helper Function Tests =====

    #[test]
    fn test_checkbox_extraction() {
        assert_eq!(extract_checkbox_item("- [ ] Task 1"), Some("Task 1".to_string()));
        assert_eq!(extract_checkbox_item("- [x] Done task"), Some("Done task".to_string()));
        assert_eq!(extract_checkbox_item("- [X] Done task"), Some("Done task".to_string()));
        assert_eq!(extract_checkbox_item("Regular text"), None);
        assert_eq!(extract_checkbox_item("- Just a bullet"), None);
    }

    #[test]
    fn test_bullet_extraction() {
        assert_eq!(extract_bullet_item("- Simple item"), Some("Simple item".to_string()));
        assert_eq!(extract_bullet_item("- Another item"), Some("Another item".to_string()));
        assert_eq!(extract_bullet_item("- [ ] Checkbox"), None); // Checkboxes excluded
        assert_eq!(extract_bullet_item("Regular text"), None);
        assert_eq!(extract_bullet_item("1. Numbered"), None);
    }

    #[test]
    fn test_numbered_item_extraction() {
        assert_eq!(extract_numbered_item("1. First item"), Some("First item".to_string()));
        assert_eq!(extract_numbered_item("10. Tenth item"), Some("Tenth item".to_string()));
        assert_eq!(extract_numbered_item("Regular text"), None);
        assert_eq!(extract_numbered_item("- Bullet point"), None);
    }
}

