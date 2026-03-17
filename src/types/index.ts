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

export interface TranscriptSegment {
  startMs: number;
  endMs: number;
  text: string;
  confidence?: number;
}

export interface OutputResult {
  /** Effective output mode used: "clipboard" or "insert". */
  mode: string;
  /** Whether the output step completed successfully. */
  success: boolean;
  /** Error description when success is false. */
  error?: string;
}

export interface TranscriptionResult {
  rawText: string;
  cleanedText: string;
  segments: TranscriptSegment[];
  language: string;
  modelId: string;
  durationMs: number;
  /** Result of the output step (clipboard write or auto-insert). */
  output?: OutputResult;
}

export interface DeviceInfo {
  id: string;
  name: string;
  isDefault: boolean;
}

export interface RecordingStatePayload {
  state: RecordingState;
  error?: string;
}

export interface OutputResultPayload {
  mode: string;
  success: boolean;
  error?: string;
}
