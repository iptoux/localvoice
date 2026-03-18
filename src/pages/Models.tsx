import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { ModelInfo, DownloadProgress } from "../types";
import { useModelsStore } from "../stores/models-store";

// ── helpers ───────────────────────────────────────────────────────────────────

function formatBytes(bytes: number): string {
  if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(1)} GB`;
  if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(0)} MB`;
  return `${(bytes / 1024).toFixed(0)} KB`;
}

const CATEGORY_LABELS: Record<string, string> = {
  standard: "Standard",
  quantized: "Quantized",
  turbo: "Turbo",
  large: "Large",
};

const CATEGORY_COLORS: Record<string, string> = {
  standard: "bg-neutral-700 text-neutral-300",
  quantized: "bg-cyan-900/60 text-cyan-300",
  turbo: "bg-orange-900/60 text-orange-300",
  large: "bg-purple-900/60 text-purple-300",
};

const SPEED_DOTS: Record<string, number> = {
  fastest: 5, fast: 4, balanced: 3, slow: 2, slowest: 1,
};

const ACCURACY_DOTS: Record<string, number> = {
  low: 1, medium: 2, good: 3, great: 4, best: 5,
};

function Dots({ filled, total = 5, color }: { filled: number; total?: number; color: string }) {
  return (
    <span className="flex gap-0.5">
      {Array.from({ length: total }).map((_, i) => (
        <span
          key={i}
          className={`w-2 h-2 rounded-full ${i < filled ? color : "bg-neutral-700"}`}
        />
      ))}
    </span>
  );
}

// ── DefaultSelector ───────────────────────────────────────────────────────────

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
            <option key={m.key} value={m.key}>{m.displayName}</option>
          ))}
        </select>
      )}
    </div>
  );
}

// ── ModelCard ─────────────────────────────────────────────────────────────────

interface ModelCardProps {
  model: ModelInfo;
  downloadState: { percent: number; bytesDownloaded: number; totalBytes: number } | undefined;
  onDownload: (key: string) => void;
  onDelete: (key: string) => void;
}

function ModelCard({ model, downloadState, onDownload, onDelete }: ModelCardProps) {
  const isDownloading = downloadState !== undefined;
  const speedDots = SPEED_DOTS[model.speed] ?? 3;
  const accuracyDots = ACCURACY_DOTS[model.accuracy] ?? 3;

  return (
    <div className={`p-4 rounded-lg border transition-colors ${
      model.installed
        ? "bg-neutral-800 border-neutral-600"
        : "bg-neutral-800/60 border-neutral-700"
    }`}>
      {/* Header row */}
      <div className="flex items-start gap-3">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 flex-wrap">
            <span className="text-white font-medium text-sm">{model.displayName}</span>
            <span className={`px-1.5 py-0.5 rounded text-xs font-medium ${CATEGORY_COLORS[model.category]}`}>
              {CATEGORY_LABELS[model.category]}
            </span>
            {model.installed && (
              <span className="px-1.5 py-0.5 rounded text-xs bg-green-900/60 text-green-300 font-medium">
                Installed
              </span>
            )}
            {model.isDefaultForDe && (
              <span className="px-1.5 py-0.5 rounded text-xs bg-blue-900/60 text-blue-300 font-medium">DE</span>
            )}
            {model.isDefaultForEn && (
              <span className="px-1.5 py-0.5 rounded text-xs bg-indigo-900/60 text-indigo-300 font-medium">EN</span>
            )}
          </div>

          {/* Description */}
          <p className="text-xs text-neutral-400 mt-1 leading-relaxed">{model.description}</p>

          {/* Stats row */}
          <div className="flex items-center gap-4 mt-2 flex-wrap">
            <span className="text-xs text-neutral-500">{formatBytes(model.fileSizeBytes)}</span>
            <span className="text-xs text-neutral-500 capitalize">
              {model.languageScope === "multilingual" ? "🌍 Multilingual" : "🇬🇧 EN only"}
            </span>
            <span className="flex items-center gap-1.5 text-xs text-neutral-500">
              <span>Speed</span>
              <Dots filled={speedDots} color="bg-green-500" />
            </span>
            <span className="flex items-center gap-1.5 text-xs text-neutral-500">
              <span>Accuracy</span>
              <Dots filled={accuracyDots} color="bg-blue-500" />
            </span>
          </div>

          {/* Recommended for */}
          <p className="text-xs text-neutral-600 mt-1">
            <span className="text-neutral-500">Best for:</span> {model.recommendedFor}
          </p>
        </div>

        {/* Action button */}
        <div className="shrink-0 mt-0.5">
          {model.installed ? (
            <button
              onClick={() => onDelete(model.key)}
              className="text-xs px-3 py-1.5 rounded border border-red-800 text-red-400 hover:bg-red-900/40 transition-colors"
            >
              Delete
            </button>
          ) : isDownloading ? (
            <button disabled className="text-xs px-3 py-1.5 rounded bg-neutral-700 text-neutral-500 cursor-not-allowed">
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

      {/* Download progress */}
      {isDownloading && (
        <div className="mt-3">
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
  );
}

// ── page ──────────────────────────────────────────────────────────────────────

const CATEGORIES = ["all", "standard", "quantized", "turbo", "large"] as const;
type CategoryFilter = typeof CATEGORIES[number];
type SortKey = "default" | "speed" | "accuracy" | "size";

const SPEED_ORDER: Record<string, number> = { fastest: 5, fast: 4, balanced: 3, slow: 2, slowest: 1 };
const ACCURACY_ORDER: Record<string, number> = { best: 5, great: 4, good: 3, medium: 2, low: 1 };

function sortModels(models: ModelInfo[], sort: SortKey): ModelInfo[] {
  if (sort === "default") return models;
  return [...models].sort((a, b) => {
    if (sort === "speed") return (SPEED_ORDER[b.speed] ?? 0) - (SPEED_ORDER[a.speed] ?? 0);
    if (sort === "accuracy") return (ACCURACY_ORDER[b.accuracy] ?? 0) - (ACCURACY_ORDER[a.accuracy] ?? 0);
    if (sort === "size") return a.fileSizeBytes - b.fileSizeBytes;
    return 0;
  });
}

export default function Models() {
  const { models, loading, downloading, error, fetch, startDownload, removeModel, setDefault, setDownloadProgress } =
    useModelsStore();
  const [filter, setFilter] = useState<CategoryFilter>("all");
  const [sort, setSort] = useState<SortKey>("default");

  useEffect(() => { fetch(); }, [fetch]);

  useEffect(() => {
    const unlisten = listen<DownloadProgress>("model-download-progress", (event) => {
      const { key, percent, bytesDownloaded, totalBytes } = event.payload;
      setDownloadProgress(key, { percent, bytesDownloaded, totalBytes });
    });
    return () => { unlisten.then((fn) => fn()); };
  }, [setDownloadProgress]);

  const defaultDe = models.find((m) => m.isDefaultForDe)?.key;
  const defaultEn = models.find((m) => m.isDefaultForEn)?.key;

  const handleDelete = (key: string) => {
    if (confirm(`Delete model "${key}"? The file will be removed from disk.`)) {
      removeModel(key);
    }
  };

  const visible = sortModels(
    filter === "all" ? models : models.filter((m) => m.category === filter),
    sort
  );

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
        <h2 className="text-xs font-semibold text-neutral-500 uppercase tracking-wider mb-3">Default models</h2>
        <div className="bg-neutral-800 border border-neutral-700 rounded-lg p-4 flex flex-col gap-3">
          <DefaultSelector label="German (DE)" language="de" models={models} currentKey={defaultDe} onSelect={setDefault} />
          <DefaultSelector label="English (EN)" language="en" models={models} currentKey={defaultEn} onSelect={setDefault} />
        </div>
      </section>

      {/* Category filter tabs */}
      <section>
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-xs font-semibold text-neutral-500 uppercase tracking-wider">Available models</h2>
          <div className="flex items-center gap-2">
            <div className="flex gap-1">
              {CATEGORIES.map((cat) => (
                <button
                  key={cat}
                  onClick={() => setFilter(cat)}
                  className={`text-xs px-2.5 py-1 rounded capitalize transition-colors ${
                    filter === cat
                      ? "bg-neutral-600 text-white"
                      : "text-neutral-500 hover:text-neutral-300"
                  }`}
                >
                  {cat === "all" ? "All" : CATEGORY_LABELS[cat]}
                </button>
              ))}
            </div>
            <select
              value={sort}
              onChange={(e) => setSort(e.target.value as SortKey)}
              className="bg-neutral-800 border border-neutral-700 text-neutral-300 text-xs rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-blue-500"
            >
              <option value="default">Default order</option>
              <option value="speed">Fastest first</option>
              <option value="accuracy">Most accurate first</option>
              <option value="size">Smallest first</option>
            </select>
          </div>
        </div>

        {error && (
          <div className="mb-4 p-3 rounded bg-red-900/40 border border-red-700 text-sm text-red-300">{error}</div>
        )}

        <div className="flex flex-col gap-2">
          {visible.map((model) => (
            <ModelCard
              key={model.key}
              model={model}
              downloadState={downloading[model.key]}
              onDownload={startDownload}
              onDelete={handleDelete}
            />
          ))}
        </div>
      </section>
    </div>
  );
}
