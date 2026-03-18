/** Message types for the text-processing Web Worker. */

// ── Request types ───────────────────────────────────────────────────────────

export interface WordDiffRequest {
  type: "word-diff";
  id: string;
  rawText: string;
  cleanedText: string;
}

export interface WordCountRequest {
  type: "word-count";
  id: string;
  text: string;
}

export interface KeywordExtractionRequest {
  type: "keyword-extraction";
  id: string;
  text: string;
  topN: number;
}

export type WorkerRequest =
  | WordDiffRequest
  | WordCountRequest
  | KeywordExtractionRequest;

// ── Response types ──────────────────────────────────────────────────────────

export interface DiffToken {
  type: "equal" | "added" | "removed";
  value: string;
}

export interface WordDiffResponse {
  type: "word-diff";
  id: string;
  tokens: DiffToken[];
}

export interface WordCountResponse {
  type: "word-count";
  id: string;
  count: number;
}

export interface KeywordEntry {
  word: string;
  count: number;
}

export interface KeywordExtractionResponse {
  type: "keyword-extraction";
  id: string;
  keywords: KeywordEntry[];
}

export interface WorkerErrorResponse {
  type: "error";
  id: string;
  message: string;
}

export type WorkerResponse =
  | WordDiffResponse
  | WordCountResponse
  | KeywordExtractionResponse
  | WorkerErrorResponse;
