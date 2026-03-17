import { useEffect } from "react";
import { useSettingsStore } from "../stores/settings-store";

export default function SettingsPage() {
  const { settings, loading, load } = useSettingsStore();

  useEffect(() => {
    load();
  }, [load]);

  if (loading) {
    return <div className="p-8 text-neutral-400 text-sm">Loading settings…</div>;
  }

  return (
    <div className="p-8">
      <h1 className="text-2xl font-semibold text-white mb-4">Settings</h1>
      <div className="space-y-2">
        {Object.entries(settings).map(([key, value]) => (
          <div key={key} className="flex gap-4 text-sm">
            <span className="text-neutral-400 w-64 shrink-0">{key}</span>
            <span className="text-neutral-200">{value}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
