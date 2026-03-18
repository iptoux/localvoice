import { useEffect, useRef, useState } from "react";
import { useAppStore } from "../../stores/app-store";
import { expandPill, collapsePill, openMainWindow } from "../../lib/tauri";
import { Waveform } from "./Waveform";
import { ExpandedPill } from "./ExpandedPill";
import type { RecordingState } from "../../types";

const STATE_COLOR: Record<RecordingState, string> = {
  idle: "bg-card",
  listening: "bg-red-600",
  processing: "bg-amber-500",
  success: "bg-green-600",
  error: "bg-rose-700",
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
        text-foreground shadow-lg border border-foreground/20
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
        {recordingState === "idle" ? (
          <IdleContent />
        ) : (
          <>
            <StateIcon state={recordingState} />
            <span data-tauri-drag-region className="flex-1 truncate">
              {recordingState === "error" && recordingError ? (
                recordingError
              ) : recordingState === "success" ? (
                <SuccessContent />
              ) : recordingState === "listening" ? (
                <Waveform />
              ) : (
                "Transcribing…"
              )}
            </span>
            {recordingState === "listening" && <ElapsedTimer />}
          </>
        )}
      </div>

      {/* Expanded content — shown below the bar when expanded */}
      {isPillExpanded && <ExpandedPill />}
    </div>
  );
}

// ── Idle content ─────────────────────────────────────────────────────────────

function IdleContent() {
  const lastTranscription = useAppStore((s) => s.lastTranscription);
  const wordCount = lastTranscription?.cleanedText
    ? lastTranscription.cleanedText.trim().split(/\s+/).filter(Boolean).length
    : undefined;

  return (
    <div
      data-tauri-drag-region
      className="flex items-center justify-start gap-3 w-full"
    >
      <img
        data-tauri-drag-region
        src="/localvoice_appiconbadge_transparent.png.png"
        alt="LocalVoice"
        className="w-8 h-8 flex-shrink-0 object-contain"
      />
      <span
        data-tauri-drag-region
        className="text-base font-bold tracking-tight bg-gradient-to-r from-foreground to-foreground/60 bg-clip-text text-transparent"
      >
        LocalVoice
      </span>
      {wordCount !== undefined && wordCount > 0 && (
        <span
          data-tauri-drag-region
          className="ml-auto text-xs text-foreground/30 tabular-nums"
        >
          {wordCount}w
        </span>
      )}
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
        <img
          data-tauri-drag-region
          src="/localvoice_appiconbadge_transparent.png.png"
          alt="LocalVoice"
          className="w-8 h-8 flex-shrink-0 object-contain"
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
        <img
          data-tauri-drag-region
          src="/localvoice_appiconbadge_transparent.png.png"
          alt="LocalVoice"
          className="w-8 h-8 flex-shrink-0"
        />
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
      className="text-white/90 text-xs tabular-nums flex-shrink-0 font-medium"
    >
      {formatted}
    </span>
  );
}
