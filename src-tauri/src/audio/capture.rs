use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, StreamTrait};
#[cfg(not(target_os = "windows"))]
use cpal::SampleRate;
use cpal::{SampleFormat, SupportedStreamConfig};
use tauri::{AppHandle, Emitter};

use crate::audio::level_meter::calculate_rms;
use crate::errors::CmdResult;
use crate::state::recording_state::ActiveRecording;

/// Target capture format — whisper.cpp expects 16 kHz 16-bit mono.
const TARGET_SAMPLE_RATE: u32 = 16_000;
#[cfg(not(target_os = "windows"))]
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
    let capture_sample_rate = config.sample_rate().0;
    let capture_channels = config.channels() as usize;

    log::info!(
        "Audio capture: device={device_name}, rate={capture_sample_rate} Hz, channels={capture_channels}, format={:?}",
        config.sample_format()
    );

    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    let samples_cb = samples.clone();

    let last_emit = Arc::new(Mutex::new(Instant::now()));
    let silence_triggered = Arc::new(AtomicBool::new(false));
    let level_stop = Arc::new(AtomicBool::new(false));
    let (level_tx, mut level_rx) = tokio::sync::mpsc::unbounded_channel::<f32>();
    let level_stop_task = level_stop.clone();
    let app_level = app.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(rms) = level_rx.recv().await {
            if level_stop_task.load(Ordering::Relaxed) {
                break;
            }
            let _ = app_level.emit("audio-level", rms);
        }
    });

    let temp_path = std::env::temp_dir().join(format!("localvoice_{}.wav", uuid::Uuid::new_v4()));

    // Silence tracking state shared with the audio callback.
    let silence_start = Arc::new(Mutex::new(None::<Instant>));

    let stream = match config.sample_format() {
        SampleFormat::F32 => build_stream_f32(
            device,
            &config.into(),
            samples_cb,
            level_tx.clone(),
            last_emit,
            silence_cfg,
            silence_triggered.clone(),
            silence_start,
            capture_sample_rate,
            capture_channels,
        )?,
        SampleFormat::I16 => build_stream_i16(
            device,
            &config.into(),
            samples_cb,
            level_tx.clone(),
            last_emit,
            silence_cfg,
            silence_triggered.clone(),
            silence_start,
            capture_sample_rate,
            capture_channels,
        )?,
        SampleFormat::U8 => build_stream_u8(
            device,
            &config.into(),
            samples_cb,
            level_tx.clone(),
            last_emit,
            silence_cfg,
            silence_triggered.clone(),
            silence_start,
            capture_sample_rate,
            capture_channels,
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
        // WAV is always written at TARGET_SAMPLE_RATE after resampling.
        sample_rate: TARGET_SAMPLE_RATE,
        device_name,
        silence_triggered,
        level_stop,
        level_tx,
    })
}

/// Stops capture, writes a WAV file, and returns its path.
pub fn stop_capture(recording: ActiveRecording) -> CmdResult<String> {
    // Drop the stream first to flush any buffered samples.
    drop(recording.stream);
    recording.level_stop.store(true, Ordering::Relaxed);
    drop(recording.level_tx);

    let samples = recording.samples.lock().unwrap();
    if samples.is_empty() {
        return Err("No audio data captured".into());
    }

    crate::audio::wav_writer::write_wav(&samples, recording.sample_rate, &recording.temp_path)?;

    Ok(recording.temp_path.to_string_lossy().into_owned())
}

/// Cancels capture without writing any file.
pub fn cancel_capture(recording: ActiveRecording) {
    drop(recording.stream);
    recording.level_stop.store(true, Ordering::Relaxed);
    drop(recording.level_tx);
    let _ = std::fs::remove_file(&recording.temp_path);
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Picks the best supported input config for whisper.cpp compatibility.
///
/// Priority:
/// 1. Mono 16 kHz (ideal — no conversion needed)
/// 2. Any config that supports 16 kHz (may be stereo — will be downmixed)
/// 3. Mono at any rate (will be resampled)
/// 4. Device default (will be downmixed + resampled)
fn select_input_config(device: &cpal::Device) -> CmdResult<SupportedStreamConfig> {
    #[cfg(target_os = "windows")]
    {
        return device
            .default_input_config()
            .map_err(|e| format!("No supported input config: {e}").into());
    }

    #[cfg(not(target_os = "windows"))]
    {
        let configs: Vec<_> = device
            .supported_input_configs()
            .map(|c| c.collect())
            .unwrap_or_default();

        // 1. Ideal: mono + supports 16 kHz exactly.
        for range in &configs {
            if range.channels() == TARGET_CHANNELS
                && range.min_sample_rate().0 <= TARGET_SAMPLE_RATE
                && range.max_sample_rate().0 >= TARGET_SAMPLE_RATE
            {
                return Ok(range.with_sample_rate(SampleRate(TARGET_SAMPLE_RATE)));
            }
        }

        // 2. Any channel count that supports 16 kHz (will downmix in callback).
        for range in &configs {
            if range.min_sample_rate().0 <= TARGET_SAMPLE_RATE
                && range.max_sample_rate().0 >= TARGET_SAMPLE_RATE
            {
                return Ok(range.with_sample_rate(SampleRate(TARGET_SAMPLE_RATE)));
            }
        }

        // 3. Mono at any rate (will resample in callback).
        for range in &configs {
            if range.channels() == TARGET_CHANNELS {
                let rate = range.max_sample_rate().0.min(48_000);
                return Ok(range.with_sample_rate(SampleRate(rate)));
            }
        }

        // 4. Fall back to device default — downmix + resample will handle it.
        device
            .default_input_config()
            .map_err(|e| format!("No supported input config: {e}").into())
    }
}

/// Downmixes interleaved multi-channel f32 samples to mono, then resamples
/// from `from_rate` to `TARGET_SAMPLE_RATE` using linear interpolation.
///
/// When `channels == 1` and `from_rate == TARGET_SAMPLE_RATE` this is a no-op
/// (returns the input converted to i16 directly).
fn downmix_and_resample(data: &[f32], channels: usize, from_rate: u32) -> Vec<i16> {
    // Step 1: downmix to mono f32.
    let mono: Vec<f32> = if channels == 1 {
        data.to_vec()
    } else {
        data.chunks_exact(channels)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32)
            .collect()
    };

    // Step 2: resample to TARGET_SAMPLE_RATE.
    if from_rate == TARGET_SAMPLE_RATE {
        return mono
            .iter()
            .map(|&x| (x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
            .collect();
    }

    let ratio = from_rate as f64 / TARGET_SAMPLE_RATE as f64;
    let out_len = (mono.len() as f64 / ratio).ceil() as usize;
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src_pos = i as f64 * ratio;
        let src_idx = src_pos as usize;
        let frac = (src_pos - src_idx as f64) as f32;

        let s0 = mono.get(src_idx).copied().unwrap_or(0.0);
        let s1 = mono.get(src_idx + 1).copied().unwrap_or(s0);
        let sample = s0 + (s1 - s0) * frac;
        out.push((sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16);
    }

    out
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
    level_tx: tokio::sync::mpsc::UnboundedSender<f32>,
    last_emit: Arc<Mutex<Instant>>,
    silence_cfg: SilenceConfig,
    silence_triggered: Arc<AtomicBool>,
    silence_start: Arc<Mutex<Option<Instant>>>,
    capture_rate: u32,
    capture_channels: usize,
) -> CmdResult<cpal::Stream> {
    device
        .build_input_stream(
            config,
            move |data: &[f32], _| {
                let chunk = downmix_and_resample(data, capture_channels, capture_rate);
                let rms = calculate_rms(data);
                samples.lock().unwrap().extend_from_slice(&chunk);
                maybe_emit_level(&level_tx, &last_emit, rms);
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
    level_tx: tokio::sync::mpsc::UnboundedSender<f32>,
    last_emit: Arc<Mutex<Instant>>,
    silence_cfg: SilenceConfig,
    silence_triggered: Arc<AtomicBool>,
    silence_start: Arc<Mutex<Option<Instant>>>,
    capture_rate: u32,
    capture_channels: usize,
) -> CmdResult<cpal::Stream> {
    device
        .build_input_stream(
            config,
            move |data: &[i16], _| {
                // Convert i16 -> f32 for unified downmix/resample path.
                let as_f32: Vec<f32> = data.iter().map(|&x| x as f32 / i16::MAX as f32).collect();
                let chunk = downmix_and_resample(&as_f32, capture_channels, capture_rate);
                let rms = {
                    let sum_sq: f32 = as_f32.iter().map(|&x| x.powi(2)).sum();
                    (sum_sq / as_f32.len() as f32).sqrt()
                };
                samples.lock().unwrap().extend_from_slice(&chunk);
                maybe_emit_level(&level_tx, &last_emit, rms);
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
    level_tx: tokio::sync::mpsc::UnboundedSender<f32>,
    last_emit: Arc<Mutex<Instant>>,
    silence_cfg: SilenceConfig,
    silence_triggered: Arc<AtomicBool>,
    silence_start: Arc<Mutex<Option<Instant>>>,
    capture_rate: u32,
    capture_channels: usize,
) -> CmdResult<cpal::Stream> {
    device
        .build_input_stream(
            config,
            move |data: &[u8], _| {
                // Convert u8 -> f32 for unified downmix/resample path.
                let as_f32: Vec<f32> = data.iter().map(|&x| (x as f32 - 128.0) / 128.0).collect();
                let chunk = downmix_and_resample(&as_f32, capture_channels, capture_rate);
                let rms = {
                    let sum_sq: f32 = as_f32.iter().map(|&x| x.powi(2)).sum();
                    (sum_sq / as_f32.len() as f32).sqrt()
                };
                samples.lock().unwrap().extend_from_slice(&chunk);
                maybe_emit_level(&level_tx, &last_emit, rms);
                check_silence(rms, &silence_cfg, &silence_start, &silence_triggered);
            },
            |err| log::error!("Audio capture error: {err}"),
            None,
        )
        .map_err(|e| format!("Failed to build audio stream: {e}").into())
}

/// Emits `audio-level` at most once per ~80 ms to avoid flooding the frontend.
fn maybe_emit_level(
    level_tx: &tokio::sync::mpsc::UnboundedSender<f32>,
    last_emit: &Mutex<Instant>,
    rms: f32,
) {
    let mut last = last_emit.lock().unwrap();
    if last.elapsed() >= Duration::from_millis(80) {
        *last = Instant::now();
        let _ = level_tx.send(rms);
    }
}
