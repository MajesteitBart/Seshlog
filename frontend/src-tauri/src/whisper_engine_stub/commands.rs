//! Stub Tauri commands for Whisper engine (Windows builds without CMake/LLVM)

use crate::whisper_engine::{ModelInfo, WhisperEngine};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::Arc;
use tauri::{command, AppHandle, Manager, Runtime};

// Global whisper engine (stub)
pub static WHISPER_ENGINE: Mutex<Option<Arc<WhisperEngine>>> = Mutex::new(None);

// Global models directory path
static MODELS_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Initialize the models directory path using app_data_dir
pub fn set_models_directory<R: Runtime>(app: &AppHandle<R>) {
    let app_data_dir = app.path().app_data_dir()
        .expect("Failed to get app data dir");

    let models_dir = app_data_dir.join("models");

    // Create directory if it doesn't exist
    if !models_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&models_dir) {
            log::error!("Failed to create models directory: {}", e);
            return;
        }
    }

    log::info!("Models directory set to: {} (Whisper disabled on Windows)", models_dir.display());

    let mut guard = MODELS_DIR.lock().unwrap();
    *guard = Some(models_dir);
}

/// Get the configured models directory
fn get_models_directory() -> Option<PathBuf> {
    MODELS_DIR.lock().unwrap().clone()
}

#[command]
pub async fn whisper_init() -> Result<(), String> {
    log::info!("Whisper stub initialized (Whisper disabled on Windows - use Deepgram)");
    let mut guard = WHISPER_ENGINE.lock().unwrap();
    *guard = Some(Arc::new(WhisperEngine));
    Ok(())
}

#[command]
pub async fn whisper_get_available_models() -> Result<Vec<ModelInfo>, String> {
    // Return empty list - Whisper not available on Windows
    Ok(vec![])
}

#[command]
pub async fn whisper_load_model(
    _app_handle: tauri::AppHandle,
    _model_name: String
) -> Result<(), String> {
    Err("Whisper is not available on Windows. Please use Deepgram for transcription.".to_string())
}

#[command]
pub async fn whisper_get_current_model() -> Result<Option<String>, String> {
    Ok(None)
}

#[command]
pub async fn whisper_is_model_loaded() -> Result<bool, String> {
    Ok(false)
}

#[command]
pub async fn whisper_has_available_models() -> Result<bool, String> {
    Ok(false)
}

#[command]
pub async fn whisper_validate_model_ready() -> Result<String, String> {
    Err("Whisper is not available on Windows. Please use Deepgram for transcription.".to_string())
}

/// Internal version that respects user's transcript config
pub async fn whisper_validate_model_ready_with_config<R: tauri::Runtime>(
    _app: &tauri::AppHandle<R>,
) -> Result<String, String> {
    Err("Whisper is not available on Windows. Please use Deepgram for transcription.".to_string())
}

#[command]
pub async fn whisper_transcribe_audio(_audio_data: Vec<f32>) -> Result<String, String> {
    Err("Whisper is not available on Windows. Please use Deepgram for transcription.".to_string())
}

#[command]
pub async fn whisper_get_models_directory() -> Result<String, String> {
    if let Some(dir) = get_models_directory() {
        Ok(dir.to_string_lossy().to_string())
    } else {
        Err("Models directory not initialized".to_string())
    }
}

#[command]
pub async fn whisper_download_model(
    _app_handle: tauri::AppHandle,
    _model_name: String,
) -> Result<(), String> {
    Err("Whisper is not available on Windows. Please use Deepgram for transcription.".to_string())
}

#[command]
pub async fn whisper_cancel_download(_model_name: String) -> Result<(), String> {
    Ok(())
}

#[command]
pub async fn whisper_delete_corrupted_model(_model_name: String) -> Result<String, String> {
    Err("Whisper is not available on Windows.".to_string())
}

/// Open the models folder in the system file explorer
#[command]
pub async fn open_models_folder() -> Result<(), String> {
    let models_dir = get_models_directory()
        .ok_or_else(|| "Models directory not initialized".to_string())?;

    // Ensure directory exists before trying to open it
    if !models_dir.exists() {
        std::fs::create_dir_all(&models_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let folder_path = models_dir.to_string_lossy().to_string();

    std::process::Command::new("explorer")
        .arg(&folder_path)
        .spawn()
        .map_err(|e| format!("Failed to open folder: {}", e))?;

    log::info!("Opened models folder: {}", folder_path);
    Ok(())
}
