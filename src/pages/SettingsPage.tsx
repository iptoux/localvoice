import { useEffect, useState } from "react";
import { useAppStore } from "../stores/app-store";
import { useSettingsStore } from "../stores/settings-store";
import {
  getAutostart,
  listInputDevices,
  setAutostart,
  updateSetting,
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
  LayoutPanelLeft,
  LogIn,
  EyeOff,
  BellOff,
  BellRing,
} from "lucide-react";

// ── Helpers ───────────────────────────────────────────────────────────────────

function SettingRow({
  label,
  description,
  icon: Icon,
  iconClass = "text-neutral-500",
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
          <p className="text-sm text-neutral-100">{label}</p>
          {description && (
            <p className="text-xs text-neutral-500 mt-0.5 leading-relaxed">{description}</p>
          )}
        </div>
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

function SectionHeader({ title }: { title: string }) {
  return (
    <h2 className="text-xs font-semibold text-neutral-500 uppercase tracking-widest pt-2 pb-1">
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
          <kbd className="px-2 py-1 bg-neutral-800 border border-neutral-700 rounded text-xs text-neutral-200 font-mono">
            {part === "CommandOrControl" ? "Ctrl" : part}
          </kbd>
          {i < parts.length - 1 && (
            <span className="text-neutral-600 text-xs">+</span>
          )}
        </span>
      ))}
    </div>
  );
}

// ── Main Component ────────────────────────────────────────────────────────────

export default function SettingsPage() {
  const { settings, loading, load } = useSettingsStore();
  const { audioDevices, setAudioDevices } = useAppStore();
  const [autostart, setAutostartState] = useState(false);

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

  const bool = (key: string, fallback = false) =>
    settings[key] === undefined ? fallback : settings[key] === "true";

  if (loading) {
    return <div className="p-8 text-neutral-500 text-sm">Loading…</div>;
  }

  const shortcut = settings["recording.shortcut"] || "CommandOrControl+Shift+Space";
  const outputMode = settings["output.mode"] || "clipboard";
  const language = settings["transcription.default_language"] || "de";
  const deviceId = settings["recording.device_id"] || "";
  const silenceMs = parseInt(settings["recording.silence_timeout_ms"] || "1500");

  return (
    <div className="p-8 max-w-2xl space-y-1">
      <h1 className="text-xl font-semibold text-white mb-6">Settings</h1>

      {/* ── Recording ── */}
      <SectionHeader title="Recording" />
      <Separator className="bg-neutral-800 mb-1" />

      <SettingRow icon={Mic} iconClass="text-blue-400" label="Microphone" description="Audio device used for recording.">
        <Select value={deviceId} onValueChange={setVal("recording.device_id")}>
          <SelectTrigger className="w-52 bg-neutral-900 border-neutral-700 text-sm">
            <SelectValue placeholder="System default" />
          </SelectTrigger>
          <SelectContent className="bg-neutral-900 border-neutral-700">
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
        <ShortcutBadge shortcut={shortcut} />
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
          <SelectTrigger className="w-32 bg-neutral-900 border-neutral-700 text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-neutral-900 border-neutral-700">
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
      <Separator className="bg-neutral-800 mb-1" />

      <SettingRow icon={Languages} iconClass="text-cyan-400" label="Language" description="Language passed to the Whisper model.">
        <Select value={language} onValueChange={setVal("transcription.default_language")}>
          <SelectTrigger className="w-44 bg-neutral-900 border-neutral-700 text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-neutral-900 border-neutral-700">
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

      {/* ── Output ── */}
      <div className="mt-10">
        <SectionHeader title="Output" />
      </div>
      <Separator className="bg-neutral-800 mb-1" />

      <SettingRow
        icon={ClipboardCopy}
        iconClass="text-emerald-400"
        label="Output mode"
        description="What happens with the transcription after recording."
      >
        <Select value={outputMode} onValueChange={setVal("output.mode")}>
          <SelectTrigger className="w-44 bg-neutral-900 border-neutral-700 text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-neutral-900 border-neutral-700">
            <SelectItem value="clipboard">Clipboard only</SelectItem>
            <SelectItem value="insert">Auto-insert</SelectItem>
          </SelectContent>
        </Select>
      </SettingRow>

      {outputMode === "insert" && (
        <div className="py-2 px-3 rounded-md bg-neutral-900 border border-neutral-800 text-xs text-neutral-400">
          Auto-insert pastes via Ctrl+V into the focused app. Your previous clipboard is restored afterwards.
        </div>
      )}

      {/* ── Appearance ── */}
      <div className="mt-10">
        <SectionHeader title="Appearance" />
      </div>
      <Separator className="bg-neutral-800 mb-1" />

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
          <SelectTrigger className="w-44 bg-neutral-900 border-neutral-700 text-sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="bg-neutral-900 border-neutral-700">
            <SelectItem value="pill">Pill (compact)</SelectItem>
            <SelectItem value="main">Main window</SelectItem>
          </SelectContent>
        </Select>
      </SettingRow>

      {/* ── System ── */}
      <div className="mt-10">
        <SectionHeader title="System" />
      </div>
      <Separator className="bg-neutral-800 mb-1" />

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
      <Separator className="bg-neutral-800 mb-1" />

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

      <Label className="sr-only">Settings</Label>
    </div>
  );
}
