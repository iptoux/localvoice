import type {
  DownloadProgress as TDownloadProgress,
  OutputResult,
  RecordingStatePayload,
  TranscriptionResult,
} from "../types";

/**
 * Centralized registry of all Tauri event channels used in LocalVoice.
 *
 * This file is the single source of truth for:
 * - Event channel names
 * - TypeScript payload types
 * - Usage examples
 *
 * ## Adding a new event
 * 1. Add the event definition here with channel name, payload type, and documentation.
 * 2. Export the channel name constant for use in components.
 * 3. Use `useThrottledEvent` or `useTauriEvent` hooks with the event type.
 */
export const EventChannels = {
  /** Pill recording state transitions (idle → listening → processing → success/error). */
  RECORDING_STATE_CHANGED: "recording-state-changed",

  /** Real-time RMS audio level during recording (0-1 range, throttled ~80ms). */
  AUDIO_LEVEL: "audio-level",

  /** Result of the output step (clipboard write or text insertion). */
  OUTPUT_RESULT: "output-result",

  /** Fired when a full transcription pipeline completes successfully or with error. */
  TRANSCRIPTION_COMPLETED: "transcription-completed",

  /** Fired when silence timeout is detected during recording. */
  SILENCE_DETECTED: "silence-detected",

  /** Fired after a session is re-transcribed with updated language/model. */
  SESSION_REPROCESSED: "session-reprocessed",

  /** Download progress for model installations (throttled to 1% changes). */
  MODEL_DOWNLOAD_PROGRESS: "model-download-progress",

  /** Frontend-to-frontend navigation event (pill → main window). */
  NAVIGATE_TO: "navigate-to",
} as const;

export type EventChannel = (typeof EventChannels)[keyof typeof EventChannels];

/**
 * Payload types for all registered events.
 * Use these types with `useThrottledEvent<T>` or `listen<T>()`.
 */
export interface EventPayloads {
  [EventChannels.RECORDING_STATE_CHANGED]: RecordingStatePayload;
  [EventChannels.AUDIO_LEVEL]: number;
  [EventChannels.OUTPUT_RESULT]: OutputResult;
  [EventChannels.TRANSCRIPTION_COMPLETED]: TranscriptionResult;
  [EventChannels.SILENCE_DETECTED]: void;
  [EventChannels.SESSION_REPROCESSED]: string;
  [EventChannels.MODEL_DOWNLOAD_PROGRESS]: TDownloadProgress;
  [EventChannels.NAVIGATE_TO]: string;
}

/**
 * Type-safe event listener that infers the payload type from the channel name.
 *
 * @example
 * ```ts
 * import { listenEvent, EventChannels } from "./lib/events";
 *
 * // Automatically infers payload as RecordingStatePayload
 * listenEvent(EventChannels.RECORDING_STATE_CHANGED, (payload) => {
 *   console.log("Recording state:", payload.state);
 * });
 * ```
 */
export async function listenEvent<C extends EventChannel>(
  channel: C,
  handler: (payload: EventPayloads[C]) => void
): Promise<() => void> {
  const { listen } = await import("@tauri-apps/api/event");
  const unlisten = await listen<EventPayloads[C]>(channel, (event) => {
    handler(event.payload);
  });
  return unlisten;
}

/**
 * Type-safe throttled event listener for high-frequency events.
 * Use for audio-level, download-progress, etc.
 *
 * @example
 * ```ts
 * import { listenThrottledEvent, EventChannels } from "./lib/events";
 *
 * listenThrottledEvent(EventChannels.AUDIO_LEVEL, (rms) => {
 *   setAudioLevel(rms);
 * });
 * ```
 */
export async function listenThrottledEvent<C extends EventChannel>(
  channel: C,
  handler: (payload: EventPayloads[C]) => void
): Promise<() => void> {
  const { listen } = await import("@tauri-apps/api/event");
  const { rafThrottle } = await import("./raf-throttle");

  const throttled = rafThrottle<EventPayloads[C]>((value) => {
    handler(value);
  });

  const unlisten = await listen<EventPayloads[C]>(channel, (event) => {
    throttled.update(event.payload);
  });

  return () => {
    throttled.cancel();
    unlisten();
  };
}
