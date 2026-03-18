use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, SupportedStreamConfig};
use tauri::{AppHandle, Emitter};

use crate::audio::level_meter::calculate_rms;
use crate::errors::CmdResult;
use crate::state::recording_state::ActiveRecording;

/// Target capture format — whisper.cpp expects 16 kHz 16-bit mono.
const TARGET_SAMPLE_RATE: u32 = 16_000;
const TARGET_CHANNELS: u16 = 1;

/// Default RMS threshold below which audio is considered silent.
const DEFAULT_SILENCE_THRESHOLD: f32 = 0.01;

/// Configuration for silence detection passed into the capture callbacks.
#[derive(Clone)]
pub struct SilenceConfig {
    /// Whether silence detection is enabled at all.
    pub enabled: bool,
    /// RMS threshold below which frames are considered silent.
    pub threshold: f32,
    /// How many milliseconds of continuous silence before triggering.
    pub timeout_ms: u64,
}

impl Default for SilenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            threshold: DEFAULT_SILENCE_THRESHOLD,
            timeout_ms: 1500,
        }
    }
}

/// Starts audio capture on the given device and returns an [ActiveRecording].
///
/// The caller must store the returned value in AppState to keep the stream alive.
/// Dropping the [ActiveRecording] automatically stops the cpal stream.
pub fn start_capture(
    device: &cpal::Device,
    app: &AppHandle,
    silence_cfg: SilenceConfig,
) -> CmdResult<ActiveRecording> {
    let device_name = device.name().unwrap_or_else(|_| "Unknown".into());

    let config = select_input_config(device)?;
    let sample_rate = config.sample_rate().0;

    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    let samples_cb = samples.clone();

    let app_cb = app.clone();
    let last_emit = Arc::new(Mutex::new(Instant::now()));
    let silence_triggered = Arc::new(AtomicBool::new(false));

    let temp_path = std::env::temp_dir()
        .join(format!("localvoice_{}.wav", uuid::Uuid::new_v4()));

    // Silence tracking state shared with the audio callback.
    let silence_start = Arc::new(Mutex::new(None::<Instant>));

    let stream = match config.sample_format() {
        SampleFormat::F32 => build_stream_f32(
            device, &config.into(), samples_cb, app_cb, last_emit,
            silence_cfg, silence_triggered.clone(), silence_start,
        )?,
        SampleFormat::I16 => build_stream_i16(
            device, &config.into(), samples_cb, app_cb, last_emit,
            silence_cfg, silence_triggered.clone(), silence_start,
        )?,
        SampleFormat::U8 => build_stream_u8(
            device, &config.into(), samples_cb, app_cb, last_emit,
            silence_cfg, silence_triggered.clone(), silence_start,
        )?,
        other => return Err(format!("Unsupported sample format: {other:?}").into()),
    };

    stream
        .play()
        .map_err(|e| format!("Failed to start audio stream: {e}"))?;

    Ok(ActiveRecording {
        stream,
        samples,
        temp_path,
        sample_rate,
        device_name,
        silence_triggered,
    })
}

/// Stops capture, writes a WAV file, and returns its path.
pub fn stop_capture(recording: ActiveRecording) -> CmdResult<String> {
    // Drop the stream first to flush any buffered samples.
    drop(recording.stream);

    let samples = recording.samples.lock().unwrap();
    if samples.is_empty() {
        return Err("No audio data captured".into());
    }

    crate::audio::wav_writer::write_wav(&samples, recording.sample_rate, &recording.temp_path)?;

    Ok(recording
        .temp_path
        .to_string_lossy()
        .into_owned())
}

/// Cancels capture without writing any file.
pub fn cancel_capture(recording: ActiveRecording) {
    drop(recording.stream);
    let _ = std::fs::remove_file(&recording.temp_path);
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Picks the best supported input config for whisper.cpp compatibility.
fn select_input_config(device: &cpal::Device) -> CmdResult<SupportedStreamConfig> {
    if let Ok(configs) = device.supported_input_configs() {
        for range in configs {
            if range.channels() == TARGET_CHANNELS
                && range.min_sample_rate().0 <= TARGET_SAMPLE_RATE
                && range.max_sample_rate().0 >= TARGET_SAMPLE_RATE
            {
                return Ok(range.with_sample_rate(SampleRate(TARGET_SAMPLE_RATE)));
            }
        }
    }
    device
        .default_input_config()
        .map_err(|e| format!("No supported input config: {e}").into())
}

/// Checks silence state and sets the `silence_triggered` flag if silence exceeds timeout.
fn check_silence(
    rms: f32,
    cfg: &SilenceConfig,
    silence_start: &Mutex<Option<Instant>>,
    silence_triggered: &AtomicBool,
) {
    if !cfg.enabled {
        return;
    }

    let mut start = silence_start.lock().unwrap();
    if rms < cfg.threshold {
        // Audio is silent.
        let began = start.get_or_insert_with(Instant::now);
        if began.elapsed() >= Duration::from_millis(cfg.timeout_ms) {
            silence_triggered.store(true, Ordering::Relaxed);
        }
    } else {
        // Audio is loud enough — reset the silence timer.
        *start = None;
    }
}

fn build_stream_f32(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Arc<Mutex<Vec<i16>>>,
    app: AppHandle,
    last_emit: Arc<Mutex<Instant>>,
    silence_cfg: SilenceConfig,
    silence_triggered: Arc<AtomicBool>,
    silence_start: Arc<Mutex<Option<Instant>>>,
) -> CmdResult<cpal::Stream> {
    device
        .build_input_stream(
            config,
            move |data: &[f32], _| {
                let chunk: Vec<i16> = data
                    .iter()
                    .map(|&x| (x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                    .collect();
                samples.lock().unwrap().extend_from_slice(&chunk);
                let rms = calculate_rms(data);
                maybe_emit_level(&app, &last_emit, rms);
                check_silence(rms, &silence_cfg, &silence_start, &silence_triggered);
            },
            |err| log::error!("Audio capture error: {err}"),
            None,
        )
        .map_err(|e| format!("Failed to build audio stream: {e}").into())
}

fn build_stream_i16(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Arc<Mutex<Vec<i16>>>,
    app: AppHandle,
    last_emit: Arc<Mutex<Instant>>,
    silence_cfg: SilenceConfig,
    silence_triggered: Arc<AtomicBool>,
    silence_start: Arc<Mutex<Option<Instant>>>,
) -> CmdResult<cpal::Stream> {
    device
        .build_input_stream(
            config,
            move |data: &[i16], _| {
                samples.lock().unwrap().extend_from_slice(data);
                let rms = {
                    let sum_sq: f32 = data
                        .iter()
                        .map(|&x| (x as f32 / i16::MAX as f32).powi(2))
                        .sum();
                    (sum_sq / data.len() as f32).sqrt()
                };
                maybe_emit_level(&app, &last_emit, rms);
                check_silence(rms, &silence_cfg, &silence_start, &silence_triggered);
            },
            |err| log::error!("Audio capture error: {err}"),
            None,
        )
        .map_err(|e| format!("Failed to build audio stream: {e}").into())
}

fn build_stream_u8(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Arc<Mutex<Vec<i16>>>,
    app: AppHandle,
    last_emit: Arc<Mutex<Instant>>,
    silence_cfg: SilenceConfig,
    silence_triggered: Arc<AtomicBool>,
    silence_start: Arc<Mutex<Option<Instant>>>,
) -> CmdResult<cpal::Stream> {
    device
        .build_input_stream(
            config,
            move |data: &[u8], _| {
                let converted: Vec<i16> = data
                    .iter()
                    .map(|&x| ((x as i16) - 128) * 256)
                    .collect();
                let rms = {
                    let sum_sq: f32 = converted
                        .iter()
                        .map(|&x| (x as f32 / i16::MAX as f32).powi(2))
                        .sum();
                    (sum_sq / converted.len() as f32).sqrt()
                };
                samples.lock().unwrap().extend_from_slice(&converted);
                maybe_emit_level(&app, &last_emit, rms);
                check_silence(rms, &silence_cfg, &silence_start, &silence_triggered);
            },
            |err| log::error!("Audio capture error: {err}"),
            None,
        )
        .map_err(|e| format!("Failed to build audio stream: {e}").into())
}

/// Emits `audio-level` at most once per ~80 ms to avoid flooding the frontend.
fn maybe_emit_level(app: &AppHandle, last_emit: &Mutex<Instant>, rms: f32) {
    let mut last = last_emit.lock().unwrap();
    if last.elapsed() >= Duration::from_millis(80) {
        *last = Instant::now();
        let _ = app.emit("audio-level", rms);
    }
}
