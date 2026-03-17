import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import type { ModelInfo, DownloadProgress } from "../types";
import { useModelsStore } from "../stores/models-store";

// ── helpers ───────────────────────────────────────────────────────────────────

function formatBytes(bytes: number): string {
  if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(1)} GB`;
  if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(0)} MB`;
  return `${(bytes / 1024).toFixed(0)} KB`;
}

// ── sub-components ────────────────────────────────────────────────────────────

interface DefaultSelectorProps {
  label: string;
  language: "de" | "en";
  models: ModelInfo[];
  currentKey: string | undefined;
  onSelect: (language: "de" | "en", key: string) => void;
}

function DefaultSelector({ label, language, models, currentKey, onSelect }: DefaultSelectorProps) {
  const installed = models.filter((m) => m.installed);

  return (
    <div className="flex items-center gap-3">
      <span className="text-sm text-neutral-400 w-32 shrink-0">{label}</span>
      {installed.length === 0 ? (
        <span className="text-xs text-neutral-500 italic">No installed models</span>
      ) : (
        <select
          className="bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-blue-500"
          value={currentKey ?? ""}
          onChange={(e) => onSelect(language, e.target.value)}
        >
          <option value="">— none —</option>
          {installed.map((m) => (
            <option key={m.key} value={m.key}>
              {m.displayName}
            </option>
          ))}
        </select>
      )}
    </div>
  );
}

interface ModelRowProps {
  model: ModelInfo;
  downloadState: { percent: number; bytesDownloaded: number; totalBytes: number } | undefined;
  onDownload: (key: string) => void;
  onDelete: (key: string) => void;
}

function ModelRow({ model, downloadState, onDownload, onDelete }: ModelRowProps) {
  const isDownloading = downloadState !== undefined;

  return (
    <div className="flex items-center gap-4 p-4 rounded-lg bg-neutral-800 border border-neutral-700">
      {/* Name + meta */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 flex-wrap">
          <span className="text-white font-medium text-sm">{model.displayName}</span>
          {model.installed && (
            <span className="px-1.5 py-0.5 rounded text-xs bg-green-900 text-green-300 font-medium">
              Installed
            </span>
          )}
          {model.isDefaultForDe && (
            <span className="px-1.5 py-0.5 rounded text-xs bg-blue-900 text-blue-300 font-medium">
              DE default
            </span>
          )}
          {model.isDefaultForEn && (
            <span className="px-1.5 py-0.5 rounded text-xs bg-purple-900 text-purple-300 font-medium">
              EN default
            </span>
          )}
        </div>
        <div className="flex items-center gap-3 mt-1 text-xs text-neutral-400">
          <span>{formatBytes(model.fileSizeBytes)}</span>
          <span className="capitalize">{model.languageScope}</span>
        </div>

        {/* Download progress bar */}
        {isDownloading && (
          <div className="mt-2">
            <div className="flex justify-between text-xs text-neutral-400 mb-1">
              <span>Downloading…</span>
              <span>
                {downloadState.percent}%
                {downloadState.totalBytes > 0 &&
                  ` · ${formatBytes(downloadState.bytesDownloaded)} / ${formatBytes(downloadState.totalBytes)}`}
              </span>
            </div>
            <div className="w-full bg-neutral-700 rounded-full h-1.5">
              <div
                className="bg-blue-500 h-1.5 rounded-full transition-all duration-100"
                style={{ width: `${downloadState.percent}%` }}
              />
            </div>
          </div>
        )}
      </div>

      {/* Actions */}
      <div className="shrink-0">
        {model.installed ? (
          <button
            onClick={() => onDelete(model.key)}
            className="text-xs px-3 py-1.5 rounded border border-red-700 text-red-400 hover:bg-red-900/40 transition-colors"
          >
            Delete
          </button>
        ) : isDownloading ? (
          <button
            disabled
            className="text-xs px-3 py-1.5 rounded bg-neutral-700 text-neutral-500 cursor-not-allowed"
          >
            Downloading…
          </button>
        ) : (
          <button
            onClick={() => onDownload(model.key)}
            className="text-xs px-3 py-1.5 rounded bg-blue-600 text-white hover:bg-blue-500 transition-colors"
          >
            Download
          </button>
        )}
      </div>
    </div>
  );
}

// ── page ──────────────────────────────────────────────────────────────────────

export default function Models() {
  const { models, loading, downloading, error, fetch, startDownload, removeModel, setDefault, setDownloadProgress } =
    useModelsStore();

  // Fetch model list on mount.
  useEffect(() => {
    fetch();
  }, [fetch]);

  // Subscribe to download progress events from the backend.
  useEffect(() => {
    const unlisten = listen<DownloadProgress>("model-download-progress", (event) => {
      const { key, percent, bytesDownloaded, totalBytes } = event.payload;
      setDownloadProgress(key, { percent, bytesDownloaded, totalBytes });
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [setDownloadProgress]);

  const defaultDe = models.find((m) => m.isDefaultForDe)?.key;
  const defaultEn = models.find((m) => m.isDefaultForEn)?.key;

  const handleDelete = (key: string) => {
    if (confirm(`Delete model "${key}"? The file will be removed from disk.`)) {
      removeModel(key);
    }
  };

  if (loading && models.length === 0) {
    return (
      <div className="p-8 max-w-3xl mx-auto">
        <h1 className="text-2xl font-semibold text-white mb-1">Models</h1>
        <p className="text-neutral-500 text-sm mt-8">Loading models…</p>
      </div>
    );
  }

  return (
    <div className="p-8 max-w-3xl mx-auto">
      <h1 className="text-2xl font-semibold text-white mb-1">Models</h1>
      <p className="text-neutral-400 text-sm mb-8">
        Download and manage local whisper.cpp transcription models.
      </p>

      {/* Default model selectors */}
      <section className="mb-8">
        <h2 className="text-xs font-semibold text-neutral-500 uppercase tracking-wider mb-3">
          Default models
        </h2>
        <div className="bg-neutral-800 border border-neutral-700 rounded-lg p-4 flex flex-col gap-3">
          <DefaultSelector
            label="German (DE)"
            language="de"
            models={models}
            currentKey={defaultDe}
            onSelect={setDefault}
          />
          <DefaultSelector
            label="English (EN)"
            language="en"
            models={models}
            currentKey={defaultEn}
            onSelect={setDefault}
          />
        </div>
      </section>

      {/* Model list */}
      <section>
        <h2 className="text-xs font-semibold text-neutral-500 uppercase tracking-wider mb-3">
          Available models
        </h2>

        {error && (
          <div className="mb-4 p-3 rounded bg-red-900/40 border border-red-700 text-sm text-red-300">
            {error}
          </div>
        )}

        <div className="flex flex-col gap-2">
          {models.length === 0 ? (
            <p className="text-neutral-500 text-sm">Loading…</p>
          ) : (
            models.map((model) => (
              <ModelRow
                key={model.key}
                model={model}
                downloadState={downloading[model.key]}
                onDownload={startDownload}
                onDelete={handleDelete}
              />
            ))
          )}
        </div>
      </section>
    </div>
  );
}
