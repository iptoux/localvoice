import type { ModelInfo } from "../types";

export type ModelSortKey = "default" | "speed" | "accuracy" | "size";

const SPEED_ORDER: Record<string, number> = {
  fastest: 5,
  fast: 4,
  balanced: 3,
  slow: 2,
  slowest: 1,
};

const ACCURACY_ORDER: Record<string, number> = {
  best: 5,
  great: 4,
  good: 3,
  medium: 2,
  low: 1,
};

const ENGINE_ACCURACY_PRIORITY: Record<string, number> = {
  nemo: 3,
  "parakeet-cpp": 2,
  "whisper-cpp": 1,
};

export function sortModels(models: ModelInfo[], sort: ModelSortKey): ModelInfo[] {
  if (sort === "default") return models;
  return [...models].sort((a, b) => {
    if (sort === "speed") {
      return compareNumberDesc(SPEED_ORDER[a.speed] ?? 0, SPEED_ORDER[b.speed] ?? 0)
        || compareNumberDesc(ACCURACY_ORDER[a.accuracy] ?? 0, ACCURACY_ORDER[b.accuracy] ?? 0)
        || compareName(a, b);
    }
    if (sort === "accuracy") {
      return compareAccuracy(a, b);
    }
    if (sort === "size") {
      return compareNumberAsc(a.fileSizeBytes, b.fileSizeBytes) || compareName(a, b);
    }
    return 0;
  });
}

export function filterModels(models: ModelInfo[], category: string): ModelInfo[] {
  return category === "all" ? models : models.filter((model) => model.category === category);
}

function compareAccuracy(a: ModelInfo, b: ModelInfo): number {
  return compareNumberDesc(ACCURACY_ORDER[a.accuracy] ?? 0, ACCURACY_ORDER[b.accuracy] ?? 0)
    || compareNumberDesc(ENGINE_ACCURACY_PRIORITY[a.engine] ?? 0, ENGINE_ACCURACY_PRIORITY[b.engine] ?? 0)
    || compareNumberDesc(artifactPrecision(a), artifactPrecision(b))
    || compareNumberDesc(a.fileSizeBytes, b.fileSizeBytes)
    || compareName(a, b);
}

function artifactPrecision(model: ModelInfo): number {
  const source = `${model.key} ${model.displayName} ${model.artifactFormat}`.toLowerCase();
  if (model.artifactFormat === "nemo") return 100;
  if (source.includes("f16") || source.includes("fp16")) return 90;
  if (source.includes("q8") || source.includes("8-bit")) return 80;
  if (source.includes("q5") || source.includes("5-bit")) return 70;
  if (source.includes("q4") || source.includes("4-bit")) return 60;
  if (model.artifactFormat === "gguf") return 50;
  return 40;
}

function compareNumberDesc(a: number, b: number): number {
  return b - a;
}

function compareNumberAsc(a: number, b: number): number {
  return a - b;
}

function compareName(a: ModelInfo, b: ModelInfo): number {
  return a.displayName.localeCompare(b.displayName, "en");
}
