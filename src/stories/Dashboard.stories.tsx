import type { Meta, StoryObj } from "@storybook/react";
import { useMemo } from "react";
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
import {
  mockTimeseries,
  mockLanguageBreakdown,
  mockWpmTrend,
  mockCorrectionStats,
} from "../mocks/tauri";

const PIE_COLORS = ["#60a5fa", "#34d399", "#fbbf24", "#f87171", "#a78bfa", "#fb923c"];
const TOOLTIP_STYLE = {
  background: "var(--color-card)",
  border: "1px solid var(--color-border)",
  borderRadius: 8,
  color: "var(--color-foreground)",
  fontSize: 12,
};

const meta: Meta = {
  title: "Dashboard/Charts",
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
    backgrounds: {
      default: "dark",
    },
  },
};

export default meta;

function shortDate(iso: string): string {
  try {
    const d = new Date(iso + "T00:00:00");
    return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
  } catch {
    return iso;
  }
}

function WordsOverTime() {
  return (
    <div className="bg-card rounded-xl border border-border p-5">
      <h2 className="text-sm font-semibold text-foreground/70 mb-4">
        Words over time
      </h2>
      <ResponsiveContainer width="100%" height={200}>
        <LineChart data={mockTimeseries} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>
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
    </div>
  );
}

function LanguagePie() {
  const total = useMemo(
    () => mockLanguageBreakdown.reduce((sum, d) => sum + d.wordCount, 0),
    []
  );
  const chartData = useMemo(
    () =>
      mockLanguageBreakdown.map((d) => ({
        name: d.language.toUpperCase(),
        value: d.wordCount,
        sessions: d.sessionCount,
      })),
    []
  );

  return (
    <div className="bg-card rounded-xl border border-border p-5">
      <h2 className="text-sm font-semibold text-foreground/70 mb-4">
        Languages
      </h2>
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
              <span
                key={d.name}
                className="flex items-center gap-1.5 text-xs text-muted-foreground"
              >
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
    </div>
  );
}

function WpmTrend() {
  return (
    <div className="bg-card rounded-xl border border-border p-5">
      <h2 className="text-sm font-semibold text-foreground/70 mb-4">
        WPM trend
      </h2>
      <ResponsiveContainer width="100%" height={200}>
        <LineChart data={mockWpmTrend} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>
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
    </div>
  );
}

function CorrectionFrequency() {
  const chartData = useMemo(
    () =>
      mockCorrectionStats.slice(0, 10).map((d) => ({
        name: `${d.sourcePhrase} → ${d.targetPhrase}`,
        count: d.usageCount,
      })),
    []
  );

  return (
    <div className="bg-card rounded-xl border border-border p-5">
      <h2 className="text-sm font-semibold text-foreground/70 mb-4">
        Top corrections
      </h2>
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

function StatCards() {
  return (
    <div className="grid grid-cols-2 xl:grid-cols-4 gap-4">
      <StatCard label="Total Words" value="15,420" />
      <StatCard label="Sessions" value="142" />
      <StatCard label="Avg WPM" value="132 wpm" />
      <StatCard label="Recording Time" value="15:00" />
    </div>
  );
}

const RANGE_LABELS = {
  "7d": "7 days",
  "30d": "30 days",
  all: "All time",
} as const;

function RangeSelector() {
  return (
    <div className="flex gap-1 bg-muted rounded-lg p-1">
      {(Object.keys(RANGE_LABELS) as Array<keyof typeof RANGE_LABELS>).map((r) => (
        <button
          key={r}
          className="px-3 py-1 text-xs rounded-md transition-colors bg-accent text-foreground"
        >
          {RANGE_LABELS[r]}
        </button>
      ))}
    </div>
  );
}

export const WordsOverTimeChart: StoryObj = {
  render: () => <WordsOverTime />,
};

export const LanguagePieChart: StoryObj = {
  render: () => <LanguagePie />,
};

export const WpmTrendChart: StoryObj = {
  render: () => <WpmTrend />,
};

export const CorrectionFrequencyChart: StoryObj = {
  render: () => <CorrectionFrequency />,
};

export const StatCardsStory: StoryObj = {
  render: () => <StatCards />,
};

export const DashboardGrid: StoryObj = {
  parameters: {
    layout: "fullscreen",
  },
  render: () => (
    <div className="p-8 space-y-6 bg-zinc-950 min-h-screen">
      <div className="flex items-center justify-between gap-4">
        <h1 className="text-2xl font-semibold text-foreground">Dashboard</h1>
        <RangeSelector />
      </div>

      <StatCards />

      <div className="grid grid-cols-1 xl:grid-cols-3 gap-4">
        <div className="xl:col-span-2">
          <WordsOverTime />
        </div>
        <LanguagePie />
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
        <WpmTrend />
        <CorrectionFrequency />
      </div>
    </div>
  ),
};
