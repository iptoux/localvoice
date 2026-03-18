import { useEffect } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  BarChart,
  Bar,
} from "recharts";
import { useDashboardStore, type RangePreset } from "../stores/dashboard-store";
import type {
  CorrectionStat,
  DashboardStats,
  LanguageBreakdown,
  TimeseriesPoint,
  WpmPoint,
} from "../types";

// ── Colors ──────────────────────────────────────────────────────────────────

const PIE_COLORS = ["#60a5fa", "#34d399", "#fbbf24", "#f87171", "#a78bfa", "#fb923c"];
const TOOLTIP_STYLE = {
  background: "var(--color-card)",
  border: "1px solid var(--color-border)",
  borderRadius: 8,
  color: "var(--color-foreground)",
  fontSize: 12,
};

// ── Main page ─────────────────────────────────────────────────────────────────

export default function Dashboard() {
  const {
    stats,
    timeseries,
    languageBreakdown,
    correctionStats,
    wpmTrend,
    range,
    loading,
    error,
    setRange,
    fetch,
  } = useDashboardStore();

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

      {/* Stat cards */}
      <StatCards stats={stats} loading={loading} />

      {/* Charts row 1: Words over time + Language breakdown */}
      <div className="grid grid-cols-1 xl:grid-cols-3 gap-4">
        <section className="bg-card rounded-xl border border-border p-5 xl:col-span-2">
          <h2 className="text-sm font-semibold text-foreground/70 mb-4">
            Words over time
          </h2>
          <WordsChart data={timeseries} loading={loading} />
        </section>

        <section className="bg-card rounded-xl border border-border p-5">
          <h2 className="text-sm font-semibold text-foreground/70 mb-4">
            Languages
          </h2>
          <LanguagePie data={languageBreakdown} loading={loading} />
        </section>
      </div>

      {/* Charts row 2: WPM trend + Correction stats */}
      <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
        <section className="bg-card rounded-xl border border-border p-5">
          <h2 className="text-sm font-semibold text-foreground/70 mb-4">
            WPM trend
          </h2>
          <WpmTrendChart data={wpmTrend} loading={loading} />
        </section>

        <section className="bg-card rounded-xl border border-border p-5">
          <h2 className="text-sm font-semibold text-foreground/70 mb-4">
            Top corrections
          </h2>
          <CorrectionFrequency data={correctionStats} loading={loading} />
        </section>
      </div>

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

// ── Date-range selector ─────────────────────────────────────────────────────

const RANGE_LABELS: Record<RangePreset, string> = {
  "7d": "7 days",
  "30d": "30 days",
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
              ? "bg-accent text-foreground"
              : "text-muted-foreground hover:text-foreground"
          }`}
        >
          {RANGE_LABELS[r]}
        </button>
      ))}
    </div>
  );
}

// ── Stat cards ──────────────────────────────────────────────────────────────

function StatCards({
  stats,
  loading,
}: {
  stats: DashboardStats | null;
  loading: boolean;
}) {
  const durationLabel = stats ? formatDuration(stats.totalDurationMs) : "—";
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

// ── Words-over-time line chart ──────────────────────────────────────────────

function WordsChart({
  data,
  loading,
}: {
  data: TimeseriesPoint[];
  loading: boolean;
}) {
  if (loading) return <ChartPlaceholder label="Loading…" />;
  if (data.length === 0)
    return <ChartPlaceholder label="No data yet. Record a few sessions to see trends." />;

  return (
    <ResponsiveContainer width="100%" height={200}>
      <LineChart data={data} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>
        <XAxis
          dataKey="date"
          tick={{ fill: "var(--color-muted-foreground)", fontSize: 11 }}
          tickFormatter={shortDate}
          tickLine={false}
          axisLine={false}
          interval="preserveStartEnd"
        />
        <YAxis
          tick={{ fill: "var(--color-muted-foreground)", fontSize: 11 }}
          tickLine={false}
          axisLine={false}
          allowDecimals={false}
        />
        <Tooltip
          contentStyle={TOOLTIP_STYLE}
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

// ── Language pie chart (TASK-215) ────────────────────────────────────────────

function LanguagePie({
  data,
  loading,
}: {
  data: LanguageBreakdown[];
  loading: boolean;
}) {
  if (loading) return <ChartPlaceholder label="Loading…" />;
  if (data.length === 0) return <ChartPlaceholder label="No sessions yet." />;

  const total = data.reduce((sum, d) => sum + d.wordCount, 0);
  const chartData = data.map((d) => ({
    name: d.language.toUpperCase(),
    value: d.wordCount,
    sessions: d.sessionCount,
  }));

  return (
    <div className="flex flex-col items-center gap-3">
      <ResponsiveContainer width="100%" height={160}>
        <PieChart>
          <Pie
            data={chartData}
            dataKey="value"
            nameKey="name"
            cx="50%"
            cy="50%"
            innerRadius={40}
            outerRadius={70}
            paddingAngle={2}
            strokeWidth={0}
          >
            {chartData.map((_, i) => (
              <Cell key={i} fill={PIE_COLORS[i % PIE_COLORS.length]} />
            ))}
          </Pie>
          <Tooltip
            contentStyle={TOOLTIP_STYLE}
            formatter={(v) => [(v as number).toLocaleString(), "Words"]}
          />
        </PieChart>
      </ResponsiveContainer>
      <div className="flex flex-wrap justify-center gap-x-4 gap-y-1">
        {chartData.map((d, i) => {
          const pct = total > 0 ? Math.round((d.value / total) * 100) : 0;
          return (
            <span key={d.name} className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <span
                className="w-2 h-2 rounded-full shrink-0"
                style={{ background: PIE_COLORS[i % PIE_COLORS.length] }}
              />
              {d.name} {pct}%
            </span>
          );
        })}
      </div>
    </div>
  );
}

// ── WPM trend chart (TASK-217) ──────────────────────────────────────────────

function WpmTrendChart({
  data,
  loading,
}: {
  data: WpmPoint[];
  loading: boolean;
}) {
  if (loading) return <ChartPlaceholder label="Loading…" />;
  if (data.length === 0) return <ChartPlaceholder label="No WPM data yet." />;

  return (
    <ResponsiveContainer width="100%" height={200}>
      <LineChart data={data} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>
        <XAxis
          dataKey="date"
          tick={{ fill: "var(--color-muted-foreground)", fontSize: 11 }}
          tickFormatter={shortDate}
          tickLine={false}
          axisLine={false}
          interval="preserveStartEnd"
        />
        <YAxis
          tick={{ fill: "var(--color-muted-foreground)", fontSize: 11 }}
          tickLine={false}
          axisLine={false}
          allowDecimals={false}
        />
        <Tooltip
          contentStyle={TOOLTIP_STYLE}
          formatter={(v) => [`${Math.round(v as number)} wpm`, "Avg WPM"]}
          labelFormatter={(l) => l}
        />
        <Line
          type="monotone"
          dataKey="avgWpm"
          stroke="#34d399"
          strokeWidth={2}
          dot={false}
          activeDot={{ r: 4, fill: "#34d399" }}
        />
      </LineChart>
    </ResponsiveContainer>
  );
}

// ── Correction frequency (TASK-216) ─────────────────────────────────────────

function CorrectionFrequency({
  data,
  loading,
}: {
  data: CorrectionStat[];
  loading: boolean;
}) {
  if (loading) return <ChartPlaceholder label="Loading…" />;
  if (data.length === 0)
    return <ChartPlaceholder label="No correction rules used yet." />;

  const maxCount = Math.max(...data.map((d) => d.usageCount), 1);
  const chartData = data.slice(0, 10).map((d) => ({
    name: `${d.sourcePhrase} → ${d.targetPhrase}`,
    count: d.usageCount,
  }));

  if (chartData.length <= 5) {
    // Simple bar list for few items.
    return (
      <div className="flex flex-col gap-2.5">
        {chartData.map((d) => {
          const pct = Math.round((d.count / maxCount) * 100);
          return (
            <div key={d.name}>
              <div className="flex items-center justify-between mb-1 gap-2">
                <span className="text-xs text-foreground/80 truncate">{d.name}</span>
                <span className="text-xs text-muted-foreground shrink-0 tabular-nums">
                  {d.count}x
                </span>
              </div>
              <div className="w-full bg-muted rounded-full h-1.5">
                <div
                  className="bg-amber-500 h-1.5 rounded-full transition-all"
                  style={{ width: `${pct}%` }}
                />
              </div>
            </div>
          );
        })}
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height={200}>
      <BarChart data={chartData} layout="vertical" margin={{ left: 0, right: 8 }}>
        <XAxis type="number" hide />
        <YAxis
          type="category"
          dataKey="name"
          tick={{ fill: "var(--color-muted-foreground)", fontSize: 10 }}
          width={120}
          tickLine={false}
          axisLine={false}
        />
        <Tooltip
          contentStyle={TOOLTIP_STYLE}
          formatter={(v) => [`${v}x`, "Used"]}
        />
        <Bar dataKey="count" fill="#fbbf24" radius={[0, 4, 4, 0]} barSize={14} />
      </BarChart>
    </ResponsiveContainer>
  );
}

// ── Top models ──────────────────────────────────────────────────────────────

function TopModels({
  stats,
  loading,
}: {
  stats: DashboardStats | null;
  loading: boolean;
}) {
  if (loading) return <ChartPlaceholder label="Loading…" />;

  const models = stats?.topModels ?? [];
  if (models.length === 0) return <ChartPlaceholder label="No sessions yet." />;

  const maxSessions = Math.max(...models.map((m) => m.sessionCount), 1);

  return (
    <div className="flex flex-col gap-3">
      {models.map((m, i) => {
        const pct = Math.round((m.sessionCount / maxSessions) * 100);
        const label = m.modelId === "unknown" ? "Unknown" : m.modelId.replace(/^ggml-/, "");
        const wpm = m.avgWpm > 0 ? `${Math.round(m.avgWpm)} wpm` : null;
        const dur = formatDuration(m.totalDurationMs);

        return (
          <div key={m.modelId} className="flex items-center gap-3">
            <span className="text-sm w-6 shrink-0 text-muted-foreground font-medium">
              #{i + 1}
            </span>
            <div className="flex-1 min-w-0">
              <div className="flex items-center justify-between mb-1 gap-2">
                <span className="text-sm text-foreground font-medium truncate capitalize">
                  {label}
                </span>
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

// ── Shared helpers ──────────────────────────────────────────────────────────

function ChartPlaceholder({ label }: { label: string }) {
  return (
    <div className="flex items-center justify-center h-32 text-muted-foreground text-sm">
      {label}
    </div>
  );
}

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

function shortDate(iso: string): string {
  try {
    const d = new Date(iso + "T00:00:00");
    return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
  } catch {
    return iso;
  }
}
