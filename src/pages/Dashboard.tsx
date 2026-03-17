import { useEffect } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
  Cell,
} from "recharts";
import { useDashboardStore, type RangePreset } from "../stores/dashboard-store";
import type { DashboardStats, TimeseriesPoint } from "../types";

// ── Language palette (consistent across sessions) ────────────────────────────
const LANG_COLORS: Record<string, string> = {
  de: "#60a5fa",
  en: "#34d399",
  fr: "#f472b6",
  es: "#fb923c",
  auto: "#a78bfa",
};
function langColor(lang: string) {
  return LANG_COLORS[lang] ?? "#94a3b8";
}

// ── Main page ─────────────────────────────────────────────────────────────────

export default function Dashboard() {
  const { stats, timeseries, range, loading, error, setRange, fetch } =
    useDashboardStore();

  // Fetch on first mount.
  useEffect(() => {
    fetch(range);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="p-8 space-y-6 overflow-y-auto">
      {/* Header + range selector */}
      <div className="flex items-center justify-between gap-4">
        <h1 className="text-2xl font-semibold text-white">Dashboard</h1>
        <RangeSelector current={range} onChange={setRange} />
      </div>

      {error && (
        <p className="text-rose-400 text-sm">Failed to load stats: {error}</p>
      )}

      {/* Stat cards (TASK-089) */}
      <StatCards stats={stats} loading={loading} />

      {/* Words-over-time chart (TASK-090) */}
      <section className="bg-neutral-900 rounded-xl border border-neutral-800 p-5">
        <h2 className="text-sm font-semibold text-neutral-300 mb-4">
          Words over time
        </h2>
        <WordsChart data={timeseries} loading={loading} />
      </section>

      {/* Language breakdown (TASK-091) */}
      <section className="bg-neutral-900 rounded-xl border border-neutral-800 p-5">
        <h2 className="text-sm font-semibold text-neutral-300 mb-4">
          Language breakdown
        </h2>
        <LanguageChart stats={stats} loading={loading} />
      </section>
    </div>
  );
}

// ── Date-range selector (TASK-092) ────────────────────────────────────────────

const RANGE_LABELS: Record<RangePreset, string> = {
  "7d": "Last 7 days",
  "30d": "Last 30 days",
  all: "All time",
};

function RangeSelector({
  current,
  onChange,
}: {
  current: RangePreset;
  onChange: (r: RangePreset) => void;
}) {
  return (
    <div className="flex gap-1 bg-neutral-800 rounded-lg p-1">
      {(["7d", "30d", "all"] as RangePreset[]).map((r) => (
        <button
          key={r}
          onClick={() => onChange(r)}
          className={`px-3 py-1 text-xs rounded-md transition-colors ${
            current === r
              ? "bg-neutral-600 text-white"
              : "text-neutral-400 hover:text-white"
          }`}
        >
          {RANGE_LABELS[r]}
        </button>
      ))}
    </div>
  );
}

// ── Stat cards (TASK-089) ─────────────────────────────────────────────────────

function StatCards({
  stats,
  loading,
}: {
  stats: DashboardStats | null;
  loading: boolean;
}) {
  const durationLabel = stats
    ? formatDuration(stats.totalDurationMs)
    : "—";
  const wpmLabel = stats
    ? stats.avgWpm > 0
      ? `${Math.round(stats.avgWpm)} wpm`
      : "—"
    : "—";

  return (
    <div className="grid grid-cols-2 xl:grid-cols-4 gap-4">
      <StatCard
        label="Total Words"
        value={loading ? "…" : (stats?.totalWordCount ?? 0).toLocaleString()}
      />
      <StatCard
        label="Sessions"
        value={loading ? "…" : (stats?.totalSessionCount ?? 0).toLocaleString()}
      />
      <StatCard label="Avg WPM" value={loading ? "…" : wpmLabel} />
      <StatCard label="Recording Time" value={loading ? "…" : durationLabel} />
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="bg-neutral-900 border border-neutral-800 rounded-xl p-5">
      <p className="text-xs text-neutral-500 mb-1">{label}</p>
      <p className="text-2xl font-semibold text-white tabular-nums">{value}</p>
    </div>
  );
}

// ── Words-over-time line chart (TASK-090) ─────────────────────────────────────

function WordsChart({
  data,
  loading,
}: {
  data: TimeseriesPoint[];
  loading: boolean;
}) {
  if (loading) {
    return <ChartPlaceholder label="Loading…" />;
  }
  if (data.length === 0) {
    return <ChartPlaceholder label="No data yet. Record a few sessions to see trends." />;
  }

  return (
    <ResponsiveContainer width="100%" height={200}>
      <LineChart data={data} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>
        <XAxis
          dataKey="date"
          tick={{ fill: "#71717a", fontSize: 11 }}
          tickFormatter={shortDate}
          tickLine={false}
          axisLine={false}
          interval="preserveStartEnd"
        />
        <YAxis
          tick={{ fill: "#71717a", fontSize: 11 }}
          tickLine={false}
          axisLine={false}
          allowDecimals={false}
        />
        <Tooltip
          contentStyle={{
            background: "#18181b",
            border: "1px solid #3f3f46",
            borderRadius: 8,
            color: "#e4e4e7",
            fontSize: 12,
          }}
          formatter={(v) => [(v as number).toLocaleString(), "Words"]}
          labelFormatter={(l) => l}
        />
        <Line
          type="monotone"
          dataKey="wordCount"
          stroke="#60a5fa"
          strokeWidth={2}
          dot={false}
          activeDot={{ r: 4, fill: "#60a5fa" }}
        />
      </LineChart>
    </ResponsiveContainer>
  );
}

// ── Language breakdown bar chart (TASK-091) ───────────────────────────────────

function LanguageChart({
  stats,
  loading,
}: {
  stats: DashboardStats | null;
  loading: boolean;
}) {
  if (loading) return <ChartPlaceholder label="Loading…" />;

  const data = stats?.languageCounts ?? [];
  if (data.length === 0) {
    return <ChartPlaceholder label="No sessions yet." />;
  }

  return (
    <div className="flex flex-col sm:flex-row gap-6 items-start">
      {/* Bar chart */}
      <ResponsiveContainer width="100%" height={160}>
        <BarChart
          data={data}
          layout="vertical"
          margin={{ top: 0, right: 16, bottom: 0, left: 8 }}
        >
          <XAxis
            type="number"
            tick={{ fill: "#71717a", fontSize: 11 }}
            tickLine={false}
            axisLine={false}
            allowDecimals={false}
          />
          <YAxis
            type="category"
            dataKey="language"
            tick={{ fill: "#a1a1aa", fontSize: 12, fontWeight: 600 }}
            tickLine={false}
            axisLine={false}
            width={32}
          />
          <Tooltip
            contentStyle={{
              background: "#18181b",
              border: "1px solid #3f3f46",
              borderRadius: 8,
              color: "#e4e4e7",
              fontSize: 12,
            }}
            formatter={(v) => [v as number, "Sessions"]}
          />
          <Bar dataKey="count" radius={[0, 4, 4, 0]}>
            {data.map((entry) => (
              <Cell key={entry.language} fill={langColor(entry.language)} />
            ))}
          </Bar>
        </BarChart>
      </ResponsiveContainer>

      {/* Legend */}
      <ul className="shrink-0 space-y-2 text-xs">
        {data.map((entry) => (
          <li key={entry.language} className="flex items-center gap-2">
            <span
              className="inline-block w-3 h-3 rounded-sm"
              style={{ background: langColor(entry.language) }}
            />
            <span className="text-neutral-300 uppercase font-mono">
              {entry.language}
            </span>
            <span className="text-neutral-500">{entry.count} sessions</span>
          </li>
        ))}
      </ul>
    </div>
  );
}

// ── Shared helpers ────────────────────────────────────────────────────────────

function ChartPlaceholder({ label }: { label: string }) {
  return (
    <div className="flex items-center justify-center h-32 text-neutral-600 text-sm">
      {label}
    </div>
  );
}

/** Format total milliseconds as `h:mm` or `m:ss`. */
function formatDuration(ms: number): string {
  if (ms <= 0) return "0:00";
  const totalSec = Math.round(ms / 1000);
  const hours = Math.floor(totalSec / 3600);
  const minutes = Math.floor((totalSec % 3600) / 60);
  const seconds = totalSec % 60;
  if (hours > 0) {
    return `${hours}:${String(minutes).padStart(2, "0")}`;
  }
  return `${minutes}:${String(seconds).padStart(2, "0")}`;
}

/** Shorten an ISO date string for chart tick labels. */
function shortDate(iso: string): string {
  try {
    const d = new Date(iso + "T00:00:00");
    return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
  } catch {
    return iso;
  }
}
