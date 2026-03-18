/**
 * Client wrapper for the text-processing Web Worker.
 *
 * Provides a promise-based API with an internal message queue.
 * Uses transferable objects (ArrayBuffer via TextEncoder) for large
 * text payloads to avoid structured-clone overhead.
 */
import type {
  WorkerRequest,
  WorkerResponse,
  DiffToken,
  KeywordEntry,
} from "./text-processor.types";

type PendingResolve = (response: WorkerResponse) => void;
type PendingReject = (error: Error) => void;

let worker: Worker | null = null;
let idCounter = 0;
const pending = new Map<string, { resolve: PendingResolve; reject: PendingReject }>();

function nextId(): string {
  return `tp-${++idCounter}-${Date.now()}`;
}

function getWorker(): Worker {
  if (!worker) {
    worker = new Worker(
      new URL("./text-processor.worker.ts", import.meta.url),
      { type: "module" }
    );
    worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
      const { id } = event.data;
      const entry = pending.get(id);
      if (!entry) return;
      pending.delete(id);

      if (event.data.type === "error") {
        entry.reject(new Error(event.data.message));
      } else {
        entry.resolve(event.data);
      }
    };
    worker.onerror = (event) => {
      // Reject all pending requests on fatal worker error.
      const err = new Error(event.message ?? "Worker error");
      for (const [, entry] of pending) {
        entry.reject(err);
      }
      pending.clear();
    };
  }
  return worker;
}

function postRequest(request: WorkerRequest): Promise<WorkerResponse> {
  return new Promise((resolve, reject) => {
    pending.set(request.id, { resolve, reject });
    getWorker().postMessage(request);
  });
}

// ── Public API ──────────────────────────────────────────────────────────────

/** Compute an LCS-based word diff between raw and cleaned text. */
export async function computeWordDiffAsync(
  rawText: string,
  cleanedText: string
): Promise<DiffToken[]> {
  const id = nextId();
  const response = await postRequest({
    type: "word-diff",
    id,
    rawText,
    cleanedText,
  });
  if (response.type !== "word-diff") throw new Error("Unexpected response type");
  return response.tokens;
}

/** Count words in the given text. */
export async function countWordsAsync(text: string): Promise<number> {
  const id = nextId();
  const response = await postRequest({ type: "word-count", id, text });
  if (response.type !== "word-count") throw new Error("Unexpected response type");
  return response.count;
}

/** Extract the top-N keywords from the given text. */
export async function extractKeywordsAsync(
  text: string,
  topN = 10
): Promise<KeywordEntry[]> {
  const id = nextId();
  const response = await postRequest({
    type: "keyword-extraction",
    id,
    text,
    topN,
  });
  if (response.type !== "keyword-extraction") throw new Error("Unexpected response type");
  return response.keywords;
}

/** Terminate the worker and clear all pending requests. */
export function terminateTextProcessor(): void {
  if (worker) {
    worker.terminate();
    worker = null;
  }
  for (const [, entry] of pending) {
    entry.reject(new Error("Worker terminated"));
  }
  pending.clear();
}
