import { useCallback, useEffect, useRef, useState } from "react";
import type { Session, SessionFilter, SessionWithSegments } from "../types";
import {
  deleteSession,
  exportSessions,
  getSessionDetail,
  listSessions,
} from "../lib/tauri";

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

  return (
    <div className="flex h-full min-h-0">
      {/* ── List panel ─────────────────────────────────────────────────────── */}
      <div className="flex flex-col flex-1 min-w-0 p-8 gap-4">
        <div className="flex items-center justify-between gap-4">
          <h1 className="text-2xl font-semibold text-white shrink-0">History</h1>
          <button
            onClick={() =>
              exportSessions(
                sessions.map((s) => s.id),
                "txt"
              ).catch(console.error)
            }
            className="text-xs text-neutral-400 hover:text-white border border-neutral-700 hover:border-neutral-500 px-3 py-1.5 rounded transition-colors"
          >
            Export page ↗
          </button>
        </div>

        {/* Search & filters (TASK-075, 076) */}
        <div className="flex flex-col gap-2">
          <input
            type="search"
            placeholder="Search transcriptions…"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="w-full bg-neutral-800 border border-neutral-700 text-white text-sm rounded-md px-3 py-2 placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-neutral-500"
          />
          <div className="flex flex-wrap gap-2">
            <select
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="bg-neutral-800 border border-neutral-700 text-neutral-300 text-xs rounded px-2 py-1.5 focus:outline-none"
            >
              <option value="">All languages</option>
              <option value="de">German (de)</option>
              <option value="en">English (en)</option>
              <option value="fr">French (fr)</option>
              <option value="es">Spanish (es)</option>
              <option value="auto">Auto-detect</option>
            </select>
            <label className="flex items-center gap-1 text-xs text-neutral-400">
              From
              <input
                type="date"
                value={dateFrom}
                onChange={(e) => setDateFrom(e.target.value)}
                className="bg-neutral-800 border border-neutral-700 text-neutral-300 rounded px-2 py-1 text-xs focus:outline-none"
              />
            </label>
            <label className="flex items-center gap-1 text-xs text-neutral-400">
              To
              <input
                type="date"
                value={dateTo}
                onChange={(e) => setDateTo(e.target.value)}
                className="bg-neutral-800 border border-neutral-700 text-neutral-300 rounded px-2 py-1 text-xs focus:outline-none"
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
                className="text-xs text-neutral-500 hover:text-white transition-colors px-2"
              >
                Clear filters ✕
              </button>
            )}
          </div>
        </div>

        {/* Session list (TASK-074) */}
        <div className="flex-1 overflow-y-auto min-h-0 space-y-1">
          {loading && (
            <p className="text-neutral-500 text-sm py-8 text-center">Loading…</p>
          )}
          {error && (
            <p className="text-rose-400 text-sm py-8 text-center">{error}</p>
          )}
          {!loading && !error && sessions.length === 0 && (
            <p className="text-neutral-500 text-sm py-8 text-center">
              No sessions found.
            </p>
          )}
          {sessions.map((session) => (
            <SessionRow
              key={session.id}
              session={session}
              active={selected?.session.id === session.id}
              onClick={() => openDetail(session)}
            />
          ))}
        </div>

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

  return (
    <button
      onClick={onClick}
      className={`
        w-full text-left px-4 py-3 rounded-lg border transition-colors
        ${
          active
            ? "border-neutral-500 bg-neutral-700"
            : "border-neutral-800 bg-neutral-900 hover:border-neutral-700 hover:bg-neutral-800"
        }
      `}
    >
      <div className="flex items-center gap-2 mb-1">
        <span className="text-xs text-neutral-400 tabular-nums">{date}</span>
        <LanguageBadge lang={session.language} />
        <span className="text-xs text-neutral-500">{session.wordCount} words</span>
        <OutputBadge mode={session.outputMode} ok={session.insertedSuccessfully} />
      </div>
      <p className="text-sm text-neutral-300 leading-snug">{preview}</p>
    </button>
  );
}

// ── Detail drawer ─────────────────────────────────────────────────────────────

function SessionDrawer({
  detail,
  loading,
  onClose,
  onDelete,
}: {
  detail: SessionWithSegments | null;
  loading: boolean;
  onClose: () => void;
  onDelete: (id: string) => void;
}) {
  const [tab, setTab] = useState<"cleaned" | "raw">("cleaned");
  const [confirmDelete, setConfirmDelete] = useState(false);

  // Reset tabs & confirm state when session changes.
  useEffect(() => {
    setTab("cleaned");
    setConfirmDelete(false);
  }, [detail?.session.id]);

  if (loading) {
    return (
      <aside className="w-96 shrink-0 border-l border-neutral-800 flex items-center justify-center">
        <p className="text-neutral-500 text-sm">Loading…</p>
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
    <aside className="w-96 shrink-0 border-l border-neutral-800 flex flex-col overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-5 py-4 border-b border-neutral-800">
        <h2 className="text-sm font-semibold text-white">Session Details</h2>
        <button
          onClick={onClose}
          className="text-neutral-500 hover:text-white text-lg leading-none"
          aria-label="Close"
        >
          ✕
        </button>
      </div>

      {/* Meta */}
      <div className="px-5 py-3 border-b border-neutral-800 space-y-1">
        <p className="text-xs text-neutral-400">{formatDate(session.startedAt)}</p>
        <div className="flex flex-wrap gap-2">
          <LanguageBadge lang={session.language} />
          {session.modelId && (
            <span className="text-xs bg-neutral-800 text-neutral-400 px-1.5 py-0.5 rounded">
              {session.modelId}
            </span>
          )}
          <span className="text-xs text-neutral-500">{session.wordCount} words</span>
          {durationSec > 0 && (
            <span className="text-xs text-neutral-500">{durationSec}s</span>
          )}
          {session.estimatedWpm && (
            <span className="text-xs text-neutral-500">
              ~{Math.round(session.estimatedWpm)} wpm
            </span>
          )}
        </div>
      </div>

      {/* Text tabs */}
      <div className="flex border-b border-neutral-800">
        {(["cleaned", "raw"] as const).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`flex-1 py-2 text-xs font-medium transition-colors ${
              tab === t
                ? "text-white border-b-2 border-white"
                : "text-neutral-500 hover:text-neutral-300"
            }`}
          >
            {t === "cleaned" ? "Cleaned" : "Raw"}
          </button>
        ))}
      </div>

      {/* Text content */}
      <div className="flex-1 overflow-y-auto px-5 py-4 min-h-0">
        <p className="text-sm text-neutral-200 leading-relaxed whitespace-pre-wrap">
          {tab === "cleaned" ? session.cleanedText : session.rawText}
        </p>

        {/* Segments list (optional) */}
        {tab === "cleaned" && segments.length > 0 && (
          <details className="mt-4">
            <summary className="text-xs text-neutral-500 cursor-pointer hover:text-neutral-300 select-none">
              {segments.length} segments
            </summary>
            <ol className="mt-2 space-y-1">
              {segments.map((seg) => (
                <li key={seg.id} className="text-xs text-neutral-400">
                  <span className="tabular-nums text-neutral-600 mr-2">
                    {msToTime(seg.startMs)}
                  </span>
                  {seg.text}
                  {seg.confidence !== undefined && (
                    <span className="text-neutral-600 ml-1">
                      ({Math.round(seg.confidence * 100)}%)
                    </span>
                  )}
                </li>
              ))}
            </ol>
          </details>
        )}
      </div>

      {/* Actions (TASK-078) */}
      <div className="flex items-center gap-2 px-5 py-3 border-t border-neutral-800">
        <button
          onClick={() =>
            copyText(tab === "cleaned" ? session.cleanedText : session.rawText)
          }
          className="flex-1 text-xs bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white rounded px-3 py-1.5 transition-colors"
        >
          Copy
        </button>
        <button
          onClick={handleExport}
          className="text-xs bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white rounded px-3 py-1.5 transition-colors"
        >
          Export ↗
        </button>
        <button
          onClick={handleDelete}
          className={`text-xs rounded px-3 py-1.5 transition-colors ${
            confirmDelete
              ? "bg-rose-700 hover:bg-rose-600 text-white"
              : "bg-neutral-800 hover:bg-neutral-700 text-rose-400 hover:text-rose-300"
          }`}
        >
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
    <div className="flex items-center justify-between text-xs text-neutral-500">
      <span>
        {from}–{to}
      </span>
      <div className="flex gap-2">
        <button
          onClick={onPrev}
          disabled={page === 0}
          className="px-3 py-1 rounded bg-neutral-800 hover:bg-neutral-700 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
        >
          ← Previous
        </button>
        <button
          onClick={onNext}
          disabled={!hasNext}
          className="px-3 py-1 rounded bg-neutral-800 hover:bg-neutral-700 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
        >
          Next →
        </button>
      </div>
    </div>
  );
}

// ── Small reusable bits ───────────────────────────────────────────────────────

function LanguageBadge({ lang }: { lang: string }) {
  return (
    <span className="text-xs bg-neutral-700 text-neutral-300 px-1.5 py-0.5 rounded font-mono uppercase">
      {lang}
    </span>
  );
}

function OutputBadge({ mode, ok }: { mode: string; ok: boolean }) {
  return (
    <span
      className={`text-xs px-1.5 py-0.5 rounded ${
        ok
          ? "bg-green-900/40 text-green-400"
          : "bg-rose-900/40 text-rose-400"
      }`}
    >
      {mode === "insert" ? "inserted" : "copied"}
    </span>
  );
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
