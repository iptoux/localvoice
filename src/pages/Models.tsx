import { useEffect, useState } from "react";
import { Download, Trash2, CheckCircle, Globe } from "lucide-react";
import { useShallow } from "zustand/react/shallow";
import type { ModelInfo, DownloadProgress } from "../types";
import { useModelsStore } from "../stores/models-store";
import { useThrottledEvent } from "../hooks/use-throttled-event";

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
  standard: "bg-accent text-foreground/70",
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
          className={`w-2 h-2 rounded-full ${i < filled ? color : "bg-accent"}`}
        />
      ))}
    </span>
  );
}

// ── DefaultSelector ───────────────────────────────────────────────────────────

const ALL_LANGUAGES = [
  { code: "de", label: "German (DE)" },
  { code: "en", label: "English (EN)" },
  { code: "fr", label: "French (FR)" },
  { code: "es", label: "Spanish (ES)" },
  { code: "it", label: "Italian (IT)" },
  { code: "pt", label: "Portuguese (PT)" },
  { code: "nl", label: "Dutch (NL)" },
  { code: "pl", label: "Polish (PL)" },
  { code: "ru", label: "Russian (RU)" },
  { code: "ja", label: "Japanese (JA)" },
  { code: "zh", label: "Chinese (ZH)" },
];

interface DefaultSelectorProps {
  label: string;
  language: string;
  models: ModelInfo[];
  currentKey: string | undefined;
  onSelect: (language: string, key: string) => void;
}

function DefaultSelector({ label, language, models, currentKey, onSelect }: DefaultSelectorProps) {
  const installed = models.filter((m) => m.installed);
  return (
    <div className="flex items-center gap-3">
      <span className="text-sm text-muted-foreground w-36 shrink-0">{label}</span>
      {installed.length === 0 ? (
        <span className="text-xs text-muted-foreground italic">No installed models</span>
      ) : (
        <select
          className="bg-muted border border-border text-foreground text-sm rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-blue-500"
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
        ? "bg-muted border-neutral-600"
        : "bg-muted/60 border-border"
    }`}>
      {/* Header row */}
      <div className="flex items-start gap-3">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 flex-wrap">
            <span className="text-foreground font-medium text-sm">{model.displayName}</span>
            <span className={`px-1.5 py-0.5 rounded text-xs font-medium ${CATEGORY_COLORS[model.category]}`}>
              {CATEGORY_LABELS[model.category]}
            </span>
            {model.installed && (
              <span className="flex items-center gap-1 px-1.5 py-0.5 rounded text-xs bg-green-900/60 text-green-300 font-medium">
                <CheckCircle size={11} /> Installed
              </span>
            )}
            {(model.defaultForLanguages ?? []).map((lang) => (
              <span key={lang} className="px-1.5 py-0.5 rounded text-xs bg-blue-900/60 text-blue-300 font-medium uppercase">
                {lang}
              </span>
            ))}
          </div>

          {/* Description */}
          <p className="text-xs text-muted-foreground mt-1 leading-relaxed">{model.description}</p>

          {/* Stats row */}
          <div className="flex items-center gap-4 mt-2 flex-wrap">
            <span className="text-xs text-muted-foreground">{formatBytes(model.fileSizeBytes)}</span>
            <span className="text-xs text-muted-foreground capitalize">
              {model.languageScope === "multilingual" ? <><Globe size={11} className="inline mr-1" />Multilingual</> : "🇬🇧 EN only"}
            </span>
            <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <span>Speed</span>
              <Dots filled={speedDots} color="bg-green-500" />
            </span>
            <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <span>Accuracy</span>
              <Dots filled={accuracyDots} color="bg-blue-500" />
            </span>
          </div>

          {/* Recommended for */}
          <p className="text-xs text-neutral-600 mt-1">
            <span className="text-muted-foreground">Best for:</span> {model.recommendedFor}
          </p>
        </div>

        {/* Action button */}
        <div className="shrink-0 mt-0.5">
          {model.installed ? (
            <button
              onClick={() => onDelete(model.key)}
              className="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded border border-red-800 text-red-400 hover:bg-red-900/40 transition-colors"
            >
              <Trash2 size={12} /> Delete
            </button>
          ) : isDownloading ? (
            <button disabled className="text-xs px-3 py-1.5 rounded bg-accent text-muted-foreground cursor-not-allowed">
              Downloading…
            </button>
          ) : (
            <button
              onClick={() => onDownload(model.key)}
              className="flex items-center gap-1.5 text-xs px-3 py-1.5 rounded bg-blue-600 text-white hover:bg-blue-500 transition-colors"
            >
              <Download size={12} /> Download
            </button>
          )}
        </div>
      </div>

      {/* Download progress */}
      {isDownloading && (
        <div className="mt-3">
          <div className="flex justify-between text-xs text-muted-foreground mb-1">
            <span>Downloading…</span>
            <span>
              {downloadState.percent}%
              {downloadState.totalBytes > 0 &&
                ` · ${formatBytes(downloadState.bytesDownloaded)} / ${formatBytes(downloadState.totalBytes)}`}
            </span>
          </div>
          <div className="w-full bg-accent rounded-full h-1.5">
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
    useModelsStore(
      useShallow((s) => ({
        models: s.models,
        loading: s.loading,
        downloading: s.downloading,
        error: s.error,
        fetch: s.fetch,
        startDownload: s.startDownload,
        removeModel: s.removeModel,
        setDefault: s.setDefault,
        setDownloadProgress: s.setDownloadProgress,
      }))
    );
  const [filter, setFilter] = useState<CategoryFilter>("all");
  const [sort, setSort] = useState<SortKey>("default");

  useEffect(() => { fetch(); }, [fetch]);

  // Throttle download progress updates to one per animation frame.
  useThrottledEvent<DownloadProgress>("model-download-progress", (payload) => {
    const { key, percent, bytesDownloaded, totalBytes } = payload;
    setDownloadProgress(key, { percent, bytesDownloaded, totalBytes });
  });

  // Build a map: language → model key from defaultForLanguages
  const defaultsByLang: Record<string, string> = {};
  for (const m of models) {
    for (const lang of m.defaultForLanguages ?? []) {
      defaultsByLang[lang] = m.key;
    }
  }
  // Fallback to legacy fields for de/en
  if (!defaultsByLang["de"]) {
    const legacyDe = models.find((m) => m.isDefaultForDe);
    if (legacyDe) defaultsByLang["de"] = legacyDe.key;
  }
  if (!defaultsByLang["en"]) {
    const legacyEn = models.find((m) => m.isDefaultForEn);
    if (legacyEn) defaultsByLang["en"] = legacyEn.key;
  }

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
        <h1 className="text-2xl font-semibold text-foreground mb-1">Models</h1>
        <p className="text-muted-foreground text-sm mt-8">Loading models…</p>
      </div>
    );
  }

  return (
    <div className="p-8">
      <h1 className="text-2xl font-semibold text-foreground mb-1">Models</h1>
      <p className="text-muted-foreground text-sm mb-8">
        Download and manage local whisper.cpp transcription models.
      </p>

      {/* Default model selectors */}
      <section className="mb-8">
        <h2 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">Default models</h2>
        <div className="bg-muted border border-border rounded-lg p-4 grid grid-cols-2 gap-3">
          {ALL_LANGUAGES.map((lang) => (
            <DefaultSelector
              key={lang.code}
              label={lang.label}
              language={lang.code}
              models={models}
              currentKey={defaultsByLang[lang.code]}
              onSelect={setDefault}
            />
          ))}
        </div>
      </section>

      {/* Category filter tabs */}
      <section>
        <div className="flex items-center justify-between mb-3">
          <h2 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Available models</h2>
          <div className="flex items-center gap-2">
            <div className="flex gap-1">
              {CATEGORIES.map((cat) => (
                <button
                  key={cat}
                  onClick={() => setFilter(cat)}
                  className={`text-xs px-2.5 py-1 rounded capitalize transition-colors ${
                    filter === cat
                      ? "bg-foreground text-background"
                      : "text-muted-foreground hover:text-foreground"
                  }`}
                >
                  {cat === "all" ? "All" : CATEGORY_LABELS[cat]}
                </button>
              ))}
            </div>
            <select
              value={sort}
              onChange={(e) => setSort(e.target.value as SortKey)}
              className="flex items-center gap-1 bg-muted border border-border text-foreground/70 text-xs rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-blue-500"
            >
              <option value="default">↕ Default order</option>
              <option value="speed">↕ Fastest first</option>
              <option value="accuracy">↕ Most accurate first</option>
              <option value="size">↕ Smallest first</option>
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
