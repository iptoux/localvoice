use serde::Serialize;

/// Static metadata for a downloadable whisper.cpp model.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDefinition {
    pub key: &'static str,
    pub display_name: &'static str,
    /// "multilingual" or "en"
    pub language_scope: &'static str,
    pub download_url: &'static str,
    pub file_size_bytes: u64,
    /// SHA-256 hex digest — None means verification is skipped.
    /// TODO TASK-111: fill in verified checksums once download URLs are confirmed.
    pub sha256_checksum: Option<&'static str>,
}

/// All downloadable models, in order of ascending size.
///
/// Download source: <https://huggingface.co/ggerganov/whisper.cpp>
/// Fallback CDN:    <https://github.com/ggerganov/whisper.cpp/releases>
pub static REGISTRY: &[ModelDefinition] = &[
    ModelDefinition {
        key: "ggml-tiny",
        display_name: "Tiny (75 MB)",
        language_scope: "multilingual",
        download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
        file_size_bytes: 75_161_332,
        sha256_checksum: None,
    },
    ModelDefinition {
        key: "ggml-base",
        display_name: "Base (142 MB)",
        language_scope: "multilingual",
        download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
        file_size_bytes: 147_964_211,
        sha256_checksum: None,
    },
    ModelDefinition {
        key: "ggml-small",
        display_name: "Small (466 MB)",
        language_scope: "multilingual",
        download_url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
        file_size_bytes: 487_601_739,
        sha256_checksum: None,
    },
    ModelDefinition {
        key: "ggml-medium",
        display_name: "Medium (1.5 GB)",
        language_scope: "multilingual",
        download_url:
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
        file_size_bytes: 1_533_774_855,
        sha256_checksum: None,
    },
];

/// Looks up a model by its key.
pub fn find(key: &str) -> Option<&'static ModelDefinition> {
    REGISTRY.iter().find(|m| m.key == key)
}
