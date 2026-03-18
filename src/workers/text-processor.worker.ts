/**
 * Web Worker for CPU-intensive text processing tasks.
 *
 * Handles word diffing (LCS), word counting, and keyword extraction
 * off the main thread to keep the UI responsive.
 */
import type {
  WorkerRequest,
  WorkerResponse,
  DiffToken,
  KeywordEntry,
} from "./text-processor.types";

// ── Message handler ─────────────────────────────────────────────────────────

self.onmessage = (event: MessageEvent<WorkerRequest>) => {
  const req = event.data;
  try {
    let response: WorkerResponse;
    switch (req.type) {
      case "word-diff":
        response = {
          type: "word-diff",
          id: req.id,
          tokens: computeWordDiff(req.rawText, req.cleanedText),
        };
        break;
      case "word-count":
        response = {
          type: "word-count",
          id: req.id,
          count: countWords(req.text),
        };
        break;
      case "keyword-extraction":
        response = {
          type: "keyword-extraction",
          id: req.id,
          keywords: extractKeywords(req.text, req.topN),
        };
        break;
    }
    self.postMessage(response);
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    self.postMessage({ type: "error", id: req.id, message } satisfies WorkerResponse);
  }
};

// ── Word diff (LCS) ─────────────────────────────────────────────────────────

function computeWordDiff(rawText: string, cleanedText: string): DiffToken[] {
  const a = rawText.split(/\s+/).filter(Boolean);
  const b = cleanedText.split(/\s+/).filter(Boolean);
  const m = a.length;
  const n = b.length;

  // Build LCS table.
  const dp: number[][] = Array.from({ length: m + 1 }, () => Array(n + 1).fill(0));
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      dp[i][j] =
        a[i - 1] === b[j - 1]
          ? dp[i - 1][j - 1] + 1
          : Math.max(dp[i - 1][j], dp[i][j - 1]);
    }
  }

  // Backtrack to produce diff tokens.
  const stack: DiffToken[] = [];
  let i = m;
  let j = n;

  while (i > 0 || j > 0) {
    if (i > 0 && j > 0 && a[i - 1] === b[j - 1]) {
      stack.push({ type: "equal", value: a[i - 1] });
      i--;
      j--;
    } else if (j > 0 && (i === 0 || dp[i][j - 1] >= dp[i - 1][j])) {
      stack.push({ type: "added", value: b[j - 1] + " " });
      j--;
    } else {
      stack.push({ type: "removed", value: a[i - 1] + " " });
      i--;
    }
  }

  stack.reverse();
  return stack;
}

// ── Word count ──────────────────────────────────────────────────────────────

function countWords(text: string): number {
  return text.trim().split(/\s+/).filter(Boolean).length;
}

// ── Keyword extraction ──────────────────────────────────────────────────────

/** Common stop words excluded from keyword extraction. */
const STOP_WORDS = new Set([
  // English
  "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
  "have", "has", "had", "do", "does", "did", "will", "would", "could",
  "should", "may", "might", "shall", "can", "need", "dare", "ought",
  "used", "to", "of", "in", "for", "on", "with", "at", "by", "from",
  "as", "into", "through", "during", "before", "after", "above", "below",
  "between", "out", "off", "over", "under", "again", "further", "then",
  "once", "here", "there", "when", "where", "why", "how", "all", "each",
  "every", "both", "few", "more", "most", "other", "some", "such", "no",
  "nor", "not", "only", "own", "same", "so", "than", "too", "very",
  "just", "because", "but", "and", "or", "if", "while", "about", "up",
  "that", "this", "these", "those", "it", "its", "i", "me", "my",
  "we", "our", "you", "your", "he", "him", "his", "she", "her",
  "they", "them", "their", "what", "which", "who", "whom",
  // German
  "der", "die", "das", "ein", "eine", "einer", "eines", "einem", "einen",
  "und", "oder", "aber", "denn", "weil", "dass", "wenn", "als", "ob",
  "ich", "du", "er", "sie", "es", "wir", "ihr", "man", "sich",
  "mein", "dein", "sein", "unser", "euer",
  "ist", "sind", "war", "waren", "wird", "werden", "wurde", "wurden",
  "hat", "haben", "hatte", "hatten", "kann", "konnte", "muss", "musste",
  "soll", "sollte", "darf", "durfte", "will", "wollte", "mag", "mochte",
  "nicht", "auch", "nur", "noch", "schon", "sehr", "mehr",
  "von", "zu", "mit", "auf", "in", "an", "aus", "bei", "nach",
  "vor", "um", "durch", "für", "über", "unter", "zwischen",
]);

function extractKeywords(text: string, topN: number): KeywordEntry[] {
  const words = text.toLowerCase().split(/\s+/).filter(Boolean);
  const freq = new Map<string, number>();

  for (const word of words) {
    // Strip surrounding punctuation.
    const clean = word.replace(/^[^\p{L}\p{N}]+|[^\p{L}\p{N}]+$/gu, "");
    if (clean.length < 2 || STOP_WORDS.has(clean)) continue;
    freq.set(clean, (freq.get(clean) ?? 0) + 1);
  }

  return [...freq.entries()]
    .sort((a, b) => b[1] - a[1])
    .slice(0, topN)
    .map(([word, count]) => ({ word, count }));
}
