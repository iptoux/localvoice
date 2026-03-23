import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import SettingsPage from "./SettingsPage";
import { useSettingsStore } from "../stores/settings-store";
import { useAppStore } from "../stores/app-store";

// Mock all Tauri command wrappers.
vi.mock("../lib/tauri", () => ({
  getSettings: vi.fn().mockResolvedValue({}),
  updateSetting: vi.fn().mockResolvedValue(undefined),
  listInputDevices: vi.fn().mockResolvedValue([]),
  getAutostart: vi.fn().mockResolvedValue(false),
  setAutostart: vi.fn().mockResolvedValue(undefined),
  updateShortcut: vi.fn().mockResolvedValue(undefined),
  clearLogs: vi.fn().mockResolvedValue(undefined),
  setLoggingEnabled: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("@tauri-apps/api/event", () => ({
  emit: vi.fn().mockResolvedValue(undefined),
  emitTo: vi.fn().mockResolvedValue(undefined),
}));

const DEFAULT_SETTINGS = {
  "app.theme": "dark",
  "app.language": "de",
  "app.start_hidden": "false",
  "app.autostart": "false",
  "ui.default_mode": "pill",
  "ui.pill.always_on_top": "true",
  "recording.shortcut": "CommandOrControl+Shift+Space",
  "recording.push_to_talk": "false",
  "recording.silence_timeout_ms": "1500",
  "recording.keep_audio": "false",
  "recording.audio_retention_days": "7",
  "transcription.default_language": "auto",
  "transcription.auto_punctuation": "true",
  "transcription.auto_capitalization": "true",
  "transcription.remove_fillers": "false",
  "output.mode": "clipboard",
  "output.auto_paste": "false",
  "output.insert_delay_ms": "100",
  "dictionary.auto_apply_rules": "true",
  "notifications.on_error": "true",
  "notifications.on_success": "false",
  "logging.enabled": "true",
};

function seedStore(overrides: Record<string, string> = {}) {
  useSettingsStore.setState({
    settings: { ...DEFAULT_SETTINGS, ...overrides },
    loading: false,
    load: vi.fn().mockResolvedValue(undefined),
    update: vi.fn().mockResolvedValue(undefined),
  });
  useAppStore.setState({ audioDevices: [] });
}

describe("SettingsPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    seedStore();
  });

  // ── Rendering ─────────────────────────────────────────────────────────────

  it("renders the Settings h1 heading", () => {
    render(<SettingsPage />);
    // Use getByRole to target the <h1> specifically.
    expect(screen.getByRole("heading", { name: "Settings" })).toBeInTheDocument();
  });

  it("renders the Recording section header", () => {
    render(<SettingsPage />);
    expect(screen.getByText("Recording")).toBeInTheDocument();
  });

  it("renders the Transcription section header", () => {
    render(<SettingsPage />);
    expect(screen.getByText("Transcription")).toBeInTheDocument();
  });

  it("renders the Output section header", () => {
    render(<SettingsPage />);
    expect(screen.getByText("Output")).toBeInTheDocument();
  });

  it("renders the Appearance section header", () => {
    render(<SettingsPage />);
    expect(screen.getByText("Appearance")).toBeInTheDocument();
  });

  // ── Loading state ─────────────────────────────────────────────────────────

  it("shows Loading text while settings are loading", () => {
    useSettingsStore.setState({ settings: {}, loading: true });
    render(<SettingsPage />);
    expect(screen.getByText(/Loading/i)).toBeInTheDocument();
  });

  // ── Setting labels (exact text from SettingRow label prop) ────────────────

  it("renders the Global Shortcut row", () => {
    render(<SettingsPage />);
    expect(screen.getByText("Global Shortcut")).toBeInTheDocument();
  });

  it("renders the Output mode row", () => {
    render(<SettingsPage />);
    // Exact label as rendered in SettingRow: "Output mode"
    expect(screen.getByText("Output mode")).toBeInTheDocument();
  });

  it("renders the Auto-punctuation row", () => {
    render(<SettingsPage />);
    expect(screen.getByText("Auto-punctuation")).toBeInTheDocument();
  });

  it("renders the Remove filler words row", () => {
    render(<SettingsPage />);
    expect(screen.getByText("Remove filler words")).toBeInTheDocument();
  });

  // ── Switch toggling ───────────────────────────────────────────────────────

  it("calls update when a Switch is toggled", async () => {
    const updateMock = vi.fn().mockResolvedValue(undefined);
    useSettingsStore.setState((s) => ({ ...s, update: updateMock }));
    render(<SettingsPage />);

    const switches = screen.getAllByRole("switch");
    expect(switches.length).toBeGreaterThan(0);

    fireEvent.click(switches[0]);
    await waitFor(() => {
      expect(updateMock).toHaveBeenCalled();
    });
  });

  // ── Shortcut badge ────────────────────────────────────────────────────────

  it("renders the configured shortcut keys", () => {
    seedStore({ "recording.shortcut": "CommandOrControl+Shift+Space" });
    render(<SettingsPage />);
    // ShortcutBadge renders "CommandOrControl" as "Ctrl"
    expect(screen.getByText("Ctrl")).toBeInTheDocument();
    expect(screen.getByText("Space")).toBeInTheDocument();
  });

  // ── Audio devices ─────────────────────────────────────────────────────────

  it("shows System default placeholder when no audio devices are present", () => {
    useAppStore.setState({ audioDevices: [] });
    render(<SettingsPage />);
    expect(screen.getByText("System default")).toBeInTheDocument();
  });

  it("renders the microphone select trigger when devices are available", () => {
    useAppStore.setState({
      audioDevices: [
        { id: "dev-1", name: "Built-in Mic", isDefault: true },
        { id: "dev-2", name: "USB Headset", isDefault: false },
      ],
    });
    render(<SettingsPage />);
    // The select trigger (combobox) for the microphone is always rendered.
    // SelectContent items are in a portal and only mounted when the dropdown is open.
    const comboboxes = screen.getAllByRole("combobox");
    expect(comboboxes.length).toBeGreaterThan(0);
  });
});
