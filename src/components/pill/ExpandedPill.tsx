import { useEffect } from "react";
import { useAppStore } from "../../stores/app-store";
import {
  startRecording,
  stopRecording,
  openMainWindowAt,
  updateSetting,
} from "../../lib/tauri";
import { useSettingsStore } from "../../stores/settings-store";

export function ExpandedPill() {
  const recordingState = useAppStore((s) => s.recordingState);
  const lastTranscription = useAppStore((s) => s.lastTranscription);
  const lastOutputResult = useAppStore((s) => s.lastOutputResult);
  const load = useSettingsStore((s) => s.load);
  const language = useSettingsStore((s) => s.settings["transcription.default_language"] || "auto");

  useEffect(() => { load(); }, [load]);
  const modelId = lastTranscription?.modelId ?? "—";
  const wordCount = lastTranscription?.cleanedText.split(/\s+/).filter(Boolean).length ?? 0;
  const transcript = lastTranscription?.cleanedText ?? "";

  const isRecording = recordingState === "listening";
  const isProcessing = recordingState === "processing";
  const isIdle = recordingState === "idle";

  const handleToggleRecord = () => {
    if (isRecording) {
      stopRecording().catch(console.error);
    } else if (isIdle) {
      startRecording().catch(console.error);
    }
  };

  const handleLanguageChange = (lang: string) => {
    updateSetting("transcription.default_language", lang).then(load);
  };

  const handleCopyAgain = () => {
    if (transcript) {
      navigator.clipboard.writeText(transcript).catch(console.error);
    }
  };

  return (
    <div className="flex flex-col gap-2 px-3 pt-1 pb-2 text-foreground text-xs select-none overflow-hidden">
      {/* Transcript preview */}
      <div className="bg-foreground/10 rounded-md px-2.5 py-2 max-h-20 overflow-y-auto text-[11px] leading-relaxed text-foreground/90">
        {transcript || (
          <span className="text-foreground/40 italic">No transcript yet</span>
        )}
      </div>

      {/* Metadata row */}
      <div className="flex items-center gap-2">
        <LanguageBadge language={language} />
        <span className="text-foreground/40 truncate text-[10px]">{modelId}</span>
        {wordCount > 0 && (
          <span className="text-foreground/40 text-[10px] ml-auto">
            {wordCount} {wordCount === 1 ? "word" : "words"}
          </span>
        )}
      </div>

      {/* Language quick-switch */}
      <div className="flex items-center gap-1">
        {["auto", "de", "en"].map((lang) => (
          <button
            key={lang}
            onClick={() => handleLanguageChange(lang)}
            className={`px-2 py-0.5 rounded text-[10px] font-semibold uppercase transition-colors ${
              language === lang
                ? "bg-foreground/20 text-foreground"
                : "bg-foreground/5 text-foreground/40 hover:bg-foreground/10 hover:text-foreground/70"
            }`}
          >
            {lang}
          </button>
        ))}
      </div>

      {/* Start/Stop button */}
      <button
        onClick={handleToggleRecord}
        disabled={isProcessing}
        className={`w-full py-1.5 rounded-md text-[11px] font-semibold transition-all ${
          isRecording
            ? "bg-red-500 hover:bg-red-400 text-white"
            : isProcessing
              ? "bg-foreground/10 text-foreground/30 cursor-not-allowed"
              : "bg-foreground/15 hover:bg-foreground/25 text-foreground"
        }`}
      >
        {isRecording ? "Stop Recording" : isProcessing ? "Transcribing…" : "Start Recording"}
      </button>

      {/* Quick actions */}
      <div className="flex items-center gap-1.5">
        <QuickAction
          label="Copy"
          disabled={!transcript}
          onClick={handleCopyAgain}
        />
        <QuickAction label="History" onClick={() => openMainWindowAt("/history")} />
        <QuickAction label="Settings" onClick={() => openMainWindowAt("/settings")} />
      </div>

      {/* Output status */}
      {lastOutputResult && (
        <div
          className={`text-center text-[10px] py-0.5 rounded ${
            lastOutputResult.success
              ? "text-green-600 dark:text-green-300/70"
              : "text-rose-600 dark:text-rose-300/70"
          }`}
        >
          {lastOutputResult.success
            ? lastOutputResult.mode === "insert"
              ? "Inserted into app"
              : "Copied to clipboard"
            : "Output failed"}
        </div>
      )}
    </div>
  );
}

function LanguageBadge({ language }: { language: string }) {
  return (
    <span className="bg-foreground/15 text-foreground/80 px-1.5 py-0.5 rounded text-[10px] font-mono uppercase">
      {language}
    </span>
  );
}

function QuickAction({
  label,
  onClick,
  disabled,
}: {
  label: string;
  onClick: () => void;
  disabled?: boolean;
}) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className="flex-1 py-1 rounded bg-foreground/5 text-foreground/50 hover:bg-foreground/10 hover:text-foreground/80 text-[10px] transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
    >
      {label}
    </button>
  );
}
