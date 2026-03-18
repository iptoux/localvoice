Let me first check the current state of the codebase to understand what has been implemented.

🔧 Tool: glob
Input: {
  "pattern": "src/**/*.css"
}

🔧 Tool: glob
Input: {
  "pattern": "src/**/*.tsx"
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
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\index.css"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx"
}

Now I have a clear picture of the codebase. The feature needs CSS containment applied to independently updating panels:
1. **Waveform** (`Waveform.tsx`) - animates at 60fps via `requestAnimationFrame`
2. **Status bar** (the compact pill bar in `Pill.tsx`) - state changes on recording transitions  
3. **Live transcription feed** (`ExpandedPill.tsx`) - updates with each result

The most efficient approach is to add a Tailwind-compatible CSS rule in `index.css` using the `@utility` directive (Tailwind v4), then apply it as a class.

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\index.css",
  "newString": "html,\nbody {\n  margin: 0;\n  padding: 0;\n  overflow: hidden;\n  background: transparent;\n}\n\n@utility contain-layout-paint {\n  contain: layout paint;\n}",
  "oldString": "html,\nbody {\n  margin: 0;\n  padding: 0;\n  overflow: hidden;\n  background: transparent;\n}"
}
Now apply the utility to each independently updating panel:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Waveform.tsx",
  "newString": "  return (\n    <canvas\n      ref={canvasRef}\n      data-tauri-drag-region\n      className=\"flex-shrink-0 contain-layout-paint\"\n      style={{ width: BAR_COUNT * (BAR_WIDTH + BAR_GAP) - BAR_GAP, height: MAX_HEIGHT + 4 }}\n    />\n  );",
  "oldString": "  return (\n    <canvas\n      ref={canvasRef}\n      data-tauri-drag-region\n      className=\"flex-shrink-0\"\n      style={{ width: BAR_COUNT * (BAR_WIDTH + BAR_GAP) - BAR_GAP, height: MAX_HEIGHT + 4 }}\n    />\n  );"
}
Now apply to the status bar (the compact pill bar's dynamic content area):

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "        {recordingState === \"idle\" ? (\n          <IdleContent />\n        ) : (\n          <>\n            <StateIcon state={recordingState} />\n            <span data-tauri-drag-region className=\"flex-1 truncate contain-layout-paint\">",
  "oldString": "        {recordingState === \"idle\" ? (\n          <IdleContent />\n        ) : (\n          <>\n            <StateIcon state={recordingState} />\n            <span data-tauri-drag-region className=\"flex-1 truncate\">"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 10,
  "offset": 95
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "            <span data-tauri-drag-region className=\"flex-1 truncate contain-layout-paint\">\n              {recordingState === \"error\" && recordingError ? (\n                recordingError\n              ) : recordingState === \"success\" ? (\n                <SuccessContent />\n              ) : recordingState === \"listening\" ? (\n                <Waveform />\n              ) : (\n                \"Transcribing…\"\n              )}\n            </span>",
  "oldString": "            <span data-tauri-drag-region className=\"flex-1 truncate\">\n              {recordingState === \"error\" && recordingError ? (\n                recordingError\n              ) : recordingState === \"success\" ? (\n                <SuccessContent />\n              ) : recordingState === \"listening\" ? (\n                <Waveform />\n              ) : (\n                \"Transcribing…\"\n              )}\n            </span>"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx"
}
Now apply to the status bar's dynamic content:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "newString": "            <span data-tauri-drag-region className=\"flex-1 truncate contain-layout-paint\">\n              {recordingState === \"error\" && recordingError ? (\n                recordingError\n              ) : recordingState === \"success\" ? (\n                <SuccessContent />\n              ) : recordingState === \"listening\" ? (\n                <Waveform />\n              ) : (\n                \"Transcribing…\"\n              )}\n            </span>",
  "oldString": "            <span data-tauri-drag-region className=\"flex-1 truncate\">\n              {recordingState === \"error\" && recordingError ? (\n                recordingError\n              ) : recordingState === \"success\" ? (\n                <SuccessContent />\n              ) : recordingState === \"listening\" ? (\n                <Waveform />\n              ) : (\n                \"Transcribing…\"\n              )}\n            </span>"
}

Now apply to the `ExpandedPill` transcript preview:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx",
  "newString": "      <div className=\"bg-foreground/10 rounded-md px-2.5 py-2 max-h-20 overflow-y-auto text-[11px] leading-relaxed text-foreground/90 contain-layout-paint\">\n        {transcript || (\n          <span className=\"text-foreground/40 italic\">No transcript yet</span>\n        )}\n      </div>",
  "oldString": "      <div className=\"bg-foreground/10 rounded-md px-2.5 py-2 max-h-20 overflow-y-auto text-[11px] leading-relaxed text-foreground/90\">\n        {transcript || (\n          <span className=\"text-foreground/40 italic\">No transcript yet</span>\n        )}\n      </div>"
}

Now let me verify the changes compile correctly:

🔧 Tool: bash
Input: {
  "command": "npm run lint 2>&1 | head -40",
  "description": "Run linter to verify changes",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm run 2>&1",
  "description": "List available npm scripts",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npx tsc --noEmit 2>&1",
  "description": "TypeScript type check",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
Pre-existing error unrelated to my changes. Let me verify my specific files are clean:

🔧 Tool: bash
Input: {
  "command": "npx tsc --noEmit src/components/pill/Pill.tsx src/components/pill/Waveform.tsx src/components/pill/ExpandedPill.tsx 2>&1",
  "description": "Type check modified files",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
Those are just CLI-only errors (missing `--jsx` flag). Let me verify via the project's build check:

🔧 Tool: bash
Input: {
  "command": "npm run build 2>&1 | tail -20",
  "description": "Run full build to verify changes",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
Pre-existing Dashboard errors only. Let me verify the Tailwind v4 `@utility` directive is correct:

🔧 Tool: bash
Input: {
  "command": "npm list tailwindcss 2>&1",
  "description": "Check Tailwind CSS version",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
Tailwind v4.2.1 — `@utility` is the correct v4 API. Let me verify the build actually processes the CSS correctly:

🔧 Tool: bash
Input: {
  "command": "npx vite build 2>&1 | tail -20",
  "description": "Vite build to verify CSS processing",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
The `History.tsx` error is pre-existing. Let me verify my specific changes are correct:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\index.css",
  "limit": 15,
  "offset": 195
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Waveform.tsx",
  "limit": 10,
  "offset": 79
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx",
  "limit": 5,
  "offset": 99
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx",
  "limit": 7,
  "offset": 46
}
<summary>
## Summary: CSS Containment for Independent UI Panels

### Changes Implemented
- Added `@utility contain-layout-paint { contain: layout paint; }` to `src/index.css` as a Tailwind v4 custom utility
- Applied `contain-layout-paint` to the **Waveform** canvas (60fps animation via `requestAnimationFrame`)
- Applied `contain-layout-paint` to the **status bar** dynamic content span in `Pill.tsx` (state transitions: idle/listening/processing/success/error)
- Applied `contain-layout-paint` to the **transcript preview** in `ExpandedPill.tsx` (updates per transcription result)

### Files Modified
- `src/index.css` — added `@utility contain-layout-paint`
- `src/components/pill/Waveform.tsx` — added `contain-layout-paint` class
- `src/components/pill/Pill.tsx` — added `contain-layout-paint` class
- `src/components/pill/ExpandedPill.tsx` — added `contain-layout-paint` class

### Notes for Developer
- Build has pre-existing errors in `Dashboard.tsx` and `History.tsx` unrelated to this feature
- The `@utility` directive is Tailwind v4's native API for custom utilities
- `contain: layout paint` scopes layout and paint recalculations to each panel, preventing cascade into the rest of the document tree during rapid updates
</summary>