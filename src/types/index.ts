export type Language = "de" | "en";
export type OutputMode = "insert" | "clipboard" | "preview";
export type RecordingState = "idle" | "listening" | "processing" | "success" | "error";

export interface Session {
  id: string;
  startedAt: string;
  endedAt: string;
  durationMs: number;
  language: string;
  modelId?: string;
  triggerType: string;
  inputDeviceId?: string;
  rawText: string;
  cleanedText: string;
  wordCount: number;
  charCount: number;
  avgConfidence?: number;
  estimatedWpm?: number;
  outputMode: string;
  outputTargetApp?: string;
  insertedSuccessfully: boolean;
  errorMessage?: string;
  createdAt: string;
  audioPath?: string;
  originalRawText?: string;
  reprocessedCount: number;
}

export interface SessionSegment {
  id: string;
  sessionId: string;
  startMs: number;
  endMs: number;
  text: string;
  confidence?: number;
  segmentIndex: number;
}

export interface SessionWithSegments {
  session: Session;
  segments: SessionSegment[];
}

export interface SessionFilter {
  query?: string;
  language?: string;
  dateFrom?: string;
  dateTo?: string;
  modelId?: string;
  limit?: number;
  offset?: number;
}

export interface DictionaryEntry {
  id: string;
  phrase: string;
  normalizedPhrase: string;
  language?: string;
  /** "term" | "name" | "acronym" | "product" | "custom" */
  entryType: string;
  notes?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CorrectionRule {
  id: string;
  sourcePhrase: string;
  normalizedSourcePhrase: string;
  targetPhrase: string;
  language?: string;
  /** "manual" | "suggested" | "learned" */
  ruleMode: string;
  isActive: boolean;
  autoApply: boolean;
  usageCount: number;
  lastUsedAt?: string;
  createdAt: string;
  updatedAt: string;
}

/** Merged view of registry metadata + DB install state returned by list_available_models. */
export interface ModelInfo {
  key: string;
  displayName: string;
  languageScope: "multilingual" | "en-only";
  fileSizeBytes: number;
  installed: boolean;
  isDefaultForDe: boolean;
  isDefaultForEn: boolean;
  /** All languages for which this model is the default. */
  defaultForLanguages: string[];
  localPath?: string;
  installedAt?: string;
  description: string;
  speed: "fastest" | "fast" | "balanced" | "slow" | "slowest";
  accuracy: "low" | "medium" | "good" | "great" | "best";
  category: "standard" | "quantized" | "turbo" | "large";
  recommendedFor: string;
}

export interface DownloadProgress {
  key: string;
  percent: number;
  bytesDownloaded: number;
  totalBytes: number;
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
  /** Filler words removed during post-processing (for stats tracking). */
  removedFillers: string[];
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

// ── Dashboard / Stats ─────────────────────────────────────────────────────────

export interface LanguageCount {
  language: string;
  count: number;
}

export interface ModelUsage {
  modelId: string;
  sessionCount: number;
  totalWordCount: number;
  totalDurationMs: number;
  avgWpm: number;
}

export interface DashboardStats {
  totalWordCount: number;
  totalSessionCount: number;
  avgWpm: number;
  totalDurationMs: number;
  languageCounts: LanguageCount[];
  topModels: ModelUsage[];
}

export interface TimeseriesPoint {
  /** ISO date string, e.g. "2026-03-17" */
  date: string;
  wordCount: number;
  sessionCount: number;
}

export interface DateRange {
  start?: string;
  end?: string;
}

export interface LanguageBreakdown {
  language: string;
  sessionCount: number;
  wordCount: number;
  durationMs: number;
}

export interface CorrectionStat {
  sourcePhrase: string;
  targetPhrase: string;
  usageCount: number;
  lastUsedAt?: string;
}

export interface WpmPoint {
  date: string;
  avgWpm: number;
  sessionCount: number;
}

export interface DailyStats {
  date: string;
  sessionCount: number;
  wordCount: number;
  durationMs: number;
  avgWpm: number;
}

export interface LogEntry {
  id: string;
  level: string;
  area: string;
  message: string;
  createdAt: string;
}

export interface AmbiguousTerm {
  id: string;
  phrase: string;
  normalizedPhrase: string;
  language?: string;
  occurrences: number;
  avgConfidence?: number;
  lastSeenAt: string;
  suggestedTarget?: string;
  dismissed: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface FillerWord {
  id: string;
  word: string;
  language: string;
  isDefault: boolean;
  createdAt: string;
}

export interface FillerStat {
  word: string;
  language: string;
  count: number;
  lastRemovedAt: string;
}

export interface BenchmarkResult {
  micToTextMs: number;
  whisperInitMs: number;
  whisperInferenceMs: number;
  postProcessingMs: number;
  totalTranscriptionMs: number;
  modelId: string;
  language: string;
  audioDurationMs: number;
  audioSampleRate: number;
  textOutput: string;
  success: boolean;
  error?: string;
}
