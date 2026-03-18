import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./index.css";
import { Pill } from "./components/pill/Pill";
import { useAppStore } from "./stores/app-store";
import { getSettings } from "./lib/tauri";
import { applyTheme, watchSystemTheme, type Theme } from "./lib/theme";
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

  useEffect(() => {
    const unlistenState = listen<RecordingStatePayload>(
      "recording-state-changed",
      (event) => {
        setRecordingState(event.payload.state);
        setRecordingError(event.payload.error ?? null);
      }
    );

    const unlistenLevel = listen<number>("audio-level", (event) => {
      setAudioLevel(event.payload);
    });

    const unlistenTranscription = listen<TranscriptionResult>(
      "transcription-completed",
      (event) => {
        setLastTranscription(event.payload);
      }
    );

    const unlistenOutput = listen<OutputResultPayload>(
      "output-result",
      (event) => {
        setLastOutputResult(event.payload);
      }
    );

    return () => {
      unlistenState.then((fn) => fn());
      unlistenLevel.then((fn) => fn());
      unlistenTranscription.then((fn) => fn());
      unlistenOutput.then((fn) => fn());
    };
  }, [
    setRecordingState,
    setAudioLevel,
    setRecordingError,
    setLastTranscription,
    setLastOutputResult,
  ]);

  return (
    <div className="w-full h-full p-1">
      <Pill />
    </div>
  );
}
