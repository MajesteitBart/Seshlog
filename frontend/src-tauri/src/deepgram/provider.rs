// deepgram/provider.rs
//
// Deepgram transcription provider implementation.
// Implements the TranscriptionProvider trait for streaming cloud-based transcription
// with speaker diarization support.

use super::websocket::{DeepgramConfig, DeepgramWebSocket, TranscriptionSegment};
use crate::audio::transcription::provider::{TranscriptionError, TranscriptionProvider, TranscriptResult};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{timeout, Duration};

// ============================================================================
// DEEPGRAM PROVIDER
// ============================================================================

/// Deepgram cloud transcription provider
/// Provides real-time streaming transcription with speaker diarization
pub struct DeepgramProvider {
    /// API key for Deepgram authentication
    api_key: String,
    /// Model to use for transcription (e.g., "nova-3", "nova-2-meeting")
    model: String,
    /// WebSocket connection (when streaming is active)
    websocket: Arc<RwLock<Option<DeepgramWebSocket>>>,
    /// Buffer for accumulating transcription results
    result_buffer: Arc<Mutex<Vec<TranscriptionSegment>>>,
    /// Whether a streaming session is active
    is_streaming: Arc<RwLock<bool>>,
}

impl DeepgramProvider {
    /// Create a new DeepgramProvider with the given API key
    /// Default model is nova-2-meeting which supports streaming diarization
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "nova-2-meeting".to_string(),
            websocket: Arc::new(RwLock::new(None)),
            result_buffer: Arc::new(Mutex::new(Vec::new())),
            is_streaming: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new DeepgramProvider with a specific model
    pub fn with_model(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            websocket: Arc::new(RwLock::new(None)),
            result_buffer: Arc::new(Mutex::new(Vec::new())),
            is_streaming: Arc::new(RwLock::new(false)),
        }
    }

    /// Start a streaming transcription session
    /// Returns a receiver for transcription segments
    pub async fn start_streaming(
        &self,
        language: Option<String>,
    ) -> Result<tokio::sync::mpsc::Receiver<TranscriptionSegment>, String> {
        // Check if already streaming
        {
            let is_streaming = self.is_streaming.read().await;
            if *is_streaming {
                return Err("Already streaming".to_string());
            }
        }

        let config = DeepgramConfig {
            api_key: self.api_key.clone(),
            model: self.model.clone(),
            language,
            sample_rate: 16000,
            channels: 1,
            encoding: "linear16".to_string(),
            diarize: true,
            punctuate: true,
            interim_results: true,
            smart_format: true,
        };

        let websocket = DeepgramWebSocket::new(config);
        let receiver = websocket.connect().await?;

        // Store the websocket connection
        {
            let mut ws = self.websocket.write().await;
            *ws = Some(websocket);
        }

        // Mark as streaming
        {
            let mut is_streaming = self.is_streaming.write().await;
            *is_streaming = true;
        }

        info!("Deepgram streaming session started");
        Ok(receiver)
    }

    /// Send audio data during a streaming session
    pub async fn send_audio_stream(&self, audio: &[f32]) -> Result<(), String> {
        let ws = self.websocket.read().await;

        if let Some(ref websocket) = *ws {
            websocket.send_audio(audio).await
        } else {
            Err("Not connected - call start_streaming first".to_string())
        }
    }

    /// Stop the streaming session
    pub async fn stop_streaming(&self) {
        // Disconnect websocket
        {
            let mut ws = self.websocket.write().await;
            if let Some(ref websocket) = *ws {
                websocket.disconnect().await;
            }
            *ws = None;
        }

        // Mark as not streaming
        {
            let mut is_streaming = self.is_streaming.write().await;
            *is_streaming = false;
        }

        // Clear the buffer
        {
            let mut buffer = self.result_buffer.lock().await;
            buffer.clear();
        }

        info!("Deepgram streaming session stopped");
    }

    /// Check if currently streaming
    pub async fn is_streaming(&self) -> bool {
        *self.is_streaming.read().await
    }

    /// Get the appropriate model for a given language
    /// Specialized models (nova-2-meeting, nova-2-phonecall, etc.) only support English
    /// For non-English languages, use the general nova-2 model which supports 30+ languages
    fn get_model_for_language(&self, language: Option<&str>) -> String {
        // Check if language is English or not specified (defaults to English)
        let is_english = match language {
            None => true, // No language = auto-detect, which works with any model
            Some(lang) => {
                let lang_lower = lang.to_lowercase();
                lang_lower.starts_with("en") || lang_lower == "multi"
            }
        };

        // If English or multi, use the configured model
        if is_english {
            return self.model.clone();
        }

        // For non-English languages, check if model is a specialized English-only model
        // These models only support English: nova-2-meeting, nova-2-phonecall,
        // nova-2-conversationalai, nova-2-voicemail, nova-2-video, nova-2-medical, nova-2-finance
        let english_only_models = [
            "nova-2-meeting",
            "nova-2-phonecall",
            "nova-2-conversationalai",
            "nova-2-voicemail",
            "nova-2-video",
            "nova-2-medical",
            "nova-2-finance",
            "nova-3",
            "nova-3-medical",
        ];

        if english_only_models.iter().any(|m| self.model == *m) {
            // Fall back to nova-2 general which supports multiple languages
            "nova-2".to_string()
        } else {
            self.model.clone()
        }
    }

    /// Perform a single-shot transcription of audio data
    /// This is a convenience method that starts a stream, sends audio, and waits for results
    async fn transcribe_single_shot(
        &self,
        audio: Vec<f32>,
        language: Option<String>,
    ) -> Result<TranscriptResult, TranscriptionError> {
        // Validate audio length - Deepgram needs at least some audio
        if audio.len() < 1600 {
            // Less than 0.1 seconds at 16kHz
            return Err(TranscriptionError::AudioTooShort {
                samples: audio.len(),
                minimum: 1600,
            });
        }

        // Determine the model to use based on language
        // Specialized models like nova-2-meeting, nova-2-phonecall etc. only support English
        // For non-English languages, fall back to nova-2 which supports 30+ languages
        let effective_model = self.get_model_for_language(language.as_deref());
        if effective_model != self.model {
            info!(
                "Using '{}' instead of '{}' for language '{}'",
                effective_model,
                self.model,
                language.as_deref().unwrap_or("auto")
            );
        }

        // Create a temporary config and websocket for this transcription
        let config = DeepgramConfig {
            api_key: self.api_key.clone(),
            model: effective_model,
            language,
            sample_rate: 16000,
            channels: 1,
            encoding: "linear16".to_string(),
            diarize: true,
            punctuate: true,
            interim_results: false, // For single-shot, we only want final results
            smart_format: true,
        };

        let websocket = DeepgramWebSocket::new(config);

        // Connect and get receiver
        let mut receiver = websocket.connect().await.map_err(|e| {
            error!("Failed to connect to Deepgram: {}", e);
            TranscriptionError::EngineFailed(e)
        })?;

        // Send audio
        websocket.send_audio(&audio).await.map_err(|e| {
            error!("Failed to send audio to Deepgram: {}", e);
            TranscriptionError::EngineFailed(e)
        })?;

        // Signal end of audio stream by sending Deepgram's CloseStream message
        // This tells Deepgram we're done sending audio but want to receive final results
        debug!("Signaling end of audio stream to Deepgram");
        websocket.signal_end_of_audio().await;

        // Wait for results with timeout
        // The receiver will close when Deepgram sends its Close frame back
        let mut final_text = String::new();
        let mut confidence: Option<f32> = None;
        let mut is_partial = true;
        let mut segment_count = 0;

        // Collect results with a timeout (10 seconds should be plenty for Deepgram)
        debug!("Waiting for transcription results from Deepgram...");
        let result = timeout(Duration::from_secs(10), async {
            while let Some(segment) = receiver.recv().await {
                segment_count += 1;
                debug!(
                    "Received segment {}: is_final={}, text='{}', confidence={:?}",
                    segment_count, segment.is_final, segment.text, segment.confidence
                );

                if segment.is_final {
                    // Build the final text with speaker labels if available
                    if !segment.speakers.is_empty() {
                        for speaker_seg in &segment.speakers {
                            if !final_text.is_empty() {
                                final_text.push('\n');
                            }
                            final_text.push_str(&format!(
                                "[Speaker {}]: {}",
                                speaker_seg.speaker_id, speaker_seg.text
                            ));
                        }
                    } else {
                        if !final_text.is_empty() {
                            final_text.push(' ');
                        }
                        final_text.push_str(&segment.text);
                    }

                    confidence = segment.confidence;
                    is_partial = false;
                }
            }
            debug!("Receiver channel closed after {} segments", segment_count);
        })
        .await;

        // Now disconnect to clean up
        websocket.disconnect().await;

        if result.is_err() {
            warn!("Transcription timed out after 10 seconds");
            // If we got some results before timeout, use them
            if !final_text.is_empty() {
                return Ok(TranscriptResult {
                    text: final_text.trim().to_string(),
                    confidence,
                    is_partial: true, // Mark as partial since we timed out
                });
            }
        }

        // If we didn't get any final results, return what we have
        if final_text.is_empty() {
            return Err(TranscriptionError::EngineFailed(
                "No transcription results received".to_string(),
            ));
        }

        Ok(TranscriptResult {
            text: final_text.trim().to_string(),
            confidence,
            is_partial,
        })
    }
}

#[async_trait]
impl TranscriptionProvider for DeepgramProvider {
    /// Transcribe audio samples to text
    ///
    /// For Deepgram, this performs a single-shot transcription:
    /// 1. Connects to Deepgram WebSocket
    /// 2. Sends all audio data
    /// 3. Waits for final transcription result
    /// 4. Returns the combined result with speaker labels
    async fn transcribe(
        &self,
        audio: Vec<f32>,
        language: Option<String>,
    ) -> std::result::Result<TranscriptResult, TranscriptionError> {
        debug!(
            "DeepgramProvider::transcribe called with {} samples, language: {:?}",
            audio.len(),
            language
        );

        // Check for empty API key
        if self.api_key.is_empty() {
            return Err(TranscriptionError::EngineFailed(
                "Deepgram API key not configured".to_string(),
            ));
        }

        // Perform single-shot transcription
        self.transcribe_single_shot(audio, language).await
    }

    /// Deepgram is a cloud service - always "loaded"
    async fn is_model_loaded(&self) -> bool {
        // Deepgram is cloud-based, so model is always "loaded"
        // We just need a valid API key
        !self.api_key.is_empty()
    }

    /// Get the current model name
    async fn get_current_model(&self) -> Option<String> {
        Some(self.model.clone())
    }

    /// Get the provider name
    fn provider_name(&self) -> &'static str {
        "Deepgram"
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Format a transcription segment with speaker labels for display
pub fn format_segment_with_speakers(segment: &TranscriptionSegment) -> String {
    if segment.speakers.is_empty() {
        segment.text.clone()
    } else {
        segment
            .speakers
            .iter()
            .map(|s| format!("[Speaker {}]: {}", s.speaker_id, s.text))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Format timestamp in HH:MM:SS format
pub fn format_timestamp(seconds: f64) -> String {
    let total_secs = seconds as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(0.0), "00:00");
        assert_eq!(format_timestamp(30.0), "00:30");
        assert_eq!(format_timestamp(90.0), "01:30");
        assert_eq!(format_timestamp(3661.0), "01:01:01");
    }

    #[test]
    fn test_provider_name() {
        let provider = DeepgramProvider::new("test_key".to_string());
        assert_eq!(provider.provider_name(), "Deepgram");
    }

    #[tokio::test]
    async fn test_is_model_loaded_with_key() {
        let provider = DeepgramProvider::new("test_key".to_string());
        assert!(provider.is_model_loaded().await);
    }

    #[tokio::test]
    async fn test_is_model_loaded_without_key() {
        let provider = DeepgramProvider::new("".to_string());
        assert!(!provider.is_model_loaded().await);
    }

    #[tokio::test]
    async fn test_get_current_model() {
        let provider = DeepgramProvider::with_model("test_key".to_string(), "nova-2-general".to_string());
        assert_eq!(provider.get_current_model().await, Some("nova-2-general".to_string()));
    }
}
