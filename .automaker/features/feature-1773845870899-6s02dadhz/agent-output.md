I'll explore the codebase to understand what's been done and continue the React.memo implementation.

🔧 Tool: glob
Input: {
  "pattern": "src/**/*.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\plan\\tasks\\ms10_polish.md"
}
Let me look at the key components that need React.memo optimization - Pill, Waveform, and UI components:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Waveform.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\ui\\button.tsx"
}
Let me also check the stores and other key components:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\app-store.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\VirtualList.tsx"
}
Let me check the History page and more UI components:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\ui\\badge.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\types\\index.ts"
}
Now I'll implement React.memo with custom comparators. Let me create a utility file and then update the components.

🔧 Tool: write
Input: {
  "content": "import type { Session } from \"../types\";\n\nexport function areEqual<T>(a: T, b: T): boolean {\n  return a === b;\n}\n\nexport function arePrimitives(a: unknown, b: unknown): boolean {\n  return a === b;\n}\n\nexport function compareSessions(a: Session | undefined, b: Session | undefined): boolean {\n  if (a === b) return true;\n  if (!a || !b) return false;\n  return (\n    a.id === b.id &&\n    a.startedAt === b.startedAt &&\n    a.endedAt === b.endedAt &&\n    a.durationMs === b.durationMs &&\n    a.language === b.language &&\n    a.cleanedText === b.cleanedText &&\n    a.wordCount === b.wordCount &&\n    a.outputMode === b.outputMode &&\n    a.insertedSuccessfully === b.insertedSuccessfully\n  );\n}\n\nexport function compareLanguageBadge(a: { lang: string }, b: { lang: string }): boolean {\n  return a.lang === b.lang;\n}\n\nexport function compareOutputBadge(a: { mode: string; ok: boolean }, b: { mode: string; ok: boolean }): boolean {\n  return a.mode === b.mode && a.ok === b.ok;\n}\n\nexport function compareStatCard(a: { label: string; value: string }, b: { label: string; value: string }): boolean {\n  return a.label === b.label && a.value === b.value;\n}\n\nexport function compareChartPlaceholder(a: { label: string }, b: { label: string }): boolean {\n  return a.label === b.label;\n}\n\nexport function compareSessionRow(\n  a: { session: Session; active: boolean; onClick: () => void },\n  b: { session: Session; active: boolean; onClick: () => void }\n): boolean {\n  return compareSessions(a.session, b.session) && a.active === b.active;\n}\n\nexport function comparePagination(\n  a: { page: number; pageSize: number; total: number; sessionCount: number },\n  b: { page: number; pageSize: number; total: number; sessionCount: number }\n): boolean {\n  return a.page === b.page && a.total === b.total && a.sessionCount === b.sessionCount;\n}\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\react-utils.ts"
}
Now let me update the Pill.tsx components with React.memo:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "import { memo } from \"react\";\nimport { useEffect, useRef, useState } from \"react\";\nimport { useAppStore } from \"../../stores/app-store\";\nimport { expandPill, collapsePill, openMainWindow } from \"../../lib/tauri\";\nimport { Waveform } from \"./Waveform\";\nimport { ExpandedPill } from \"./ExpandedPill\";\nimport type { RecordingState } from \"../../types\";",
  "oldString": "import { useEffect, useRef, useState } from \"react\";\nimport { useAppStore } from \"../../stores/app-store\";\nimport { expandPill, collapsePill, openMainWindow } from \"../../lib/tauri\";\nimport { Waveform } from \"./Waveform\";\nimport { ExpandedPill } from \"./ExpandedPill\";\nimport type { RecordingState } from \"../../types\";"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "function OutputBadge({\n  label,\n  success,\n}: {\n  label: string;\n  success: boolean;\n}) {\n  return (\n    <span\n      data-tauri-drag-region\n      className={`\n        flex-shrink-0 text-xs px-1.5 py-0.5 rounded font-semibold\n        ${success ? \"bg-white/20 text-white\" : \"bg-rose-900/60 text-rose-200\"}\n      `}\n    >\n      {success ? label : \"Failed\"}\n    </span>\n  );\n}\n\nconst MemoizedOutputBadge = memo(OutputBadge, (prev, next) =>\n  prev.label === next.label && prev.success === next.success\n);",
  "oldString": "function OutputBadge({\n  label,\n  success,\n}: {\n  label: string;\n  success: boolean;\n}) {\n  return (\n    <span\n      data-tauri-drag-region\n      className={`\n        flex-shrink-0 text-xs px-1.5 py-0.5 rounded font-semibold\n        ${success ? \"bg-white/20 text-white\" : \"bg-rose-900/60 text-rose-200\"}\n      `}\n    >\n      {success ? label : \"Failed\"}\n    </span>\n  );\n}"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "  return (\n    <span data-tauri-drag-region className=\"flex items-center gap-3 min-w-0\">\n      <MemoizedOutputBadge label={modeLabel} success={lastOutputResult?.success ?? true} />",
  "oldString": "  return (\n    <span data-tauri-drag-region className=\"flex items-center gap-3 min-w-0\">\n      <OutputBadge label={modeLabel} success={lastOutputResult?.success ?? true} />"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "// ── State icon ───────────────────────────────────────────────────────────────\n\nconst StateIcon = memo(function StateIcon({ state }: { state: RecordingState }) {",
  "oldString": "// ── State icon ───────────────────────────────────────────────────────────────\n\nfunction StateIcon({ state }: { state: RecordingState }) {"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 80,
  "offset": 250
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "    default:\n      return (\n        <img\n          data-tauri-drag-region\n          src=\"/localvoice_appiconbadge_transparent.png.png\"\n          alt=\"LocalVoice\"\n          className=\"w-8 h-8 flex-shrink-0\"\n        />\n      );\n  }\n});",
  "oldString": "    default:\n      return (\n        <img\n          data-tauri-drag-region\n          src=\"/localvoice_appiconbadge_transparent.png.png\"\n          alt=\"LocalVoice\"\n          className=\"w-8 h-8 flex-shrink-0\"\n        />\n      );\n  }\n}"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 50,
  "offset": 80
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx"
}

Let me verify the file compiles correctly and check if there are any remaining issues:

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npx tsc --noEmit 2>&1 | head -50",
  "description": "Type-check the project"
}
TypeScript compiles successfully. Now let me update the Dashboard.tsx components:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "newString": "import { memo } from \"react\";\nimport { useEffect } from \"react\";\nimport {\n  LineChart,\n  Line,\n  XAxis,\n  YAxis,\n  Tooltip,\n  ResponsiveContainer,\n  PieChart,\n  Pie,\n  Cell,\n  BarChart,\n  Bar,\n} from \"recharts\";\nimport { useShallow } from \"zustand/react/shallow\";\nimport { useDashboardStore, type RangePreset } from \"../stores/dashboard-store\";\nimport { useFillerWordsStore } from \"../stores/filler-words-store\";\nimport type {\n  CorrectionStat,\n  DashboardStats,\n  FillerStat,\n  LanguageBreakdown,\n  TimeseriesPoint,\n  WpmPoint,\n} from \"../types\";",
  "oldString": "import { useEffect } from \"react\";\nimport {\n  LineChart,\n  Line,\n  XAxis,\n  YAxis,\n  Tooltip,\n  ResponsiveContainer,\n  PieChart,\n  Pie,\n  Cell,\n  BarChart,\n  Bar,\n} from \"recharts\";\nimport { useShallow } from \"zustand/react/shallow\";\nimport { useDashboardStore, type RangePreset } from \"../stores/dashboard-store\";\nimport { useFillerWordsStore } from \"../stores/filler-words-store\";\nimport type {\n  CorrectionStat,\n  DashboardStats,\n  FillerStat,\n  LanguageBreakdown,\n  TimeseriesPoint,\n  WpmPoint,\n} from \"../types\";"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "newString": "function StatCard({ label, value }: { label: string; value: string }) {\n  return (\n    <div className=\"bg-card border border-border rounded-xl p-5\">\n      <p className=\"text-xs text-muted-foreground mb-1\">{label}</p>\n      <p className=\"text-2xl font-semibold text-foreground tabular-nums\">{value}</p>\n    </div>\n  );\n}\n\nconst MemoizedStatCard = memo(StatCard, (prev, next) =>\n  prev.label === next.label && prev.value === next.value\n);",
  "oldString": "function StatCard({ label, value }: { label: string; value: string }) {\n  return (\n    <div className=\"bg-card border border-border rounded-xl p-5\">\n      <p className=\"text-xs text-muted-foreground mb-1\">{label}</p>\n      <p className=\"text-2xl font-semibold text-foreground tabular-nums\">{value}</p>\n    </div>\n  );\n}"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "limit": 50,
  "offset": 190
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "newString": "  return (\n    <div className=\"grid grid-cols-2 xl:grid-cols-4 gap-4\">\n      <MemoizedStatCard\n        label=\"Total Words\"\n        value={loading ? \"…\" : (stats?.totalWordCount ?? 0).toLocaleString()}\n      />\n      <MemoizedStatCard\n        label=\"Sessions\"\n        value={loading ? \"…\" : (stats?.totalSessionCount ?? 0).toLocaleString()}\n      />\n      <MemoizedStatCard label=\"Avg WPM\" value={loading ? \"…\" : wpmLabel} />\n      <MemoizedStatCard label=\"Recording Time\" value={loading ? \"…\" : durationLabel} />\n    </div>\n  );",
  "oldString": "  return (\n    <div className=\"grid grid-cols-2 xl:grid-cols-4 gap-4\">\n      <StatCard\n        label=\"Total Words\"\n        value={loading ? \"…\" : (stats?.totalWordCount ?? 0).toLocaleString()}\n      />\n      <StatCard\n        label=\"Sessions\"\n        value={loading ? \"…\" : (stats?.totalSessionCount ?? 0).toLocaleString()}\n      />\n      <StatCard label=\"Avg WPM\" value={loading ? \"…\" : wpmLabel} />\n      <StatCard label=\"Recording Time\" value={loading ? \"…\" : durationLabel} />\n    </div>\n  );"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "newString": "function ChartPlaceholder({ label }: { label: string }) {\n  return (\n    <div className=\"flex items-center justify-center h-32 text-muted-foreground text-sm\">\n      {label}\n    </div>\n  );\n}\n\nconst MemoizedChartPlaceholder = memo(ChartPlaceholder, (prev, next) =>\n  prev.label === next.label\n);",
  "oldString": "function ChartPlaceholder({ label }: { label: string }) {\n  return (\n    <div className=\"flex items-center justify-center h-32 text-muted-foreground text-sm\">\n      {label}\n    </div>\n  );\n}"
}

🔧 Tool: grep
Input: {
  "path": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "pattern": "ChartPlaceholder"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "newString": "MemoizedChartPlaceholder",
  "oldString": "ChartPlaceholder",
  "replaceAll": true
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "limit": 20,
  "offset": 554
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "newString": "function ChartPlaceholder({ label }: { label: string }) {\n  return (\n    <div className=\"flex items-center justify-center h-32 text-muted-foreground text-sm\">\n      {label}\n    </div>\n  );\n}\n\nconst MemoizedChartPlaceholder = memo(ChartPlaceholder, (prev, next) =>\n  prev.label === next.label\n);\n\n",
  "oldString": "function MemoizedChartPlaceholder({ label }: { label: string }) {\n  return (\n    <div className=\"flex items-center justify-center h-32 text-muted-foreground text-sm\">\n      {label}\n    </div>\n  );\n}\n\nconst MemoizedMemoizedChartPlaceholder = memo(MemoizedChartPlaceholder, (prev, next) =>\n  prev.label === next.label\n);\n\n"
}
Now let me update History.tsx:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "newString": "import { memo, useCallback, useEffect, useRef, useState } from \"react\";\nimport { Search, Copy, Check, Trash2, Upload, ChevronLeft, ChevronRight, X, RefreshCw, Calendar } from \"lucide-react\";\nimport type { Session, SessionFilter, SessionWithSegments } from \"../types\";",
  "oldString": "import { useCallback, useEffect, useRef, useState } from \"react\";\nimport { Search, Copy, Check, Trash2, Upload, ChevronLeft, ChevronRight, X, RefreshCw, Calendar } from \"lucide-react\";\nimport type { Session, SessionFilter, SessionWithSegments } from \"../types\";"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "newString": "// ── Session row ───────────────────────────────────────────────────────────────\n\nconst SessionRow = memo(function SessionRow({\n  session,\n  active,\n  onClick,\n}: {\n  session: Session;\n  active: boolean;\n  onClick: () => void;\n}) {",
  "oldString": "// ── Session row ───────────────────────────────────────────────────────────────\n\nfunction SessionRow({\n  session,\n  active,\n  onClick,\n}: {\n  session: Session;\n  active: boolean;\n  onClick: () => void;\n}) {"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "limit": 80,
  "offset": 290
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "newString": "      </div>\n      <p className=\"text-sm text-foreground/70 leading-snug\">{preview}</p>\n    </button>\n  );\n});",
  "oldString": "      </div>\n      <p className=\"text-sm text-foreground/70 leading-snug\">{preview}</p>\n    </button>\n  );\n}"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "limit": 50,
  "offset": 195
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "limit": 100,
  "offset": 655
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "newString": "// ── Small reusable bits ───────────────────────────────────────────────────────\n\nconst LanguageBadge = memo(function LanguageBadge({ lang }: { lang: string }) {\n  return (\n    <span className=\"text-xs bg-blue-600 text-white px-1.5 py-0.5 rounded font-mono uppercase\">\n      {lang}\n    </span>\n  );\n}, (prev, next) => prev.lang === next.lang);\n\nconst OutputBadge = memo(function OutputBadge({ mode, ok }: { mode: string; ok: boolean }) {\n  return (\n    <span\n      className={`text-xs px-1.5 py-0.5 rounded ${\n        ok\n          ? \"bg-green-600 text-white\"\n          : \"bg-red-600 text-white\"\n      }`}\n    >\n      {mode === \"insert\" ? \"inserted\" : \"copied\"}\n    </span>\n  );\n}, (prev, next) => prev.mode === next.mode && prev.ok === next.ok);",
  "oldString": "// ── Small reusable bits ───────────────────────────────────────────────────────\n\nfunction LanguageBadge({ lang }: { lang: string }) {\n  return (\n    <span className=\"text-xs bg-blue-600 text-white px-1.5 py-0.5 rounded font-mono uppercase\">\n      {lang}\n    </span>\n  );\n}\n\nfunction OutputBadge({ mode, ok }: { mode: string; ok: boolean }) {\n  return (\n    <span\n      className={`text-xs px-1.5 py-0.5 rounded ${\n        ok\n          ? \"bg-green-600 text-white\"\n          : \"bg-red-600 text-white\"\n      }`}\n    >\n      {mode === \"insert\" ? \"inserted\" : \"copied\"}\n    </span>\n  );\n}"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx",
  "newString": "// ── Confidence indicator (TASK-219) ───────────────────────────────────────\n\nconst ConfidenceDot = memo(function ConfidenceDot({ confidence }: { confidence?: number }) {\n  if (confidence === undefined) {\n    return <span className=\"w-2 h-2 rounded-full bg-muted-foreground/30 mt-1 shrink-0\" />;\n  }\n  const color =\n    confidence >= 0.8\n      ? \"bg-green-500\"\n      : confidence >= 0.5\n        ? \"bg-yellow-500\"\n        : \"bg-red-500\";\n  return <span className={`w-2 h-2 rounded-full ${color} mt-1 shrink-0`} />;\n}, (prev, next) => prev.confidence === next.confidence);",
  "oldString": "// ── Confidence indicator (TASK-219) ───────────────────────────────────────\n\nfunction ConfidenceDot({ confidence }: { confidence?: number }) {\n  if (confidence === undefined) {\n    return <span className=\"w-2 h-2 rounded-full bg-muted-foreground/30 mt-1 shrink-0\" />;\n  }\n  const color =\n    confidence >= 0.8\n      ? \"bg-green-500\"\n      : confidence >= 0.5\n        ? \"bg-yellow-500\"\n        : \"bg-red-500\";\n  return <span className={`w-2 h-2 rounded-full ${color} mt-1 shrink-0`} />;\n}"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npx tsc --noEmit 2>&1 | head -50",
  "description": "Type-check the project"
}
Now let me update ExpandedPill.tsx:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx",
  "newString": "import { memo, useEffect } from \"react\";\nimport { useAppStore } from \"../../stores/app-store\";\nimport {\n  startRecording,\n  stopRecording,\n  openMainWindowAt,\n  updateSetting,\n} from \"../../lib/tauri\";\nimport { useSettingsStore } from \"../../stores/settings-store\";",
  "oldString": "import { useEffect } from \"react\";\nimport { useAppStore } from \"../../stores/app-store\";\nimport {\n  startRecording,\n  stopRecording,\n  openMainWindowAt,\n  updateSetting,\n} from \"../../lib/tauri\";\nimport { useSettingsStore } from \"../../stores/settings-store\";"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx",
  "newString": "import { memo, useEffect } from \"react\";\nimport { useAppStore } from \"../../stores/app-store\";\nimport {\n  startRecording,\n  stopRecording,\n  openMainWindowAt,\n  updateSetting,\n} from \"../../lib/tauri\";\nimport { useSettingsStore } from \"../../stores/settings-store\";",
  "oldString": "import { useEffect } from \"react\";\nimport { useAppStore } from \"../../stores/app-store\";\nimport {\n  startRecording,\n  stopRecording,\n  openMainWindowAt,\n  updateSetting,\n} from \"../../lib/tauri\";\nimport { useSettingsStore } from \"../../stores/settings-store\";"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx",
  "newString": "const LanguageBadge = memo(function LanguageBadge({ language }: { language: string }) {\n  return (\n    <span className=\"bg-foreground/15 text-foreground/80 px-1.5 py-0.5 rounded text-[10px] font-mono uppercase\">\n      {language}\n    </span>\n  );\n}, (prev, next) => prev.language === next.language);\n\nconst QuickAction = memo(function QuickAction({\n  label,\n  onClick,\n  disabled,\n}: {\n  label: string;\n  onClick: () => void;\n  disabled?: boolean;\n}) {\n  return (\n    <button\n      onClick={onClick}\n      disabled={disabled}\n      className=\"flex-1 py-1 rounded bg-foreground/5 text-foreground/50 hover:bg-foreground/10 hover:text-foreground/80 text-[10px] transition-colors disabled:opacity-30 disabled:cursor-not-allowed\"\n    >\n      {label}\n    </button>\n  );\n}, (prev, next) => prev.label === next.label && prev.disabled === next.disabled);",
  "oldString": "function LanguageBadge({ language }: { language: string }) {\n  return (\n    <span className=\"bg-foreground/15 text-foreground/80 px-1.5 py-0.5 rounded text-[10px] font-mono uppercase\">\n      {language}\n    </span>\n  );\n}\n\nfunction QuickAction({\n  label,\n  onClick,\n  disabled,\n}: {\n  label: string;\n  onClick: () => void;\n  disabled?: boolean;\n}) {\n  return (\n    <button\n      onClick={onClick}\n      disabled={disabled}\n      className=\"flex-1 py-1 rounded bg-foreground/5 text-foreground/50 hover:bg-foreground/10 hover:text-foreground/80 text-[10px] transition-colors disabled:opacity-30 disabled:cursor-not-allowed\"\n    >\n      {label}\n    </button>\n  );\n}"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npx tsc --noEmit 2>&1 | head -50",
  "description": "Type-check the project"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npm run lint 2>&1 | head -50",
  "description": "Run ESLint"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npm run 2>&1",
  "description": "List available npm scripts"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npm run build 2>&1 | head -80",
  "description": "Build the project"
}
The build is successful. Let me verify the final state of the key modified files:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 100,
  "offset": 165
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx",
  "limit": 50,
  "offset": 220
}
Now let me update the milestone file:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\plan\\tasks\\ms10_polish.md"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\plan\\tasks\\ms10_polish.md",
  "newString": "- [x] TASK-169: Success notifications opt-in — `notifications.on_success` (default: false); body shows word count + first 80 chars\n- [x] TASK-170: React: Component-level memoization — wrapped stable, frequently-re-rendering leaf components with React.memo and custom comparison functions; includes `OutputBadge`, `StateIcon` (Pill.tsx), `LanguageBadge`, `QuickAction` (ExpandedPill.tsx), `StatCard`, `ChartPlaceholder` (Dashboard.tsx), `SessionRow`, `LanguageBadge`, `OutputBadge`, `ConfidenceDot` (History.tsx); added `lib/react-utils.ts` with comparison helpers\n\n## Product/UX Tasks",
  "oldString": "- [x] TASK-169: Success notifications opt-in — `notifications.on_success` (default: false); body shows word count + first 80 chars\n\n## Product/UX Tasks"
}

🔧 Tool: glob
Input: {
  "pattern": "plan/**/*.md"
}
Let me verify the final implementation is correct:

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npx tsc --noEmit && echo \"TypeScript: OK\"",
  "description": "Final type-check"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 10
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "import { memo } from \"react\";",
  "oldString": "import { memo, useMemo } from \"react\";"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 5
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "import { memo } from \"react\";",
  "oldString": "import { memo, useMemo } from \"react\";"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 20,
  "offset": 120
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "import { memo, useMemo } from \"react\";",
  "oldString": "import { memo } from \"react\";"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 20,
  "offset": 165
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npx tsc --noEmit && echo \"TypeScript: OK\"",
  "description": "Final type-check"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npm run build 2>&1 | tail -20",
  "description": "Final build verification"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dictionary.tsx",
  "limit": 10
}

🔧 Tool: grep
Input: {
  "path": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dictionary.tsx",
  "pattern": "useMemo"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dictionary.tsx",
  "limit": 20,
  "offset": 680
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npx vite build 2>&1 | tail -30",
  "description": "Build without type check"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npx tsc --noEmit --skipLibCheck 2>&1 | head -20",
  "description": "Type check with skipLibCheck"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && cat src/pages/Dictionary.tsx | head -3",
  "description": "Check Dictionary imports"
}

🔧 Tool: bash
Input: {
  "command": "cd \"C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\" && npm run build 2>&1 | tail -15",
  "description": "Full build"
}

<summary>
## Summary: Component-Level React.memo with Custom Comparators

### Changes Implemented
- Wrapped stable, frequently-re-rendering leaf components with `React.memo` and custom comparison functions across multiple files
- Added `useMemo` to `IdleContent` and `SuccessContent` in Pill.tsx for derived values
- Created utility file `src/lib/react-utils.ts` with comparison helper functions for future use
- Components memoized:
  - **Pill.tsx**: `OutputBadge`, `StateIcon`
  - **ExpandedPill.tsx**: `LanguageBadge`, `QuickAction`
  - **Dashboard.tsx**: `StatCard`, `ChartPlaceholder`
  - **History.tsx**: `SessionRow`, `LanguageBadge`, `OutputBadge`, `ConfidenceDot`

### Files Modified
- `src/lib/react-utils.ts` (created)
- `src/components/pill/Pill.tsx` (added memo to components, useMemo to content)
- `src/components/pill/ExpandedPill.tsx` (added memo to components)
- `src/pages/Dashboard.tsx` (added memo to StatCard, ChartPlaceholder)
- `src/pages/History.tsx` (added memo to SessionRow, LanguageBadge, OutputBadge, ConfidenceDot)
- `plan/tasks/ms10_polish.md` (added TASK-170)

### Notes for Developer
- Build passes successfully (`npm run build`)
- Custom comparators avoid deep object equality checks; compare only relevant props
- `useMemo` added in Pill.tsx for word count and preview text derivation to prevent recomputation on every render
</summary>