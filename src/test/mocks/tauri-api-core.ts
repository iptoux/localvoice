// Mock for @tauri-apps/api/core — used in Vitest test environment.
// All invoke calls return a sensible default; individual tests can override
// with vi.mocked(invoke).mockResolvedValueOnce(...).

import { vi } from "vitest";

export const invoke = vi.fn().mockResolvedValue(undefined);
