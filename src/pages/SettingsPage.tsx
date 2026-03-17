import { useEffect } from "react";
import { useAppStore } from "../stores/app-store";
import { useSettingsStore } from "../stores/settings-store";
import { listInputDevices, updateSetting } from "../lib/tauri";

export default function SettingsPage() {
  const { settings, loading, load } = useSettingsStore();
  const { audioDevices, setAudioDevices } = useAppStore();

  useEffect(() => {
    load();
  }, [load]);

  useEffect(() => {
    listInputDevices()
      .then(setAudioDevices)
      .catch((e) => console.error("Failed to load audio devices:", e));
  }, [setAudioDevices]);

  const selectedDeviceId = settings["recording.device_id"] ?? "";
  const shortcut = settings["recording.shortcut"] ?? "CommandOrControl+Shift+Space";
  const selectedLanguage = settings["transcription.default_language"] ?? "de";

  function handleDeviceChange(deviceId: string) {
    updateSetting("recording.device_id", deviceId).then(load);
  }

  function handleLanguageChange(lang: string) {
    updateSetting("transcription.default_language", lang).then(load);
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
