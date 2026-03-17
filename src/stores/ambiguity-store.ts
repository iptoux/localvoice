import { create } from "zustand";
import type { AmbiguousTerm } from "../types";
import {
  acceptAmbiguitySuggestion,
  dismissAmbiguitySuggestion,
  listAmbiguousTerms,
} from "../lib/tauri";

interface AmbiguityState {
  terms: AmbiguousTerm[];
  loading: boolean;
  error: string | null;
  fetch: () => Promise<void>;
  accept: (id: string, targetPhrase: string) => Promise<void>;
  dismiss: (id: string) => Promise<void>;
}

export const useAmbiguityStore = create<AmbiguityState>((set) => ({
  terms: [],
  loading: false,
  error: null,

  fetch: async () => {
    set({ loading: true, error: null });
    try {
      const terms = await listAmbiguousTerms();
      set({ terms, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  accept: async (id, targetPhrase) => {
    await acceptAmbiguitySuggestion(id, targetPhrase);
    set((s) => ({ terms: s.terms.filter((t) => t.id !== id) }));
  },

  dismiss: async (id) => {
    await dismissAmbiguitySuggestion(id);
    set((s) => ({ terms: s.terms.filter((t) => t.id !== id) }));
  },
}));
