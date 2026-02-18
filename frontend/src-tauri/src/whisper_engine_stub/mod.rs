//! Stub implementation of whisper_engine for Windows builds without CMake/LLVM
//!
//! This module provides no-op implementations that allow the app to compile
//! when Whisper is not available. Deepgram is the primary transcription provider
//! for Meeting Companion, so this stub is sufficient for the core workflow.

pub mod commands;
pub mod parallel_commands;

// Re-export types that other modules expect
pub use commands::*;
pub use parallel_commands::*;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Model status for transcription models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    Available,
    Missing,
    Downloading { progress: u8 },
    Error(String),
    Corrupted { file_size: u64, expected_min_size: u64 },
}

/// Information about a transcription model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub path: PathBuf,
    pub size_mb: u32,
    pub accuracy: String,
    pub speed: String,
    pub status: ModelStatus,
    pub description: String,
}

/// Stub WhisperEngine - does nothing but satisfies type requirements
pub struct WhisperEngine;

impl WhisperEngine {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub fn new_with_models_dir(_models_dir: Option<PathBuf>) -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub async fn discover_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(vec![])
    }

    pub async fn load_model(&self, _model_name: &str) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Whisper is not available on Windows. Use Deepgram for transcription."))
    }

    pub async fn unload_model(&self) -> bool {
        false
    }

    pub async fn get_current_model(&self) -> Option<String> {
        None
    }

    pub async fn is_model_loaded(&self) -> bool {
        false
    }

    pub async fn transcribe_audio(&self, _audio_data: Vec<f32>, _language: Option<String>) -> anyhow::Result<String> {
        Err(anyhow::anyhow!("Whisper is not available on Windows. Use Deepgram for transcription."))
    }

    /// Transcribe audio with confidence score and partial indicator
    pub async fn transcribe_audio_with_confidence(&self, _audio_data: Vec<f32>, _language: Option<String>) -> anyhow::Result<(String, f32, bool)> {
        Err(anyhow::anyhow!("Whisper is not available on Windows. Use Deepgram for transcription."))
    }

    pub async fn get_models_directory(&self) -> PathBuf {
        PathBuf::from(".")
    }

    pub async fn download_model(&self, _model_name: &str, _progress_callback: Option<Box<dyn Fn(u8) + Send + Sync>>) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Whisper is not available on Windows. Use Deepgram for transcription."))
    }

    pub async fn cancel_download(&self, _model_name: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn delete_model(&self, _model_name: &str) -> anyhow::Result<String> {
        Err(anyhow::anyhow!("Whisper is not available on Windows. Use Deepgram for transcription."))
    }
}
