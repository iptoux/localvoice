/**
 * React hook for Tauri event listeners with requestAnimationFrame-aligned throttling.
 *
 * Wraps `@tauri-apps/api/event.listen()` so that high-frequency events
 * (e.g. audio level meters, download progress) are coalesced and delivered
 * at most once per animation frame. This prevents excessive React state
 * updates and keeps the UI in sync with the display refresh rate.
 */
import { useEffect, useRef } from "react";
import { listen, type EventCallback } from "@tauri-apps/api/event";
import { rafThrottle } from "../lib/raf-throttle";

/**
 * Subscribe to a Tauri event with RAF-aligned throttling.
 *
 * @param eventName  Tauri event name to listen for
 * @param handler    Callback receiving the latest payload (called at most once per frame)
 * @param deps       Additional dependency array entries — the hook re-subscribes when these change
 *
 * @example
 * ```tsx
 * useThrottledEvent<number>("audio-level", (level) => {
 *   setAudioLevel(level);
 * });
 * ```
 */
export function useThrottledEvent<T>(
  eventName: string,
  handler: (payload: T) => void,
  deps: React.DependencyList = [],
): void {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const throttled = rafThrottle<T>((value) => {
      handlerRef.current(value);
    });

    const unlistenPromise = listen<T>(eventName, (event) => {
      throttled.update(event.payload);
    });

    return () => {
      throttled.cancel();
      unlistenPromise.then((fn) => fn());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [eventName, ...deps]);
}

/**
 * Subscribe to a Tauri event without throttling.
 *
 * Convenience wrapper around `listen()` with proper cleanup — use this for
 * low-frequency discrete events (state transitions, completion signals)
 * where every payload matters and no coalescing is desired.
 *
 * @example
 * ```tsx
 * useTauriEvent<RecordingStatePayload>("recording-state-changed", (p) => {
 *   setRecordingState(p.state);
 * });
 * ```
 */
export function useTauriEvent<T>(
  eventName: string,
  handler: EventCallback<T>,
  deps: React.DependencyList = [],
): void {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const unlistenPromise = listen<T>(eventName, (event) => {
      handlerRef.current(event);
    });

    return () => {
      unlistenPromise.then((fn) => fn());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [eventName, ...deps]);
}
