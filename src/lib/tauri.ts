import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "../types";

// ── Settings ──────────────────────────────────────────────────────────────────

export const getSettings = (): Promise<Settings> =>
  invoke<Settings>("get_settings");

export const updateSetting = (key: string, value: string): Promise<void> =>
  invoke<void>("update_setting", { key, value });

export const resetSettings = (): Promise<void> =>
  invoke<void>("reset_settings");

// ── Window ────────────────────────────────────────────────────────────────────

export const showPill = (): Promise<void> => invoke<void>("show_pill");

export const hidePill = (): Promise<void> => invoke<void>("hide_pill");

export const openMainWindow = (): Promise<void> =>
  invoke<void>("open_main_window");

export const setPillPosition = (x: number, y: number): Promise<void> =>
  invoke<void>("set_pill_position", { x, y });
