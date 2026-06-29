import { describe, expect, it } from "vitest";
import type { ModelInfo } from "../types";
import { sortModels } from "./model-sort";

function model(overrides: Partial<ModelInfo> & Pick<ModelInfo, "key" | "displayName">): ModelInfo {
  const { key, displayName, ...rest } = overrides;
  return {
    key,
    displayName,
    languageScope: "multilingual",
    engine: "whisper-cpp",
    artifactFormat: "ggml-bin",
    runtime: "bundled-sidecar",
    fileSizeBytes: 100,
    installed: false,
    isDefaultForDe: false,
    isDefaultForEn: false,
    defaultForLanguages: [],
    description: "",
    speed: "balanced",
    accuracy: "good",
    category: "standard",
    recommendedFor: "",
    supportsStreaming: false,
    supportsWordTimestamps: false,
    supportsConfidence: true,
    licenseId: "mit",
    licenseUrl: "",
    languageLocales: ["de-DE", "en-US"],
    ...rest,
  };
}

describe("model sorting", () => {
  it("ranks NeMo and Parakeet models in most-accurate ordering", () => {
    const sorted = sortModels(
      [
        model({ key: "whisper-large", displayName: "Whisper Large", accuracy: "best", fileSizeBytes: 3_100 }),
        model({
          key: "parakeet-q5",
          displayName: "Parakeet Q5",
          engine: "parakeet-cpp",
          artifactFormat: "gguf",
          category: "parakeet",
          accuracy: "best",
          fileSizeBytes: 780,
          supportsStreaming: true,
        }),
        model({
          key: "nemo",
          displayName: "NeMo",
          engine: "nemo",
          artifactFormat: "nemo",
          runtime: "optional-nemo",
          category: "nemo",
          accuracy: "best",
          fileSizeBytes: 2_300,
          supportsStreaming: true,
        }),
        model({ key: "whisper-medium", displayName: "Whisper Medium", accuracy: "great" }),
      ],
      "accuracy"
    );

    expect(sorted.map((item) => item.key)).toEqual([
      "nemo",
      "parakeet-q5",
      "whisper-large",
      "whisper-medium",
    ]);
  });

  it("uses artifact precision, size, and display name as deterministic tie-breakers", () => {
    const sorted = sortModels(
      [
        model({
          key: "parakeet-q5",
          displayName: "Parakeet Q5",
          engine: "parakeet-cpp",
          artifactFormat: "gguf",
          category: "parakeet",
          accuracy: "best",
          fileSizeBytes: 700,
        }),
        model({
          key: "parakeet-f16",
          displayName: "Parakeet F16",
          engine: "parakeet-cpp",
          artifactFormat: "gguf",
          category: "parakeet",
          accuracy: "best",
          fileSizeBytes: 1_400,
        }),
      ],
      "accuracy"
    );

    expect(sorted.map((item) => item.key)).toEqual(["parakeet-f16", "parakeet-q5"]);
  });
});
