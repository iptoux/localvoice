import { invoke } from "@tauri-apps/api/core";
import type {
  AmbiguousTerm,
  CorrectionRule,
  LogEntry,
  DashboardStats,
  DateRange,
  DeviceInfo,
  DictionaryEntry,
  ModelInfo,
  RecordingState,
  Session,
  SessionFilter,
  SessionWithSegments,
  Settings,
  TimeseriesPoint,
  TranscriptionResult,
} from "../types";

// ── Settings ──────────────────────────────────────────────────────────────────

export const getSettings = (): Promise<Settings> =>
  invoke<Settings>("get_settings");

export const updateSetting = (key: string, value: string): Promise<void> =>
  invoke<void>("update_setting", { key, value });

export const resetSettings = (): Promise<void> =>
  invoke<void>("reset_settings");

export const updateShortcut = (shortcut: string): Promise<void> =>
  invoke<void>("update_shortcut", { shortcut });

// ── Window ────────────────────────────────────────────────────────────────────

export const showPill = (): Promise<void> => invoke<void>("show_pill");

export const hidePill = (): Promise<void> => invoke<void>("hide_pill");

export const openMainWindow = (): Promise<void> =>
  invoke<void>("open_main_window");

export const setPillPosition = (x: number, y: number): Promise<void> =>
  invoke<void>("set_pill_position", { x, y });

export const expandPill = (): Promise<void> =>
  invoke<void>("expand_pill");

export const collapsePill = (): Promise<void> =>
  invoke<void>("collapse_pill");

export const reprocessSession = (
  sessionId: string,
  language?: string,
  modelId?: string
): Promise<SessionWithSegments> =>
  invoke<SessionWithSegments>("reprocess_session", {
    sessionId,
    language: language ?? null,
    modelId: modelId ?? null,
  });

// ── Recording ─────────────────────────────────────────────────────────────────

export const startRecording = (): Promise<void> =>
  invoke<void>("start_recording");

export const stopRecording = (): Promise<string> =>
  invoke<string>("stop_recording");

export const cancelRecording = (): Promise<void> =>
  invoke<void>("cancel_recording");

export const getRecordingState = (): Promise<RecordingState> =>
  invoke<RecordingState>("get_recording_state");

export const listInputDevices = (): Promise<DeviceInfo[]> =>
  invoke<DeviceInfo[]>("list_input_devices");

// ── Transcription ─────────────────────────────────────────────────────────────

export const transcribeLastRecording = (
  language?: string,
  modelId?: string
): Promise<TranscriptionResult> =>
  invoke<TranscriptionResult>("transcribe_last_recording", {
    language: language ?? null,
    modelId: modelId ?? null,
  });

export const getLastTranscription = (): Promise<TranscriptionResult | null> =>
  invoke<TranscriptionResult | null>("get_last_transcription");

// ── History ───────────────────────────────────────────────────────────────────

export const listSessions = (filter: SessionFilter = {}): Promise<Session[]> =>
  invoke<Session[]>("list_sessions", { filter });

export const getSessionDetail = (
  sessionId: string
): Promise<SessionWithSegments> =>
  invoke<SessionWithSegments>("get_session", { sessionId });

export const deleteSession = (sessionId: string): Promise<void> =>
  invoke<void>("delete_session", { sessionId });

export const exportSessions = (
  sessionIds: string[],
  format: "txt" | "json"
): Promise<string> =>
  invoke<string>("export_sessions", { sessionIds, format });

// ── Stats ─────────────────────────────────────────────────────────────────────

export const getDashboardStats = (range: DateRange = {}): Promise<DashboardStats> =>
  invoke<DashboardStats>("get_dashboard_stats", { range });

export const getUsageTimeseries = (
  range: DateRange = {},
  bucket: "day" | "week" = "day"
): Promise<TimeseriesPoint[]> =>
  invoke<TimeseriesPoint[]>("get_usage_timeseries", { range, bucket });

// ── Models ────────────────────────────────────────────────────────────────────

export const listAvailableModels = (): Promise<ModelInfo[]> =>
  invoke<ModelInfo[]>("list_available_models");

export const downloadModel = (key: string): Promise<void> =>
  invoke<void>("download_model", { key });

export const deleteModel = (key: string): Promise<void> =>
  invoke<void>("delete_model", { key });

export const setDefaultModel = (language: "de" | "en", key: string): Promise<void> =>
  invoke<void>("set_default_model", { language, key });

// ── Dictionary ────────────────────────────────────────────────────────────────

export const listDictionaryEntries = (): Promise<DictionaryEntry[]> =>
  invoke<DictionaryEntry[]>("list_dictionary_entries");

export const createDictionaryEntry = (payload: {
  phrase: string;
  language?: string;
  entryType: string;
  notes?: string;
}): Promise<DictionaryEntry> =>
  invoke<DictionaryEntry>("create_dictionary_entry", { payload });

export const updateDictionaryEntry = (
  id: string,
  payload: { phrase: string; language?: string; entryType: string; notes?: string }
): Promise<void> =>
  invoke<void>("update_dictionary_entry", { id, payload });

export const deleteDictionaryEntry = (id: string): Promise<void> =>
  invoke<void>("delete_dictionary_entry", { id });

export const listCorrectionRules = (): Promise<CorrectionRule[]> =>
  invoke<CorrectionRule[]>("list_correction_rules");

export const createCorrectionRule = (payload: {
  sourcePhrase: string;
  targetPhrase: string;
  language?: string;
  autoApply: boolean;
}): Promise<CorrectionRule> =>
  invoke<CorrectionRule>("create_correction_rule", { payload });

export const updateCorrectionRule = (
  id: string,
  payload: {
    sourcePhrase: string;
    targetPhrase: string;
    language?: string;
    isActive: boolean;
    autoApply: boolean;
  }
): Promise<void> =>
  invoke<void>("update_correction_rule", { id, payload });

export const deleteCorrectionRule = (id: string): Promise<void> =>
  invoke<void>("delete_correction_rule", { id });

// ── Ambiguity ──────────────────────────────────────────────────────────────────

export const listAmbiguousTerms = (): Promise<AmbiguousTerm[]> =>
  invoke<AmbiguousTerm[]>("list_ambiguous_terms");

export const acceptAmbiguitySuggestion = (
  id: string,
  targetPhrase: string
): Promise<void> =>
  invoke<void>("accept_ambiguity_suggestion", { id, targetPhrase });

export const dismissAmbiguitySuggestion = (id: string): Promise<void> =>
  invoke<void>("dismiss_ambiguity_suggestion", { id });

// ── System ─────────────────────────────────────────────────────────────────────

export const checkFirstRun = (): Promise<boolean> =>
  invoke<boolean>("check_first_run");

export const setAutostart = (enabled: boolean): Promise<void> =>
  invoke<void>("set_autostart", { enabled });

export const getAutostart = (): Promise<boolean> =>
  invoke<boolean>("get_autostart");

// ── Logs ───────────────────────────────────────────────────────────────────────

export const listLogs = (levelFilter?: string, limit?: number): Promise<LogEntry[]> =>
  invoke<LogEntry[]>("list_logs", {
    levelFilter: levelFilter ?? null,
    limit: limit ?? null,
  });

export const exportLogs = (): Promise<void> =>
  invoke<void>("export_logs");

export const clearLogs = (): Promise<void> =>
  invoke<void>("clear_logs");

export const setLoggingEnabled = (enabled: boolean): Promise<void> =>
  invoke<void>("set_logging_enabled", { enabled });
