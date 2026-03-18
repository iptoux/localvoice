import type { Session } from "../types";

export function areEqual<T>(a: T, b: T): boolean {
  return a === b;
}

export function arePrimitives(a: unknown, b: unknown): boolean {
  return a === b;
}

export function compareSessions(a: Session | undefined, b: Session | undefined): boolean {
  if (a === b) return true;
  if (!a || !b) return false;
  return (
    a.id === b.id &&
    a.startedAt === b.startedAt &&
    a.endedAt === b.endedAt &&
    a.durationMs === b.durationMs &&
    a.language === b.language &&
    a.cleanedText === b.cleanedText &&
    a.wordCount === b.wordCount &&
    a.outputMode === b.outputMode &&
    a.insertedSuccessfully === b.insertedSuccessfully
  );
}

export function compareLanguageBadge(a: { lang: string }, b: { lang: string }): boolean {
  return a.lang === b.lang;
}

export function compareOutputBadge(a: { mode: string; ok: boolean }, b: { mode: string; ok: boolean }): boolean {
  return a.mode === b.mode && a.ok === b.ok;
}

export function compareStatCard(a: { label: string; value: string }, b: { label: string; value: string }): boolean {
  return a.label === b.label && a.value === b.value;
}

export function compareChartPlaceholder(a: { label: string }, b: { label: string }): boolean {
  return a.label === b.label;
}

export function compareSessionRow(
  a: { session: Session; active: boolean; onClick: () => void },
  b: { session: Session; active: boolean; onClick: () => void }
): boolean {
  return compareSessions(a.session, b.session) && a.active === b.active;
}

export function comparePagination(
  a: { page: number; pageSize: number; total: number; sessionCount: number },
  b: { page: number; pageSize: number; total: number; sessionCount: number }
): boolean {
  return a.page === b.page && a.total === b.total && a.sessionCount === b.sessionCount;
}
