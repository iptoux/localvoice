import { useEffect, useState } from "react";
import { useAppStore } from "../stores/app-store";
import { useSettingsStore } from "../stores/settings-store";
import {
  getAutostart,
  listInputDevices,
  setAutostart,
  updateSetting,
} from "../lib/tauri";

export default function SettingsPage() {
  const { settings, loading, load } = useSettingsStore();
  const { audioDevices, setAudioDevices } = useAppStore();
  const [autostart, setAutostartState] = useState(false);

  useEffect(() => {
    load();
  }, [load]);

  useEffect(() => {
    listInputDevices()
      .then(setAudioDevices)
      .catch((e) => console.error("Failed to load audio devices:", e));
  }, [setAudioDevices]);

  useEffect(() => {
    getAutostart().then(setAutostartState).catch(() => {});
  }, []);

  const handleAutostartToggle = async (enabled: boolean) => {
    try {
      await setAutostart(enabled);
      setAutostartState(enabled);
    } catch (e) {
      console.error("Failed to set autostart:", e);
    }
  };

  const selectedDeviceId = settings["recording.device_id"] ?? "";
  const shortcut = settings["recording.shortcut"] ?? "CommandOrControl+Shift+Space";
  const selectedLanguage = settings["transcription.default_language"] ?? "de";
  const outputMode = settings["output.mode"] ?? "clipboard";

  function handleDeviceChange(deviceId: string) {
    updateSetting("recording.device_id", deviceId).then(load);
  }

  function handleLanguageChange(lang: string) {
    updateSetting("transcription.default_language", lang).then(load);
  }

  function handleOutputModeChange(mode: string) {
    updateSetting("output.mode", mode).then(load);
  }

  if (loading) {
    return <div className="p-8 text-neutral-400 text-sm">Loading settings…</div>;
  }

  return (
    <div className="p-8 space-y-8">
      <h1 className="text-2xl font-semibold text-white">Settings</h1>

      {/* Recording section */}
      <section className="space-y-4">
        <h2 className="text-sm font-semibold text-neutral-400 uppercase tracking-wider">
          Recording
        </h2>

        {/* Microphone selector */}
        <div className="space-y-1">
          <label className="text-sm text-neutral-300">Microphone</label>
          <select
            value={selectedDeviceId}
            onChange={(e) => handleDeviceChange(e.target.value)}
            className="w-full max-w-sm bg-neutral-800 border border-neutral-700 text-white text-sm rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-neutral-500"
          >
            <option value="">System default</option>
            {audioDevices.map((device) => (
              <option key={device.id} value={device.id}>
                {device.name}
                {device.isDefault ? " (default)" : ""}
              </option>
            ))}
          </select>
          <p className="text-xs text-neutral-500">
            Audio is captured in mono at 16 kHz for local transcription.
          </p>
        </div>

        {/* Transcription language */}
        <div className="space-y-1">
          <label className="text-sm text-neutral-300">Transcription Language</label>
          <select
            value={selectedLanguage}
            onChange={(e) => handleLanguageChange(e.target.value)}
            className="w-full max-w-sm bg-neutral-800 border border-neutral-700 text-white text-sm rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-neutral-500"
          >
            <option value="auto">Auto-detect</option>
            <option value="de">German (de)</option>
            <option value="en">English (en)</option>
            <option value="fr">French (fr)</option>
            <option value="es">Spanish (es)</option>
            <option value="it">Italian (it)</option>
            <option value="pt">Portuguese (pt)</option>
            <option value="nl">Dutch (nl)</option>
            <option value="pl">Polish (pl)</option>
            <option value="ru">Russian (ru)</option>
            <option value="ja">Japanese (ja)</option>
            <option value="zh">Chinese (zh)</option>
          </select>
          <p className="text-xs text-neutral-500">
            Passed to whisper.cpp via <code className="text-neutral-400">-l</code>.
            Auto-detect is slower but language-agnostic.
          </p>
        </div>

        {/* Global shortcut display */}
        <div className="space-y-1">
          <label className="text-sm text-neutral-300">Global Shortcut</label>
          <div className="flex items-center gap-3">
            <ShortcutBadge shortcut={shortcut} />
          </div>
          <p className="text-xs text-neutral-500">
            Press this shortcut anywhere to start or stop a recording. To change
            it, edit the <code className="text-neutral-400">recording.shortcut</code>{" "}
            setting directly in the database (shortcut editor coming in a later
            milestone).
          </p>
        </div>
      </section>

      {/* Output section */}
      <section className="space-y-4">
        <h2 className="text-sm font-semibold text-neutral-400 uppercase tracking-wider">
          Output
        </h2>

        <div className="space-y-3">
          <label className="text-sm text-neutral-300">Output Mode</label>
          <div className="flex flex-col gap-2">
            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="radio"
                name="output-mode"
                value="clipboard"
                checked={outputMode === "clipboard"}
                onChange={() => handleOutputModeChange("clipboard")}
                className="mt-0.5 accent-neutral-400"
              />
              <span>
                <span className="text-sm text-neutral-200 block">Clipboard</span>
                <span className="text-xs text-neutral-500">
                  Transcription is copied to the clipboard. Paste it manually
                  wherever you need it.
                </span>
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="radio"
                name="output-mode"
                value="insert"
                checked={outputMode === "insert"}
                onChange={() => handleOutputModeChange("insert")}
                className="mt-0.5 accent-neutral-400"
              />
              <span>
                <span className="text-sm text-neutral-200 block">Auto-insert</span>
                <span className="text-xs text-neutral-500">
                  Transcription is copied to the clipboard and immediately pasted
                  into the focused application via Ctrl+V. Your previous clipboard
                  content is restored afterwards. Best-effort — some apps may not
                  support this.
                </span>
              </span>
            </label>
          </div>
        </div>
      </section>

      {/* System section */}
      <section className="space-y-4">
        <h2 className="text-sm font-semibold text-neutral-400 uppercase tracking-wider">
          System
        </h2>
        <Toggle
          label="Launch at login"
          description="Start LocalVoice automatically when you log in to Windows."
          checked={autostart}
          onChange={handleAutostartToggle}
        />
      </section>

      {/* Notifications section */}
      <section className="space-y-4">
        <h2 className="text-sm font-semibold text-neutral-400 uppercase tracking-wider">
          Notifications
        </h2>
        <Toggle
          label="Error notifications"
          description="Show a native OS notification when transcription fails."
          checked={settings["notifications.on_error"] === "true"}
          onChange={(v) =>
            updateSetting("notifications.on_error", v ? "true" : "false").then(load)
          }
        />
        <Toggle
          label="Success notifications"
          description="Show a notification after each successful transcription (includes word count and preview)."
          checked={settings["notifications.on_success"] === "true"}
          onChange={(v) =>
            updateSetting("notifications.on_success", v ? "true" : "false").then(load)
          }
        />
      </section>

      {/* All settings dump — useful during development */}
      <section className="space-y-2">
        <h2 className="text-sm font-semibold text-neutral-400 uppercase tracking-wider">
          All Settings
        </h2>
        <div className="space-y-1">
          {Object.entries(settings).map(([key, value]) => (
            <div key={key} className="flex gap-4 text-sm">
              <span className="text-neutral-400 w-64 shrink-0 font-mono text-xs">
                {key}
              </span>
              <span className="text-neutral-200 text-xs">{value}</span>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}

function Toggle({
  label,
  description,
  checked,
  onChange,
}: {
  label: string;
  description?: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <label className="flex items-start gap-4 cursor-pointer group">
      <div className="relative mt-0.5 shrink-0">
        <input
          type="checkbox"
          className="sr-only"
          checked={checked}
          onChange={(e) => onChange(e.target.checked)}
        />
        <div
          className={`w-10 h-5 rounded-full transition-colors ${
            checked ? "bg-blue-600" : "bg-neutral-600"
          }`}
        />
        <div
          className={`absolute top-0.5 left-0.5 w-4 h-4 rounded-full bg-white transition-transform ${
            checked ? "translate-x-5" : "translate-x-0"
          }`}
        />
      </div>
      <span>
        <span className="text-sm text-neutral-200 block">{label}</span>
        {description && (
          <span className="text-xs text-neutral-500">{description}</span>
        )}
      </span>
    </label>
  );
}

function ShortcutBadge({ shortcut }: { shortcut: string }) {
  const parts = shortcut.split("+");
  return (
    <div className="flex items-center gap-1">
      {parts.map((part, i) => (
        <span key={i} className="flex items-center gap-1">
          <kbd className="px-2 py-1 bg-neutral-700 border border-neutral-600 rounded text-xs text-white font-mono">
            {part}
          </kbd>
          {i < parts.length - 1 && (
            <span className="text-neutral-500 text-xs">+</span>
          )}
        </span>
      ))}
    </div>
  );
}
