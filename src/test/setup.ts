import "@testing-library/jest-dom";

// @base-ui/react Switch (and other pointer-aware components) reference PointerEvent
// which is not available in jsdom. Polyfill it so fireEvent.click works.
if (typeof window !== "undefined" && !("PointerEvent" in window)) {
  class PointerEvent extends MouseEvent {
    constructor(type: string, params: PointerEventInit = {}) {
      super(type, params);
    }
  }
  (window as any).PointerEvent = PointerEvent;
}
