import { useEffect, useState } from "react";
import { emit, emitTo } from "@tauri-apps/api/event";
import { useAppStore } from "../stores/app-store";
import { useSettingsStore } from "../stores/settings-store";
import {
  getAutostart,
  listInputDevices,
  setAutostart,
  updateSetting,
  updateShortcut,
  clearLogs,
  setLoggingEnabled,
} from "../lib/tauri";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Mic,
  Keyboard,
  Radio,
  Timer,
  Languages,
  WholeWord,
  Pilcrow,
  Scissors,
  ClipboardCopy,
  Clock,
  LayoutPanelLeft,
  LogIn,
  EyeOff,
  BellOff,
  BellRing,
  ScrollText,
  Trash2,
  Palette,
  HardDrive,
  Calendar,
  Database,
  BookOpen,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { applyTheme, type Theme } from "../lib/theme";

// ── Helpers ───────────────────────────────────────────────────────────────────

function SettingRow({
  label,
  description,
  icon: Icon,
  iconClass = "text-muted-foreground",
  children,
}: {
  label: string;
  description?: string;
  icon: React.ElementType;
  iconClass?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex items-center justify-between gap-8 py-3">
      <div className="flex items-start gap-3 flex-1 min-w-0">
        <Icon className={`size-4 mt-0.5 shrink-0 ${iconClass}`} />
        <div className="min-w-0">
          <p className="text-sm text-foreground">{label}</p>
          {description && (
            <p className="text-xs text-muted-foreground mt-0.5 leading-relaxed">{description}</p>
          )}
        </div>
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

function SectionHeader({ title }: { title: string }) {
  return (
    <h2 className="text-xs font-semibold text-muted-foreground uppercase tracking-widest pt-2 pb-1">
      {title}
    </h2>
  );
}

function ShortcutBadge({ shortcut }: { shortcut: string }) {
  const parts = shortcut.split("+");
  return (
    <div className="flex items-center gap-1">
      {parts.map((part, i) => (
        <span key={i} className="flex items-center gap-1">
          <kbd className="px-2 py-1 bg-muted border border-border rounded text-xs text-foreground/80 font-mono">
            {part === "CommandOrControl" ? "Ctrl" : part}
          </kbd>
          {i < parts.length - 1 && (
            <span className="text-muted-foreground/50 text-xs">+</span>
          )}
        </span>
      ))}
    </div>
  );
}

function ShortcutRecorder({
  shortcut,
  onSave,
}: {
  shortcut: string;
  onSave: (shortcut: string) => void;
}) {
  const [recording, setRecording] = useState(false);
  const [pending, setPending] = useState<string | null>(null);

  useEffect(() => {
    if (!recording) return;

    const handler = (e: KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();

      // Ignore lone modifier presses.
      if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return;

      const parts: string[] = [];
      if (e.ctrlKey || e.metaKey) parts.push("CommandOrControl");
      if (e.shiftKey) parts.push("Shift");
      if (e.altKey) parts.push("Alt");

      // Normalize the key name.
      const key = e.key === " " ? "Space" : e.key.length === 1 ? e.key.toUpperCase() : e.key;
      parts.push(key);

      setPending(parts.join("+"));
      setRecording(false);
    };

    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, [recording]);

  if (recording) {
    return (
      <div className="flex items-center gap-2">
        <span className="text-xs text-muted-foreground animate-pulse">
          Press a key combination…
        </span>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setRecording(false)}
          className="border-border bg-card text-foreground/70 text-xs"
        >
          Cancel
        </Button>
      </div>
    );
  }

  if (pending) {
    return (
      <div className="flex items-center gap-2">
        <ShortcutBadge shortcut={pending} />
        <Button
          variant="outline"
          size="sm"
          onClick={() => { onSave(pending); setPending(null); }}
          className="border-border bg-card text-foreground/70 text-xs"
        >
          Save
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPending(null)}
          className="border-border bg-card text-foreground/70 text-xs"
        >
          Cancel
        </Button>
      </div>
    );
  }

  return (
    <div className="flex items-center gap-2">
      <ShortcutBadge shortcut={shortcut} />
      <Button
        variant="outline"
        size="sm"
        onClick={() => setRecording(true)}
        className="border-border bg-card text-foreground/70 text-xs"
      >
        Change
      </Button>
    </div>
  );
}

// ── Main Component ────────────────────────────────────────────────────────────

export default function SettingsPage() {
  const { settings, loading, load } = useSettingsStore();
  const { audioDevices, setAudioDevices } = useAppStore();
  const [autostart, setAutostartState] = useState(false);
  const [clearingLogs, setClearingLogs] = useState(false);

  useEffect(() => { load(); }, [load]);

  useEffect(() => {
    listInputDevices()
      .then(setAudioDevices)
      .catch((e) => console.error("Failed to load audio devices:", e));
  }, [setAudioDevices]);

  useEffect(() => {
    getAutostart().then(setAutostartState).catch(() => {});
  }, []);

  const set = (key: string, value: string) =>
    updateSetting(key, value).then(load);

  const setVal = (key: string) => (v: string | null) => {
    if (v !== null) set(key, v);
  };

  const handleLoggingToggle = async (enabled: boolean) => {
    await set("logging.enabled", String(enabled));
    await setLoggingEnabled(enabled);
  };

  const handleClearLogs = async () => {
    setClearingLogs(true);
    try {
      await clearLogs();
    } finally {
      setClearingLogs(false);
    }
  };

  const bool = (key: string, fallback = false) =>
    settings[key] === undefined ? fallback : settings[key] === "true";

  if (loading) {
    return <div className="p-8 text-muted-foreground text-sm">Loading…</div>;
  }

  const shortcut = settings["recording.shortcut"] || "CommandOrControl+Shift+Space";
  const outputMode = settings["output.mode"] || "clipboard";
  const language = settings["transcription.default_language"] || "de";
  const deviceId = settings["recording.device_id"] || "";
  const silenceMs = parseInt(settings["recording.silence_timeout_ms"] || "1500");

  return (
    <div className="p-8 max-w-2xl space-y-1">
      <h1 className="text-xl font-semibold text-foreground mb-6">Settings</h1>

      {/* ── Recording ── */}
      <SectionHeader title="Recording" />
      <Separator className="bg-border mb-1" />

      <SettingRow icon={Mic} iconClass="text-blue-400" label="Microphone" description="Audio device used for recording.">
        <Select value={deviceId} onValueChange={setVal("recording.device_id")}>
          <SelectTrigger className="w-52 bg-card border-border text-sm">
            <SelectValue placeholder="System default" />
          </SelectTrigger>
          <SelectContent className="bg-card border-border">
            <SelectItem value="">System default</SelectItem>
            {audioDevices.map((d) => (
              <SelectItem key={d.id} value={d.id}>
                {d.name}{d.isDefault ? " (default)" : ""}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </SettingRow>

      <SettingRow
        icon={Keyboard}
        iconClass="text-violet-400"
        label="Global Shortcut"
        description="Press anywhere to start or stop recording."
      >
        <ShortcutRecorder
          shortcut={shortcut}
          onSave={(s) => {
            updateShortcut(s).then(load).catch(console.error);
          }}
        />
      </SettingRow>

      <SettingRow
        icon={Radio}
        iconClass="text-rose-400"
        label="Push-to-Talk"
        description="Hold the shortcut to record, release to stop."
      >
        <Switch
          checked={bool("recording.push_to_talk")}
          onCheckedChange={(v) => set("recording.push_to_talk", String(v))}
        />
      </SettingRow>

      <SettingRow
        icon={Timer}
        iconClass="text-amber-400"
        label="Silence timeout"
        description="Seconds of silence before recording stops automatically."
      >
        <Select
          value={String(silenceMs)}
          onValueChange={setVal("recording.silence_timeout_ms")}
        >
          <SelectTrigger className="w-32 bg-card border-border text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-card border-border">
            <SelectItem value="500">0.5 s</SelectItem>
            <SelectItem value="1000">1 s</SelectItem>
            <SelectItem value="1500">1.5 s</SelectItem>
            <SelectItem value="2000">2 s</SelectItem>
            <SelectItem value="3000">3 s</SelectItem>
            <SelectItem value="5000">5 s</SelectItem>
          </SelectContent>
        </Select>
      </SettingRow>

      {/* ── Transcription ── */}
      <div className="mt-10">
        <SectionHeader title="Transcription" />
      </div>
      <Separator className="bg-border mb-1" />

      <SettingRow icon={Languages} iconClass="text-cyan-400" label="Language" description="Language passed to the Whisper model.">
        <Select value={language} onValueChange={setVal("transcription.default_language")}>
          <SelectTrigger className="w-44 bg-card border-border text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-card border-border">
            <SelectItem value="auto">Auto-detect</SelectItem>
            <SelectItem value="de">German</SelectItem>
            <SelectItem value="en">English</SelectItem>
            <SelectItem value="fr">French</SelectItem>
            <SelectItem value="es">Spanish</SelectItem>
            <SelectItem value="it">Italian</SelectItem>
            <SelectItem value="pt">Portuguese</SelectItem>
            <SelectItem value="nl">Dutch</SelectItem>
            <SelectItem value="pl">Polish</SelectItem>
            <SelectItem value="ru">Russian</SelectItem>
            <SelectItem value="ja">Japanese</SelectItem>
            <SelectItem value="zh">Chinese</SelectItem>
          </SelectContent>
        </Select>
      </SettingRow>

      <SettingRow
        icon={WholeWord}
        iconClass="text-sky-400"
        label="Auto-capitalization"
        description="Capitalize the first word of each transcription."
      >
        <Switch
          checked={bool("transcription.auto_capitalization", true)}
          onCheckedChange={(v) => set("transcription.auto_capitalization", String(v))}
        />
      </SettingRow>

      <SettingRow
        icon={Pilcrow}
        iconClass="text-teal-400"
        label="Auto-punctuation"
        description="Add punctuation automatically where missing."
      >
        <Switch
          checked={bool("transcription.auto_punctuation", true)}
          onCheckedChange={(v) => set("transcription.auto_punctuation", String(v))}
        />
      </SettingRow>

      <SettingRow
        icon={Scissors}
        iconClass="text-orange-400"
        label="Remove filler words"
        description='Strip words like "uh", "um", "äh" from the output.'
      >
        <Switch
          checked={bool("transcription.remove_fillers")}
          onCheckedChange={(v) => set("transcription.remove_fillers", String(v))}
        />
      </SettingRow>

      <SettingRow
        icon={BookOpen}
        iconClass="text-purple-400"
        label="Auto-apply dictionary rules"
        description="Automatically replace words using your correction rules."
      >
        <Switch
          checked={bool("dictionary.auto_apply_rules", true)}
          onCheckedChange={(v) => set("dictionary.auto_apply_rules", String(v))}
        />
      </SettingRow>

      {/* ── Audio Storage ── */}
      <div className="mt-10">
        <SectionHeader title="Audio Storage" />
      </div>
      <Separator className="bg-border mb-1" />

      <SettingRow
        icon={HardDrive}
        iconClass="text-blue-400"
        label="Keep audio files"
        description="Save recorded audio for reprocessing with a different model later."
      >
        <Switch
          checked={bool("recording.keep_audio")}
          onCheckedChange={(v) => set("recording.keep_audio", String(v))}
        />
      </SettingRow>

      {bool("recording.keep_audio") && (
        <>
          <SettingRow
            icon={Calendar}
            iconClass="text-green-400"
            label="Retention period"
            description="Automatically delete audio files older than this."
          >
            <Select
              value={settings["recording.audio_retention_days"] || "7"}
              onValueChange={setVal("recording.audio_retention_days")}
            >
              <SelectTrigger className="w-32 bg-card border-border text-sm">
                <SelectValue />
              </SelectTrigger>
              <SelectContent className="bg-card border-border">
                <SelectItem value="1">1 day</SelectItem>
                <SelectItem value="3">3 days</SelectItem>
                <SelectItem value="7">7 days</SelectItem>
                <SelectItem value="14">14 days</SelectItem>
                <SelectItem value="30">30 days</SelectItem>
                <SelectItem value="90">90 days</SelectItem>
              </SelectContent>
            </Select>
          </SettingRow>

          <SettingRow
            icon={Database}
            iconClass="text-yellow-400"
            label="Max storage"
            description="Maximum disk space for audio files."
          >
            <Select
              value={settings["recording.max_audio_storage_mb"] || "500"}
              onValueChange={setVal("recording.max_audio_storage_mb")}
            >
              <SelectTrigger className="w-32 bg-card border-border text-sm">
                <SelectValue />
              </SelectTrigger>
              <SelectContent className="bg-card border-border">
                <SelectItem value="100">100 MB</SelectItem>
                <SelectItem value="250">250 MB</SelectItem>
                <SelectItem value="500">500 MB</SelectItem>
                <SelectItem value="1000">1 GB</SelectItem>
                <SelectItem value="2000">2 GB</SelectItem>
              </SelectContent>
            </Select>
          </SettingRow>
        </>
      )}

      {/* ── Output ── */}
      <div className="mt-10">
        <SectionHeader title="Output" />
      </div>
      <Separator className="bg-border mb-1" />

      <SettingRow
        icon={ClipboardCopy}
        iconClass="text-emerald-400"
        label="Output mode"
        description="What happens with the transcription after recording."
      >
        <Select value={outputMode} onValueChange={setVal("output.mode")}>
          <SelectTrigger className="w-44 bg-card border-border text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-card border-border">
            <SelectItem value="clipboard">Clipboard only</SelectItem>
            <SelectItem value="insert">Auto-insert</SelectItem>
          </SelectContent>
        </Select>
      </SettingRow>

      {outputMode === "insert" && (
        <>
          <div className="py-2 px-3 rounded-md bg-card border border-border text-xs text-muted-foreground">
            Auto-insert pastes via Ctrl+V into the focused app. Your previous clipboard is restored afterwards.
          </div>

          <SettingRow
            icon={Clock}
            iconClass="text-lime-400"
            label="Insert delay"
            description="Pause before pasting — increase if text is lost in slow apps."
          >
            <div className="flex items-center gap-3 w-52">
              <Slider
                min={50}
                max={500}
                step={50}
                value={[parseInt(settings["output.insert_delay_ms"] || "100")]}
                onValueChange={(val) => set("output.insert_delay_ms", String(Array.isArray(val) ? val[0] : val))}
                className="flex-1"
              />
              <span className="text-xs text-muted-foreground tabular-nums w-12 text-right">
                {settings["output.insert_delay_ms"] || "100"} ms
              </span>
            </div>
          </SettingRow>
        </>
      )}

      {/* ── Appearance ── */}
      <div className="mt-10">
        <SectionHeader title="Appearance" />
      </div>
      <Separator className="bg-border mb-1" />

      <SettingRow
        icon={Palette}
        iconClass="text-pink-400"
        label="Theme"
        description="Appearance for the main window and pill."
      >
        <Select
          value={settings["app.theme"] || "system"}
          onValueChange={(v) => {
            if (v) {
              set("app.theme", v);
              applyTheme(v as Theme);
              emit("theme-changed", v);
              emitTo("pill", "theme-changed", v).catch(() => {});
            }
          }}
        >
          <SelectTrigger className="w-44 bg-card border-border text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-card border-border">
            <SelectItem value="light">Light</SelectItem>
            <SelectItem value="dark">Dark</SelectItem>
            <SelectItem value="system">System</SelectItem>
          </SelectContent>
        </Select>
      </SettingRow>

      <SettingRow
        icon={LayoutPanelLeft}
        iconClass="text-indigo-400"
        label="Default view"
        description="Which window opens when you launch the app."
      >
        <Select
          value={settings["ui.default_mode"] || "pill"}
          onValueChange={setVal("ui.default_mode")}
        >
          <SelectTrigger className="w-44 bg-card border-border text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-card border-border">
            <SelectItem value="pill">Pill (compact)</SelectItem>
            <SelectItem value="main">Main window</SelectItem>
          </SelectContent>
        </Select>
      </SettingRow>

      {/* ── System ── */}
      <div className="mt-10">
        <SectionHeader title="System" />
      </div>
      <Separator className="bg-border mb-1" />

      <SettingRow icon={LogIn} iconClass="text-green-400" label="Launch at login" description="Start automatically when you log in to Windows.">
        <Switch
          checked={autostart}
          onCheckedChange={async (v) => {
            await setAutostart(v);
            setAutostartState(v);
          }}
        />
      </SettingRow>

      <SettingRow
        icon={EyeOff}
        iconClass="text-slate-400"
        label="Start minimized"
        description="Hide the main window on startup, only show the pill."
      >
        <Switch
          checked={bool("app.start_hidden")}
          onCheckedChange={(v) => set("app.start_hidden", String(v))}
        />
      </SettingRow>

      {/* ── Notifications ── */}
      <div className="mt-10">
        <SectionHeader title="Notifications" />
      </div>
      <Separator className="bg-border mb-1" />

      <SettingRow icon={BellOff} iconClass="text-red-400" label="Error notifications" description="Show a system notification when transcription fails.">
        <Switch
          checked={bool("notifications.on_error", true)}
          onCheckedChange={(v) => set("notifications.on_error", String(v))}
        />
      </SettingRow>

      <SettingRow icon={BellRing} iconClass="text-yellow-400" label="Success notifications" description="Show a notification after each successful transcription.">
        <Switch
          checked={bool("notifications.on_success")}
          onCheckedChange={(v) => set("notifications.on_success", String(v))}
        />
      </SettingRow>

      {/* ── Logging ── */}
      <div className="mt-10">
        <SectionHeader title="Logging" />
      </div>
      <Separator className="bg-border mb-1" />

      <SettingRow
        icon={ScrollText}
        iconClass="text-fuchsia-400"
        label="Enable logging"
        description="Buffer app events and errors for the Logs page."
      >
        <Switch
          checked={bool("logging.enabled", true)}
          onCheckedChange={handleLoggingToggle}
        />
      </SettingRow>

      <SettingRow
        icon={Trash2}
        iconClass="text-red-400"
        label="Clear logs"
        description="Delete all buffered log entries immediately."
      >
        <Button
          variant="outline"
          size="sm"
          disabled={clearingLogs}
          onClick={handleClearLogs}
          className="border-neutral-700 bg-neutral-900 text-foreground/70 hover:bg-border hover:text-foreground text-xs"
        >
          {clearingLogs ? "Clearing…" : "Clear now"}
        </Button>
      </SettingRow>

      <Label className="sr-only">Settings</Label>
    </div>
  );
}
