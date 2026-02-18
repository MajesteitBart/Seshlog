//! Stub parallel processing commands for Whisper engine (Windows builds without CMake/LLVM)

use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Stub parallel configuration
#[derive(Clone, Debug)]
pub struct ParallelConfig {
    pub max_workers: usize,
    pub memory_budget_mb: u64,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            max_workers: 1,
            memory_budget_mb: 512,
        }
    }
}

/// Stub audio chunk representation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioChunk {
    pub id: u32,
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub start_time_ms: f64,
    pub duration_ms: f64,
}

/// Stub processing status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessingStatus {
    pub is_processing: bool,
    pub is_paused: bool,
    pub total_chunks: usize,
    pub processed_chunks: usize,
    pub failed_chunks: usize,
    pub current_worker_count: usize,
}

impl Default for ProcessingStatus {
    fn default() -> Self {
        Self {
            is_processing: false,
            is_paused: false,
            total_chunks: 0,
            processed_chunks: 0,
            failed_chunks: 0,
            current_worker_count: 0,
        }
    }
}

/// Stub resource status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceStatus {
    pub can_proceed: bool,
    pub warnings: Vec<String>,
}

impl ResourceStatus {
    pub fn get_primary_constraint(&self) -> Option<String> {
        self.warnings.first().cloned()
    }
}

/// Stub system resources
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemResources {
    pub cpu_usage_percent: f64,
    pub memory_used_percent: f64,
    pub cpu_cores: usize,
    pub available_memory_mb: u64,
}

/// Stub system monitor
pub struct SystemMonitor;

impl SystemMonitor {
    pub fn new() -> Self {
        Self
    }

    pub async fn refresh_system_info(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn get_current_resources(&self) -> anyhow::Result<SystemResources> {
        Ok(SystemResources {
            cpu_usage_percent: 0.0,
            memory_used_percent: 0.0,
            cpu_cores: 1,
            available_memory_mb: 1024,
        })
    }

    pub async fn check_resource_constraints(&self) -> anyhow::Result<ResourceStatus> {
        Ok(ResourceStatus {
            can_proceed: false,
            warnings: vec!["Whisper is not available on Windows. Use Deepgram for transcription.".to_string()],
        })
    }

    pub async fn calculate_safe_worker_count(&self) -> anyhow::Result<usize> {
        Ok(0)
    }
}

/// Stub parallel processor
pub struct ParallelProcessor;

impl ParallelProcessor {
    pub fn new(_config: ParallelConfig, _monitor: Arc<SystemMonitor>) -> anyhow::Result<(Self, tokio::sync::mpsc::Receiver<()>)> {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Ok((Self, rx))
    }

    pub async fn start_processing(&mut self, _chunks: Vec<AudioChunk>, _model_name: String) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("Whisper is not available on Windows. Use Deepgram for transcription."))
    }

    pub async fn pause_processing(&self) {}

    pub async fn resume_processing(&self) {}

    pub async fn stop_processing(&mut self) {}

    pub async fn get_processing_status(&self) -> ProcessingStatus {
        ProcessingStatus::default()
    }
}

/// Global state for parallel processor
pub struct ParallelProcessorState {
    pub processor: Arc<RwLock<Option<ParallelProcessor>>>,
    pub system_monitor: Arc<SystemMonitor>,
}

impl ParallelProcessorState {
    pub fn new() -> Self {
        Self {
            processor: Arc::new(RwLock::new(None)),
            system_monitor: Arc::new(SystemMonitor::new()),
        }
    }
}

#[tauri::command]
pub async fn initialize_parallel_processor(
    _state: State<'_, ParallelProcessorState>,
    _max_workers: Option<usize>,
    _memory_budget_mb: Option<u64>,
) -> Result<String, String> {
    Err("Whisper parallel processing is not available on Windows. Use Deepgram for transcription.".to_string())
}

#[tauri::command]
pub async fn start_parallel_processing(
    _state: State<'_, ParallelProcessorState>,
    _audio_chunks: Vec<serde_json::Value>,
    _model_name: String,
) -> Result<String, String> {
    Err("Whisper parallel processing is not available on Windows. Use Deepgram for transcription.".to_string())
}

#[tauri::command]
pub async fn pause_parallel_processing(
    _state: State<'_, ParallelProcessorState>,
) -> Result<String, String> {
    Ok("No processing to pause (Whisper disabled on Windows)".to_string())
}

#[tauri::command]
pub async fn resume_parallel_processing(
    _state: State<'_, ParallelProcessorState>,
) -> Result<String, String> {
    Ok("No processing to resume (Whisper disabled on Windows)".to_string())
}

#[tauri::command]
pub async fn stop_parallel_processing(
    _state: State<'_, ParallelProcessorState>,
) -> Result<String, String> {
    Ok("No processing to stop (Whisper disabled on Windows)".to_string())
}

#[tauri::command]
pub async fn get_parallel_processing_status(
    _state: State<'_, ParallelProcessorState>,
) -> Result<ProcessingStatus, String> {
    Ok(ProcessingStatus::default())
}

#[tauri::command]
pub async fn get_system_resources(
    state: State<'_, ParallelProcessorState>,
) -> Result<serde_json::Value, String> {
    let resources = state.system_monitor.get_current_resources()
        .await
        .map_err(|e| format!("Failed to get system resources: {}", e))?;

    serde_json::to_value(resources)
        .map_err(|e| format!("Failed to serialize resources: {}", e))
}

#[tauri::command]
pub async fn check_resource_constraints(
    state: State<'_, ParallelProcessorState>,
) -> Result<serde_json::Value, String> {
    let status = state.system_monitor.check_resource_constraints()
        .await
        .map_err(|e| format!("Failed to check resource constraints: {}", e))?;

    serde_json::to_value(status)
        .map_err(|e| format!("Failed to serialize resource status: {}", e))
}

#[tauri::command]
pub async fn calculate_optimal_workers(
    _state: State<'_, ParallelProcessorState>,
) -> Result<usize, String> {
    Ok(0)
}

#[tauri::command]
pub async fn prepare_audio_chunks(
    audio_data: Vec<f32>,
    sample_rate: u32,
    chunk_duration_ms: Option<f64>,
) -> Result<Vec<AudioChunk>, String> {
    let duration_ms = chunk_duration_ms.unwrap_or(30000.0);
    let samples_per_chunk = ((sample_rate as f64 * duration_ms) / 1000.0) as usize;

    let mut chunks = Vec::new();
    let mut chunk_id = 0;

    for (i, chunk_samples) in audio_data.chunks(samples_per_chunk).enumerate() {
        let start_time_ms = i as f64 * duration_ms;
        let actual_duration_ms = (chunk_samples.len() as f64 / sample_rate as f64) * 1000.0;

        let chunk = AudioChunk {
            id: chunk_id,
            data: chunk_samples.to_vec(),
            sample_rate,
            start_time_ms,
            duration_ms: actual_duration_ms,
        };

        chunks.push(chunk);
        chunk_id += 1;
    }

    Ok(chunks)
}

#[tauri::command]
pub async fn test_parallel_processing_setup(
    _state: State<'_, ParallelProcessorState>,
) -> Result<String, String> {
    Ok("❌ Whisper parallel processing is not available on Windows.\n\
        ✅ Deepgram cloud transcription is the primary provider for Meeting Companion.\n\
        💡 To enable Whisper, install CMake and LLVM:\n\
           winget install CMake.CMake\n\
           winget install LLVM.LLVM".to_string())
}
