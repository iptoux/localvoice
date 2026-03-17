import { create } from "zustand";
import type { ModelInfo } from "../types";
import {
  deleteModel,
  downloadModel,
  listAvailableModels,
  setDefaultModel,
} from "../lib/tauri";

interface DownloadState {
  percent: number;
  bytesDownloaded: number;
  totalBytes: number;
}

interface ModelsStore {
  models: ModelInfo[];
  loading: boolean;
  /** Map of model key → current download progress (only present while downloading). */
  downloading: Record<string, DownloadState>;
  error: string | null;

  fetch: () => Promise<void>;
  startDownload: (key: string) => Promise<void>;
  removeModel: (key: string) => Promise<void>;
  setDefault: (language: "de" | "en", key: string) => Promise<void>;
  setDownloadProgress: (key: string, state: DownloadState | null) => void;
}

export const useModelsStore = create<ModelsStore>((set, get) => ({
  models: [],
  loading: false,
  downloading: {},
  error: null,

  fetch: async () => {
    set({ loading: true, error: null });
    try {
      const models = await listAvailableModels();
      set({ models, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  startDownload: async (key: string) => {
    set((s) => ({ downloading: { ...s.downloading, [key]: { percent: 0, bytesDownloaded: 0, totalBytes: 0 } } }));
    try {
      await downloadModel(key);
      await get().fetch();
    } catch (e) {
      set({ error: String(e) });
    } finally {
      set((s) => {
        const next = { ...s.downloading };
        delete next[key];
        return { downloading: next };
      });
    }
  },

  removeModel: async (key: string) => {
    try {
      await deleteModel(key);
      await get().fetch();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  setDefault: async (language: "de" | "en", key: string) => {
    try {
      await setDefaultModel(language, key);
      await get().fetch();
    } catch (e) {
      set({ error: String(e) });
    }
  },

  setDownloadProgress: (key: string, state: DownloadState | null) => {
    set((s) => {
      const next = { ...s.downloading };
      if (state === null) {
        delete next[key];
      } else {
        next[key] = state;
      }
      return { downloading: next };
    });
  },
}));
