import { useEffect } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import { useDashboardStore, type RangePreset } from "../stores/dashboard-store";
import type { DashboardStats, TimeseriesPoint } from "../types";

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
        <h1 className="text-2xl font-semibold text-foreground">Dashboard</h1>
        <RangeSelector current={range} onChange={setRange} />
      </div>

      {error && (
        <p className="text-rose-400 text-sm">Failed to load stats: {error}</p>
      )}

      {/* Stat cards (TASK-089) */}
      <StatCards stats={stats} loading={loading} />

      {/* Words-over-time chart (TASK-090) */}
      <section className="bg-card rounded-xl border border-border p-5">
        <h2 className="text-sm font-semibold text-foreground/70 mb-4">
          Words over time
        </h2>
        <WordsChart data={timeseries} loading={loading} />
      </section>

      {/* Top models */}
      <section className="bg-card rounded-xl border border-border p-5">
        <h2 className="text-sm font-semibold text-foreground/70 mb-4">
          Top models
        </h2>
        <TopModels stats={stats} loading={loading} />
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
    <div className="flex gap-1 bg-muted rounded-lg p-1">
      {(["7d", "30d", "all"] as RangePreset[]).map((r) => (
        <button
          key={r}
          onClick={() => onChange(r)}
          className={`px-3 py-1 text-xs rounded-md transition-colors ${
            current === r
              ? "bg-neutral-600 text-foreground"
              : "text-muted-foreground hover:text-foreground"
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
    <div className="bg-card border border-border rounded-xl p-5">
      <p className="text-xs text-muted-foreground mb-1">{label}</p>
      <p className="text-2xl font-semibold text-foreground tabular-nums">{value}</p>
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

// ── Top models ────────────────────────────────────────────────────────────────

function TopModels({
  stats,
  loading,
}: {
  stats: DashboardStats | null;
  loading: boolean;
}) {
  if (loading) return <ChartPlaceholder label="Loading…" />;

  const models = stats?.topModels ?? [];
  if (models.length === 0) {
    return <ChartPlaceholder label="No sessions yet." />;
  }

  const maxSessions = Math.max(...models.map((m) => m.sessionCount), 1);

  return (
    <div className="flex flex-col gap-3">
      {models.map((m, i) => {
        const pct = Math.round((m.sessionCount / maxSessions) * 100);
        const label = m.modelId === "unknown" ? "Unknown" : m.modelId.replace(/^ggml-/, "");
        const wpm = m.avgWpm > 0 ? `${Math.round(m.avgWpm)} wpm` : null;
        const dur = formatDuration(m.totalDurationMs);
        const rank = ["🥇", "🥈", "🥉"][i] ?? `#${i + 1}`;

        return (
          <div key={m.modelId} className="flex items-center gap-3">
            <span className="text-base w-6 shrink-0">{rank}</span>
            <div className="flex-1 min-w-0">
              <div className="flex items-center justify-between mb-1 gap-2">
                <span className="text-sm text-foreground font-medium truncate capitalize">{label}</span>
                <span className="text-xs text-muted-foreground shrink-0">
                  {m.sessionCount} {m.sessionCount === 1 ? "session" : "sessions"}
                </span>
              </div>
              <div className="w-full bg-muted rounded-full h-1.5 mb-1">
                <div
                  className="bg-blue-500 h-1.5 rounded-full transition-all"
                  style={{ width: `${pct}%` }}
                />
              </div>
              <div className="flex gap-3 text-xs text-muted-foreground">
                <span>{m.totalWordCount.toLocaleString()} words</span>
                <span>{dur} recorded</span>
                {wpm && <span>{wpm} avg</span>}
              </div>
            </div>
          </div>
        );
      })}
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
