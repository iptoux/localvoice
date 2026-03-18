import type { Meta, StoryObj } from "@storybook/react";
import { useMemo, useState } from "react";
import { Search, Copy, Check, Trash2, Upload, ChevronLeft, ChevronRight, X, RefreshCw, Calendar } from "lucide-react";
import type { Session, SessionWithSegments } from "../types";
import { mockSessions, mockSessionDetail } from "../mocks/tauri";

const meta: Meta = {
  title: "History",
  tags: ["autodocs"],
  parameters: {
    layout: "padded",
    backgrounds: {
      default: "dark",
    },
  },
};

export default meta;

function formatDate(iso: string): string {
  try {
    return new Date(iso).toLocaleString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  } catch {
    return iso;
  }
}

function LanguageBadge({ lang }: { lang: string }) {
  return (
    <span className="text-xs bg-blue-600 text-white px-1.5 py-0.5 rounded font-mono uppercase">
      {lang}
    </span>
  );
}

function OutputBadge({ mode, ok }: { mode: string; ok: boolean }) {
  return (
    <span
      className={`text-xs px-1.5 py-0.5 rounded ${
        ok ? "bg-green-600 text-white" : "bg-red-600 text-white"
      }`}
    >
      {mode === "insert" ? "inserted" : "copied"}
    </span>
  );
}

function SessionRow({
  session,
  active,
  onClick,
}: {
  session: Session;
  active: boolean;
  onClick: () => void;
}) {
  const [copied, setCopied] = useState(false);
  const preview = useMemo(
    () =>
      session.cleanedText.length > 80
        ? session.cleanedText.slice(0, 78) + "…"
        : session.cleanedText,
    [session.cleanedText]
  );

  const handleCopy = (e: React.MouseEvent) => {
    e.stopPropagation();
    navigator.clipboard.writeText(session.cleanedText).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    });
  };

  return (
    <button
      onClick={onClick}
      className={`
        w-full text-left px-4 py-3 rounded-lg border transition-colors
        ${
          active
            ? "border-primary/60 bg-muted ring-1 ring-primary/30"
            : "border-border bg-card hover:border-border hover:bg-muted"
        }
      `}
    >
      <div className="flex items-center gap-2 mb-1">
        <span className="text-xs text-muted-foreground tabular-nums">{formatDate(session.startedAt)}</span>
        <LanguageBadge lang={session.language} />
        <span className="text-xs text-muted-foreground">{session.wordCount} words</span>
        <OutputBadge mode={session.outputMode} ok={session.insertedSuccessfully} />
        <span className="flex-1" />
        <button
          onClick={handleCopy}
          title="Copy to clipboard"
          className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors px-1.5 py-0.5 rounded hover:bg-muted"
        >
          {copied ? <Check size={12} className="text-green-400" /> : <Copy size={12} />}
          {copied ? "Copied" : "Copy"}
        </button>
      </div>
      <p className="text-sm text-foreground/70 leading-snug">{preview}</p>
    </button>
  );
}

function SearchFilters() {
  return (
    <div className="flex flex-col gap-2">
      <div className="relative">
        <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <input
          type="search"
          placeholder="Search transcriptions…"
          className="w-full bg-muted border border-border text-foreground text-sm rounded-md pl-9 pr-3 py-2 placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-neutral-500"
        />
      </div>
      <div className="flex flex-wrap gap-2">
        <select className="bg-muted border border-border text-foreground/70 text-xs rounded px-2 py-1.5 focus:outline-none">
          <option value="">All languages</option>
          <option value="de">German (de)</option>
          <option value="en">English (en)</option>
        </select>
        <label className="flex items-center gap-1 text-xs text-muted-foreground">
          <Calendar size={12} />
          From
          <input
            type="date"
            className="bg-muted border border-border text-foreground/70 rounded px-2 py-1 text-xs focus:outline-none"
          />
        </label>
        <label className="flex items-center gap-1 text-xs text-muted-foreground">
          <Calendar size={12} />
          To
          <input
            type="date"
            className="bg-muted border border-border text-foreground/70 rounded px-2 py-1 text-xs focus:outline-none"
          />
        </label>
      </div>
    </div>
  );
}

function Pagination() {
  return (
    <div className="flex items-center justify-between text-xs text-muted-foreground">
      <span>1–3</span>
      <div className="flex gap-2">
        <button className="flex items-center gap-1 px-3 py-1 rounded bg-muted hover:bg-accent disabled:opacity-40 disabled:cursor-not-allowed transition-colors">
          <ChevronLeft size={12} /> Previous
        </button>
        <button className="flex items-center gap-1 px-3 py-1 rounded bg-muted hover:bg-accent disabled:opacity-40 disabled:cursor-not-allowed transition-colors">
          Next <ChevronRight size={12} />
        </button>
      </div>
    </div>
  );
}

function SessionDrawer({
  detail,
  onClose,
}: {
  detail: SessionWithSegments | null;
  onClose: () => void;
}) {
  const [tab, setTab] = useState<"cleaned" | "raw" | "diff">("cleaned");

  if (!detail) return null;

  const { session } = detail;
  const durationSec = Math.round(session.durationMs / 1000);

  return (
    <aside className="w-96 shrink-0 border-l border-border flex flex-col overflow-hidden">
      <div className="flex items-center justify-between px-5 py-4 border-b border-border">
        <h2 className="text-sm font-semibold text-foreground">Session Details</h2>
        <button onClick={onClose} className="text-muted-foreground hover:text-foreground">
          <X size={16} />
        </button>
      </div>

      <div className="px-5 py-3 border-b border-border space-y-1">
        <p className="text-xs text-muted-foreground">{formatDate(session.startedAt)}</p>
        <div className="flex flex-wrap gap-2">
          <LanguageBadge lang={session.language} />
          <span className="text-xs bg-muted text-muted-foreground px-1.5 py-0.5 rounded">
            {session.modelId}
          </span>
          <span className="text-xs text-muted-foreground">{session.wordCount} words</span>
          {durationSec > 0 && (
            <span className="text-xs text-muted-foreground">{durationSec}s</span>
          )}
          {session.estimatedWpm && (
            <span className="text-xs text-muted-foreground">
              ~{Math.round(session.estimatedWpm)} wpm
            </span>
          )}
        </div>
      </div>

      <div className="flex border-b border-border">
        {(["cleaned", "raw", "diff"] as const).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`flex-1 py-2 text-xs font-medium transition-colors ${
              tab === t
                ? "text-foreground border-b-2 border-foreground"
                : "text-muted-foreground hover:text-foreground/70"
            }`}
          >
            {t === "cleaned" ? "Cleaned" : t === "raw" ? "Raw" : "Diff"}
          </button>
        ))}
      </div>

      <div className="flex-1 overflow-y-auto px-5 py-4 min-h-0">
        <p className="text-sm text-foreground/80 leading-relaxed whitespace-pre-wrap">
          {tab === "cleaned"
            ? session.cleanedText
            : tab === "raw"
              ? session.rawText
              : "Diff view would show changes here"}
        </p>
      </div>

      <div className="flex items-center gap-2 px-5 py-3 border-t border-border">
        <button className="flex items-center gap-1.5 flex-1 text-xs bg-muted hover:bg-accent text-foreground/70 hover:text-foreground rounded px-3 py-1.5 transition-colors">
          <Copy size={12} />
          Copy
        </button>
        <button className="flex items-center gap-1.5 text-xs rounded px-3 py-1.5 transition-colors bg-muted hover:bg-accent text-foreground/70 hover:text-foreground">
          <RefreshCw size={12} />
          Reprocess
        </button>
        <button className="flex items-center gap-1.5 text-xs bg-muted hover:bg-accent text-foreground/70 hover:text-foreground rounded px-3 py-1.5 transition-colors">
          <Upload size={12} />
          Export
        </button>
        <button className="flex items-center gap-1.5 text-xs bg-muted hover:bg-rose-900/30 text-rose-400 hover:text-rose-300 rounded px-3 py-1.5 transition-colors">
          <Trash2 size={12} />
          Delete
        </button>
      </div>
    </aside>
  );
}

export const Default: StoryObj = {
  render: () => {
    const [selected, setSelected] = useState<SessionWithSegments | null>(null);

    return (
      <div className="flex h-96 min-h-0 bg-zinc-950 rounded-lg overflow-hidden">
        <div className="flex flex-col flex-1 min-w-0 p-4 gap-3">
          <div className="flex items-center justify-between">
            <h1 className="text-xl font-semibold text-foreground shrink-0">History</h1>
            <button className="flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground border border-border hover:border-neutral-500 px-3 py-1.5 rounded transition-colors">
              <Upload size={12} />
              Export page
            </button>
          </div>

          <SearchFilters />

          <div className="flex-1 min-h-0 overflow-y-auto space-y-2">
            {mockSessions.map((session) => (
              <SessionRow
                key={session.id}
                session={session}
                active={selected?.session.id === session.id}
                onClick={() => setSelected(selected?.session.id === session.id ? null : mockSessionDetail)}
              />
            ))}
          </div>

          <Pagination />
        </div>

        {selected && (
          <SessionDrawer detail={selected} onClose={() => setSelected(null)} />
        )}
      </div>
    );
  },
};

export const Empty: StoryObj = {
  render: () => (
    <div className="flex flex-col h-96 min-h-0 bg-zinc-950 rounded-lg overflow-hidden">
      <div className="flex flex-col flex-1 min-w-0 p-4 gap-3">
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-semibold text-foreground shrink-0">History</h1>
        </div>

        <SearchFilters />

        <div className="flex-1 min-h-0 overflow-y-auto space-y-2">
          <p className="text-muted-foreground text-sm py-8 text-center">
            No sessions found.
          </p>
        </div>
      </div>
    </div>
  ),
};

export const Loading: StoryObj = {
  render: () => (
    <div className="flex flex-col h-96 min-h-0 bg-zinc-950 rounded-lg overflow-hidden">
      <div className="flex flex-col flex-1 min-w-0 p-4 gap-3">
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-semibold text-foreground shrink-0">History</h1>
        </div>

        <SearchFilters />

        <div className="flex-1 min-h-0 overflow-y-auto space-y-2">
          <p className="text-muted-foreground text-sm py-8 text-center">
            Loading…
          </p>
        </div>
      </div>
    </div>
  ),
};
