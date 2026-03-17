import { useAppStore } from "../../stores/app-store";
import { openMainWindow } from "../../lib/tauri";
import type { RecordingState } from "../../types";

const STATE_LABEL: Record<RecordingState, string> = {
  idle: "Ready",
  listening: "Listening...",
  processing: "Transcribing...",
  success: "Done",
  error: "Error",
};

const STATE_COLOR: Record<RecordingState, string> = {
  idle: "bg-neutral-800",
  listening: "bg-red-600",
  processing: "bg-amber-500",
  success: "bg-green-600",
  error: "bg-rose-700",
};

export function Pill() {
  const recordingState = useAppStore((s) => s.recordingState);

  return (
    <div
      data-tauri-drag-region
      onDoubleClick={() => openMainWindow()}
      className={`
        flex items-center gap-2 px-4 h-16 w-full rounded-full select-none cursor-default
        ${STATE_COLOR[recordingState]}
        text-white text-sm font-medium shadow-lg
        transition-colors duration-200
      `}
    >
      <MicIcon state={recordingState} />
      {/* data-tauri-drag-region on children prevents them from swallowing the mousedown on Windows */}
      <span data-tauri-drag-region>{STATE_LABEL[recordingState]}</span>
    </div>
  );
}

function MicIcon({ state }: { state: RecordingState }) {
  const pulse = state === "listening";
  return (
    <div
      data-tauri-drag-region
      className={`w-4 h-4 rounded-full border-2 border-white/80 flex-shrink-0 ${
        pulse ? "animate-pulse" : ""
      }`}
    />
  );
}
