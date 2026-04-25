# Windows audio capture callbacks should stay minimal

**Date:** 2026-04-25
**Area:** Tauri / Rust / Audio
**Milestone:** Hotfix

## What Happened

A Windows 10 crash report showed LocalVoice exiting immediately after recording started with exception code `0xc0000409`, reported as memory corruption in `localvoice.exe`.

## Root Cause

The crash could not be reproduced from the event log alone, but the recording path was doing too much work inside the real-time `cpal` input callback: sample conversion, locking, silence tracking, and Tauri event emission. On Windows audio driver callback threads, crossing into UI/event infrastructure from the callback increases the blast radius of driver or WASAPI instability.

## Fix / Solution

Audio-level events are now handed off through a Tokio channel and emitted from an async relay task outside the audio callback. Windows capture also uses the device default input configuration and resamples afterward instead of forcing a 16 kHz stream configuration that some drivers may advertise poorly.

## Learning / Rule of Thumb

Keep audio callbacks minimal and free of UI/event framework calls. Capture data, update small atomic/channel state, and move everything else to a normal application task.

## References

- `src-tauri/src/audio/capture.rs`
- Bug report: Windows exception code `0xc0000409` after starting recording
