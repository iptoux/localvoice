/**
 * React hooks for the text-processing Web Worker.
 *
 * Provides async-aware hooks that run CPU-intensive text operations
 * off the main thread, keeping the UI responsive.
 */
import { useEffect, useState } from "react";
import {
  computeWordDiffAsync,
  extractKeywordsAsync,
} from "../workers/text-processor-client";
import type { DiffToken, KeywordEntry } from "../workers/text-processor.types";

/** Compute a word diff off the main thread. Returns null while computing. */
export function useWordDiff(
  rawText: string | undefined,
  cleanedText: string | undefined
): DiffToken[] | null {
  const [tokens, setTokens] = useState<DiffToken[] | null>(null);

  useEffect(() => {
    if (!rawText || !cleanedText) {
      setTokens(null);
      return;
    }

    let cancelled = false;
    setTokens(null);

    computeWordDiffAsync(rawText, cleanedText)
      .then((result) => {
        if (!cancelled) setTokens(result);
      })
      .catch(() => {
        if (!cancelled) setTokens(null);
      });

    return () => {
      cancelled = true;
    };
  }, [rawText, cleanedText]);

  return tokens;
}

/** Extract top-N keywords off the main thread. Returns null while computing. */
export function useKeywordExtraction(
  text: string | undefined,
  topN = 10
): KeywordEntry[] | null {
  const [keywords, setKeywords] = useState<KeywordEntry[] | null>(null);

  useEffect(() => {
    if (!text) {
      setKeywords(null);
      return;
    }

    let cancelled = false;
    setKeywords(null);

    extractKeywordsAsync(text, topN)
      .then((result) => {
        if (!cancelled) setKeywords(result);
      })
      .catch(() => {
        if (!cancelled) setKeywords(null);
      });

    return () => {
      cancelled = true;
    };
  }, [text, topN]);

  return keywords;
}
