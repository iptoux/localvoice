use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};

use crate::errors::{AppError, CmdResult};
use crate::os::{clipboard, text_insertion};
use crate::postprocess::normalize;
use crate::transcription::engine::{ModelRuntime, ENGINE_NEMO, ENGINE_PARAKEET_CPP};
use crate::transcription::types::{TranscriptSegment, TranscriptWord};
use crate::transcription::{language, nemo_worker, orchestrator, parakeet_parser};

const STREAM_EVENT: &str = "transcription-stream-update";
const DEFAULT_CHUNK_MS: u64 = 320;
const MIN_CHUNK_MS: u64 = 160;
const MAX_CHUNK_MS: u64 = 2_000;
const LOAD_TIMEOUT: Duration = Duration::from_secs(120);
const FINALIZE_TIMEOUT: Duration = Duration::from_secs(45);

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamingOutputMode {
    Preview,
    LiveInsert,
}

impl StreamingOutputMode {
    fn from_setting(value: Option<&String>) -> Self {
        match value.map(String::as_str) {
            Some("live_insert") | Some("live-insert") => Self::LiveInsert,
            _ => Self::Preview,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::LiveInsert => "live_insert",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StreamingSettings {
    pub enabled: bool,
    pub chunk_ms: u64,
    pub output_mode: StreamingOutputMode,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingTranscriptUpdate {
    pub session_id: String,
    pub sequence: u64,
    pub text: String,
    pub delta: String,
    pub is_final: bool,
    pub model_id: String,
    pub engine: String,
    pub output_mode: String,
    pub live_inserted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StreamedTranscript {
    pub raw_text: String,
    pub segments: Vec<TranscriptSegment>,
    pub words: Vec<TranscriptWord>,
    pub language: String,
    pub model_runtime: ModelRuntime,
    pub duration_ms: u64,
    pub live_inserted: bool,
}

pub struct StreamingSessionManager {
    active: Option<ActiveStreamingSession>,
}

impl StreamingSessionManager {
    pub fn new() -> Self {
        Self { active: None }
    }

    pub fn start_if_eligible(
        &mut self,
        app: &AppHandle,
        samples: Arc<Mutex<Vec<i16>>>,
        sample_rate: u32,
        settings: &HashMap<String, String>,
    ) -> CmdResult<Option<String>> {
        self.cancel_active();

        let streaming_settings = streaming_settings(settings);
        if !streaming_settings.enabled {
            return Ok(None);
        }

        let lang_code = settings
            .get("transcription.default_language")
            .cloned()
            .unwrap_or_else(|| "auto".to_string());

        let model_runtime =
            match orchestrator::resolve_model_runtime(app, &lang_code, None, settings) {
                Ok(runtime) => runtime,
                Err(e) => {
                    log::warn!("Streaming disabled for this recording: {e}");
                    return Ok(None);
                }
            };

        if !model_runtime.supports_streaming {
            log::info!(
                "Selected model '{}' does not support streaming; using stop-to-transcribe",
                model_runtime.model_key
            );
            return Ok(None);
        }

        let worker = match WorkerLaunch::for_runtime(app, &model_runtime, settings) {
            Ok(worker) => worker,
            Err(e) => {
                log::warn!(
                    "Streaming worker is not available for '{}': {e}. Falling back to WAV transcription.",
                    model_runtime.model_key
                );
                return Ok(None);
            }
        };

        let session_id = uuid::Uuid::new_v4().to_string();
        let control = Arc::new(StreamControl::new());
        let (final_tx, final_rx) = mpsc::sync_channel(1);

        let worker_settings = WorkerRunSettings {
            session_id: session_id.clone(),
            language: language::to_parakeet_locale(&lang_code).to_string(),
            sample_rate,
            chunk_ms: streaming_settings.chunk_ms,
            output_mode: streaming_settings.output_mode,
            insert_delay_ms: settings
                .get("output.insert_delay_ms")
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
        };

        log::info!(
            "Streaming transcription starting for model '{}' with output_mode={}, chunk_ms={}, sample_rate={}",
            model_runtime.model_key,
            worker_settings.output_mode.as_str(),
            worker_settings.chunk_ms,
            worker_settings.sample_rate
        );

        let thread_control = control.clone();
        let app_for_thread = app.clone();
        let model_for_thread = model_runtime.clone();
        thread::spawn(move || {
            let result = run_streaming_worker(
                app_for_thread,
                samples,
                worker,
                model_for_thread,
                worker_settings,
                thread_control,
            );
            let _ = final_tx.send(result);
        });

        emit_stream_update(
            app,
            StreamingTranscriptUpdate {
                session_id: session_id.clone(),
                sequence: 0,
                text: String::new(),
                delta: String::new(),
                is_final: false,
                model_id: model_runtime.model_key.clone(),
                engine: model_runtime.engine.clone(),
                output_mode: streaming_settings.output_mode.as_str().to_string(),
                live_inserted: false,
                error: None,
            },
        );

        self.active = Some(ActiveStreamingSession {
            session_id: session_id.clone(),
            control,
            final_rx,
        });

        Ok(Some(session_id))
    }

    pub fn finalize_active(&mut self, timeout: Duration) -> Option<CmdResult<StreamedTranscript>> {
        let active = self.active.take()?;
        active.control.finalize.store(true, Ordering::SeqCst);
        Some(
            active
                .final_rx
                .recv_timeout(timeout)
                .unwrap_or_else(|_| Err("Streaming finalization timed out.".into())),
        )
    }

    pub fn cancel_active(&mut self) {
        if let Some(active) = self.active.take() {
            log::debug!("Cancelling streaming session {}", active.session_id);
            active.control.cancel.store(true, Ordering::SeqCst);
        }
    }
}

impl Default for StreamingSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

struct ActiveStreamingSession {
    session_id: String,
    control: Arc<StreamControl>,
    final_rx: mpsc::Receiver<CmdResult<StreamedTranscript>>,
}

struct StreamControl {
    finalize: AtomicBool,
    cancel: AtomicBool,
}

impl StreamControl {
    fn new() -> Self {
        Self {
            finalize: AtomicBool::new(false),
            cancel: AtomicBool::new(false),
        }
    }
}

#[derive(Clone)]
enum WorkerLaunch {
    Parakeet { binary: PathBuf },
    Nemo { python: PathBuf, script: PathBuf },
}

impl WorkerLaunch {
    fn for_runtime(
        app: &AppHandle,
        runtime: &ModelRuntime,
        settings: &HashMap<String, String>,
    ) -> CmdResult<Self> {
        match runtime.engine.as_str() {
            ENGINE_PARAKEET_CPP => Ok(Self::Parakeet {
                binary: crate::transcription::parakeet_stream_worker::resolve_binary(app)?,
            }),
            ENGINE_NEMO => Ok(Self::Nemo {
                python: nemo_worker::resolve_python(
                    settings
                        .get("transcription.nemo.python_path")
                        .map(String::as_str),
                )?,
                script: nemo_worker::resolve_worker_script(app)?,
            }),
            other => Err(format!("Engine '{other}' is not a streaming worker engine.").into()),
        }
    }

    fn spawn(&self) -> CmdResult<Child> {
        let mut command = match self {
            Self::Parakeet { binary } => {
                let mut command = Command::new(binary);
                crate::transcription::parakeet_runtime::configure_command_environment(
                    &mut command,
                    binary,
                );
                command.current_dir(binary.parent().unwrap_or_else(|| std::path::Path::new(".")));
                command
            }
            Self::Nemo { python, script } => {
                let mut command = Command::new(python);
                command.arg(script);
                command
            }
        };
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        #[cfg(target_os = "windows")]
        command.creation_flags(CREATE_NO_WINDOW);
        command
            .spawn()
            .map_err(|e| format!("Failed to start streaming worker: {e}").into())
    }
}

struct WorkerRunSettings {
    session_id: String,
    language: String,
    sample_rate: u32,
    chunk_ms: u64,
    output_mode: StreamingOutputMode,
    insert_delay_ms: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerResponse {
    #[serde(rename = "type")]
    kind: String,
    ok: Option<bool>,
    message: Option<String>,
    text: Option<String>,
    delta: Option<String>,
    finalized_text: Option<String>,
    sequence: Option<u64>,
    segments: Option<Vec<WorkerSegment>>,
    words: Option<Vec<WorkerWord>>,
    engine_json: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerSegment {
    start_ms: i64,
    end_ms: i64,
    text: String,
    confidence: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerWord {
    #[serde(alias = "w", alias = "word")]
    text: String,
    #[serde(alias = "start")]
    start_ms: Option<i64>,
    #[serde(alias = "end")]
    end_ms: Option<i64>,
    confidence: Option<f32>,
    #[serde(alias = "conf", alias = "probability")]
    prob: Option<f32>,
}

struct WorkerState {
    stable_text: String,
    segments: Vec<TranscriptSegment>,
    words: Vec<TranscriptWord>,
    sequence: u64,
    live_inserted: bool,
}

impl WorkerState {
    fn new() -> Self {
        Self {
            stable_text: String::new(),
            segments: Vec::new(),
            words: Vec::new(),
            sequence: 0,
            live_inserted: false,
        }
    }
}

fn run_streaming_worker(
    app: AppHandle,
    samples: Arc<Mutex<Vec<i16>>>,
    worker: WorkerLaunch,
    model_runtime: ModelRuntime,
    settings: WorkerRunSettings,
    control: Arc<StreamControl>,
) -> CmdResult<StreamedTranscript> {
    let started = Instant::now();
    let mut child = worker.spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| AppError("Streaming worker stdin was not available.".to_string()))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| AppError("Streaming worker stdout was not available.".to_string()))?;

    let (event_tx, event_rx) = mpsc::channel::<CmdResult<WorkerResponse>>();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) if !line.trim().is_empty() => {
                    let parsed = serde_json::from_str::<WorkerResponse>(&line).map_err(|e| {
                        AppError(format!("Invalid streaming worker JSON: {e}; line={line}"))
                    });
                    if event_tx.send(parsed).is_err() {
                        break;
                    }
                }
                Ok(_) => continue,
                Err(e) => {
                    let _ = event_tx.send(Err(format!(
                        "Failed to read streaming worker response: {e}"
                    )
                    .into()));
                    break;
                }
            }
        }
    });

    send_worker_request(
        &mut stdin,
        json!({
            "type": "load",
            "modelPath": model_runtime.local_path.clone(),
            "language": settings.language,
            "sampleRate": settings.sample_rate
        }),
    )?;
    wait_for_loaded(&event_rx)?;

    let mut cursor = 0usize;
    let mut sequence = 0u64;
    let mut state = WorkerState::new();
    let chunk_sleep = Duration::from_millis(settings.chunk_ms);

    loop {
        if control.cancel.load(Ordering::SeqCst) {
            let _ = send_worker_request(&mut stdin, json!({ "type": "cancel" }));
            let _ = child.kill();
            return Err("Streaming session cancelled.".into());
        }

        let should_finalize = control.finalize.load(Ordering::SeqCst);
        if let Some(chunk) = copy_new_samples(&samples, &mut cursor) {
            sequence += 1;
            send_worker_request(
                &mut stdin,
                json!({
                    "type": "audio",
                    "sequence": sequence,
                    "sampleRate": settings.sample_rate,
                    "pcm16": encode_pcm16_base64(&chunk)
                }),
            )?;
        }

        drain_worker_events(
            &app,
            &event_rx,
            &model_runtime,
            &settings,
            &mut state,
            false,
        )?;

        if should_finalize {
            send_worker_request(
                &mut stdin,
                json!({
                    "type": "finalize",
                    "sequence": sequence,
                }),
            )?;
            wait_for_final(
                &app,
                &event_rx,
                &model_runtime,
                &settings,
                &mut state,
                FINALIZE_TIMEOUT,
            )?;
            let _ = child.kill();

            if settings.output_mode == StreamingOutputMode::LiveInsert && !state.live_inserted {
                log::warn!(
                    "Streaming live insert produced no inserted deltas before finalize; final output will be handled after stop"
                );
            }

            if state.segments.is_empty() && !state.stable_text.trim().is_empty() {
                state.segments.push(TranscriptSegment {
                    start_ms: state.words.first().map(|w| w.start_ms).unwrap_or(0),
                    end_ms: state.words.last().map(|w| w.end_ms).unwrap_or(0),
                    text: state.stable_text.clone(),
                    confidence: mean_confidence(&state.words),
                });
            }

            return Ok(StreamedTranscript {
                raw_text: state.stable_text.trim().to_string(),
                segments: state.segments,
                words: state.words,
                language: locale_to_language_code(&settings.language),
                model_runtime,
                duration_ms: started.elapsed().as_millis() as u64,
                live_inserted: state.live_inserted,
            });
        }

        thread::sleep(chunk_sleep);
    }
}

fn wait_for_loaded(event_rx: &mpsc::Receiver<CmdResult<WorkerResponse>>) -> CmdResult<()> {
    let deadline = Instant::now() + LOAD_TIMEOUT;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err("Streaming worker did not load the model before timeout.".into());
        }
        let resp = event_rx
            .recv_timeout(remaining.min(Duration::from_millis(500)))
            .map_err(|_| "Streaming worker did not send a load response.")??;
        match resp.kind.as_str() {
            "loaded" => {
                if resp.ok.unwrap_or(true) {
                    return Ok(());
                }
                return Err(resp
                    .message
                    .unwrap_or_else(|| "Streaming worker failed to load the model.".to_string())
                    .into());
            }
            "error" => {
                return Err(resp
                    .message
                    .unwrap_or_else(|| "Streaming worker returned an error.".to_string())
                    .into());
            }
            _ => continue,
        }
    }
}

fn wait_for_final(
    app: &AppHandle,
    event_rx: &mpsc::Receiver<CmdResult<WorkerResponse>>,
    model_runtime: &ModelRuntime,
    settings: &WorkerRunSettings,
    state: &mut WorkerState,
    timeout: Duration,
) -> CmdResult<()> {
    let deadline = Instant::now() + timeout;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(
                "Streaming worker did not return a final transcript before timeout.".into(),
            );
        }
        let resp = event_rx
            .recv_timeout(remaining.min(Duration::from_millis(500)))
            .map_err(|_| "Streaming worker did not send a final response.")??;
        match resp.kind.as_str() {
            "final" => {
                apply_worker_response(app, model_runtime, settings, state, resp, true)?;
                return Ok(());
            }
            "partial" => apply_worker_response(app, model_runtime, settings, state, resp, false)?,
            "error" => {
                return Err(resp
                    .message
                    .unwrap_or_else(|| "Streaming worker returned an error.".to_string())
                    .into());
            }
            _ => continue,
        }
    }
}

fn drain_worker_events(
    app: &AppHandle,
    event_rx: &mpsc::Receiver<CmdResult<WorkerResponse>>,
    model_runtime: &ModelRuntime,
    settings: &WorkerRunSettings,
    state: &mut WorkerState,
    force_final: bool,
) -> CmdResult<()> {
    loop {
        match event_rx.try_recv() {
            Ok(Ok(resp)) => match resp.kind.as_str() {
                "partial" => {
                    apply_worker_response(app, model_runtime, settings, state, resp, force_final)?
                }
                "error" => {
                    return Err(resp
                        .message
                        .unwrap_or_else(|| "Streaming worker returned an error.".to_string())
                        .into());
                }
                _ => continue,
            },
            Ok(Err(e)) => return Err(e),
            Err(mpsc::TryRecvError::Empty) => return Ok(()),
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err("Streaming worker output channel disconnected.".into())
            }
        }
    }
}

fn apply_worker_response(
    app: &AppHandle,
    model_runtime: &ModelRuntime,
    settings: &WorkerRunSettings,
    state: &mut WorkerState,
    resp: WorkerResponse,
    is_final: bool,
) -> CmdResult<()> {
    let engine_json = resp
        .engine_json
        .as_ref()
        .and_then(|value| parakeet_parser::parse_stdout(&value.to_string()));

    if let Some(parsed) = &engine_json {
        state.words.extend(parsed.words.clone());
        if is_final && !parsed.segments.is_empty() {
            state.segments = parsed.segments.clone();
        }
    }

    let response_words = resp.words.unwrap_or_default();
    state
        .words
        .extend(response_words.into_iter().filter_map(|word| {
            let text = normalize::remove_language_tags(&word.text)
                .trim()
                .to_string();
            if text.is_empty() {
                return None;
            }
            let start_ms = word.start_ms.unwrap_or(0);
            let end_ms = word.end_ms.unwrap_or(start_ms);
            Some(TranscriptWord {
                start_ms,
                end_ms,
                text,
                confidence: word.confidence.or(word.prob),
            })
        }));

    let response_segments = resp.segments.unwrap_or_default();
    if !response_segments.is_empty() {
        state.segments = response_segments
            .into_iter()
            .filter_map(|segment| {
                let text = normalize::remove_language_tags(&segment.text)
                    .trim()
                    .to_string();
                if text.is_empty() {
                    return None;
                }
                Some(TranscriptSegment {
                    start_ms: segment.start_ms,
                    end_ms: segment.end_ms,
                    text,
                    confidence: segment.confidence,
                })
            })
            .collect();
    }

    let previous = state.stable_text.clone();
    let full_text = resp
        .text
        .or_else(|| engine_json.map(|parsed| parsed.text))
        .map(|text| normalize::remove_language_tags(&text));
    let delta = resp
        .delta
        .or(resp.finalized_text)
        .map(|text| normalize::remove_language_tags(&text))
        .unwrap_or_else(|| derive_delta(&previous, full_text.as_deref().unwrap_or("")));

    if let Some(full_text) = full_text {
        state.stable_text = full_text;
    } else if !delta.trim().is_empty() {
        append_delta(&mut state.stable_text, &delta);
    }

    if state.stable_text == previous && delta.trim().is_empty() && !is_final {
        return Ok(());
    }

    state.sequence = resp.sequence.unwrap_or(state.sequence + 1);
    let live_inserted = if settings.output_mode == StreamingOutputMode::LiveInsert
        && !delta.trim().is_empty()
        && !is_final
    {
        log::info!(
            "Live streaming insert delta received: sequence={}, chars={}",
            resp.sequence.unwrap_or(state.sequence + 1),
            delta.chars().count()
        );
        insert_live_delta(&delta, settings.insert_delay_ms);
        state.live_inserted = true;
        true
    } else {
        false
    };

    emit_stream_update(
        app,
        StreamingTranscriptUpdate {
            session_id: settings.session_id.clone(),
            sequence: state.sequence,
            text: state.stable_text.clone(),
            delta,
            is_final,
            model_id: model_runtime.model_key.clone(),
            engine: model_runtime.engine.clone(),
            output_mode: settings.output_mode.as_str().to_string(),
            live_inserted,
            error: None,
        },
    );

    Ok(())
}

fn insert_live_delta(delta: &str, insert_delay_ms: u64) {
    if let Err(e) = text_insertion::insert(delta, insert_delay_ms) {
        log::warn!("Live streaming insert failed ({e}); copying delta to clipboard");
        if let Err(clipboard_error) = clipboard::write(delta) {
            log::warn!("Live streaming clipboard fallback failed: {clipboard_error}");
        }
    }
}

fn send_worker_request(stdin: &mut impl Write, payload: Value) -> CmdResult<()> {
    writeln!(stdin, "{payload}")
        .and_then(|_| stdin.flush())
        .map_err(|e| format!("Failed to send streaming worker request: {e}").into())
}

fn copy_new_samples(samples: &Arc<Mutex<Vec<i16>>>, cursor: &mut usize) -> Option<Vec<i16>> {
    let guard = samples.lock().unwrap();
    if *cursor >= guard.len() {
        return None;
    }
    let chunk = guard[*cursor..].to_vec();
    *cursor = guard.len();
    Some(chunk)
}

fn append_delta(target: &mut String, delta: &str) {
    let delta = delta.trim();
    if delta.is_empty() {
        return;
    }
    if target.trim().is_empty() {
        target.push_str(delta);
        return;
    }
    if delta
        .chars()
        .next()
        .map(|c| c.is_ascii_punctuation())
        .unwrap_or(false)
    {
        target.push_str(delta);
    } else {
        target.push(' ');
        target.push_str(delta);
    }
}

fn derive_delta(previous: &str, next: &str) -> String {
    if next.starts_with(previous) {
        next[previous.len()..].trim().to_string()
    } else if next.trim().is_empty() {
        String::new()
    } else {
        next.to_string()
    }
}

fn mean_confidence(words: &[TranscriptWord]) -> Option<f32> {
    let values = words
        .iter()
        .filter_map(|w| w.confidence)
        .collect::<Vec<_>>();
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f32>() / values.len() as f32)
    }
}

fn locale_to_language_code(locale: &str) -> String {
    if locale.eq_ignore_ascii_case("auto") {
        return "auto".to_string();
    }
    locale
        .split(['-', '_'])
        .next()
        .filter(|part| !part.is_empty())
        .unwrap_or("auto")
        .to_lowercase()
}

fn emit_stream_update(app: &AppHandle, payload: StreamingTranscriptUpdate) {
    if let Err(e) = app.emit(STREAM_EVENT, &payload) {
        log::error!("Failed to emit {STREAM_EVENT}: {e}");
    }
}

pub fn streaming_settings(settings: &HashMap<String, String>) -> StreamingSettings {
    let enabled = settings
        .get("transcription.streaming.enabled")
        .map(|value| value == "true")
        .unwrap_or(false);
    let chunk_ms = settings
        .get("transcription.streaming.chunk_ms")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_CHUNK_MS)
        .clamp(MIN_CHUNK_MS, MAX_CHUNK_MS);
    let output_mode =
        StreamingOutputMode::from_setting(settings.get("transcription.streaming.output_mode"));
    StreamingSettings {
        enabled,
        chunk_ms,
        output_mode,
    }
}

fn encode_pcm16_base64(samples: &[i16]) -> String {
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    encode_base64(&bytes)
}

fn encode_base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0b0000_0011) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[(((b1 & 0b0000_1111) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(b2 & 0b0011_1111) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_streaming_chunk_setting() {
        let mut settings = HashMap::new();
        settings.insert(
            "transcription.streaming.enabled".to_string(),
            "true".to_string(),
        );
        settings.insert(
            "transcription.streaming.chunk_ms".to_string(),
            "10".to_string(),
        );
        let parsed = streaming_settings(&settings);
        assert!(parsed.enabled);
        assert_eq!(parsed.chunk_ms, MIN_CHUNK_MS);

        settings.insert(
            "transcription.streaming.chunk_ms".to_string(),
            "5000".to_string(),
        );
        let parsed = streaming_settings(&settings);
        assert_eq!(parsed.chunk_ms, MAX_CHUNK_MS);
    }

    #[test]
    fn parses_live_insert_output_mode() {
        let mut settings = HashMap::new();
        settings.insert(
            "transcription.streaming.output_mode".to_string(),
            "live_insert".to_string(),
        );
        assert_eq!(
            streaming_settings(&settings).output_mode,
            StreamingOutputMode::LiveInsert
        );
    }

    #[test]
    fn derives_delta_from_growing_text() {
        assert_eq!(derive_delta("hello", "hello world"), "world");
        assert_eq!(derive_delta("hello", "different"), "different");
    }

    #[test]
    fn base64_encodes_pcm16_little_endian() {
        let encoded = encode_pcm16_base64(&[0, 1, -1]);
        assert_eq!(encoded, "AAABAP//");
    }
}
