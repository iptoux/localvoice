import { create } from "zustand";
import type { FillerStat, FillerWord } from "../types";
import {
  listFillerWords, addFillerWord, deleteFillerWord, resetFillerWords,
  getFillerStats, getFillerTotalCount,
} from "../lib/tauri";

interface FillerWordsStore {
  words: FillerWord[];
  stats: FillerStat[];
  totalRemoved: number;
  loading: boolean;
  error: string | null;
  fetch: (language?: string) => Promise<void>;
  fetchStats: (language?: string) => Promise<void>;
  add: (word: string, language: string) => Promise<void>;
  remove: (id: string) => Promise<void>;
  reset: (language: string) => Promise<void>;
}

export const useFillerWordsStore = create<FillerWordsStore>((set) => ({
  words: [],
  stats: [],
  totalRemoved: 0,
  loading: false,
  error: null,

  fetch: async (language) => {
    set({ loading: true, error: null });
    try {
      const words = await listFillerWords(language);
      set({ words, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  fetchStats: async (language) => {
    try {
      const [stats, totalRemoved] = await Promise.all([
        getFillerStats(language),
        getFillerTotalCount(),
      ]);
      set({ stats, totalRemoved });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  add: async (word, language) => {
    const added = await addFillerWord(word, language);
    set((s) => ({ words: [...s.words, added] }));
  },

  remove: async (id) => {
    await deleteFillerWord(id);
    set((s) => ({ words: s.words.filter((w) => w.id !== id) }));
  },

  reset: async (language) => {
    const words = await resetFillerWords(language);
    set((s) => ({ words: [...s.words.filter((w) => w.language !== language), ...words] }));
  },
}));
