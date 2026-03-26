import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, fireEvent, act } from "@testing-library/react";
import { Pill } from "./Pill";
import { useAppStore } from "../../stores/app-store";
import type { RecordingState } from "../../types";

// Mock lib/tauri.ts — all calls return resolved promises.
vi.mock("../../lib/tauri", () => ({
  getSettings: vi.fn().mockResolvedValue({
    "recording.push_to_talk": "false",
  }),
  expandPill: vi.fn().mockResolvedValue(undefined),
  collapsePill: vi.fn().mockResolvedValue(undefined),
  openMainWindow: vi.fn().mockResolvedValue(undefined),
}));

// Mock Waveform — replaces SVG animation that doesn't work in jsdom.
vi.mock("./Waveform", () => ({
  Waveform: () => <div data-testid="waveform" />,
}));

// Mock ExpandedPill.
vi.mock("./ExpandedPill", () => ({
  ExpandedPill: () => <div data-testid="expanded-pill" />,
}));

// Color classes for each recording state, matching STATE_COLOR in Pill.tsx.
const STATE_COLORS: Record<RecordingState, string> = {
  idle: "bg-card",
  listening: "bg-red-600",
  processing: "bg-amber-500",
  success: "bg-green-600",
  error: "bg-rose-700",
};

function setStoreState(partial: Partial<ReturnType<typeof useAppStore.getState>>) {
  useAppStore.setState(partial);
}

describe("Pill", () => {
  beforeEach(() => {
    // Reset store to a known initial state before each test.
    useAppStore.setState({
      recordingState: "idle",
      recordingError: null,
      isPillExpanded: false,
      lastTranscription: null,
      lastOutputResult: null,
    });
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  // ── Color mapping ──────────────────────────────────────────────────────────

  it.each(Object.entries(STATE_COLORS) as [RecordingState, string][])(
    "applies %s color class for state %s",
    (state, colorClass) => {
      setStoreState({ recordingState: state });
      const { container } = render(<Pill />);
      expect(container.firstChild).toHaveClass(colorClass);
    }
  );

  // ── Idle state ────────────────────────────────────────────────────────────

  it("shows LocalVoice title in idle state", () => {
    setStoreState({ recordingState: "idle" });
    render(<Pill />);
    expect(screen.getByText("LocalVoice")).toBeInTheDocument();
  });

  // ── Listening state ───────────────────────────────────────────────────────

  it("shows waveform when listening", () => {
    setStoreState({ recordingState: "listening" });
    render(<Pill />);
    expect(screen.getByTestId("waveform")).toBeInTheDocument();
  });

  // ── Processing state ──────────────────────────────────────────────────────

  it("shows transcribing text when processing", () => {
    setStoreState({ recordingState: "processing" });
    render(<Pill />);
    expect(screen.getByText("Transcribing…")).toBeInTheDocument();
  });

  // ── Error state ───────────────────────────────────────────────────────────

  it("shows error message in error state", () => {
    setStoreState({
      recordingState: "error",
      recordingError: "Microphone not found",
    });
    render(<Pill />);
    expect(screen.getByText("Microphone not found")).toBeInTheDocument();
  });

  // ── Success state ─────────────────────────────────────────────────────────

  it("shows Copied badge in success state when output is clipboard", () => {
    setStoreState({
      recordingState: "success",
      lastTranscription: {
        rawText: "hello",
        cleanedText: "Hello.",
        segments: [],
        language: "en",
        modelId: "ggml-base",
        durationMs: 1000,
        output: undefined,
        removedFillers: [],
      },
      lastOutputResult: { mode: "clipboard", success: true, error: undefined },
    });
    render(<Pill />);
    expect(screen.getByText("Copied")).toBeInTheDocument();
  });

  it("shows Inserted badge when output mode is insert", () => {
    setStoreState({
      recordingState: "success",
      lastTranscription: {
        rawText: "hello",
        cleanedText: "Hello.",
        segments: [],
        language: "en",
        modelId: "ggml-base",
        durationMs: 1000,
        output: undefined,
        removedFillers: [],
      },
      lastOutputResult: { mode: "insert", success: true, error: undefined },
    });
    render(<Pill />);
    expect(screen.getByText("Inserted")).toBeInTheDocument();
  });

  it("truncates long text in success state preview", () => {
    const longText = "This is a very long text that should be truncated in the preview";
    setStoreState({
      recordingState: "success",
      lastTranscription: {
        rawText: longText,
        cleanedText: longText,
        segments: [],
        language: "en",
        modelId: "ggml-base",
        durationMs: 1000,
        output: undefined,
        removedFillers: [],
      },
      lastOutputResult: null,
    });
    render(<Pill />);
    // Preview is limited to 30 chars + "…"
    const preview = screen.getByText(/…$/);
    expect(preview.textContent?.length).toBeLessThanOrEqual(33);
  });

  // ── Success auto-transition ───────────────────────────────────────────────

  it("auto-transitions from success to idle after SUCCESS_DISPLAY_MS", async () => {
    vi.useFakeTimers();
    setStoreState({ recordingState: "success", lastTranscription: null, lastOutputResult: null });
    render(<Pill />);

    // Before timeout: still success
    expect(useAppStore.getState().recordingState).toBe("success");

    // After SUCCESS_DISPLAY_MS (3000ms): transitions to idle
    await act(async () => {
      vi.advanceTimersByTime(3100);
    });
    expect(useAppStore.getState().recordingState).toBe("idle");
  });

  // ── Interaction handlers ──────────────────────────────────────────────────

  it("calls openMainWindow on double-click", async () => {
    const { openMainWindow } = await import("../../lib/tauri");
    setStoreState({ recordingState: "idle", isPillExpanded: false });
    const { container } = render(<Pill />);
    const dragRegion = container.querySelector("[data-tauri-drag-region]");
    if (dragRegion) {
      fireEvent.doubleClick(dragRegion);
    }
    expect(openMainWindow).toHaveBeenCalled();
  });

  it("calls expandPill on right-click when not expanded", async () => {
    const { expandPill } = await import("../../lib/tauri");
    setStoreState({ recordingState: "idle", isPillExpanded: false });
    const { container } = render(<Pill />);
    const dragRegion = container.querySelector("[data-tauri-drag-region]");
    if (dragRegion) {
      fireEvent.contextMenu(dragRegion);
    }
    expect(expandPill).toHaveBeenCalled();
  });

  it("calls collapsePill on right-click when already expanded", async () => {
    const { collapsePill } = await import("../../lib/tauri");
    setStoreState({ recordingState: "idle", isPillExpanded: true });
    const { container } = render(<Pill />);
    const dragRegion = container.querySelector("[data-tauri-drag-region]");
    if (dragRegion) {
      fireEvent.contextMenu(dragRegion);
    }
    expect(collapsePill).toHaveBeenCalled();
  });

  // ── Expanded state ────────────────────────────────────────────────────────

  it("renders ExpandedPill when isPillExpanded is true", () => {
    setStoreState({ recordingState: "idle", isPillExpanded: true });
    render(<Pill />);
    expect(screen.getByTestId("expanded-pill")).toBeInTheDocument();
  });

  it("does not render ExpandedPill when not expanded", () => {
    setStoreState({ recordingState: "idle", isPillExpanded: false });
    render(<Pill />);
    expect(screen.queryByTestId("expanded-pill")).not.toBeInTheDocument();
  });
});
