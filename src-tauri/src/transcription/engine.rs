use serde::{Deserialize, Serialize};

use crate::models::registry;

pub const ENGINE_WHISPER_CPP: &str = "whisper-cpp";
pub const ENGINE_PARAKEET_CPP: &str = "parakeet-cpp";
pub const ENGINE_NEMO: &str = "nemo";

pub const FORMAT_GGML_BIN: &str = "ggml-bin";
pub const FORMAT_GGUF: &str = "gguf";
pub const FORMAT_NEMO: &str = "nemo";

pub const RUNTIME_BUNDLED_SIDECAR: &str = "bundled-sidecar";
pub const RUNTIME_OPTIONAL_NEMO: &str = "optional-nemo";
pub const RUNTIME_EXTERNAL_PATH: &str = "external-path";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionEngineInfo {
    pub key: String,
    pub display_name: String,
    pub runtime: String,
    pub artifact_formats: Vec<String>,
    pub bundled: bool,
    pub optional: bool,
    pub supports_streaming: bool,
    pub description: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ModelRuntime {
    pub model_key: String,
    pub display_name: String,
    pub local_path: String,
    pub engine: String,
    pub artifact_format: String,
    pub runtime: String,
    pub supports_streaming: bool,
    pub supports_word_timestamps: bool,
    pub supports_confidence: bool,
}

impl ModelRuntime {
    pub fn from_registry(model_key: &str, local_path: String) -> Self {
        if let Some(def) = registry::find(model_key) {
            return Self {
                model_key: def.key.to_string(),
                display_name: def.display_name.to_string(),
                local_path,
                engine: def.engine.to_string(),
                artifact_format: def.artifact_format.to_string(),
                runtime: def.runtime.to_string(),
                supports_streaming: def.supports_streaming,
                supports_word_timestamps: def.supports_word_timestamps,
                supports_confidence: def.supports_confidence,
            };
        }

        Self::legacy_whisper(model_key, local_path)
    }

    pub fn legacy_whisper(model_key: &str, local_path: String) -> Self {
        Self {
            model_key: model_key.to_string(),
            display_name: model_key.to_string(),
            local_path,
            engine: ENGINE_WHISPER_CPP.to_string(),
            artifact_format: FORMAT_GGML_BIN.to_string(),
            runtime: RUNTIME_EXTERNAL_PATH.to_string(),
            supports_streaming: false,
            supports_word_timestamps: false,
            supports_confidence: true,
        }
    }
}

pub fn list_engines() -> Vec<TranscriptionEngineInfo> {
    vec![
        TranscriptionEngineInfo {
            key: ENGINE_WHISPER_CPP.to_string(),
            display_name: "Whisper.cpp".to_string(),
            runtime: RUNTIME_BUNDLED_SIDECAR.to_string(),
            artifact_formats: vec![FORMAT_GGML_BIN.to_string()],
            bundled: true,
            optional: false,
            supports_streaming: false,
            description: "Existing local Whisper transcription runtime.".to_string(),
        },
        TranscriptionEngineInfo {
            key: ENGINE_PARAKEET_CPP.to_string(),
            display_name: "Parakeet.cpp".to_string(),
            runtime: RUNTIME_BUNDLED_SIDECAR.to_string(),
            artifact_formats: vec![FORMAT_GGUF.to_string()],
            bundled: true,
            optional: false,
            supports_streaming: true,
            description: "Bundled C++/GGUF runtime for NVIDIA Parakeet and Nemotron models."
                .to_string(),
        },
        TranscriptionEngineInfo {
            key: ENGINE_NEMO.to_string(),
            display_name: "NVIDIA NeMo".to_string(),
            runtime: RUNTIME_OPTIONAL_NEMO.to_string(),
            artifact_formats: vec![FORMAT_NEMO.to_string()],
            bundled: false,
            optional: true,
            supports_streaming: false,
            description: "Optional Python/NeMo runtime for native .nemo checkpoints.".to_string(),
        },
    ]
}
