import { useEffect, useState } from "react";
import type { CorrectionRule, DictionaryEntry } from "../types";
import { useDictionaryStore } from "../stores/dictionary-store";

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
          ? "border-blue-500 text-white"
          : "border-transparent text-neutral-400 hover:text-neutral-200"
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
        className="bg-neutral-900 border border-neutral-700 rounded-xl p-6 w-full max-w-md shadow-xl"
      >
        <h2 className="text-white font-semibold mb-4">
          {initial ? "Edit Entry" : "Add Entry"}
        </h2>
        <div className="flex flex-col gap-3">
          <input
            className="bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500"
            placeholder="Phrase"
            value={phrase}
            onChange={(e) => setPhrase(e.target.value)}
            autoFocus
            required
          />
          <select
            className="bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-3 py-2 focus:outline-none"
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
            className="bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-3 py-2 focus:outline-none"
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
            className="bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500 resize-none"
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
            className="px-4 py-1.5 text-sm text-neutral-400 hover:text-white transition-colors"
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
        className="bg-neutral-900 border border-neutral-700 rounded-xl p-6 w-full max-w-md shadow-xl"
      >
        <h2 className="text-white font-semibold mb-4">
          {initial ? "Edit Rule" : "Add Rule"}
        </h2>
        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2">
            <input
              className="flex-1 bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="Heard text (e.g. clawd)"
              value={source}
              onChange={(e) => setSource(e.target.value)}
              autoFocus
              required
            />
            <span className="text-neutral-500">→</span>
            <input
              className="flex-1 bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="Corrected text (e.g. Claude)"
              value={target}
              onChange={(e) => setTarget(e.target.value)}
              required
            />
          </div>
          <select
            className="bg-neutral-800 border border-neutral-700 text-white text-sm rounded px-3 py-2 focus:outline-none"
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
          >
            {LANGUAGES.map((l) => (
              <option key={l.value} value={l.value}>
                {l.label}
              </option>
            ))}
          </select>
          <label className="flex items-center gap-2 text-sm text-neutral-300 cursor-pointer">
            <input
              type="checkbox"
              checked={autoApply}
              onChange={(e) => setAutoApply(e.target.checked)}
              className="accent-blue-500"
            />
            Apply automatically during transcription
          </label>
          {initial && (
            <label className="flex items-center gap-2 text-sm text-neutral-300 cursor-pointer">
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
            className="px-4 py-1.5 text-sm text-neutral-400 hover:text-white transition-colors"
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
  const { entries, addEntry, editEntry, removeEntry } = useDictionaryStore();
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
        <span className="text-xs text-neutral-500">{entries.length} entries</span>
        <button
          onClick={openAdd}
          className="text-xs px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white rounded transition-colors"
        >
          + Add entry
        </button>
      </div>

      {entries.length === 0 ? (
        <p className="text-neutral-500 text-sm">No entries yet. Add terms, names, or acronyms that whisper frequently mishears.</p>
      ) : (
        <div className="flex flex-col gap-1.5">
          {entries.map((entry) => (
            <div
              key={entry.id}
              className="flex items-center gap-3 px-4 py-2.5 rounded-lg bg-neutral-800 border border-neutral-700"
            >
              <div className="flex-1 min-w-0">
                <span className="text-white text-sm font-medium">{entry.phrase}</span>
                <span className="ml-2 text-xs text-neutral-500 capitalize">{entry.entryType}</span>
                {entry.language && (
                  <span className="ml-1 text-xs text-neutral-500 uppercase">{entry.language}</span>
                )}
                {entry.notes && (
                  <p className="text-xs text-neutral-500 mt-0.5 truncate">{entry.notes}</p>
                )}
              </div>
              <button
                onClick={() => openEdit(entry)}
                className="text-xs text-neutral-400 hover:text-white transition-colors"
              >
                Edit
              </button>
              <button
                onClick={() => handleDelete(entry)}
                className="text-xs text-red-500 hover:text-red-400 transition-colors"
              >
                Delete
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
  const { rules, addRule, editRule, toggleRule, removeRule } = useDictionaryStore();
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
        <span className="text-xs text-neutral-500">{rules.length} rules</span>
        <button
          onClick={openAdd}
          className="text-xs px-3 py-1.5 bg-blue-600 hover:bg-blue-500 text-white rounded transition-colors"
        >
          + Add rule
        </button>
      </div>

      {rules.length === 0 ? (
        <p className="text-neutral-500 text-sm">No correction rules yet. Rules replace misheard words automatically during transcription.</p>
      ) : (
        <div className="flex flex-col gap-1.5">
          {rules.map((rule) => (
            <div
              key={rule.id}
              className={`flex items-center gap-3 px-4 py-2.5 rounded-lg border ${
                rule.isActive
                  ? "bg-neutral-800 border-neutral-700"
                  : "bg-neutral-900 border-neutral-800 opacity-60"
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
                <span className="text-white text-sm font-mono">{rule.sourcePhrase}</span>
                <span className="text-neutral-500 text-xs">→</span>
                <span className="text-green-400 text-sm font-mono">{rule.targetPhrase}</span>
                {rule.language && (
                  <span className="text-xs text-neutral-500 uppercase">{rule.language}</span>
                )}
              </div>

              {/* Usage count */}
              <span className="text-xs text-neutral-500 shrink-0">
                {rule.usageCount}×
              </span>

              <button
                onClick={() => openEdit(rule)}
                className="text-xs text-neutral-400 hover:text-white transition-colors shrink-0"
              >
                Edit
              </button>
              <button
                onClick={() => handleDelete(rule)}
                className="text-xs text-red-500 hover:text-red-400 transition-colors shrink-0"
              >
                Delete
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

// ── page ──────────────────────────────────────────────────────────────────────

type Tab = "terms" | "rules";

export default function Dictionary() {
  const [activeTab, setActiveTab] = useState<Tab>("rules");
  const { fetchAll, error } = useDictionaryStore();

  useEffect(() => {
    fetchAll();
  }, [fetchAll]);

  return (
    <div className="p-8 max-w-3xl mx-auto">
      <h1 className="text-2xl font-semibold text-white mb-1">Dictionary</h1>
      <p className="text-neutral-400 text-sm mb-6">
        Correction rules are applied automatically to every transcript. Terms are reference entries for future features.
      </p>

      {error && (
        <div className="mb-4 p-3 rounded bg-red-900/40 border border-red-700 text-sm text-red-300">
          {error}
        </div>
      )}

      {/* Tabs */}
      <div className="flex gap-1 border-b border-neutral-700 mb-5">
        <TabButton active={activeTab === "rules"} onClick={() => setActiveTab("rules")}>
          Rules
        </TabButton>
        <TabButton active={activeTab === "terms"} onClick={() => setActiveTab("terms")}>
          Terms
        </TabButton>
      </div>

      {activeTab === "rules" ? <RulesTab /> : <TermsTab />}
    </div>
  );
}
