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

/// Starts audio capture on the given device and returns an [ActiveRecording].
///
/// The caller must store the returned value in AppState to keep the stream alive.
/// Dropping the [ActiveRecording] automatically stops the cpal stream.
pub fn start_capture(
    device: &cpal::Device,
    app: &AppHandle,
) -> CmdResult<ActiveRecording> {
    let device_name = device.name().unwrap_or_else(|_| "Unknown".into());

    let config = select_input_config(device)?;
    let sample_rate = config.sample_rate().0;

    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    let samples_cb = samples.clone();

    let app_cb = app.clone();
    let last_emit = Arc::new(Mutex::new(Instant::now()));

    let temp_path = std::env::temp_dir()
        .join(format!("localvoice_{}.wav", uuid::Uuid::new_v4()));

    let stream = match config.sample_format() {
        SampleFormat::F32 => build_stream_f32(device, &config.into(), samples_cb, app_cb, last_emit)?,
        SampleFormat::I16 => build_stream_i16(device, &config.into(), samples_cb, app_cb, last_emit)?,
        SampleFormat::U8 => build_stream_u8(device, &config.into(), samples_cb, app_cb, last_emit)?,
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
/// Deletes the temp path reservation if it exists on disk (it shouldn't yet, but
/// guards against partial writes if we ever buffer-to-file in the future).
pub fn cancel_capture(recording: ActiveRecording) {
    drop(recording.stream);
    let _ = std::fs::remove_file(&recording.temp_path);
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Picks the best supported input config for whisper.cpp compatibility.
fn select_input_config(device: &cpal::Device) -> CmdResult<SupportedStreamConfig> {
    // Prefer 16 kHz mono.
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

    // Fall back to the device default — the WAV will record at whatever rate the
    // device supports. MS-03 transcription will handle resampling if needed.
    device
        .default_input_config()
        .map_err(|e| format!("No supported input config: {e}").into())
}

fn build_stream_f32(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Arc<Mutex<Vec<i16>>>,
    app: AppHandle,
    last_emit: Arc<Mutex<Instant>>,
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
                maybe_emit_level(&app, &last_emit, calculate_rms(data));
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
