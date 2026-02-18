use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Parsed meeting preparation data from an Obsidian markdown file
/// Supports the standard Obsidian meeting template format:
/// ```markdown
/// ## Agenda
/// - Item 1
///
/// ## Notes
/// - Note 1
///
/// ## Action Items
/// - [ ] Task 1
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingPrep {
    /// Path to the source markdown file
    pub file_path: PathBuf,
    /// Parsed YAML frontmatter
    pub frontmatter: MeetingFrontmatter,
    /// Meeting title (from first # heading or filename)
    pub title: String,
    /// Agenda items (from ## Agenda section, bullet list)
    pub agenda: Vec<String>,
    /// Notes content (from ## Notes section, bullet list or raw markdown)
    pub notes: Vec<String>,
    /// Action items (from ## Action Items section, checkbox items)
    pub action_items: Vec<String>,
    /// Goal items (from ### Goals section, checkbox items) - legacy support
    pub goals: Vec<String>,
    /// Context content (from ### Context section, raw markdown) - legacy support
    pub context: String,
    /// Raw file content (preserved for later merge operations)
    pub raw_content: String,
}

/// YAML frontmatter parsed from the meeting file
/// Supports the standard Obsidian meeting template format:
/// ```yaml
/// tags: meeting
/// date: 2026-01-23
/// time_start: 2026-01-23T10:00:00
/// time_end:
/// attendees:
///   - Person Name
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingFrontmatter {
    /// Tags for the meeting (can be string or array)
    pub tags: Option<StringOrVec>,
    /// Meeting date (e.g., "2026-01-22")
    pub date: Option<String>,
    /// Meeting start time (ISO format, e.g., "2026-01-23T10:00:00")
    pub time_start: Option<String>,
    /// Meeting end time (ISO format)
    pub time_end: Option<String>,
    /// Meeting type (from "type" field in YAML) - legacy support
    #[serde(rename = "type", alias = "meeting_type")]
    pub meeting_type: Option<String>,
    /// List of attendees
    pub attendees: Option<Vec<String>>,
    /// Meeting status (e.g., "scheduled", "completed")
    pub status: Option<String>,
}

/// Helper type to handle tags that can be either a string or array
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

impl StringOrVec {
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            StringOrVec::Single(s) => vec![s.clone()],
            StringOrVec::Multiple(v) => v.clone(),
        }
    }
}

impl MeetingPrep {
    /// Create a new MeetingPrep with default/empty values
    pub fn new(file_path: PathBuf, raw_content: String) -> Self {
        Self {
            file_path,
            frontmatter: MeetingFrontmatter::default(),
            title: String::new(),
            agenda: Vec::new(),
            notes: Vec::new(),
            action_items: Vec::new(),
            goals: Vec::new(),
            context: String::new(),
            raw_content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meeting_prep_new() {
        let prep = MeetingPrep::new(PathBuf::from("test.md"), "# Test".to_string());
        assert_eq!(prep.file_path, PathBuf::from("test.md"));
        assert_eq!(prep.raw_content, "# Test");
        assert!(prep.agenda.is_empty());
        assert!(prep.notes.is_empty());
        assert!(prep.action_items.is_empty());
        assert!(prep.goals.is_empty());
        assert!(prep.context.is_empty());
    }

    #[test]
    fn test_frontmatter_default() {
        let fm = MeetingFrontmatter::default();
        assert!(fm.tags.is_none());
        assert!(fm.date.is_none());
        assert!(fm.time_start.is_none());
        assert!(fm.time_end.is_none());
        assert!(fm.meeting_type.is_none());
        assert!(fm.attendees.is_none());
        assert!(fm.status.is_none());
    }

    #[test]
    fn test_string_or_vec() {
        let single = StringOrVec::Single("meeting".to_string());
        assert_eq!(single.as_vec(), vec!["meeting".to_string()]);

        let multiple = StringOrVec::Multiple(vec!["meeting".to_string(), "important".to_string()]);
        assert_eq!(multiple.as_vec(), vec!["meeting".to_string(), "important".to_string()]);
    }
}
