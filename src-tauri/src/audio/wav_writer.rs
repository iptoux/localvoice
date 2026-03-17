use std::path::Path;

use hound::{SampleFormat, WavSpec, WavWriter};

use crate::errors::CmdResult;

/// Writes raw 16-bit PCM samples to a WAV file.
///
/// # Parameters
/// - `samples`     — 16-bit signed mono PCM samples
/// - `sample_rate` — sample rate in Hz (e.g. 16000)
/// - `path`        — destination file path (created / overwritten)
pub fn write_wav(samples: &[i16], sample_rate: u32, path: &Path) -> CmdResult<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer =
        WavWriter::create(path, spec).map_err(|e| format!("Failed to create WAV file: {e}"))?;

    for &sample in samples {
        writer
            .write_sample(sample)
            .map_err(|e| format!("Failed to write WAV sample: {e}"))?;
    }

    writer
        .finalize()
        .map_err(|e| format!("Failed to finalize WAV file: {e}"))?;

    Ok(())
}
