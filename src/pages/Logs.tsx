import { useEffect, useState } from "react";
import { RefreshCw, Download, Trash2, AlertTriangle, Info, AlertCircle, List } from "lucide-react";
import { listLogs, exportLogs, clearLogs } from "../lib/tauri";
import type { LogEntry } from "../types";
import { VirtualList } from "../components/VirtualList";

type LevelFilter = "all" | "info" | "warn" | "error";

const LEVEL_COLORS: Record<string, string> = {
  info: "text-blue-700 dark:text-blue-300 bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800/50",
  warn: "text-yellow-700 dark:text-yellow-400 bg-yellow-50 dark:bg-yellow-900/30 border-yellow-300 dark:border-yellow-700/50",
  error: "text-red-700 dark:text-red-400 bg-red-50 dark:bg-red-900/30 border-red-300 dark:border-red-700/50",
};

const LEVEL_ICONS: Record<string, React.ReactNode> = {
  info: <Info size={12} />,
  warn: <AlertTriangle size={12} />,
  error: <AlertCircle size={12} />,
};

export default function Logs() {
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [filter, setFilter] = useState<LevelFilter>("all");
  const [loading, setLoading] = useState(false);

  const load = async (f: LevelFilter = filter) => {
    setLoading(true);
    try {
      const data = await listLogs(f === "all" ? undefined : f);
      setEntries(data);
    } catch (e) {
      console.error("Failed to load logs:", e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, [filter]);

  const handleExport = async () => {
    try {
      await exportLogs();
    } catch (e) {
      console.error("Export failed:", e);
    }
  };

  const handleClear = async () => {
    if (!confirm("Clear all log entries?")) return;
    try {
      await clearLogs();
      setEntries([]);
    } catch (e) {
      console.error("Clear failed:", e);
    }
  };

  return (
    <div className="flex flex-col h-full p-8">
      <h1 className="text-2xl font-semibold text-foreground mb-1">Logs</h1>
      <p className="text-muted-foreground text-sm mb-6">
        Application logs captured during this session. Filter by level to focus on what matters.
      </p>

      {/* Controls */}
      <div className="flex items-center gap-3 mb-5 flex-wrap">
        {(["all", "info", "warn", "error"] as LevelFilter[]).map((level) => (
          <button
            key={level}
            onClick={() => setFilter(level)}
            className={`flex items-center gap-1.5 px-3 py-1.5 text-xs rounded-full border transition-colors capitalize ${
              filter === level
                ? "bg-accent border-neutral-500 text-foreground"
                : "border-border text-muted-foreground hover:text-foreground"
            }`}
          >
            {level === "all" ? <List size={11} /> : LEVEL_ICONS[level]}
            {level === "all" ? "All" : level}
          </button>
        ))}
        <div className="flex-1" />
        <button
          onClick={() => load()}
          className="flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground transition-colors"
        >
          <RefreshCw size={13} /> Refresh
        </button>
        <button
          onClick={handleExport}
          className="flex items-center gap-1.5 text-xs px-3 py-1.5 bg-muted hover:bg-accent border border-border text-foreground rounded transition-colors"
        >
          <Download size={13} /> Export JSON
        </button>
        <button
          onClick={handleClear}
          className="flex items-center gap-1.5 text-xs px-3 py-1.5 text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 border border-red-300 dark:border-red-800 hover:border-red-400 dark:hover:border-red-600 rounded transition-colors"
        >
          <Trash2 size={13} /> Clear
        </button>
      </div>

      {/* Count */}
      <p className="text-xs text-muted-foreground mb-3">
        {entries.length} {entries.length === 1 ? "entry" : "entries"}
      </p>

      {/* List — virtualized */}
      {loading ? (
        <p className="text-muted-foreground text-sm">Loading…</p>
      ) : entries.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground text-sm">
          No log entries for this filter. Logs will appear here as they occur.
        </div>
      ) : (
        <VirtualList
          items={entries}
          estimateSize={48}
          className="flex-1 min-h-0"
          renderItem={(entry) => (
            <div
              className={`px-4 py-2.5 rounded-lg border text-sm ${
                LEVEL_COLORS[entry.level] ?? "text-foreground/70 bg-muted border-border"
              }`}
            >
              <div className="flex items-start gap-3 flex-wrap">
                <span className="flex items-center gap-1 text-xs font-mono uppercase shrink-0 font-semibold">
                  {LEVEL_ICONS[entry.level]}
                  {entry.level}
                </span>
                <span className="text-xs text-muted-foreground shrink-0">
                  {new Date(entry.createdAt).toLocaleTimeString()}
                </span>
                {entry.area && (
                  <span className="text-xs text-muted-foreground shrink-0 font-mono">
                    {entry.area}
                  </span>
                )}
                <span className="flex-1 break-words">{entry.message}</span>
              </div>
            </div>
          )}
        />
      )}
    </div>
  );
}
