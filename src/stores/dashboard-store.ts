import { create } from "zustand";
import type { DashboardStats, DateRange, TimeseriesPoint } from "../types";
import { getDashboardStats, getUsageTimeseries } from "../lib/tauri";

export type RangePreset = "7d" | "30d" | "all";

interface DashboardStore {
  stats: DashboardStats | null;
  timeseries: TimeseriesPoint[];
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
      const [stats, timeseries] = await Promise.all([
        getDashboardStats(dateRange),
        getUsageTimeseries(dateRange, bucket),
      ]);
      set({ stats, timeseries, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },
}));
