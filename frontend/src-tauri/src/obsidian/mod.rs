//! Obsidian file handling module
//!
//! Provides parsing and writing capabilities for Obsidian markdown meeting files.
//! This module enables the app to:
//! - Load meeting prep content from `.md` files
//! - Parse YAML frontmatter and structured sections
//! - Write transcripts back to the source file

pub mod parser;
pub mod types;
pub mod writer;

pub use parser::{parse_meeting_file, ParseError};
pub use types::{MeetingFrontmatter, MeetingPrep};
pub use writer::{
    MarkdownTranscriptEntry, MeetingMarkdownData, SaveTranscriptRequest, TranscriptSegment,
    generate_meeting_markdown, save_meeting_markdown, save_meeting_markdown_to_folder,
    update_meeting_markdown_with_summary,
};

/// Tauri command to open and parse a meeting file
///
/// # Arguments
/// * `path` - Path to the markdown file to open
///
/// # Returns
/// * `Ok(MeetingPrep)` - Parsed meeting preparation data
/// * `Err(String)` - Error message if parsing fails
#[tauri::command]
pub async fn open_meeting_file(path: String) -> Result<MeetingPrep, String> {
    log::info!("Opening meeting file: {}", path);

    let path_buf = std::path::PathBuf::from(&path);

    // Verify file exists
    if !path_buf.exists() {
        return Err(format!("File not found: {}", path));
    }

    // Verify it's a markdown file
    match path_buf.extension() {
        Some(ext) if ext == "md" || ext == "markdown" => {}
        _ => {
            return Err("File must be a markdown file (.md or .markdown)".to_string());
        }
    }

    // Read file content
    let content = tokio::fs::read_to_string(&path_buf)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse the file
    parse_meeting_file(&path_buf, &content)
        .map_err(|e| format!("Failed to parse meeting file: {}", e))
}

/// Tauri command to save transcript to a meeting file
///
/// # Arguments
/// * `request` - Contains file path, transcript segments, and options
///
/// # Returns
/// * `Ok(())` - Success
/// * `Err(String)` - Error message if save fails
#[tauri::command]
pub async fn save_meeting_transcript(request: SaveTranscriptRequest) -> Result<(), String> {
    log::info!("Saving transcript to: {}", request.file_path);

    let path = std::path::Path::new(&request.file_path);

    // Verify file exists
    if !path.exists() {
        return Err(format!("File not found: {}", request.file_path));
    }

    // Verify it's a markdown file
    match path.extension() {
        Some(ext) if ext == "md" || ext == "markdown" => {}
        _ => {
            return Err("File must be a markdown file (.md or .markdown)".to_string());
        }
    }

    writer::save_transcript(&request)
        .await
        .map_err(|e| format!("Failed to save transcript: {}", e))
}

/// Tauri command to save meeting markdown to a folder
///
/// # Arguments
/// * `folder` - Path to the folder where the markdown should be saved
/// * `filename` - Name for the markdown file (without .md extension)
/// * `data` - Meeting data including segments and optional summary
///
/// # Returns
/// * `Ok(String)` - Path to the created markdown file
/// * `Err(String)` - Error message if save fails
#[tauri::command]
pub async fn save_meeting_markdown_file(
    folder: String,
    filename: String,
    data: MeetingMarkdownData,
) -> Result<String, String> {
    log::info!("Saving meeting markdown to folder: {}", folder);

    let folder_path = std::path::Path::new(&folder);

    // Verify folder exists
    if !folder_path.exists() {
        return Err(format!("Folder not found: {}", folder));
    }

    save_meeting_markdown_to_folder(folder_path, &filename, &data)
        .await
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Failed to save meeting markdown: {}", e))
}

/// Find the markdown file in a meeting folder
///
/// Looks for a .md file in the folder, excluding common system files
fn find_markdown_file_in_folder(folder_path: &std::path::Path) -> Option<std::path::PathBuf> {
    if !folder_path.is_dir() {
        return None;
    }

    let entries = std::fs::read_dir(folder_path).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            // Skip common system/hidden files
            let filename = path.file_name()?.to_str()?;
            if !filename.starts_with('.') && !filename.starts_with('_') {
                return Some(path);
            }
        }
    }
    None
}

/// Tauri command to update existing meeting markdown with summary
///
/// # Arguments
/// * `path` - Path to the existing markdown file OR folder containing the markdown file
/// * `summary` - Summary markdown content to insert
///
/// # Returns
/// * `Ok(())` - Success
/// * `Err(String)` - Error message if update fails
#[tauri::command]
pub async fn update_meeting_summary(path: String, summary: String) -> Result<(), String> {
    log::info!("Updating meeting markdown with summary: {}", path);

    let input_path = std::path::Path::new(&path);

    // Determine the actual markdown file path
    let md_path = if input_path.is_dir() {
        // Path is a folder - find the .md file in it
        find_markdown_file_in_folder(input_path)
            .ok_or_else(|| format!("No markdown file found in folder: {}", path))?
    } else if input_path.exists() {
        // Path is an existing file
        input_path.to_path_buf()
    } else {
        // Path doesn't exist - maybe it's a file path but wrong filename
        // Try to find .md in parent folder
        if let Some(parent) = input_path.parent() {
            if parent.is_dir() {
                log::warn!("File not found at {}, searching parent folder for .md file", path);
                find_markdown_file_in_folder(parent)
                    .ok_or_else(|| format!("File not found: {} (also searched parent folder)", path))?
            } else {
                return Err(format!("File not found: {}", path));
            }
        } else {
            return Err(format!("File not found: {}", path));
        }
    };

    log::info!("Found markdown file: {}", md_path.display());

    update_meeting_markdown_with_summary(&md_path, &summary)
        .await
        .map_err(|e| format!("Failed to update summary: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify types are exported
        let _fm: MeetingFrontmatter = MeetingFrontmatter::default();
        let _prep: MeetingPrep =
            MeetingPrep::new(std::path::PathBuf::from("test.md"), "# Test".to_string());
    }
}
