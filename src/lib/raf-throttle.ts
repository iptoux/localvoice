/**
 * requestAnimationFrame-aligned throttle for high-frequency callbacks.
 *
 * Buffers incoming values and flushes the latest one to the callback
 * on the next animation frame. Multiple calls within a single frame
 * are coalesced — only the most recent value is delivered, keeping
 * UI updates in sync with the display refresh rate.
 */
export function rafThrottle<T>(callback: (value: T) => void): {
  /** Accept a new value. Will be flushed on the next animation frame. */
  update: (value: T) => void;
  /** Cancel any pending frame and prevent further flushes. */
  cancel: () => void;
} {
  let pending: T | undefined;
  let frameId: number | null = null;

  function flush() {
    frameId = null;
    if (pending !== undefined) {
      callback(pending);
      pending = undefined;
    }
  }

  function update(value: T) {
    pending = value;
    if (frameId === null) {
      frameId = requestAnimationFrame(flush);
    }
  }

  function cancel() {
    if (frameId !== null) {
      cancelAnimationFrame(frameId);
      frameId = null;
    }
    pending = undefined;
  }

  return { update, cancel };
}
