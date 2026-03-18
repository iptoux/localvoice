import { create } from "zustand";
import type {
  CorrectionStat,
  DashboardStats,
  DateRange,
  LanguageBreakdown,
  TimeseriesPoint,
  WpmPoint,
} from "../types";
import {
  getCorrectionStats,
  getDashboardStats,
  getLanguageBreakdown,
  getUsageTimeseries,
  getWpmTrend,
} from "../lib/tauri";

export type RangePreset = "7d" | "30d" | "all";

interface DashboardStore {
  stats: DashboardStats | null;
  timeseries: TimeseriesPoint[];
  languageBreakdown: LanguageBreakdown[];
  correctionStats: CorrectionStat[];
  wpmTrend: WpmPoint[];
  range: RangePreset;
  loading: boolean;
  error: string | null;

  setRange: (range: RangePreset) => void;
  fetch: (range: RangePreset) => Promise<void>;
}

function presetToDateRange(preset: RangePreset): DateRange {
  if (preset === "all") return {};
  const days = preset === "7d" ? 7 : 30;
  const start = new Date();
  start.setDate(start.getDate() - days);
  return { start: start.toISOString() };
}

export const useDashboardStore = create<DashboardStore>((set, get) => ({
  stats: null,
  timeseries: [],
  languageBreakdown: [],
  correctionStats: [],
  wpmTrend: [],
  range: "30d",
  loading: false,
  error: null,

  setRange: (range) => {
    set({ range });
    get().fetch(range);
  },

  fetch: async (range) => {
    set({ loading: true, error: null });
    const dateRange = presetToDateRange(range);
    const bucket = range === "7d" ? "day" : range === "30d" ? "day" : "week";
    try {
      const [stats, timeseries, languageBreakdown, correctionStats, wpmTrend] =
        await Promise.all([
          getDashboardStats(dateRange),
          getUsageTimeseries(dateRange, bucket),
          getLanguageBreakdown(dateRange),
          getCorrectionStats(),
          getWpmTrend(dateRange, bucket),
        ]);
      set({ stats, timeseries, languageBreakdown, correctionStats, wpmTrend, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },
}));
