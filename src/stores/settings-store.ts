import { create } from "zustand";
import type { Settings } from "../types";
import { getSettings, updateSetting } from "../lib/tauri";

interface SettingsStore {
  settings: Settings;
  loading: boolean;
  load: () => Promise<void>;
  update: (key: string, value: string) => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: {},
  loading: false,

  load: async () => {
    set({ loading: true });
    try {
      const settings = await getSettings();
      set({ settings });
    } finally {
      set({ loading: false });
    }
  },

  update: async (key, value) => {
    await updateSetting(key, value);
    set((state) => ({
      settings: { ...state.settings, [key]: value },
    }));
  },
}));
