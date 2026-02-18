// deepgram/websocket.rs
//
// WebSocket connection management for Deepgram streaming transcription API.
// Handles connection establishment, audio streaming, and response parsing.

use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        http::Request,
        Message,
    },
};

// ============================================================================
// DEEPGRAM API RESPONSE TYPES
// ============================================================================

/// Deepgram transcription response
#[derive(Debug, Clone, Deserialize)]
pub struct DeepgramResponse {
    #[serde(rename = "type")]
    pub response_type: Option<String>,
    pub channel_index: Option<Vec<i32>>,
    pub duration: Option<f64>,
    pub start: Option<f64>,
    pub is_final: Option<bool>,
    pub speech_final: Option<bool>,
    pub channel: Option<DeepgramChannel>,
    pub metadata: Option<DeepgramMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepgramChannel {
    pub alternatives: Vec<DeepgramAlternative>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepgramAlternative {
    pub transcript: String,
    pub confidence: Option<f64>,
    pub words: Option<Vec<DeepgramWord>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepgramWord {
    pub word: String,
    pub start: f64,
    pub end: f64,
    pub confidence: f64,
    pub speaker: Option<i32>,
    pub punctuated_word: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepgramMetadata {
    pub request_id: Option<String>,
    pub model_info: Option<DeepgramModelInfo>,
    pub model_uuid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepgramModelInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

// ============================================================================
// TRANSCRIPTION RESULT TYPES
// ============================================================================

/// Result from a transcription segment
#[derive(Debug, Clone)]
pub struct TranscriptionSegment {
    pub text: String,
    pub confidence: Option<f32>,
    pub is_final: bool,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub speakers: Vec<SpeakerSegment>,
}

/// Speaker segment within a transcription
#[derive(Debug, Clone)]
pub struct SpeakerSegment {
    pub speaker_id: i32,
    pub text: String,
    pub start_time: f64,
    pub end_time: f64,
}

// ============================================================================
// WEBSOCKET CONNECTION STATE
// ============================================================================

/// State of the WebSocket connection
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// Configuration for Deepgram connection
#[derive(Debug, Clone)]
pub struct DeepgramConfig {
    pub api_key: String,
    pub model: String,
    pub language: Option<String>,
    pub sample_rate: u32,
    pub channels: u8,
    pub encoding: String,
    pub diarize: bool,
    pub punctuate: bool,
    pub interim_results: bool,
    pub smart_format: bool,
}

impl Default for DeepgramConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "nova-2".to_string(),
            language: Some("en".to_string()),
            sample_rate: 16000,
            channels: 1,
            encoding: "linear16".to_string(),
            diarize: true,
            punctuate: true,
            interim_results: true,
            smart_format: true,
        }
    }
}

impl DeepgramConfig {
    /// Build the WebSocket URL with query parameters
    pub fn build_url(&self) -> String {
        let mut url = format!(
            "wss://api.deepgram.com/v1/listen?model={}&encoding={}&sample_rate={}&channels={}",
            self.model, self.encoding, self.sample_rate, self.channels
        );

        // Only add language parameter if it's a valid Deepgram language code
        // Skip "auto-translate" and other special values - let Deepgram auto-detect
        if let Some(ref lang) = self.language {
            // Valid language codes are typically 2-letter (en, es) or locale (en-US, en-GB)
            // Skip special values like "auto-translate", "auto", "detect" etc.
            let is_valid_language_code = lang.len() >= 2
                && lang.len() <= 10
                && !lang.contains("auto")
                && !lang.contains("translate")
                && !lang.contains("detect");

            if is_valid_language_code {
                url.push_str(&format!("&language={}", lang));
            }
            // If not a valid code, omit language parameter - Deepgram will auto-detect
        }

        if self.diarize {
            url.push_str("&diarize=true");
        }

        if self.punctuate {
            url.push_str("&punctuate=true");
        }

        if self.interim_results {
            url.push_str("&interim_results=true");
        }

        if self.smart_format {
            url.push_str("&smart_format=true");
        }

        url
    }
}

// ============================================================================
// WEBSOCKET CONNECTION MANAGER
// ============================================================================

/// Internal message type for the audio send channel
#[derive(Debug)]
enum AudioMessage {
    /// Audio data to send
    Audio(Vec<u8>),
    /// Signal to send CloseStream message (Deepgram protocol)
    CloseStream,
}

/// Manages the WebSocket connection to Deepgram
pub struct DeepgramWebSocket {
    config: DeepgramConfig,
    state: Arc<RwLock<ConnectionState>>,
    sender: Arc<Mutex<Option<mpsc::Sender<AudioMessage>>>>,
    transcript_receiver: Arc<Mutex<Option<mpsc::Receiver<TranscriptionSegment>>>>,
}

impl DeepgramWebSocket {
    /// Create a new DeepgramWebSocket instance
    pub fn new(config: DeepgramConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            sender: Arc::new(Mutex::new(None)),
            transcript_receiver: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the current connection state
    pub async fn get_state(&self) -> ConnectionState {
        self.state.read().await.clone()
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        matches!(*self.state.read().await, ConnectionState::Connected)
    }

    /// Connect to Deepgram WebSocket API
    pub async fn connect(&self) -> Result<mpsc::Receiver<TranscriptionSegment>, String> {
        // Check if already connected
        {
            let state = self.state.read().await;
            if *state == ConnectionState::Connected {
                return Err("Already connected".to_string());
            }
        }

        // Update state to connecting
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Connecting;
        }

        let url = self.config.build_url();
        info!("Connecting to Deepgram: {}", url);

        // Build the WebSocket request with authentication
        let request = Request::builder()
            .uri(&url)
            .header("Authorization", format!("Token {}", self.config.api_key))
            .header("Host", "api.deepgram.com")
            .header("Upgrade", "websocket")
            .header("Connection", "Upgrade")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_websocket_key())
            .body(())
            .map_err(|e| format!("Failed to build request: {}", e))?;

        // Connect to WebSocket
        let (ws_stream, _response) = connect_async(request)
            .await
            .map_err(|e| {
                error!("WebSocket connection failed: {}", e);
                format!("Failed to connect to Deepgram: {}", e)
            })?;

        info!("Connected to Deepgram WebSocket");

        // Split the WebSocket stream
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Create channels for communication
        let (audio_tx, mut audio_rx) = mpsc::channel::<AudioMessage>(100);
        let (transcript_tx, transcript_rx) = mpsc::channel::<TranscriptionSegment>(100);

        // Store the audio sender
        {
            let mut sender = self.sender.lock().await;
            *sender = Some(audio_tx);
        }

        // Update state to connected
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Connected;
        }

        let state_clone = self.state.clone();
        let sender_clone = self.sender.clone();

        // Spawn task to send audio data
        tokio::spawn(async move {
            while let Some(msg) = audio_rx.recv().await {
                match msg {
                    AudioMessage::Audio(audio_data) => {
                        if let Err(e) = ws_sender.send(Message::Binary(audio_data)).await {
                            error!("Failed to send audio data: {}", e);
                            break;
                        }
                    }
                    AudioMessage::CloseStream => {
                        // Send Deepgram CloseStream message to signal end of audio
                        // This tells Deepgram to finish processing and send final results
                        let close_stream_msg = r#"{"type": "CloseStream"}"#;
                        debug!("Sending CloseStream message to Deepgram");
                        if let Err(e) = ws_sender.send(Message::Text(close_stream_msg.to_string())).await {
                            error!("Failed to send CloseStream message: {}", e);
                        }
                        // Don't break - wait for channel to close
                    }
                }
            }

            // Only send WebSocket close frame after channel is fully closed
            debug!("Audio channel closed, sending WebSocket close frame");
            let _ = ws_sender.send(Message::Close(None)).await;
            debug!("Audio sender task completed");
        });

        // Spawn task to receive transcription responses
        tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<DeepgramResponse>(&text) {
                            Ok(response) => {
                                // Log response type for debugging
                                if let Some(ref response_type) = response.response_type {
                                    debug!("Deepgram response type: {}", response_type);
                                }

                                if let Some(segment) = parse_deepgram_response(&response) {
                                    debug!(
                                        "Parsed segment: is_final={}, text='{}', speakers={}",
                                        segment.is_final,
                                        &segment.text[..segment.text.len().min(50)],
                                        segment.speakers.len()
                                    );
                                    if let Err(e) = transcript_tx.send(segment).await {
                                        warn!("Failed to send transcript segment: {}", e);
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                debug!("Failed to parse Deepgram response: {} - Raw: {}", e, text);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("Deepgram connection closed");
                        break;
                    }
                    Ok(Message::Ping(data)) => {
                        debug!("Received ping from Deepgram");
                        // Pong is handled automatically by tungstenite
                    }
                    Ok(Message::Pong(_)) => {
                        debug!("Received pong from Deepgram");
                    }
                    Ok(Message::Binary(_)) => {
                        debug!("Received unexpected binary message from Deepgram");
                    }
                    Ok(Message::Frame(_)) => {
                        // Internal frame, can be ignored
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                }
            }

            // Update state to disconnected
            {
                let mut state = state_clone.write().await;
                *state = ConnectionState::Disconnected;
            }

            // Clear the sender
            {
                let mut sender = sender_clone.lock().await;
                *sender = None;
            }

            debug!("Transcript receiver task completed");
        });

        Ok(transcript_rx)
    }

    /// Send audio data to Deepgram
    /// Audio should be f32 samples at 16kHz mono - will be converted to linear16
    pub async fn send_audio(&self, audio: &[f32]) -> Result<(), String> {
        let sender = self.sender.lock().await;

        if let Some(ref tx) = *sender {
            // Convert f32 audio to linear16 (i16)
            let linear16_data = convert_f32_to_linear16(audio);

            tx.send(AudioMessage::Audio(linear16_data))
                .await
                .map_err(|e| format!("Failed to queue audio data: {}", e))
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Signal end of audio stream without fully disconnecting
    /// This sends the Deepgram CloseStream message to tell Deepgram we're done
    /// sending audio, but keeps the connection open to receive final results
    pub async fn signal_end_of_audio(&self) {
        // Send CloseStream message via the channel
        let sender = self.sender.lock().await;
        if let Some(ref tx) = *sender {
            if let Err(e) = tx.send(AudioMessage::CloseStream).await {
                warn!("Failed to send CloseStream message: {}", e);
            } else {
                debug!("Signaled end of audio stream with CloseStream message");
            }
        }
        // Note: Don't drop the sender yet - we still need to receive results
        // The sender will be cleared in disconnect()
    }

    /// Disconnect from Deepgram
    pub async fn disconnect(&self) {
        // Clear the sender to signal the send task to close
        {
            let mut sender = self.sender.lock().await;
            *sender = None;
        }

        // Update state
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Disconnected;
        }

        info!("Disconnected from Deepgram");
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convert f32 audio samples to linear16 (i16) format
/// Input: f32 samples in range [-1.0, 1.0]
/// Output: Vec<u8> containing i16 samples in little-endian format
pub fn convert_f32_to_linear16(audio: &[f32]) -> Vec<u8> {
    let mut output = Vec::with_capacity(audio.len() * 2);

    for &sample in audio {
        // Clamp and scale to i16 range
        let clamped = sample.clamp(-1.0, 1.0);
        let scaled = (clamped * 32767.0) as i16;

        // Convert to little-endian bytes
        output.extend_from_slice(&scaled.to_le_bytes());
    }

    output
}

/// Parse a Deepgram response into a TranscriptionSegment
fn parse_deepgram_response(response: &DeepgramResponse) -> Option<TranscriptionSegment> {
    // Only process transcription results
    if response.response_type.as_deref() != Some("Results") {
        return None;
    }

    let channel = response.channel.as_ref()?;
    let alternative = channel.alternatives.first()?;

    // Skip empty transcripts
    if alternative.transcript.trim().is_empty() {
        return None;
    }

    let is_final = response.is_final.unwrap_or(false);
    let confidence = alternative.confidence.map(|c| c as f32);

    // Extract speaker segments from words
    let speakers = extract_speaker_segments(alternative.words.as_deref().unwrap_or(&[]));

    Some(TranscriptionSegment {
        text: alternative.transcript.clone(),
        confidence,
        is_final,
        start_time: response.start,
        end_time: response.start.map(|s| s + response.duration.unwrap_or(0.0)),
        speakers,
    })
}

/// Extract speaker segments from word array
fn extract_speaker_segments(words: &[DeepgramWord]) -> Vec<SpeakerSegment> {
    if words.is_empty() {
        return Vec::new();
    }

    let mut segments: Vec<SpeakerSegment> = Vec::new();
    let mut current_speaker: Option<i32> = None;
    let mut current_text = String::new();
    let mut segment_start: f64 = 0.0;
    let mut segment_end: f64 = 0.0;

    for word in words {
        let speaker = word.speaker.unwrap_or(0);
        let word_text = word.punctuated_word.as_ref().unwrap_or(&word.word);

        if current_speaker.is_none() {
            // First word
            current_speaker = Some(speaker);
            current_text = word_text.clone();
            segment_start = word.start;
            segment_end = word.end;
        } else if current_speaker == Some(speaker) {
            // Same speaker, append word
            if !current_text.is_empty() {
                current_text.push(' ');
            }
            current_text.push_str(word_text);
            segment_end = word.end;
        } else {
            // Speaker changed, save current segment and start new one
            if let Some(spk) = current_speaker {
                segments.push(SpeakerSegment {
                    speaker_id: spk,
                    text: current_text.trim().to_string(),
                    start_time: segment_start,
                    end_time: segment_end,
                });
            }

            current_speaker = Some(speaker);
            current_text = word_text.clone();
            segment_start = word.start;
            segment_end = word.end;
        }
    }

    // Don't forget the last segment
    if let Some(spk) = current_speaker {
        if !current_text.is_empty() {
            segments.push(SpeakerSegment {
                speaker_id: spk,
                text: current_text.trim().to_string(),
                start_time: segment_start,
                end_time: segment_end,
            });
        }
    }

    segments
}

/// Generate a random WebSocket key
fn generate_websocket_key() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    base64_encode(&bytes)
}

/// Simple base64 encoding (standard alphabet)
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let chunks = data.chunks(3);

    for chunk in chunks {
        let mut n: u32 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            n |= (byte as u32) << (16 - 8 * i);
        }

        let indices = [
            ((n >> 18) & 0x3F) as usize,
            ((n >> 12) & 0x3F) as usize,
            ((n >> 6) & 0x3F) as usize,
            (n & 0x3F) as usize,
        ];

        for (i, &idx) in indices.iter().enumerate() {
            if i < chunk.len() + 1 {
                result.push(ALPHABET[idx] as char);
            } else {
                result.push('=');
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_f32_to_linear16() {
        // Test conversion of common values
        let audio = vec![0.0f32, 1.0, -1.0, 0.5, -0.5];
        let result = convert_f32_to_linear16(&audio);

        // Each f32 becomes 2 bytes
        assert_eq!(result.len(), 10);

        // 0.0 should become 0
        assert_eq!(i16::from_le_bytes([result[0], result[1]]), 0);

        // 1.0 should become 32767 (max positive)
        assert_eq!(i16::from_le_bytes([result[2], result[3]]), 32767);

        // -1.0 should become -32767 (close to min negative)
        assert_eq!(i16::from_le_bytes([result[4], result[5]]), -32767);
    }

    #[test]
    fn test_config_build_url() {
        let config = DeepgramConfig {
            api_key: "test_key".to_string(),
            model: "nova-2".to_string(),
            language: Some("en".to_string()),
            sample_rate: 16000,
            channels: 1,
            encoding: "linear16".to_string(),
            diarize: true,
            punctuate: true,
            interim_results: true,
            smart_format: false,
        };

        let url = config.build_url();
        assert!(url.contains("model=nova-2"));
        assert!(url.contains("encoding=linear16"));
        assert!(url.contains("sample_rate=16000"));
        assert!(url.contains("channels=1"));
        assert!(url.contains("language=en"));
        assert!(url.contains("diarize=true"));
        assert!(url.contains("punctuate=true"));
        assert!(url.contains("interim_results=true"));
        assert!(!url.contains("smart_format=true"));
    }

    #[test]
    fn test_config_build_url_skips_auto_translate() {
        let config = DeepgramConfig {
            api_key: "test_key".to_string(),
            model: "nova-2-meeting".to_string(),
            language: Some("auto-translate".to_string()), // Invalid - should be skipped
            sample_rate: 16000,
            channels: 1,
            encoding: "linear16".to_string(),
            diarize: true,
            punctuate: true,
            interim_results: true,
            smart_format: true,
        };

        let url = config.build_url();
        assert!(url.contains("model=nova-2-meeting"));
        // "auto-translate" should NOT appear in URL - Deepgram will auto-detect
        assert!(!url.contains("language="));
        assert!(url.contains("diarize=true"));
    }

    #[test]
    fn test_extract_speaker_segments_single_speaker() {
        let words = vec![
            DeepgramWord {
                word: "hello".to_string(),
                start: 0.0,
                end: 0.5,
                confidence: 0.99,
                speaker: Some(0),
                punctuated_word: Some("Hello".to_string()),
            },
            DeepgramWord {
                word: "world".to_string(),
                start: 0.5,
                end: 1.0,
                confidence: 0.99,
                speaker: Some(0),
                punctuated_word: Some("world.".to_string()),
            },
        ];

        let segments = extract_speaker_segments(&words);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].speaker_id, 0);
        assert_eq!(segments[0].text, "Hello world.");
    }

    #[test]
    fn test_extract_speaker_segments_multiple_speakers() {
        let words = vec![
            DeepgramWord {
                word: "hello".to_string(),
                start: 0.0,
                end: 0.5,
                confidence: 0.99,
                speaker: Some(0),
                punctuated_word: Some("Hello.".to_string()),
            },
            DeepgramWord {
                word: "hi".to_string(),
                start: 0.6,
                end: 0.8,
                confidence: 0.99,
                speaker: Some(1),
                punctuated_word: Some("Hi!".to_string()),
            },
        ];

        let segments = extract_speaker_segments(&words);
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].speaker_id, 0);
        assert_eq!(segments[0].text, "Hello.");
        assert_eq!(segments[1].speaker_id, 1);
        assert_eq!(segments[1].text, "Hi!");
    }
}
