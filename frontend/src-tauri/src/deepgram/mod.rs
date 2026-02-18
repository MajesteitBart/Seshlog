// deepgram/mod.rs
//
// Deepgram transcription module providing cloud-based speech-to-text
// with real-time streaming and speaker diarization support.
//
// This module implements the TranscriptionProvider trait for integration
// with the existing audio transcription pipeline.

pub mod provider;
pub mod websocket;

// Re-export commonly used types
pub use provider::DeepgramProvider;
pub use websocket::{
    convert_f32_to_linear16,
    ConnectionState,
    DeepgramConfig,
    DeepgramWebSocket,
    SpeakerSegment,
    TranscriptionSegment,
};
