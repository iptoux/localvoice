import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./index.css";
import { Pill } from "./components/pill/Pill";
import { useAppStore } from "./stores/app-store";
import type { RecordingStatePayload } from "./types";

export function PillApp() {
  const setRecordingState = useAppStore((s) => s.setRecordingState);
  const setAudioLevel = useAppStore((s) => s.setAudioLevel);
  const setRecordingError = useAppStore((s) => s.setRecordingError);

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

    return () => {
      unlistenState.then((fn) => fn());
      unlistenLevel.then((fn) => fn());
    };
  }, [setRecordingState, setAudioLevel, setRecordingError]);

  return (
    <div className="w-full h-full flex items-center justify-center p-1">
      <Pill />
    </div>
  );
}
