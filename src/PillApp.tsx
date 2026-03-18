import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./index.css";
import { Pill } from "./components/pill/Pill";
import { useAppStore } from "./stores/app-store";
import { getSettings } from "./lib/tauri";
import { applyTheme, watchSystemTheme, type Theme } from "./lib/theme";
import { useThrottledEvent, useTauriEvent } from "./hooks/use-throttled-event";
import type {
  OutputResultPayload,
  RecordingStatePayload,
  TranscriptionResult,
} from "./types";

export function PillApp() {
  const setRecordingState = useAppStore((s) => s.setRecordingState);
  const setAudioLevel = useAppStore((s) => s.setAudioLevel);
  const setRecordingError = useAppStore((s) => s.setRecordingError);
  const setLastTranscription = useAppStore((s) => s.setLastTranscription);
  const setLastOutputResult = useAppStore((s) => s.setLastOutputResult);

  // Apply persisted theme on mount and react to theme changes from main window.
  useEffect(() => {
    let currentTheme: Theme = "dark";
    getSettings()
      .then((s) => {
        currentTheme = (s["app.theme"] as Theme) || "dark";
        applyTheme(currentTheme);
      })
      .catch(() => applyTheme("dark"));

    const unlistenTheme = listen<string>("theme-changed", (event) => {
      currentTheme = event.payload as Theme;
      applyTheme(currentTheme);
    });

    const unlistenSystem = watchSystemTheme(() => currentTheme);

    return () => {
      unlistenTheme.then((fn) => fn());
      unlistenSystem();
    };
  }, []);

  // High-frequency event: throttle audio level updates to one per animation frame.
  useThrottledEvent<number>("audio-level", setAudioLevel);

  // Discrete, low-frequency events: no throttling needed.
  useTauriEvent<RecordingStatePayload>("recording-state-changed", (event) => {
    setRecordingState(event.payload.state);
    setRecordingError(event.payload.error ?? null);
  });

  useTauriEvent<TranscriptionResult>("transcription-completed", (event) => {
    setLastTranscription(event.payload);
  });

  useTauriEvent<OutputResultPayload>("output-result", (event) => {
    setLastOutputResult(event.payload);
  });

  return (
    <div className="w-full h-full p-1">
      <Pill />
    </div>
  );
}
