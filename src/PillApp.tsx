import { useEffect, useState } from "react";
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
  PillMode,
  TranscriptionStreamUpdate,
  TranscriptionResult,
} from "./types";

function normalizePillMode(value: string | undefined): PillMode {
  return value === "classic" ? "classic" : "overlay";
}

export function PillApp() {
  const [pillMode, setPillMode] = useState<PillMode>("overlay");
  const setRecordingState = useAppStore((s) => s.setRecordingState);
  const setAudioLevel = useAppStore((s) => s.setAudioLevel);
  const setRecordingError = useAppStore((s) => s.setRecordingError);
  const setLastTranscription = useAppStore((s) => s.setLastTranscription);
  const setLastOutputResult = useAppStore((s) => s.setLastOutputResult);
  const setStreamingTranscription = useAppStore((s) => s.setStreamingTranscription);
  const resetStreamingTranscription = useAppStore((s) => s.resetStreamingTranscription);

  // Apply persisted theme on mount and react to theme changes from main window.
  useEffect(() => {
    let currentTheme: Theme = "dark";
    getSettings()
      .then((s) => {
        currentTheme = (s["app.theme"] as Theme) || "dark";
        setPillMode(normalizePillMode(s["ui.pill.mode"]));
        applyTheme(currentTheme);
      })
      .catch(() => applyTheme("dark"));

    const unlistenTheme = listen<string>("theme-changed", (event) => {
      currentTheme = event.payload as Theme;
      applyTheme(currentTheme);
    });
    const unlistenPillMode = listen<string>("pill-mode-changed", (event) => {
      setPillMode(normalizePillMode(event.payload));
    });

    const unlistenSystem = watchSystemTheme(() => currentTheme);

    return () => {
      unlistenTheme.then((fn) => fn());
      unlistenPillMode.then((fn) => fn());
      unlistenSystem();
    };
  }, []);

  // High-frequency event: throttle audio level updates to one per animation frame.
  useThrottledEvent<number>("audio-level", setAudioLevel);

  // Discrete, low-frequency events: no throttling needed.
  useTauriEvent<RecordingStatePayload>("recording-state-changed", (event) => {
    setRecordingState(event.payload.state);
    setRecordingError(event.payload.error ?? null);
    if (event.payload.state === "listening" || event.payload.state === "idle" || event.payload.state === "error") {
      resetStreamingTranscription();
    }
  });

  useTauriEvent<TranscriptionResult>("transcription-completed", (event) => {
    setLastTranscription(event.payload);
    resetStreamingTranscription();
  });

  useTauriEvent<TranscriptionStreamUpdate>("transcription-stream-update", (event) => {
    setStreamingTranscription(event.payload);
  });

  useTauriEvent<OutputResultPayload>("output-result", (event) => {
    setLastOutputResult(event.payload);
  });

  return (
    <div className={pillMode === "overlay" ? "w-full h-full" : "w-full h-full p-1"}>
      <Pill mode={pillMode} />
    </div>
  );
}
