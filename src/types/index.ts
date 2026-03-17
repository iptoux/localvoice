export type Language = "de" | "en";
export type OutputMode = "insert" | "clipboard" | "preview";
export type RecordingState = "idle" | "listening" | "processing" | "success" | "error";

export interface Session {
  id: string;
  startedAt: string;
  endedAt: string;
  durationMs: number;
  language: Language;
  modelId?: string;
  rawText: string;
  cleanedText: string;
  wordCount: number;
  charCount: number;
  avgConfidence?: number;
  estimatedWpm?: number;
  outputMode: OutputMode;
  insertedSuccessfully: boolean;
  errorMessage?: string;
}

export interface CorrectionRule {
  id: string;
  sourcePhrase: string;
  targetPhrase: string;
  language?: Language;
  ruleMode: "manual" | "suggested" | "learned";
  isActive: boolean;
  autoApply: boolean;
  usageCount: number;
}

export interface ModelInstallation {
  id: string;
  modelKey: string;
  displayName: string;
  languageScope: "multilingual" | Language;
  localPath: string;
  installed: boolean;
  version?: string;
  isDefaultForDe: boolean;
  isDefaultForEn: boolean;
}

export type Settings = Record<string, string>;

export interface DeviceInfo {
  id: string;
  name: string;
  isDefault: boolean;
}

export interface RecordingStatePayload {
  state: RecordingState;
  error?: string;
}
