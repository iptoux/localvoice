import type {
  AmbiguousTerm,
  CorrectionRule,
  CorrectionStat,
  DailyStats,
  DashboardStats,
  DeviceInfo,
  DictionaryEntry,
  FillerStat,
  FillerWord,
  LanguageBreakdown,
  LogEntry,
  ModelInfo,
  RecordingState,
  Session,
  SessionFilter,
  SessionWithSegments,
  Settings,
  TimeseriesPoint,
  TranscriptionResult,
  WpmPoint,
} from "../types";

export const mockDevices: DeviceInfo[] = [
  { id: "mic-1", name: "Built-in Microphone", isDefault: true },
  { id: "mic-2", name: "USB Headset Microphone", isDefault: false },
];

export const mockSettings: Settings = {
  "transcription.default_language": "auto",
  "transcription.default_model": "ggml-base",
  "output.mode": "clipboard",
  "recording.hotkey": "CommandOrControl+Shift+D",
  "recording.auto_gain": "true",
  "recording.noise_reduction": "true",
  "ui.pill_opacity": "100",
};

export const mockSession: Session = {
  id: "session-1",
  startedAt: "2026-03-18T10:00:00Z",
  endedAt: "2026-03-18T10:01:30Z",
  durationMs: 90000,
  language: "de",
  modelId: "ggml-base",
  triggerType: "hotkey",
  inputDeviceId: "mic-1",
  rawText: "Das ist ein Test. Hallo Welt. Wie geht es dir?",
  cleanedText: "Das ist ein Test. Hallo Welt. Wie geht es dir?",
  wordCount: 11,
  charCount: 51,
  avgConfidence: 0.92,
  estimatedWpm: 140,
  outputMode: "clipboard",
  insertedSuccessfully: true,
  createdAt: "2026-03-18T10:00:00Z",
  reprocessedCount: 0,
};

export const mockSessions: Session[] = [
  mockSession,
  {
    ...mockSession,
    id: "session-2",
    startedAt: "2026-03-17T14:30:00Z",
    endedAt: "2026-03-17T14:31:00Z",
    durationMs: 60000,
    language: "en",
    rawText: "Hello world, this is a test recording.",
    cleanedText: "Hello world, this is a test recording.",
    wordCount: 8,
    charCount: 44,
    avgConfidence: 0.88,
    estimatedWpm: 130,
  },
  {
    ...mockSession,
    id: "session-3",
    startedAt: "2026-03-16T09:15:00Z",
    endedAt: "2026-03-16T09:16:45Z",
    durationMs: 105000,
    language: "de",
    rawText: "Mehrsprachige Aufnahme mit verschiedenen Wörtern.",
    cleanedText: "Mehrsprachige Aufnahme mit verschiedenen Wörtern.",
    wordCount: 6,
    charCount: 52,
    avgConfidence: 0.85,
    estimatedWpm: 125,
  },
];

export const mockTranscription: TranscriptionResult = {
  rawText: "Das ist ein Test. Hallo Welt. Wie geht es dir?",
  cleanedText: "Das ist ein Test. Hallo Welt. Wie geht es dir?",
  segments: [
    { startMs: 0, endMs: 2000, text: "Das ist ein Test.", confidence: 0.95 },
    { startMs: 2000, endMs: 4000, text: "Hallo Welt.", confidence: 0.92 },
    { startMs: 4000, endMs: 6000, text: "Wie geht es dir?", confidence: 0.88 },
  ],
  language: "de",
  modelId: "ggml-base",
  durationMs: 6000,
  output: { mode: "clipboard", success: true },
};

export const mockDashboardStats: DashboardStats = {
  totalWordCount: 15420,
  totalSessionCount: 142,
  avgWpm: 132,
  totalDurationMs: 54000000,
  languageCounts: [
    { language: "de", count: 98 },
    { language: "en", count: 44 },
  ],
  topModels: [
    {
      modelId: "ggml-base",
      sessionCount: 85,
      totalWordCount: 9800,
      totalDurationMs: 32000000,
      avgWpm: 135,
    },
    {
      modelId: "ggml-small",
      sessionCount: 42,
      totalWordCount: 4200,
      totalDurationMs: 15000000,
      avgWpm: 128,
    },
    {
      modelId: "ggml-medium",
      sessionCount: 15,
      totalWordCount: 1420,
      totalDurationMs: 7000000,
      avgWpm: 140,
    },
  ],
};

export const mockTimeseries: TimeseriesPoint[] = [
  { date: "2026-03-11", wordCount: 120, sessionCount: 2 },
  { date: "2026-03-12", wordCount: 250, sessionCount: 4 },
  { date: "2026-03-13", wordCount: 180, sessionCount: 3 },
  { date: "2026-03-14", wordCount: 320, sessionCount: 5 },
  { date: "2026-03-15", wordCount: 410, sessionCount: 6 },
  { date: "2026-03-16", wordCount: 280, sessionCount: 4 },
  { date: "2026-03-17", wordCount: 350, sessionCount: 5 },
  { date: "2026-03-18", wordCount: 190, sessionCount: 3 },
];

export const mockLanguageBreakdown: LanguageBreakdown[] = [
  { language: "de", sessionCount: 85, wordCount: 10200, durationMs: 36000000 },
  { language: "en", sessionCount: 57, wordCount: 5220, durationMs: 18000000 },
];

export const mockCorrectionStats: CorrectionStat[] = [
  { sourcePhrase: "Sprachausgabe", targetPhrase: "Sprachausgabe", usageCount: 24 },
  { sourcePhrase: "das", targetPhrase: "dass", usageCount: 18 },
  { sourcePhrase: "ne", targetPhrase: "ne", usageCount: 15 },
  { sourcePhrase: "Audio Datei", targetPhrase: "Audiodatei", usageCount: 12 },
  { sourcePhrase: "Local Voice", targetPhrase: "LocalVoice", usageCount: 10 },
];

export const mockWpmTrend: WpmPoint[] = [
  { date: "2026-03-11", avgWpm: 125, sessionCount: 2 },
  { date: "2026-03-12", avgWpm: 130, sessionCount: 4 },
  { date: "2026-03-13", avgWpm: 128, sessionCount: 3 },
  { date: "2026-03-14", avgWpm: 135, sessionCount: 5 },
  { date: "2026-03-15", avgWpm: 140, sessionCount: 6 },
  { date: "2026-03-16", avgWpm: 132, sessionCount: 4 },
  { date: "2026-03-17", avgWpm: 138, sessionCount: 5 },
  { date: "2026-03-18", avgWpm: 135, sessionCount: 3 },
];

export const mockModels: ModelInfo[] = [
  {
    key: "ggml-base",
    displayName: "Base (EN)",
    languageScope: "en-only",
    fileSizeBytes: 140000000,
    installed: true,
    isDefaultForDe: false,
    isDefaultForEn: true,
    defaultForLanguages: ["en"],
    localPath: "~/.localvoice/models/ggml-base.bin",
    installedAt: "2026-01-15T10:00:00Z",
    description: "Fast and lightweight, English only",
    speed: "fastest",
    accuracy: "low",
    category: "standard",
    recommendedFor: "Quick dictation, English",
  },
  {
    key: "ggml-small",
    displayName: "Small (Multilingual)",
    languageScope: "multilingual",
    fileSizeBytes: 480000000,
    installed: true,
    isDefaultForDe: true,
    isDefaultForEn: false,
    defaultForLanguages: ["de", "fr", "es"],
    localPath: "~/.localvoice/models/ggml-small.bin",
    installedAt: "2026-02-01T10:00:00Z",
    description: "Good balance of speed and accuracy",
    speed: "fast",
    accuracy: "medium",
    category: "standard",
    recommendedFor: "German, French, Spanish",
  },
  {
    key: "ggml-medium",
    displayName: "Medium (Multilingual)",
    languageScope: "multilingual",
    fileSizeBytes: 1500000000,
    installed: true,
    isDefaultForDe: false,
    isDefaultForEn: false,
    defaultForLanguages: [],
    localPath: "~/.localvoice/models/ggml-medium.bin",
    installedAt: "2026-02-15T10:00:00Z",
    description: "Higher accuracy, slower processing",
    speed: "balanced",
    accuracy: "good",
    category: "standard",
    recommendedFor: "Accurate transcription",
  },
  {
    key: "ggml-large-v3",
    displayName: "Large v3 (Multilingual)",
    languageScope: "multilingual",
    fileSizeBytes: 2900000000,
    installed: false,
    isDefaultForDe: false,
    isDefaultForEn: false,
    defaultForLanguages: [],
    description: "Best accuracy, slowest processing",
    speed: "slow",
    accuracy: "great",
    category: "large",
    recommendedFor: "Maximum accuracy",
  },
];

export const mockDictionaryEntries: DictionaryEntry[] = [
  {
    id: "dict-1",
    phrase: "LocalVoice",
    normalizedPhrase: "localvoice",
    language: "en",
    entryType: "product",
    notes: "Product name",
    createdAt: "2026-02-01T10:00:00Z",
    updatedAt: "2026-02-01T10:00:00Z",
  },
  {
    id: "dict-2",
    phrase: "Audiodatei",
    normalizedPhrase: "audiodatei",
    language: "de",
    entryType: "term",
    notes: "Combined word",
    createdAt: "2026-02-05T10:00:00Z",
    updatedAt: "2026-02-05T10:00:00Z",
  },
];

export const mockCorrectionRules: CorrectionRule[] = [
  {
    id: "rule-1",
    sourcePhrase: "Sprachausgabe",
    normalizedSourcePhrase: "sprachausgabe",
    targetPhrase: "Sprachausgabe",
    language: "de",
    ruleMode: "manual",
    isActive: true,
    autoApply: true,
    usageCount: 24,
    lastUsedAt: "2026-03-18T09:30:00Z",
    createdAt: "2026-02-01T10:00:00Z",
    updatedAt: "2026-02-01T10:00:00Z",
  },
  {
    id: "rule-2",
    sourcePhrase: "das",
    normalizedSourcePhrase: "das",
    targetPhrase: "dass",
    language: "de",
    ruleMode: "suggested",
    isActive: true,
    autoApply: false,
    usageCount: 18,
    lastUsedAt: "2026-03-17T15:00:00Z",
    createdAt: "2026-02-10T10:00:00Z",
    updatedAt: "2026-02-10T10:00:00Z",
  },
];

export const mockAmbiguousTerms: AmbiguousTerm[] = [
  {
    id: "amb-1",
    phrase: "Auto",
    normalizedPhrase: "auto",
    language: "de",
    occurrences: 12,
    avgConfidence: 0.65,
    lastSeenAt: "2026-03-18T09:00:00Z",
    suggestedTarget: "Auto",
    dismissed: false,
    createdAt: "2026-03-01T10:00:00Z",
    updatedAt: "2026-03-18T09:00:00Z",
  },
  {
    id: "amb-2",
    phrase: "Bau",
    normalizedPhrase: "bau",
    language: "de",
    occurrences: 8,
    avgConfidence: 0.58,
    lastSeenAt: "2026-03-17T14:00:00Z",
    suggestedTarget: "bau",
    dismissed: false,
    createdAt: "2026-03-05T10:00:00Z",
    updatedAt: "2026-03-17T14:00:00Z",
  },
];

export const mockFillerWords: FillerWord[] = [
  { id: "filler-1", word: "äh", language: "de", isDefault: true, createdAt: "2026-01-01T00:00:00Z" },
  { id: "filler-2", word: "ähhm", language: "de", isDefault: true, createdAt: "2026-01-01T00:00:00Z" },
  { id: "filler-3", word: "uh", language: "en", isDefault: true, createdAt: "2026-01-01T00:00:00Z" },
  { id: "filler-4", word: "um", language: "en", isDefault: true, createdAt: "2026-01-01T00:00:00Z" },
];

export const mockFillerStats: FillerStat[] = [
  { word: "äh", language: "de", count: 45, lastRemovedAt: "2026-03-18T09:30:00Z" },
  { word: "ähhm", language: "de", count: 23, lastRemovedAt: "2026-03-18T09:00:00Z" },
  { word: "um", language: "en", count: 18, lastRemovedAt: "2026-03-17T15:00:00Z" },
  { word: "uh", language: "en", count: 12, lastRemovedAt: "2026-03-17T14:00:00Z" },
];

export const mockLogs: LogEntry[] = [
  { id: "log-1", level: "info", area: "recording", message: "Recording started", createdAt: "2026-03-18T10:00:00Z" },
  { id: "log-2", level: "info", area: "recording", message: "Recording stopped", createdAt: "2026-03-18T10:01:30Z" },
  { id: "log-3", level: "info", area: "transcription", message: "Transcription completed", createdAt: "2026-03-18T10:01:35Z" },
  { id: "log-4", level: "debug", area: "audio", message: "Audio level: 0.75", createdAt: "2026-03-18T10:00:30Z" },
  { id: "log-5", level: "warn", area: "audio", message: "High noise detected", createdAt: "2026-03-18T10:00:45Z" },
];

export const mockSessionDetail: SessionWithSegments = {
  session: mockSession,
  segments: [
    { id: "seg-1", sessionId: "session-1", startMs: 0, endMs: 2000, text: "Das ist ein Test.", confidence: 0.95, segmentIndex: 0 },
    { id: "seg-2", sessionId: "session-1", startMs: 2000, endMs: 4000, text: "Hallo Welt.", confidence: 0.92, segmentIndex: 1 },
    { id: "seg-3", sessionId: "session-1", startMs: 4000, endMs: 6000, text: "Wie geht es dir?", confidence: 0.88, segmentIndex: 2 },
  ],
};

export const mockInvoke = async (cmd: string, args?: Record<string, unknown>): Promise<unknown> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  switch (cmd) {
    case "get_settings":
      return mockSettings;
    case "update_setting":
      return undefined;
    case "reset_settings":
      return undefined;
    case "update_shortcut":
      return undefined;
    case "show_pill":
      return undefined;
    case "hide_pill":
      return undefined;
    case "open_main_window":
      return undefined;
    case "expand_pill":
      return undefined;
    case "collapse_pill":
      return undefined;
    case "set_pill_position":
      return undefined;
    case "reprocess_session":
      return mockSessionDetail;
    case "start_recording":
      return undefined;
    case "stop_recording":
      return "session-mock";
    case "cancel_recording":
      return undefined;
    case "get_recording_state":
      return "idle" as RecordingState;
    case "list_input_devices":
      return mockDevices;
    case "transcribe_last_recording":
      return mockTranscription;
    case "get_last_transcription":
      return mockTranscription;
    case "list_sessions": {
      const filter = (args?.filter as SessionFilter) ?? {};
      let result = [...mockSessions];
      if (filter.query) {
        result = result.filter((s) => s.cleanedText.toLowerCase().includes(filter.query!.toLowerCase()));
      }
      if (filter.language) {
        result = result.filter((s) => s.language === filter.language);
      }
      return result;
    }
    case "get_session":
      return mockSessionDetail;
    case "delete_session":
      return undefined;
    case "export_sessions":
      return "exported/path.txt";
    case "get_dashboard_stats":
      return mockDashboardStats;
    case "get_usage_timeseries":
      return mockTimeseries;
    case "get_language_breakdown":
      return mockLanguageBreakdown;
    case "get_correction_stats":
      return mockCorrectionStats;
    case "get_wpm_trend":
      return mockWpmTrend;
    case "get_daily_comparison":
      return [
        { date: "2026-03-15", sessionCount: 6, wordCount: 410, durationMs: 18000000, avgWpm: 140 } as DailyStats,
        { date: "2026-03-16", sessionCount: 4, wordCount: 280, durationMs: 10500000, avgWpm: 132 } as DailyStats,
      ];
    case "list_available_models":
      return mockModels;
    case "download_model":
      return undefined;
    case "delete_model":
      return undefined;
    case "set_default_model":
      return undefined;
    case "list_dictionary_entries":
      return mockDictionaryEntries;
    case "create_dictionary_entry":
      return mockDictionaryEntries[0];
    case "update_dictionary_entry":
      return undefined;
    case "delete_dictionary_entry":
      return undefined;
    case "list_correction_rules":
      return mockCorrectionRules;
    case "create_correction_rule":
      return mockCorrectionRules[0];
    case "update_correction_rule":
      return undefined;
    case "delete_correction_rule":
      return undefined;
    case "list_ambiguous_terms":
      return mockAmbiguousTerms;
    case "accept_ambiguity_suggestion":
      return undefined;
    case "dismiss_ambiguity_suggestion":
      return undefined;
    case "check_first_run":
      return false;
    case "has_default_model":
      return true;
    case "set_autostart":
      return undefined;
    case "get_autostart":
      return false;
    case "list_logs":
      return mockLogs;
    case "export_logs":
      return undefined;
    case "clear_logs":
      return undefined;
    case "set_logging_enabled":
      return undefined;
    case "list_filler_words":
      return mockFillerWords;
    case "add_filler_word":
      return mockFillerWords[0];
    case "delete_filler_word":
      return undefined;
    case "reset_filler_words":
      return mockFillerWords;
    case "get_filler_stats":
      return mockFillerStats;
    case "get_filler_total_count":
      return 98;
    default:
      throw new Error(`Unknown mock command: ${cmd}`);
  }
};

export const mockTauri = {
  invoke: mockInvoke,
};
