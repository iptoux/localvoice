import { invoke } from "@tauri-apps/api/core";
import type {
  DeviceInfo,
  RecordingState,
  Settings,
  TranscriptionResult,
} from "../types";

// ── Settings ──────────────────────────────────────────────────────────────────

export const getSettings = (): Promise<Settings> =>
  invoke<Settings>("get_settings");

export const updateSetting = (key: string, value: string): Promise<void> =>
  invoke<void>("update_setting", { key, value });

export const resetSettings = (): Promise<void> =>
  invoke<void>("reset_settings");

// ── Window ────────────────────────────────────────────────────────────────────

export const showPill = (): Promise<void> => invoke<void>("show_pill");

export const hidePill = (): Promise<void> => invoke<void>("hide_pill");

export const openMainWindow = (): Promise<void> =>
  invoke<void>("open_main_window");

export const setPillPosition = (x: number, y: number): Promise<void> =>
  invoke<void>("set_pill_position", { x, y });

// ── Recording ─────────────────────────────────────────────────────────────────

export const startRecording = (): Promise<void> =>
  invoke<void>("start_recording");

export const stopRecording = (): Promise<string> =>
  invoke<string>("stop_recording");

export const cancelRecording = (): Promise<void> =>
  invoke<void>("cancel_recording");

export const getRecordingState = (): Promise<RecordingState> =>
  invoke<RecordingState>("get_recording_state");

export const listInputDevices = (): Promise<DeviceInfo[]> =>
  invoke<DeviceInfo[]>("list_input_devices");

// ── Transcription ─────────────────────────────────────────────────────────────

export const transcribeLastRecording = (
  language?: string,
  modelId?: string
): Promise<TranscriptionResult> =>
  invoke<TranscriptionResult>("transcribe_last_recording", {
    language: language ?? null,
    modelId: modelId ?? null,
  });

export const getLastTranscription = (): Promise<TranscriptionResult | null> =>
  invoke<TranscriptionResult | null>("get_last_transcription");
