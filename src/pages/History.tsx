import { useCallback, useEffect, useRef, useState } from "react";
import { Search, Copy, Check, Trash2, Upload, ChevronLeft, ChevronRight, X, RefreshCw, Calendar } from "lucide-react";
import type { Session, SessionFilter, SessionWithSegments } from "../types";
import {
  deleteSession,
  exportSessions,
  getSessionDetail,
  listSessions,
  reprocessSession,
  listAvailableModels,
} from "../lib/tauri";
import type { ModelInfo } from "../types";
import { VirtualList } from "../components/VirtualList";
import { useWordDiff } from "../hooks/use-text-processor";

const PAGE_SIZE = 50;

// ── Main page ─────────────────────────────────────────────────────────────────

export default function History() {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Filters (TASK-075, 076)
  const [query, setQuery] = useState("");
  const [language, setLanguage] = useState("");
  const [dateFrom, setDateFrom] = useState("");
  const [dateTo, setDateTo] = useState("");
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Detail drawer (TASK-077)
  const [selected, setSelected] = useState<SessionWithSegments | null>(null);
  const [drawerLoading, setDrawerLoading] = useState(false);

  const load = useCallback(
    async (filter: SessionFilter, pageIndex: number) => {
      setLoading(true);
      setError(null);
      try {
        const data = await listSessions({
          ...filter,
          limit: PAGE_SIZE,
          offset: pageIndex * PAGE_SIZE,
        });
        setSessions(data);
        // Approximate total: if we got a full page there may be more.
        setTotal(
          data.length === PAGE_SIZE ? (pageIndex + 1) * PAGE_SIZE + 1 : pageIndex * PAGE_SIZE + data.length
        );
      } catch (e) {
        setError(String(e));
      } finally {
        setLoading(false);
      }
    },
    []
  );

  // Debounced re-load when filters change (TASK-075).
  useEffect(() => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => {
      setPage(0);
      load({ query, language, dateFrom, dateTo }, 0);
    }, 300);
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, [query, language, dateFrom, dateTo, load]);

  // Re-load when page changes.
  useEffect(() => {
    load({ query, language, dateFrom, dateTo }, page);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [page]);

  async function openDetail(session: Session) {
    setDrawerLoading(true);
    setSelected(null);
    try {
      const detail = await getSessionDetail(session.id);
      setSelected(detail);
    } catch {
      setSelected({ session, segments: [] });
    } finally {
      setDrawerLoading(false);
    }
  }

  async function handleDelete(sessionId: string) {
    await deleteSession(sessionId);
    setSelected(null);
    load({ query, language, dateFrom, dateTo }, page);
  }

  async function handleReprocess(detail: SessionWithSegments) {
    setSelected(detail);
    load({ query, language, dateFrom, dateTo }, page);
  }

  return (
    <div className="flex h-full min-h-0">
      {/* ── List panel ─────────────────────────────────────────────────────── */}
      <div className="flex flex-col flex-1 min-w-0 p-8 gap-4">
        <div className="flex items-center justify-between gap-4">
          <h1 className="text-2xl font-semibold text-foreground shrink-0">History</h1>
          <button
            onClick={() =>
              exportSessions(
                sessions.map((s) => s.id),
                "txt"
              ).catch(console.error)
            }
            className="flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground border border-border hover:border-neutral-500 px-3 py-1.5 rounded transition-colors"
          >
            <Upload size={12} />
            Export page
          </button>
        </div>

          <div className="flex flex-col gap-2">
          <div className="relative">
            <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
            <input
              type="search"
              placeholder="Search transcriptions…"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              className="w-full bg-muted border border-border text-foreground text-sm rounded-md pl-9 pr-3 py-2 placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-neutral-500"
            />
          </div>
          <div className="flex flex-wrap gap-2">
            <select
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="bg-muted border border-border text-foreground/70 text-xs rounded px-2 py-1.5 focus:outline-none"
            >
              <option value="">All languages</option>
              <option value="de">German (de)</option>
              <option value="en">English (en)</option>
              <option value="fr">French (fr)</option>
              <option value="es">Spanish (es)</option>
              <option value="auto">Auto-detect</option>
            </select>
            <label className="flex items-center gap-1 text-xs text-muted-foreground">
              <Calendar size={12} />
              From
              <input
                type="date"
                value={dateFrom}
                onChange={(e) => setDateFrom(e.target.value)}
                className="bg-muted border border-border text-foreground/70 rounded px-2 py-1 text-xs focus:outline-none"
              />
            </label>
            <label className="flex items-center gap-1 text-xs text-muted-foreground">
              <Calendar size={12} />
              To
              <input
                type="date"
                value={dateTo}
                onChange={(e) => setDateTo(e.target.value)}
                className="bg-muted border border-border text-foreground/70 rounded px-2 py-1 text-xs focus:outline-none"
              />
            </label>
            {(query || language || dateFrom || dateTo) && (
              <button
                onClick={() => {
                  setQuery("");
                  setLanguage("");
                  setDateFrom("");
                  setDateTo("");
                }}
                className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors px-2"
              >
                <X size={12} />
                Clear filters
              </button>
            )}
          </div>
        </div>

        {/* Session list (TASK-074) — virtualized */}
        {loading && (
          <p className="text-muted-foreground text-sm py-8 text-center">Loading…</p>
        )}
        {error && (
          <p className="text-rose-400 text-sm py-8 text-center">{error}</p>
        )}
        {!loading && !error && sessions.length === 0 && (
          <p className="text-muted-foreground text-sm py-8 text-center">
            No sessions found.
          </p>
        )}
        {!loading && !error && sessions.length > 0 && (
          <VirtualList
            items={sessions}
            estimateSize={72}
            className="flex-1 min-h-0"
            renderItem={(session) => (
              <SessionRow
                session={session}
                active={selected?.session.id === session.id}
                onClick={() => openDetail(session)}
              />
            )}
          />
        )}

        {/* Pagination (TASK-079) */}
        <Pagination
          page={page}
          pageSize={PAGE_SIZE}
          total={total}
          sessionCount={sessions.length}
          onPrev={() => setPage((p) => Math.max(0, p - 1))}
          onNext={() => setPage((p) => p + 1)}
        />
      </div>

      {/* ── Detail drawer (TASK-077, 078) ──────────────────────────────────── */}
      {(selected || drawerLoading) && (
        <SessionDrawer
          detail={selected}
          loading={drawerLoading}
          onClose={() => setSelected(null)}
          onDelete={handleDelete}
          onReprocess={handleReprocess}
        />
      )}
    </div>
  );
}

// ── Session row ───────────────────────────────────────────────────────────────

function SessionRow({
  session,
  active,
  onClick,
}: {
  session: Session;
  active: boolean;
  onClick: () => void;
}) {
  const date = formatDate(session.startedAt);
  const preview =
    session.cleanedText.length > 80
      ? session.cleanedText.slice(0, 78) + "…"
      : session.cleanedText;

  const [copied, setCopied] = useState(false);

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
            ? "border-neutral-500 bg-accent"
            : "border-border bg-card hover:border-border hover:bg-accent"
        }
      `}
    >
      <div className="flex items-center gap-2 mb-1">
        <span className="text-xs text-muted-foreground tabular-nums">{date}</span>
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

// ── Detail drawer ─────────────────────────────────────────────────────────────

function SessionDrawer({
  detail,
  loading,
  onClose,
  onDelete,
  onReprocess,
}: {
  detail: SessionWithSegments | null;
  loading: boolean;
  onClose: () => void;
  onDelete: (id: string) => void;
  onReprocess: (detail: SessionWithSegments) => void;
}) {
  const [tab, setTab] = useState<"cleaned" | "raw" | "original" | "diff">("cleaned");
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [showReprocess, setShowReprocess] = useState(false);
  const [reprocessing, setReprocessing] = useState(false);
  const [reprocessLang, setReprocessLang] = useState("");
  const [reprocessModel, setReprocessModel] = useState("");
  const [models, setModels] = useState<ModelInfo[]>([]);

  // Reset tabs & confirm state when session changes.
  useEffect(() => {
    setTab("cleaned");
    setConfirmDelete(false);
    setShowReprocess(false);
    setReprocessing(false);
  }, [detail?.session.id]);

  // Load models when reprocess dialog opens.
  useEffect(() => {
    if (showReprocess) {
      listAvailableModels().then(setModels).catch(console.error);
      setReprocessLang(detail?.session.language ?? "");
      setReprocessModel("");
    }
  }, [showReprocess, detail?.session.language]);

  async function handleReprocess() {
    if (!detail) return;
    setReprocessing(true);
    try {
      const updated = await reprocessSession(
        detail.session.id,
        reprocessLang || undefined,
        reprocessModel || undefined
      );
      setShowReprocess(false);
      onReprocess(updated);
    } catch (e) {
      console.error("Reprocess failed:", e);
    } finally {
      setReprocessing(false);
    }
  }

  if (loading) {
    return (
      <aside className="w-96 shrink-0 border-l border-border flex items-center justify-center">
        <p className="text-muted-foreground text-sm">Loading…</p>
      </aside>
    );
  }

  if (!detail) return null;

  const { session, segments } = detail;
  const durationSec = Math.round(session.durationMs / 1000);

  function copyText(text: string) {
    navigator.clipboard.writeText(text).catch(console.error);
  }

  function handleExport() {
    exportSessions([session.id], "txt").catch(console.error);
  }

  function handleDelete() {
    if (!confirmDelete) {
      setConfirmDelete(true);
      return;
    }
    onDelete(session.id);
  }

  return (
    <aside className="w-96 shrink-0 border-l border-border flex flex-col overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-5 py-4 border-b border-border">
        <h2 className="text-sm font-semibold text-foreground">Session Details</h2>
        <button
          onClick={onClose}
          className="text-muted-foreground hover:text-foreground"
          aria-label="Close"
        >
          <X size={16} />
        </button>
      </div>

      {/* Meta */}
      <div className="px-5 py-3 border-b border-border space-y-1">
        <p className="text-xs text-muted-foreground">{formatDate(session.startedAt)}</p>
        <div className="flex flex-wrap gap-2">
          <LanguageBadge lang={session.language} />
          {session.modelId && (
            <span className="text-xs bg-muted text-muted-foreground px-1.5 py-0.5 rounded">
              {session.modelId}
            </span>
          )}
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

      {/* Text tabs */}
      <div className="flex border-b border-border">
        {(
          session.originalRawText
            ? (["cleaned", "raw", "original", "diff"] as const)
            : (["cleaned", "raw", "diff"] as const)
        ).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`flex-1 py-2 text-xs font-medium transition-colors ${
              tab === t
                ? "text-foreground border-b-2 border-foreground"
                : "text-muted-foreground hover:text-foreground/70"
            }`}
          >
            {t === "cleaned" ? "Cleaned" : t === "raw" ? "Raw" : t === "original" ? "Original" : "Diff"}
          </button>
        ))}
      </div>

      {/* Reprocessed badge */}
      {session.reprocessedCount > 0 && (
        <div className="px-5 pt-2">
          <span className="text-xs bg-blue-900/40 text-blue-400 px-1.5 py-0.5 rounded">
            Reprocessed {session.reprocessedCount}x
          </span>
        </div>
      )}

      {/* Text content */}
      <div className="flex-1 overflow-y-auto px-5 py-4 min-h-0">
        {tab === "diff" ? (
          <WordDiff rawText={session.rawText} cleanedText={session.cleanedText} />
        ) : (
          <p className="text-sm text-foreground/80 leading-relaxed whitespace-pre-wrap">
            {tab === "cleaned"
              ? session.cleanedText
              : tab === "raw"
                ? session.rawText
                : session.originalRawText ?? session.rawText}
          </p>
        )}

        {/* Confidence-colored segments (TASK-219) — virtualized for large lists */}
        {tab === "cleaned" && segments.length > 0 && (
          <details className="mt-4" open>
            <summary className="text-xs text-muted-foreground cursor-pointer hover:text-foreground/70 select-none">
              {segments.length} segments
            </summary>
            <VirtualList
              items={segments}
              estimateSize={28}
              overscan={10}
              gap={6}
              className="mt-2 max-h-64"
              renderItem={(seg) => (
                <div
                  className="text-xs flex items-start gap-2 group"
                  title={
                    seg.confidence !== undefined
                      ? `Confidence: ${Math.round(seg.confidence * 100)}%`
                      : "No confidence data"
                  }
                >
                  <span className="tabular-nums text-muted-foreground/60 shrink-0 w-10">
                    {msToTime(seg.startMs)}
                  </span>
                  <ConfidenceDot confidence={seg.confidence} />
                  <span className="text-foreground/70">{seg.text}</span>
                  {seg.confidence !== undefined && (
                    <span className="text-muted-foreground/50 shrink-0 tabular-nums ml-auto">
                      {Math.round(seg.confidence * 100)}%
                    </span>
                  )}
                </div>
              )}
            />
          </details>
        )}
      </div>

      {/* Reprocess panel */}
      {showReprocess && (
        <div className="px-5 py-3 border-t border-border space-y-2">
          <p className="text-xs font-medium text-foreground">Reprocess Session</p>
          <div className="flex gap-2">
            <select
              value={reprocessLang}
              onChange={(e) => setReprocessLang(e.target.value)}
              className="flex-1 bg-muted border border-border text-foreground/70 text-xs rounded px-2 py-1.5"
            >
              <option value="">Same language</option>
              <option value="de">German</option>
              <option value="en">English</option>
              <option value="fr">French</option>
              <option value="es">Spanish</option>
              <option value="auto">Auto-detect</option>
            </select>
            <select
              value={reprocessModel}
              onChange={(e) => setReprocessModel(e.target.value)}
              className="flex-1 bg-muted border border-border text-foreground/70 text-xs rounded px-2 py-1.5"
            >
              <option value="">Default model</option>
              {models
                .filter((m) => m.installed)
                .map((m) => (
                  <option key={m.key} value={m.key}>
                    {m.displayName}
                  </option>
                ))}
            </select>
          </div>
          {session.originalRawText && (
            <details className="text-xs">
              <summary className="text-muted-foreground cursor-pointer hover:text-foreground/70 select-none">
                Original raw text
              </summary>
              <p className="mt-1 text-foreground/60 whitespace-pre-wrap leading-relaxed">
                {session.originalRawText}
              </p>
            </details>
          )}
          <div className="flex gap-2">
            <button
              onClick={handleReprocess}
              disabled={reprocessing}
              className="flex-1 text-xs bg-blue-600 hover:bg-blue-500 text-white rounded px-3 py-1.5 transition-colors disabled:opacity-50"
            >
              {reprocessing ? "Reprocessing…" : "Reprocess"}
            </button>
            <button
              onClick={() => setShowReprocess(false)}
              className="text-xs bg-muted hover:bg-accent text-foreground/70 rounded px-3 py-1.5 transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      <div className="flex items-center gap-2 px-5 py-3 border-t border-border">
        <button
          onClick={() =>
            copyText(tab === "cleaned" ? session.cleanedText : session.rawText)
          }
          className="flex items-center gap-1.5 flex-1 text-xs bg-muted hover:bg-accent text-foreground/70 hover:text-foreground rounded px-3 py-1.5 transition-colors"
        >
          <Copy size={12} />
          Copy
        </button>
        {session.audioPath && (
          <button
            onClick={() => setShowReprocess(!showReprocess)}
            className={`flex items-center gap-1.5 text-xs rounded px-3 py-1.5 transition-colors ${
              showReprocess
                ? "bg-blue-700 text-white"
                : "bg-muted hover:bg-accent text-foreground/70 hover:text-foreground"
            }`}
          >
            <RefreshCw size={12} />
            Reprocess
          </button>
        )}
        <button
          onClick={handleExport}
          className="flex items-center gap-1.5 text-xs bg-muted hover:bg-accent text-foreground/70 hover:text-foreground rounded px-3 py-1.5 transition-colors"
        >
          <Upload size={12} />
          Export
        </button>
        <button
          onClick={handleDelete}
          className={`flex items-center gap-1.5 text-xs rounded px-3 py-1.5 transition-colors ${
            confirmDelete
              ? "bg-rose-700 hover:bg-rose-600 text-white"
              : "bg-muted hover:bg-accent text-rose-400 hover:text-rose-300"
          }`}
        >
          <Trash2 size={12} />
          {confirmDelete ? "Confirm delete" : "Delete"}
        </button>
      </div>
    </aside>
  );
}

// ── Pagination ────────────────────────────────────────────────────────────────

function Pagination({
  page,
  pageSize,
  total,
  sessionCount,
  onPrev,
  onNext,
}: {
  page: number;
  pageSize: number;
  total: number;
  sessionCount: number;
  onPrev: () => void;
  onNext: () => void;
}) {
  const from = page * pageSize + 1;
  const to = page * pageSize + sessionCount;
  const hasNext = sessionCount === pageSize;

  if (total === 0) return null;

  return (
    <div className="flex items-center justify-between text-xs text-muted-foreground">
      <span>
        {from}–{to}
      </span>
      <div className="flex gap-2">
        <button
          onClick={onPrev}
          disabled={page === 0}
          className="flex items-center gap-1 px-3 py-1 rounded bg-muted hover:bg-accent disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
        >
          <ChevronLeft size={12} /> Previous
        </button>
        <button
          onClick={onNext}
          disabled={!hasNext}
          className="flex items-center gap-1 px-3 py-1 rounded bg-muted hover:bg-accent disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
        >
          Next <ChevronRight size={12} />
        </button>
      </div>
    </div>
  );
}

// ── Small reusable bits ───────────────────────────────────────────────────────

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
        ok
          ? "bg-green-600 text-white"
          : "bg-red-600 text-white"
      }`}
    >
      {mode === "insert" ? "inserted" : "copied"}
    </span>
  );
}

// ── Word diff (TASK-220) — offloaded to Web Worker ────────────────────────

function WordDiff({ rawText, cleanedText }: { rawText: string; cleanedText: string }) {
  const tokens = useWordDiff(rawText, cleanedText);

  if (!tokens) {
    return (
      <div className="text-sm leading-relaxed">
        <p className="text-xs text-muted-foreground mb-2">
          Raw → Cleaned comparison
        </p>
        <p className="text-xs text-muted-foreground italic">Computing diff…</p>
      </div>
    );
  }

  return (
    <div className="text-sm leading-relaxed">
      <p className="text-xs text-muted-foreground mb-2">
        Raw → Cleaned comparison
      </p>
      <p className="whitespace-pre-wrap">
        {tokens.map((token, i) => {
          if (token.type === "equal") {
            return <span key={i}>{token.value} </span>;
          }
          if (token.type === "removed") {
            return (
              <span key={i} className="bg-red-900/30 text-red-400 line-through">
                {token.value}
              </span>
            );
          }
          return (
            <span key={i} className="bg-green-900/30 text-green-400">
              {token.value}
            </span>
          );
        })}
      </p>
    </div>
  );
}

// ── Confidence indicator (TASK-219) ───────────────────────────────────────

function ConfidenceDot({ confidence }: { confidence?: number }) {
  if (confidence === undefined) {
    return <span className="w-2 h-2 rounded-full bg-muted-foreground/30 mt-1 shrink-0" />;
  }
  const color =
    confidence >= 0.8
      ? "bg-green-500"
      : confidence >= 0.5
        ? "bg-yellow-500"
        : "bg-red-500";
  return <span className={`w-2 h-2 rounded-full ${color} mt-1 shrink-0`} />;
}

// ── Utilities ─────────────────────────────────────────────────────────────────

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

function msToTime(ms: number): string {
  const s = Math.floor(ms / 1000);
  const m = Math.floor(s / 60);
  return `${String(m).padStart(2, "0")}:${String(s % 60).padStart(2, "0")}`;
}
