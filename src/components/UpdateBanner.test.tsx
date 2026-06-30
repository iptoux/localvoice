import { describe, expect, it, vi, beforeEach } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { UpdateBanner } from "./UpdateBanner";
import { useUpdaterStore } from "../stores/updater-store";

vi.mock("../lib/tauri", () => ({
  checkForUpdate: vi.fn().mockResolvedValue(null),
  getUpdateStatus: vi.fn().mockResolvedValue({
    phase: "idle",
    available: null,
    progress: null,
    lastError: null,
  }),
  installPendingUpdate: vi.fn().mockResolvedValue(undefined),
}));

const BASE_STATUS = {
  phase: "available",
  available: {
    version: "0.3.0",
    currentVersion: "0.2.3",
  },
  progress: null,
  lastError: null,
};

describe("UpdateBanner", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useUpdaterStore.setState({
      status: BASE_STATUS,
      dismissedVersion: null,
      loading: false,
    });
  });

  it("renders an available update and install action", () => {
    render(<UpdateBanner />);

    expect(screen.getByText("LocalVoice 0.3.0 is available")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Update Now/i })).toBeEnabled();
  });

  it("keeps the banner sticky at the top of the content area", () => {
    render(<UpdateBanner />);

    expect(screen.getByTestId("update-banner")).toHaveClass(
      "sticky",
      "top-0",
      "z-50",
      "border-b",
      "bg-card/95",
      "backdrop-blur",
    );
  });

  it("hides after dismissing until another update event changes state", async () => {
    render(<UpdateBanner />);

    fireEvent.click(screen.getByLabelText("Dismiss update"));

    await waitFor(() => {
      expect(screen.queryByText("LocalVoice 0.3.0 is available")).not.toBeInTheDocument();
    });
  });

  it("shows progress while downloading", () => {
    useUpdaterStore.setState({
      status: {
        ...BASE_STATUS,
        phase: "downloading",
        progress: {
          downloadedBytes: 50,
          totalBytes: 100,
          percent: 50,
        },
      },
      loading: true,
    });

    render(<UpdateBanner />);

    expect(screen.getByText(/50%/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Updating/i })).toBeDisabled();
  });
});
