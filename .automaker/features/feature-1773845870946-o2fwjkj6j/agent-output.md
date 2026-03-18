Let me verify the implementation is complete by running the lint and checking the modified files.

🔧 Tool: bash
Input: {
  "command": "npm run lint 2>&1 | head -30",
  "description": "Run ESLint to verify code",
  "timeout": 60000
}

🔧 Tool: bash
Input: {
  "command": "npm run 2>&1",
  "description": "List available npm scripts"
}

🔧 Tool: bash
Input: {
  "command": "npm run build 2>&1",
  "description": "Run build to verify TypeScript",
  "timeout": 120000
}
Build verified successfully. The `useMemo` optimizations are complete across all target files.

<summary>
## Summary: useMemo for Derived State in Transcription Components

### Changes Implemented
- **Dashboard.tsx**: `useMemo` for `durationLabel`, `wpmLabel` (< 0.1ms), `total`, `chartData` (< 0.5ms) in StatCards/LanguagePie; `maxCount`, `chartData` (< 0.5ms) in CorrectionFrequency; `maxSessions` (< 0.3ms) in TopModels; `maxCount` (< 0.3ms) in FillerStatsChart

- **History.tsx**: `useMemo` for `preview` (< 0.1ms) in SessionRow; `installedModels` (< 0.2ms) filter in SessionDrawer

- **Models.tsx**: `useMemo` for `defaultsByLang` (< 1ms) lookup map; `visible` (< 1ms) filtered/sorted list; `installed` (< 0.2ms) in DefaultSelector

- **Pill.tsx**: `useMemo` for `wordCount` (< 0.1ms) in IdleContent; `preview` (< 0.1ms) in SuccessContent

- **ExpandedPill.tsx**: `useMemo` for `wordCount` (< 0.1ms) computation

- **Dictionary.tsx**: `useMemo` for `filtered` (< 0.2ms) language filter in FillerWordsTab

### Files Modified
- `src/pages/Dashboard.tsx`
- `src/pages/History.tsx`
- `src/pages/Models.tsx`
- `src/pages/Dictionary.tsx`
- `src/components/pill/Pill.tsx`
- `src/components/pill/ExpandedPill.tsx`

### Notes for Developer
- All `useMemo` hooks include performance budget comments (e.g., `// < 0.5ms`) to flag future regressions
- Precise dependency arrays ensure re-computation only when necessary
- Build verified with `npm run build`; TypeScript compilation passed with no errors
</summary>