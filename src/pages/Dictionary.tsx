import { useEffect, useState } from "react";
import { Plus, Pencil, Trash2, RefreshCw, BookOpen, Wand2, Lightbulb, Filter, X } from "lucide-react";
import { useShallow } from "zustand/react/shallow";
import type { AmbiguousTerm, CorrectionRule, DictionaryEntry, FillerWord } from "../types";
import { useAmbiguityStore } from "../stores/ambiguity-store";
import { useDictionaryStore } from "../stores/dictionary-store";
import { useFillerWordsStore } from "../stores/filler-words-store";

// ── shared ────────────────────────────────────────────────────────────────────

const LANGUAGES = [
  { value: "", label: "All languages" },
  { value: "de", label: "German (DE)" },
  { value: "en", label: "English (EN)" },
];

const ENTRY_TYPES = ["term", "name", "acronym", "product", "custom"] as const;

function TabButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`px-4 py-2 text-sm font-medium rounded-t-lg border-b-2 transition-colors ${
        active
          ? "border-blue-500 text-foreground"
          : "border-transparent text-muted-foreground hover:text-foreground/80"
      }`}
    >
      {children}
    </button>
  );
}

// ── Entry form modal ──────────────────────────────────────────────────────────

interface EntryFormProps {
  initial?: DictionaryEntry;
  onSave: (data: { phrase: string; language?: string; entryType: string; notes?: string }) => void;
  onClose: () => void;
}

function EntryForm({ initial, onSave, onClose }: EntryFormProps) {
  const [phrase, setPhrase] = useState(initial?.phrase ?? "");
  const [language, setLanguage] = useState(initial?.language ?? "");
  const [entryType, setEntryType] = useState(initial?.entryType ?? "term");
  const [notes, setNotes] = useState(initial?.notes ?? "");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!phrase.trim()) return;
    onSave({
      phrase: phrase.trim(),
      language: language || undefined,
      entryType,
      notes: notes.trim() || undefined,
    });
  };

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
      <form
        onSubmit={handleSubmit}
        className="bg-card border border-border rounded-xl p-6 w-full max-w-md shadow-xl"
      >
        <h2 className="text-foreground font-semibold mb-4">
          {initial ? "Edit Entry" : "Add Entry"}
        </h2>
        <div className="flex flex-col gap-3">
          <input
            className="bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500"
            placeholder="Phrase"
            value={phrase}
            onChange={(e) => setPhrase(e.target.value)}
            autoFocus
            required
          />
          <select
            className="bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none"
            value={entryType}
            onChange={(e) => setEntryType(e.target.value)}
          >
            {ENTRY_TYPES.map((t) => (
              <option key={t} value={t} className="capitalize">
                {t}
              </option>
            ))}
          </select>
          <select
            className="bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none"
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
          >
            {LANGUAGES.map((l) => (
              <option key={l.value} value={l.value}>
                {l.label}
              </option>
            ))}
          </select>
          <textarea
            className="bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500 resize-none"
            placeholder="Notes (optional)"
            rows={2}
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
          />
        </div>
        <div className="flex justify-end gap-2 mt-5">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            className="px-4 py-1.5 text-sm bg-blue-600 hover:bg-blue-500 text-white rounded transition-colors"
          >
            Save
          </button>
        </div>
      </form>
    </div>
  );
}

// ── Rule form modal ───────────────────────────────────────────────────────────

interface RuleFormProps {
  initial?: CorrectionRule;
  onSave: (data: {
    sourcePhrase: string;
    targetPhrase: string;
    language?: string;
    isActive: boolean;
    autoApply: boolean;
  }) => void;
  onClose: () => void;
}

function RuleForm({ initial, onSave, onClose }: RuleFormProps) {
  const [source, setSource] = useState(initial?.sourcePhrase ?? "");
  const [target, setTarget] = useState(initial?.targetPhrase ?? "");
  const [language, setLanguage] = useState(initial?.language ?? "");
  const [autoApply, setAutoApply] = useState(initial?.autoApply ?? true);
  const [isActive, setIsActive] = useState(initial?.isActive ?? true);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!source.trim() || !target.trim()) return;
    onSave({
      sourcePhrase: source.trim(),
      targetPhrase: target.trim(),
      language: language || undefined,
      isActive,
      autoApply,
    });
  };

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
      <form
        onSubmit={handleSubmit}
        className="bg-card border border-border rounded-xl p-6 w-full max-w-md shadow-xl"
      >
        <h2 className="text-foreground font-semibold mb-4">
          {initial ? "Edit Rule" : "Add Rule"}
        </h2>
        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2">
            <input
              className="flex-1 bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="Heard text (e.g. clawd)"
              value={source}
              onChange={(e) => setSource(e.target.value)}
              autoFocus
              required
            />
            <span className="text-muted-foreground">→</span>
            <input
              className="flex-1 bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="Corrected text (e.g. Claude)"
              value={target}
              onChange={(e) => setTarget(e.target.value)}
              required
            />
          </div>
          <select
            className="bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none"
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
          >
            {LANGUAGES.map((l) => (
              <option key={l.value} value={l.value}>
                {l.label}
              </option>
            ))}
          </select>
          <label className="flex items-center gap-2 text-sm text-foreground/70 cursor-pointer">
            <input
              type="checkbox"
              checked={autoApply}
              onChange={(e) => setAutoApply(e.target.checked)}
              className="accent-blue-500"
            />
            Apply automatically during transcription
          </label>
          {initial && (
            <label className="flex items-center gap-2 text-sm text-foreground/70 cursor-pointer">
              <input
                type="checkbox"
                checked={isActive}
                onChange={(e) => setIsActive(e.target.checked)}
                className="accent-blue-500"
              />
              Rule is active
            </label>
          )}
        </div>
        <div className="flex justify-end gap-2 mt-5">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            className="px-4 py-1.5 text-sm bg-blue-600 hover:bg-blue-500 text-white rounded transition-colors"
          >
            Save
          </button>
        </div>
      </form>
    </div>
  );
}

// ── Terms tab ─────────────────────────────────────────────────────────────────

function TermsTab() {
  const { entries, addEntry, editEntry, removeEntry } = useDictionaryStore(
    useShallow((s) => ({
      entries: s.entries,
      addEntry: s.addEntry,
      editEntry: s.editEntry,
      removeEntry: s.removeEntry,
    }))
  );
  const [showForm, setShowForm] = useState(false);
  const [editing, setEditing] = useState<DictionaryEntry | undefined>();

  const openAdd = () => {
    setEditing(undefined);
    setShowForm(true);
  };
  const openEdit = (entry: DictionaryEntry) => {
    setEditing(entry);
    setShowForm(true);
  };
  const handleSave = async (data: Parameters<typeof addEntry>[0]) => {
    if (editing) {
      await editEntry(editing.id, data);
    } else {
      await addEntry(data);
    }
    setShowForm(false);
  };
  const handleDelete = (entry: DictionaryEntry) => {
    if (confirm(`Delete entry "${entry.phrase}"?`)) removeEntry(entry.id);
  };

  return (
    <>
      <div className="flex justify-between items-center mb-3">
        <span className="text-xs text-muted-foreground">{entries.length} entries</span>
        <button
          onClick={openAdd}
          className="flex items-center gap-1.5 text-xs px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white rounded transition-colors"
        >
          <Plus size={12} /> Add entry
        </button>
      </div>

      {entries.length === 0 ? (
        <p className="text-muted-foreground text-sm">No entries yet. Add terms, names, or acronyms that whisper frequently mishears.</p>
      ) : (
        <div className="flex flex-col gap-1.5">
          {entries.map((entry) => (
            <div
              key={entry.id}
              className="flex items-center gap-3 px-4 py-2.5 rounded-lg bg-muted border border-border"
            >
              <div className="flex-1 min-w-0">
                <span className="text-foreground text-sm font-medium">{entry.phrase}</span>
                <span className="ml-2 text-xs text-muted-foreground capitalize">{entry.entryType}</span>
                {entry.language && (
                  <span className="ml-1 text-xs text-muted-foreground uppercase">{entry.language}</span>
                )}
                {entry.notes && (
                  <p className="text-xs text-muted-foreground mt-0.5 truncate">{entry.notes}</p>
                )}
              </div>
              <button
                onClick={() => openEdit(entry)}
                className="text-xs text-muted-foreground hover:text-foreground transition-colors"
              >
                <Pencil size={13} />
              </button>
              <button
                onClick={() => handleDelete(entry)}
                className="text-xs text-red-500 hover:text-red-400 transition-colors"
              >
                <Trash2 size={13} />
              </button>
            </div>
          ))}
        </div>
      )}

      {showForm && (
        <EntryForm
          initial={editing}
          onSave={handleSave}
          onClose={() => setShowForm(false)}
        />
      )}
    </>
  );
}

// ── Rules tab ─────────────────────────────────────────────────────────────────

function RulesTab() {
  const { rules, addRule, editRule, toggleRule, removeRule } = useDictionaryStore(
    useShallow((s) => ({
      rules: s.rules,
      addRule: s.addRule,
      editRule: s.editRule,
      toggleRule: s.toggleRule,
      removeRule: s.removeRule,
    }))
  );
  const [showForm, setShowForm] = useState(false);
  const [editing, setEditing] = useState<CorrectionRule | undefined>();

  const openAdd = () => {
    setEditing(undefined);
    setShowForm(true);
  };
  const openEdit = (rule: CorrectionRule) => {
    setEditing(rule);
    setShowForm(true);
  };
  const handleSave = async (data: {
    sourcePhrase: string;
    targetPhrase: string;
    language?: string;
    isActive: boolean;
    autoApply: boolean;
  }) => {
    if (editing) {
      await editRule(editing.id, data);
    } else {
      await addRule({ ...data, autoApply: data.autoApply });
    }
    setShowForm(false);
  };
  const handleDelete = (rule: CorrectionRule) => {
    if (confirm(`Delete rule "${rule.sourcePhrase} → ${rule.targetPhrase}"?`))
      removeRule(rule.id);
  };

  return (
    <>
      <div className="flex justify-between items-center mb-3">
        <span className="text-xs text-muted-foreground">{rules.length} rules</span>
        <button
          onClick={openAdd}
          className="flex items-center gap-1.5 text-xs px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white rounded transition-colors"
        >
          <Plus size={12} /> Add rule
        </button>
      </div>

      {rules.length === 0 ? (
        <p className="text-muted-foreground text-sm">No correction rules yet. Rules replace misheard words automatically during transcription.</p>
      ) : (
        <div className="flex flex-col gap-1.5">
          {rules.map((rule) => (
            <div
              key={rule.id}
              className={`flex items-center gap-3 px-4 py-2.5 rounded-lg border ${
                rule.isActive
                  ? "bg-muted border-border"
                  : "bg-card border-border opacity-60"
              }`}
            >
              {/* Active toggle */}
              <button
                onClick={() => toggleRule(rule)}
                title={rule.isActive ? "Disable rule" : "Enable rule"}
                className={`w-8 h-4 rounded-full transition-colors shrink-0 ${
                  rule.isActive ? "bg-green-600" : "bg-neutral-600"
                }`}
              >
                <span
                  className={`block w-3 h-3 rounded-full bg-white transition-transform mx-0.5 ${
                    rule.isActive ? "translate-x-4" : "translate-x-0"
                  }`}
                />
              </button>

              {/* Content */}
              <div className="flex-1 min-w-0 flex items-center gap-2 flex-wrap">
                <span className="text-foreground text-sm font-mono">{rule.sourcePhrase}</span>
                <span className="text-muted-foreground text-xs">→</span>
                <span className="text-green-400 text-sm font-mono">{rule.targetPhrase}</span>
                {rule.language && (
                  <span className="text-xs text-muted-foreground uppercase">{rule.language}</span>
                )}
              </div>

              {/* Usage count */}
              <span className="text-xs text-muted-foreground shrink-0">
                {rule.usageCount}×
              </span>

              <button
                onClick={() => openEdit(rule)}
                className="text-xs text-muted-foreground hover:text-foreground transition-colors shrink-0"
              >
                <Pencil size={13} />
              </button>
              <button
                onClick={() => handleDelete(rule)}
                className="text-xs text-red-500 hover:text-red-400 transition-colors shrink-0"
              >
                <Trash2 size={13} />
              </button>
            </div>
          ))}
        </div>
      )}

      {showForm && (
        <RuleForm
          initial={editing}
          onSave={handleSave}
          onClose={() => setShowForm(false)}
        />
      )}
    </>
  );
}

// ── Suggestions tab ───────────────────────────────────────────────────────────

function SuggestionsTab() {
  const { terms, loading, error, fetch, accept, dismiss } = useAmbiguityStore(
    useShallow((s) => ({
      terms: s.terms,
      loading: s.loading,
      error: s.error,
      fetch: s.fetch,
      accept: s.accept,
      dismiss: s.dismiss,
    }))
  );
  const [editingId, setEditingId] = useState<string | null>(null);
  const [customTarget, setCustomTarget] = useState("");

  useEffect(() => {
    fetch();
  }, [fetch]);

  const handleAccept = async (term: AmbiguousTerm) => {
    const target =
      editingId === term.id ? customTarget.trim() : term.suggestedTarget ?? "";
    if (!target) return;
    await accept(term.id, target);
    setEditingId(null);
    setCustomTarget("");
  };

  const openCustom = (term: AmbiguousTerm) => {
    setEditingId(term.id);
    setCustomTarget(term.suggestedTarget ?? "");
  };

  const confidenceColor = (conf?: number) => {
    if (conf === undefined) return "text-muted-foreground";
    if (conf < 0.4) return "text-red-400";
    if (conf < 0.55) return "text-orange-400";
    return "text-yellow-400";
  };

  if (loading) {
    return <p className="text-muted-foreground text-sm">Loading…</p>;
  }

  return (
    <>
      {error && (
        <div className="mb-3 p-3 rounded bg-red-900/40 border border-red-700 text-sm text-red-300">
          {error}
        </div>
      )}

      <div className="flex justify-between items-center mb-3">
        <span className="text-xs text-muted-foreground">{terms.length} suggestions</span>
        <button
          onClick={() => fetch()}
          className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors"
        >
          <RefreshCw size={12} /> Refresh
        </button>
      </div>

      {terms.length === 0 ? (
        <p className="text-muted-foreground text-sm">
          No suggestions yet. Ambiguous terms are detected automatically when
          whisper transcribes segments with low confidence (≥ 3 occurrences by
          default).
        </p>
      ) : (
        <div className="flex flex-col gap-2">
          {terms.map((term) => (
            <div
              key={term.id}
              className="px-4 py-3 rounded-lg bg-muted border border-border"
            >
              {/* Header row */}
              <div className="flex items-center gap-3 flex-wrap">
                <span className="text-foreground text-sm font-mono font-medium">
                  {term.phrase}
                </span>
                <span className="text-xs text-muted-foreground">
                  {term.occurrences}×
                </span>
                {term.avgConfidence !== undefined && (
                  <span
                    className={`text-xs font-mono ${confidenceColor(term.avgConfidence)}`}
                  >
                    conf {(term.avgConfidence * 100).toFixed(0)}%
                  </span>
                )}
                {term.language && (
                  <span className="text-xs text-muted-foreground uppercase">
                    {term.language}
                  </span>
                )}
              </div>

              {/* Suggestion / edit row */}
              <div className="mt-2">
                {editingId === term.id ? (
                  <div className="flex items-center gap-2">
                    <span className="text-muted-foreground text-xs shrink-0">
                      Replace with:
                    </span>
                    <input
                      className="flex-1 bg-accent border border-neutral-600 text-foreground text-sm rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-blue-500"
                      value={customTarget}
                      onChange={(e) => setCustomTarget(e.target.value)}
                      autoFocus
                      onKeyDown={(e) => {
                        if (e.key === "Enter") handleAccept(term);
                        if (e.key === "Escape") {
                          setEditingId(null);
                          setCustomTarget("");
                        }
                      }}
                    />
                    <button
                      onClick={() => handleAccept(term)}
                      disabled={!customTarget.trim()}
                      className="text-xs px-3 py-1 bg-green-700 hover:bg-green-600 disabled:opacity-40 text-white rounded transition-colors"
                    >
                      Create Rule
                    </button>
                    <button
                      onClick={() => {
                        setEditingId(null);
                        setCustomTarget("");
                      }}
                      className="text-xs text-muted-foreground hover:text-foreground transition-colors"
                    >
                      Cancel
                    </button>
                  </div>
                ) : term.suggestedTarget ? (
                  <div className="flex items-center gap-2 flex-wrap">
                    <span className="text-muted-foreground text-xs">→</span>
                    <span className="text-green-400 text-sm font-mono">
                      {term.suggestedTarget}
                    </span>
                    <button
                      onClick={() => handleAccept(term)}
                      className="text-xs px-3 py-1 bg-green-700 hover:bg-green-600 text-white rounded transition-colors"
                    >
                      Accept
                    </button>
                    <button
                      onClick={() => openCustom(term)}
                      className="text-xs text-muted-foreground hover:text-foreground transition-colors"
                    >
                      Edit
                    </button>
                    <button
                      onClick={() => dismiss(term.id)}
                      className="text-xs text-muted-foreground hover:text-red-400 transition-colors"
                    >
                      Dismiss
                    </button>
                  </div>
                ) : (
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => openCustom(term)}
                      className="text-xs px-3 py-1 bg-accent hover:bg-neutral-600 text-foreground rounded transition-colors"
                    >
                      Create Rule…
                    </button>
                    <button
                      onClick={() => dismiss(term.id)}
                      className="text-xs text-muted-foreground hover:text-red-400 transition-colors"
                    >
                      Dismiss
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </>
  );
}

// ── Filler Words tab ──────────────────────────────────────────────────────────

const FILLER_LANGUAGES = [
  { value: "de", label: "DE" },
  { value: "en", label: "EN" },
  { value: "fr", label: "FR" },
  { value: "es", label: "ES" },
  { value: "it", label: "IT" },
  { value: "pt", label: "PT" },
  { value: "nl", label: "NL" },
  { value: "pl", label: "PL" },
  { value: "ru", label: "RU" },
  { value: "ja", label: "JA" },
  { value: "zh", label: "ZH" },
];

function FillerWordsTab() {
  const { words, loading, error, fetch, add, remove, reset } = useFillerWordsStore(
    useShallow((s) => ({
      words: s.words,
      loading: s.loading,
      error: s.error,
      fetch: s.fetch,
      add: s.add,
      remove: s.remove,
      reset: s.reset,
    }))
  );
  const [lang, setLang] = useState("de");
  const [newWord, setNewWord] = useState("");

  useEffect(() => { fetch(lang); }, [lang, fetch]);

  const filtered = words.filter((w) => w.language === lang);

  const handleAdd = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = newWord.trim().toLowerCase();
    if (!trimmed) return;
    await add(trimmed, lang);
    setNewWord("");
  };

  const handleReset = async () => {
    if (!confirm(`Reset all ${lang.toUpperCase()} filler words to defaults?`)) return;
    await reset(lang);
  };

  return (
    <>
      {/* Language tabs */}
      <div className="flex flex-wrap gap-1 mb-4">
        {FILLER_LANGUAGES.map((l) => (
          <button
            key={l.value}
            onClick={() => setLang(l.value)}
            className={`px-2.5 py-1 text-xs rounded transition-colors ${
              lang === l.value
                ? "bg-blue-600 text-white"
                : "bg-muted text-muted-foreground hover:text-foreground"
            }`}
          >
            {l.label}
          </button>
        ))}
        <span className="flex-1" />
        <button
          onClick={handleReset}
          className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors"
        >
          <RefreshCw size={12} /> Reset to defaults
        </button>
      </div>

      {/* Add form */}
      <form onSubmit={handleAdd} className="flex gap-2 mb-4">
        <input
          className="flex-1 bg-muted border border-border text-foreground text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500"
          placeholder={`Add filler word (${lang.toUpperCase()})…`}
          value={newWord}
          onChange={(e) => setNewWord(e.target.value)}
        />
        <button
          type="submit"
          disabled={!newWord.trim()}
          className="flex items-center gap-1.5 text-xs px-3 py-1.5 bg-blue-600 hover:bg-blue-500 disabled:opacity-40 text-white rounded transition-colors"
        >
          <Plus size={12} /> Add
        </button>
      </form>

      {error && (
        <div className="mb-3 p-3 rounded bg-red-900/40 border border-red-700 text-sm text-red-300">{error}</div>
      )}

      {loading ? (
        <p className="text-muted-foreground text-sm">Loading…</p>
      ) : filtered.length === 0 ? (
        <p className="text-muted-foreground text-sm">No filler words for {lang.toUpperCase()} yet.</p>
      ) : (
        <div className="flex flex-wrap gap-2">
          {filtered.map((w: FillerWord) => (
            <span
              key={w.id}
              className="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-muted border border-border text-sm"
            >
              <span className="text-foreground">{w.word}</span>
              {w.isDefault && (
                <span className="text-xs text-muted-foreground">·</span>
              )}
              <button
                onClick={() => remove(w.id)}
                className="text-muted-foreground hover:text-red-400 transition-colors leading-none"
                title="Remove"
              >
                <X size={12} />
              </button>
            </span>
          ))}
        </div>
      )}
    </>
  );
}

// ── page ──────────────────────────────────────────────────────────────────────

type Tab = "terms" | "rules" | "suggestions" | "fillers";

export default function Dictionary() {
  const [activeTab, setActiveTab] = useState<Tab>("rules");
  const { fetchAll, error } = useDictionaryStore(
    useShallow((s) => ({ fetchAll: s.fetchAll, error: s.error }))
  );

  useEffect(() => {
    fetchAll();
  }, [fetchAll]);

  return (
    <div className="p-8">
      <h1 className="text-2xl font-semibold text-foreground mb-1">Dictionary</h1>
      <p className="text-muted-foreground text-sm mb-6">
        Correction rules are applied automatically to every transcript. Terms are reference entries. Suggestions surface low-confidence phrases for review.
      </p>

      {error && (
        <div className="mb-4 p-3 rounded bg-red-900/40 border border-red-700 text-sm text-red-300">
          {error}
        </div>
      )}

      {/* Tabs */}
      <div className="flex gap-1 border-b border-border mb-5">
        <TabButton active={activeTab === "rules"} onClick={() => setActiveTab("rules")}>
          <Wand2 size={13} className="inline mr-1.5 -mt-0.5" />Rules
        </TabButton>
        <TabButton active={activeTab === "suggestions"} onClick={() => setActiveTab("suggestions")}>
          <Lightbulb size={13} className="inline mr-1.5 -mt-0.5" />Suggestions
        </TabButton>
        <TabButton active={activeTab === "terms"} onClick={() => setActiveTab("terms")}>
          <BookOpen size={13} className="inline mr-1.5 -mt-0.5" />Terms
        </TabButton>
        <TabButton active={activeTab === "fillers"} onClick={() => setActiveTab("fillers")}>
          <Filter size={13} className="inline mr-1.5 -mt-0.5" />Filler Words
        </TabButton>
      </div>

      {activeTab === "rules" && <RulesTab />}
      {activeTab === "suggestions" && <SuggestionsTab />}
      {activeTab === "terms" && <TermsTab />}
      {activeTab === "fillers" && <FillerWordsTab />}
    </div>
  );
}
