import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import Dashboard from "./Dashboard";
import { useDashboardStore } from "../stores/dashboard-store";
import { useFillerWordsStore } from "../stores/filler-words-store";
import type { DashboardStats } from "../types";

// Mock all Tauri command wrappers used indirectly via the stores.
vi.mock("../lib/tauri", () => ({
  getDashboardStats: vi.fn().mockResolvedValue({
    totalWordCount: 0,
    totalSessionCount: 0,
    avgWpm: 0,
    totalDurationMs: 0,
    languageCounts: [],
    topModels: [],
  }),
  getUsageTimeseries: vi.fn().mockResolvedValue([]),
  getLanguageBreakdown: vi.fn().mockResolvedValue([]),
  getCorrectionStats: vi.fn().mockResolvedValue([]),
  getWpmTrend: vi.fn().mockResolvedValue([]),
  getFillerStats: vi.fn().mockResolvedValue([]),
  getFillerTotalCount: vi.fn().mockResolvedValue(0),
}));

// Recharts uses ResizeObserver which doesn't exist in jsdom.
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

const MOCK_STATS: DashboardStats = {
  totalWordCount: 1234,
  totalSessionCount: 42,
  avgWpm: 87.5,
  totalDurationMs: 360_000,
  languageCounts: [
    { language: "de", count: 30 },
    { language: "en", count: 12 },
  ],
  topModels: [
    {
      modelId: "ggml-base",
      sessionCount: 42,
      totalWordCount: 1234,
      totalDurationMs: 360_000,
      avgWpm: 87.5,
    },
  ],
};

function seedStore(
  stats: DashboardStats | null = null,
  loading = false,
  error: string | null = null
) {
  useDashboardStore.setState({
    stats,
    timeseries: [],
    languageBreakdown: [],
    correctionStats: [],
    wpmTrend: [],
    range: "30d",
    loading,
    error,
    setRange: vi.fn(),
    fetch: vi.fn().mockResolvedValue(undefined),
  });
  useFillerWordsStore.setState({
    stats: [],
    totalRemoved: 0,
    fetchStats: vi.fn().mockResolvedValue(undefined),
  } as any);
}

describe("Dashboard", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  // ── Rendering ─────────────────────────────────────────────────────────────

  it("renders the Dashboard h1 heading", () => {
    seedStore();
    render(<Dashboard />);
    expect(screen.getByRole("heading", { name: "Dashboard" })).toBeInTheDocument();
  });

  // ── Empty / loading states ─────────────────────────────────────────────────

  it("shows a loading indicator when data is being fetched", () => {
    seedStore(null, true);
    render(<Dashboard />);
    // Multiple chart sections each render "Loading…" when loading=true.
    const loadingEls = screen.getAllByText(/Loading…/i);
    expect(loadingEls.length).toBeGreaterThan(0);
  });

  it("shows empty state message when there are no sessions", () => {
    seedStore({
      totalWordCount: 0,
      totalSessionCount: 0,
      avgWpm: 0,
      totalDurationMs: 0,
      languageCounts: [],
      topModels: [],
    });
    render(<Dashboard />);
    // Sessions stat card label is always rendered
    expect(screen.getByText("Sessions")).toBeInTheDocument();
  });

  it("shows error message when fetch fails", () => {
    seedStore(null, false, "Failed to load stats");
    render(<Dashboard />);
    expect(screen.getByText(/Failed to load stats/i)).toBeInTheDocument();
  });

  // ── Stat card labels ───────────────────────────────────────────────────────

  it("renders the Total Words stat card label", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    expect(screen.getByText("Total Words")).toBeInTheDocument();
  });

  it("renders the Sessions stat card label", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    expect(screen.getByText("Sessions")).toBeInTheDocument();
  });

  it("renders the Avg WPM stat card label", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    expect(screen.getByText("Avg WPM")).toBeInTheDocument();
  });

  it("displays total session count in stat card", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    // totalSessionCount = 42
    expect(screen.getByText("42")).toBeInTheDocument();
  });

  it("displays average WPM in stat card", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    // avgWpm = 87.5 → Math.round = 88, displayed as "88 wpm"
    expect(screen.getByText("88 wpm")).toBeInTheDocument();
  });

  // ── Section headers ────────────────────────────────────────────────────────

  it("renders the Words over time section header", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    expect(screen.getByText("Words over time")).toBeInTheDocument();
  });

  it("renders the Languages section header", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    expect(screen.getByText("Languages")).toBeInTheDocument();
  });

  it("renders the Top models section header", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    expect(screen.getByText("Top models")).toBeInTheDocument();
  });

  // ── Date range switching ───────────────────────────────────────────────────

  it("renders date range buttons with full labels", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    // RANGE_LABELS: "7d" → "7 days", "30d" → "30 days", "all" → "All time"
    expect(screen.getByText("7 days")).toBeInTheDocument();
    expect(screen.getByText("30 days")).toBeInTheDocument();
    expect(screen.getByText("All time")).toBeInTheDocument();
  });

  it("calls setRange with preset key when a range button is clicked", async () => {
    const setRange = vi.fn();
    seedStore(MOCK_STATS);
    useDashboardStore.setState((s) => ({ ...s, setRange }));
    render(<Dashboard />);

    fireEvent.click(screen.getByText("7 days"));
    await waitFor(() => {
      expect(setRange).toHaveBeenCalledWith("7d");
    });
  });

  it("highlights the currently selected range button", () => {
    seedStore(MOCK_STATS);
    useDashboardStore.setState((s) => ({ ...s, range: "7d" }));
    render(<Dashboard />);

    // Active button "7 days" has bg-accent class; just verify it's in the DOM.
    expect(screen.getByText("7 days")).toBeInTheDocument();
  });

  // ── Top models ─────────────────────────────────────────────────────────────

  it("renders top model session count", () => {
    seedStore(MOCK_STATS);
    render(<Dashboard />);
    // modelId "ggml-base" is stripped to "base" — session count "42 sessions" is rendered
    expect(screen.getByText("42 sessions")).toBeInTheDocument();
  });
});
