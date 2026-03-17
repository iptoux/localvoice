import { create } from "zustand";
import type { RecordingState } from "../types";

interface AppStore {
  recordingState: RecordingState;
  setRecordingState: (state: RecordingState) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  recordingState: "idle",
  setRecordingState: (recordingState) => set({ recordingState }),
}));
