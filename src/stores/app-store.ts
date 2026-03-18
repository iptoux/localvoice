import { create } from "zustand";
import type {
  DeviceInfo,
  OutputResult,
  RecordingState,
  TranscriptionResult,
} from "../types";

interface AppStore {
  recordingState: RecordingState;
  setRecordingState: (state: RecordingState) => void;

  /** Last emitted audio level from Rust (0–1 RMS). */
  audioLevel: number;
  setAudioLevel: (level: number) => void;

  /** Last error message from the Error state, if any. */
  recordingError: string | null;
  setRecordingError: (error: string | null) => void;

  /** Available input devices, populated by SettingsPage. */
  audioDevices: DeviceInfo[];
  setAudioDevices: (devices: DeviceInfo[]) => void;

  /** Most recent completed transcription result. */
  lastTranscription: TranscriptionResult | null;
  setLastTranscription: (result: TranscriptionResult | null) => void;

  /** Result of the most recent output step (clipboard / insert). */
  lastOutputResult: OutputResult | null;
  setLastOutputResult: (result: OutputResult | null) => void;

  /** Whether the pill is in expanded mode. */
  isPillExpanded: boolean;
  setIsPillExpanded: (expanded: boolean) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  recordingState: "idle",
  setRecordingState: (recordingState) => set({ recordingState }),

  audioLevel: 0,
  setAudioLevel: (audioLevel) => set({ audioLevel }),

  recordingError: null,
  setRecordingError: (recordingError) => set({ recordingError }),

  audioDevices: [],
  setAudioDevices: (audioDevices) => set({ audioDevices }),

  lastTranscription: null,
  setLastTranscription: (lastTranscription) => set({ lastTranscription }),

  lastOutputResult: null,
  setLastOutputResult: (lastOutputResult) => set({ lastOutputResult }),

  isPillExpanded: false,
  setIsPillExpanded: (isPillExpanded) => set({ isPillExpanded }),
}));
