import { create } from "zustand";
import type { UpdateDownloadProgress, UpdateInfo, UpdateStatus } from "../types";
import {
  checkForUpdate,
  getUpdateStatus,
  installPendingUpdate,
} from "../lib/tauri";

interface UpdaterStore {
  status: UpdateStatus;
  dismissedVersion: string | null;
  loading: boolean;
  load: () => Promise<void>;
  check: (manual?: boolean) => Promise<UpdateInfo | null>;
  install: () => Promise<void>;
  dismiss: () => void;
  setAvailable: (info: UpdateInfo) => void;
  setProgress: (progress: UpdateDownloadProgress) => void;
  setError: (message: string) => void;
}

const IDLE_STATUS: UpdateStatus = {
  phase: "idle",
  available: null,
  progress: null,
  lastError: null,
};

export const useUpdaterStore = create<UpdaterStore>((set, get) => ({
  status: IDLE_STATUS,
  dismissedVersion: null,
  loading: false,

  load: async () => {
    const status = await getUpdateStatus();
    set({ status });
  },

  check: async (manual = false) => {
    set((state) => ({
      loading: true,
      status: { ...state.status, phase: "checking", lastError: null },
    }));
    try {
      const info = await checkForUpdate(manual);
      const status = await getUpdateStatus();
      set((state) => ({
        status,
        loading: false,
        dismissedVersion:
          info && state.dismissedVersion === info.version ? null : state.dismissedVersion,
      }));
      return info;
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      set((state) => ({
        loading: false,
        status: { ...state.status, phase: "error", lastError: message },
      }));
      throw e;
    }
  },

  install: async () => {
    set((state) => ({
      loading: true,
      status: { ...state.status, phase: "downloading", lastError: null },
    }));
    try {
      await installPendingUpdate();
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      set((state) => ({
        loading: false,
        status: { ...state.status, phase: "error", lastError: message },
      }));
      throw e;
    }
  },

  dismiss: () => {
    const version = get().status.available?.version ?? null;
    set({ dismissedVersion: version });
  },

  setAvailable: (info) => {
    set((state) => ({
      status: {
        ...state.status,
        phase: "available",
        available: info,
        progress: null,
        lastError: null,
      },
      dismissedVersion:
        state.dismissedVersion === info.version ? null : state.dismissedVersion,
    }));
  },

  setProgress: (progress) => {
    set((state) => ({
      status: {
        ...state.status,
        phase: progress.percent === 100 ? "installing" : "downloading",
        progress,
        lastError: null,
      },
    }));
  },

  setError: (message) => {
    set((state) => ({
      loading: false,
      status: { ...state.status, phase: "error", lastError: message },
    }));
  },
}));

