use cpal::traits::{DeviceTrait, HostTrait};

use crate::errors::CmdResult;
use crate::state::DeviceInfo;

/// Returns all available audio input devices for the default host.
/// The first device with `is_default = true` is the system default.
pub fn list_input_devices() -> CmdResult<Vec<DeviceInfo>> {
    let host = cpal::default_host();

    let default_name = host
        .default_input_device()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    let devices = host
        .input_devices()
        .map_err(|e| format!("Failed to enumerate audio devices: {e}"))?;

    let mut result = Vec::new();
    for device in devices {
        let name = device.name().unwrap_or_else(|_| "Unknown device".into());
        let is_default = name == default_name;
        result.push(DeviceInfo {
            id: name.clone(),
            name,
            is_default,
        });
    }

    Ok(result)
}

/// Finds an input device by its name-based id.
/// Returns the default input device when `device_id` is None.
pub fn get_input_device(device_id: Option<&str>) -> CmdResult<cpal::Device> {
    let host = cpal::default_host();

    match device_id {
        None => host
            .default_input_device()
            .ok_or_else(|| "No default input device found".into()),
        Some(id) => {
            let devices = host
                .input_devices()
                .map_err(|e| format!("Failed to enumerate devices: {e}"))?;
            devices
                .filter(|d| d.name().map(|n| n == id).unwrap_or(false))
                .next()
                .ok_or_else(|| format!("Audio device not found: {id}").into())
        }
    }
}
