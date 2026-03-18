use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// All possible recording states serialized to/from the frontend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordingState {
    Idle,
    Listening,
    Processing,
    Success,
    Error,
}

impl Default for RecordingState {
    fn default() -> Self {
        RecordingState::Idle
    }
}

/// Payload emitted with every `recording-state-changed` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingStatePayload {
    pub state: RecordingState,
    /// Non-empty only in the Error variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Metadata about an audio input device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

/// Holds everything needed while a recording is in progress.
pub struct ActiveRecording {
    /// The live cpal input stream — dropping this stops the hardware capture.
    pub stream: cpal::Stream,
    /// Accumulated 16-bit PCM samples (mono, `sample_rate` Hz).
    pub samples: Arc<Mutex<Vec<i16>>>,
    /// Path where the final WAV will be written on `stop_recording`.
    pub temp_path: std::path::PathBuf,
    /// Actual sample rate negotiated with the device.
    pub sample_rate: u32,
    /// Human-readable device name (for logging and MS-03 session metadata).
    #[allow(dead_code)]
    pub device_name: String,
    /// Set to `true` by the audio callback when silence exceeds the configured timeout.
    /// Polled by the silence watcher thread to trigger auto-stop.
    pub silence_triggered: Arc<AtomicBool>,
}

// cpal::Stream is unsafe impl Send on WASAPI (Windows desktop target).
// Declaring this explicitly here so the compiler rejects non-Send platforms early.
unsafe impl Send for ActiveRecording {}
