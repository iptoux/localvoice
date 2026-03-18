import { useEffect, useRef, useState } from "react";
import { useAppStore } from "../../stores/app-store";
import { expandPill, collapsePill, openMainWindow } from "../../lib/tauri";
import { Waveform } from "./Waveform";
import { ExpandedPill } from "./ExpandedPill";
import type { RecordingState } from "../../types";

const STATE_COLOR: Record<RecordingState, string> = {
  idle: "bg-card/80",
  listening: "bg-red-600/90",
  processing: "bg-amber-500/90",
  success: "bg-green-600/90",
  error: "bg-rose-700/90",
};

/** Duration (ms) before the success state fades back to idle. */
const SUCCESS_DISPLAY_MS = 3000;

export function Pill() {
  const recordingState = useAppStore((s) => s.recordingState);
  const setRecordingState = useAppStore((s) => s.setRecordingState);
  const recordingError = useAppStore((s) => s.recordingError);
  const isPillExpanded = useAppStore((s) => s.isPillExpanded);
  const setIsPillExpanded = useAppStore((s) => s.setIsPillExpanded);

  // Track whether we're fading out from success → idle.
  const [isFadingOut, setIsFadingOut] = useState(false);

  // Auto-transition: success → idle after 3 seconds with opacity fade.
  useEffect(() => {
    if (recordingState !== "success") {
      setIsFadingOut(false);
      return;
    }

    // Start the fade-out slightly before resetting to idle.
    const fadeTimer = setTimeout(() => setIsFadingOut(true), SUCCESS_DISPLAY_MS - 300);
    const resetTimer = setTimeout(() => {
      setRecordingState("idle");
      setIsFadingOut(false);
    }, SUCCESS_DISPLAY_MS);

    return () => {
      clearTimeout(fadeTimer);
      clearTimeout(resetTimer);
    };
  }, [recordingState, setRecordingState]);

  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault();
    if (isPillExpanded) {
      collapsePill().then(() => setIsPillExpanded(false));
    } else {
      expandPill().then(() => setIsPillExpanded(true));
    }
  };

  const handleDoubleClick = () => {
    if (isPillExpanded) {
      collapsePill().then(() => setIsPillExpanded(false));
    }
    openMainWindow();
  };

  // Collapse on window blur.
  useEffect(() => {
    const handleBlur = () => {
      if (isPillExpanded) {
        collapsePill().then(() => setIsPillExpanded(false));
      }
    };
    window.addEventListener("blur", handleBlur);
    return () => window.removeEventListener("blur", handleBlur);
  }, [isPillExpanded, setIsPillExpanded]);

  return (
    <div
      className={`
        w-full rounded-tl-2xl rounded-br-2xl select-none cursor-default overflow-hidden
        ${STATE_COLOR[recordingState]}
        backdrop-blur-xl
        text-foreground shadow-lg border border-border
        transition-all duration-300 ease-in-out
        ${isFadingOut ? "opacity-80" : "opacity-100"}
      `}
    >
      {/* Compact pill bar — always visible */}
      <div
        data-tauri-drag-region
        onContextMenu={handleContextMenu}
        onDoubleClick={handleDoubleClick}
        className="flex items-center gap-2 px-4 h-16 text-sm font-medium"
      >
        <StateIcon state={recordingState} />
        <span data-tauri-drag-region className="flex-1 truncate">
          {recordingState === "error" && recordingError ? (
            recordingError
          ) : recordingState === "success" ? (
            <SuccessContent />
          ) : recordingState === "listening" ? (
            <Waveform />
          ) : recordingState === "processing" ? (
            "Transcribing…"
          ) : (
            "LocalVoice"
          )}
        </span>
        {recordingState === "listening" && <ElapsedTimer />}
      </div>

      {/* Expanded content — shown below the bar when expanded */}
      {isPillExpanded && <ExpandedPill />}
    </div>
  );
}

// ── Success content ──────────────────────────────────────────────────────────

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

// ── State icon ───────────────────────────────────────────────────────────────

function StateIcon({ state }: { state: RecordingState }) {
  const isColored = state !== "idle";
  const strokeColor = isColored ? "white" : "currentColor";

  switch (state) {
    case "listening":
      return (
        <div
          data-tauri-drag-region
          className="w-4 h-4 rounded-full bg-white/80 flex-shrink-0 animate-pulse"
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
          stroke={strokeColor}
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
          stroke={strokeColor}
          strokeWidth="2"
          strokeLinecap="round"
        >
          <line x1="8" y1="3" x2="8" y2="9" />
          <circle cx="8" cy="12" r="1" fill={strokeColor} />
        </svg>
      );
    default:
      return (
        <svg
          data-tauri-drag-region
          className="w-4 h-4 flex-shrink-0 text-foreground/70"
          viewBox="0 0 16 16"
          fill="none"
          stroke="currentColor"
          strokeWidth="1.5"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <rect x="5" y="1" width="6" height="9" rx="3" />
          <path d="M2 8a6 6 0 0 0 12 0" />
          <line x1="8" y1="14" x2="8" y2="15" />
          <line x1="5" y1="15" x2="11" y2="15" />
        </svg>
      );
  }
}

// ── Elapsed timer ────────────────────────────────────────────────────────────

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
      className="text-muted-foreground text-xs tabular-nums flex-shrink-0"
    >
      {formatted}
    </span>
  );
}
