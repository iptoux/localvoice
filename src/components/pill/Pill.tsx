import { useEffect, useRef, useState } from "react";
import { useAppStore } from "../../stores/app-store";
import { openMainWindow } from "../../lib/tauri";
import type { RecordingState } from "../../types";

const STATE_COLOR: Record<RecordingState, string> = {
  idle: "bg-neutral-800",
  listening: "bg-red-600",
  processing: "bg-amber-500",
  success: "bg-green-600",
  error: "bg-rose-700",
};

export function Pill() {
  const recordingState = useAppStore((s) => s.recordingState);
  const recordingError = useAppStore((s) => s.recordingError);

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
      <StateIcon state={recordingState} />
      <span data-tauri-drag-region className="flex-1 truncate">
        {recordingState === "error" && recordingError ? (
          recordingError
        ) : recordingState === "success" ? (
          <SuccessContent />
        ) : recordingState === "listening" ? (
          "Listening…"
        ) : recordingState === "processing" ? (
          "Transcribing…"
        ) : (
          "Ready"
        )}
      </span>
      {recordingState === "listening" && <ElapsedTimer />}
    </div>
  );
}

// ── Success content (TASK-059, TASK-061) ─────────────────────────────────────

function SuccessContent() {
  const lastTranscription = useAppStore((s) => s.lastTranscription);
  const lastOutputResult = useAppStore((s) => s.lastOutputResult);

  const text = lastTranscription?.cleanedText ?? "Done";
  const preview = text.length > 32 ? text.slice(0, 30) + "…" : text;

  const modeLabel =
    lastOutputResult?.mode === "insert" ? "Inserted" : "Copied";

  return (
    <span data-tauri-drag-region className="flex items-center gap-2 min-w-0">
      <OutputBadge label={modeLabel} success={lastOutputResult?.success ?? true} />
      <span data-tauri-drag-region className="truncate" title={text}>
        {preview}
      </span>
    </span>
  );
}

function OutputBadge({
  label,
  success,
}: {
  label: string;
  success: boolean;
}) {
  return (
    <span
      data-tauri-drag-region
      className={`
        flex-shrink-0 text-xs px-1.5 py-0.5 rounded font-semibold
        ${success ? "bg-white/20 text-white" : "bg-rose-900/60 text-rose-200"}
      `}
    >
      {success ? label : "Failed"}
    </span>
  );
}

// ── State icon ────────────────────────────────────────────────────────────────

function StateIcon({ state }: { state: RecordingState }) {
  switch (state) {
    case "listening":
      return (
        <div
          data-tauri-drag-region
          className="w-4 h-4 rounded-full border-2 border-white/80 flex-shrink-0 animate-pulse"
        />
      );
    case "processing":
      return (
        <div
          data-tauri-drag-region
          className="w-4 h-4 flex-shrink-0 border-2 border-white/80 border-t-transparent rounded-full animate-spin"
        />
      );
    case "success":
      return (
        <svg
          data-tauri-drag-region
          className="w-4 h-4 flex-shrink-0"
          viewBox="0 0 16 16"
          fill="none"
          stroke="white"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <polyline points="2,8 6,12 14,4" />
        </svg>
      );
    case "error":
      return (
        <svg
          data-tauri-drag-region
          className="w-4 h-4 flex-shrink-0"
          viewBox="0 0 16 16"
          fill="none"
          stroke="white"
          strokeWidth="2"
          strokeLinecap="round"
        >
          <line x1="8" y1="3" x2="8" y2="9" />
          <circle cx="8" cy="12" r="1" fill="white" />
        </svg>
      );
    default:
      // idle — mic circle
      return (
        <div
          data-tauri-drag-region
          className="w-4 h-4 rounded-full border-2 border-white/80 flex-shrink-0"
        />
      );
  }
}

// ── Elapsed timer (shown only in Listening state) ─────────────────────────────

function ElapsedTimer() {
  const [elapsed, setElapsed] = useState(0);
  const startRef = useRef(Date.now());

  useEffect(() => {
    startRef.current = Date.now();
    setElapsed(0);

    const id = setInterval(() => {
      setElapsed(Math.floor((Date.now() - startRef.current) / 1000));
    }, 1000);

    return () => clearInterval(id);
  }, []);

  const minutes = Math.floor(elapsed / 60);
  const seconds = elapsed % 60;
  const formatted = `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;

  return (
    <span
      data-tauri-drag-region
      className="text-white/70 text-xs tabular-nums flex-shrink-0"
    >
      {formatted}
    </span>
  );
}
