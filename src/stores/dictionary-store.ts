import { create } from "zustand";
import type { CorrectionRule, DictionaryEntry } from "../types";
import {
  createCorrectionRule,
  createDictionaryEntry,
  deleteCorrectionRule,
  deleteDictionaryEntry,
  listCorrectionRules,
  listDictionaryEntries,
  updateCorrectionRule,
  updateDictionaryEntry,
} from "../lib/tauri";

interface DictionaryStore {
  entries: DictionaryEntry[];
  rules: CorrectionRule[];
  error: string | null;

  fetchAll: () => Promise<void>;

  // Entries
  addEntry: (payload: { phrase: string; language?: string; entryType: string; notes?: string }) => Promise<void>;
  editEntry: (id: string, payload: { phrase: string; language?: string; entryType: string; notes?: string }) => Promise<void>;
  removeEntry: (id: string) => Promise<void>;

  // Rules
  addRule: (payload: { sourcePhrase: string; targetPhrase: string; language?: string; autoApply: boolean }) => Promise<void>;
  editRule: (id: string, payload: { sourcePhrase: string; targetPhrase: string; language?: string; isActive: boolean; autoApply: boolean }) => Promise<void>;
  toggleRule: (rule: CorrectionRule) => Promise<void>;
  removeRule: (id: string) => Promise<void>;
}

export const useDictionaryStore = create<DictionaryStore>((set, get) => ({
  entries: [],
  rules: [],
  error: null,

  fetchAll: async () => {
    try {
      const [entries, rules] = await Promise.all([
        listDictionaryEntries(),
        listCorrectionRules(),
      ]);
      set({ entries, rules, error: null });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  addEntry: async (payload) => {
    try {
      await createDictionaryEntry(payload);
      await get().fetchAll();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  editEntry: async (id, payload) => {
    try {
      await updateDictionaryEntry(id, payload);
      await get().fetchAll();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  removeEntry: async (id) => {
    try {
      await deleteDictionaryEntry(id);
      await get().fetchAll();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  addRule: async (payload) => {
    try {
      await createCorrectionRule(payload);
      await get().fetchAll();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  editRule: async (id, payload) => {
    try {
      await updateCorrectionRule(id, payload);
      await get().fetchAll();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  toggleRule: async (rule) => {
    try {
      await updateCorrectionRule(rule.id, {
        sourcePhrase: rule.sourcePhrase,
        targetPhrase: rule.targetPhrase,
        language: rule.language,
        isActive: !rule.isActive,
        autoApply: rule.autoApply,
      });
      await get().fetchAll();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  removeRule: async (id) => {
    try {
      await deleteCorrectionRule(id);
      await get().fetchAll();
    } catch (e) {
      set({ error: String(e) });
    }
  },
}));
