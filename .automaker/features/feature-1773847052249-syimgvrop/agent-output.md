
🔧 Tool: glob
Input: {
  "pattern": ".storybook/**/*"
}

🔧 Tool: glob
Input: {
  "pattern": "**/*.stories.tsx"
}

🔧 Tool: glob
Input: {
  "pattern": "src/**/*.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\package.json"
}

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
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\app-store.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\types\\index.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\vite.config.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\tsconfig.json"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\ExpandedPill.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\ui\\button.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\ExpandedPill.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\Dashboard.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\pages\\History.tsx"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "in_progress"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "pending"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\tauri.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\dashboard-store.ts"
}

🔧 Tool: glob
Input: {
  "pattern": "src/stores/*.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\settings-store.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\filler-words-store.ts"
}

🔧 Tool: bash
Input: {
  "command": "npm install -D storybook @storybook/react @storybook/react-vite @storybook/addon-controls @storybook/addon-docs @storybook/addon-essentials @storybook/blocks",
  "description": "Install Storybook dependencies",
  "timeout": 180000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm ls storybook",
  "description": "Check installed Storybook version",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm install -D storybook@8 @storybook/react@8 @storybook/react-vite@8 @storybook/addon-essentials@8 @storybook/addon-controls@8 @storybook/blocks@8",
  "description": "Install Storybook 8.x dependencies",
  "timeout": 180000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm install -D storybook@10 @storybook/react@10 @storybook/react-vite@10 @storybook/addon-essentials@10 @storybook/addon-controls@10 @storybook/blocks@10",
  "description": "Install Storybook 10.x with Vite 7 support",
  "timeout": 180000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm install -D storybook@10 @storybook/react@10 @storybook/react-vite@10 @storybook/addon-essentials@10",
  "description": "Install Storybook 10.x core packages",
  "timeout": 180000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm view @storybook/addon-essentials versions --json | tail -20",
  "description": "Check available addon-essentials versions",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm view storybook versions --json | tail -30",
  "description": "Check available storybook versions",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm install -D storybook@10.3.0",
  "description": "Install Storybook 10.3.0",
  "timeout": 180000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "in_progress"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "pending"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: bash
Input: {
  "command": "mkdir -p .storybook",
  "description": "Create .storybook directory",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "mkdir .storybook",
  "description": "Create .storybook directory",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\.storybook"
}

🔧 Tool: write
Input: {
  "content": "import type { StorybookConfig } from \"@storybook/react-vite\";\n\nconst config: StorybookConfig = {\n  stories: [\"../src/**/*.mdx\", \"../src/**/*.stories.@(js|jsx|mjs|ts|tsx)\"],\n  addons: [\n    \"@storybook/addon-essentials\",\n    \"@storybook/addon-controls\",\n  ],\n  framework: {\n    name: \"@storybook/react-vite\",\n    options: {},\n  },\n  docs: {\n    autodocs: \"tag\",\n  },\n  typescript: {\n    check: false,\n  },\n};\n\nexport default config;\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\.storybook\\main.ts"
}

🔧 Tool: bash
Input: {
  "command": "npm ls @storybook/react-vite",
  "description": "Check react-vite package",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm install -D @storybook/react-vite",
  "description": "Install Storybook React Vite package",
  "timeout": 120000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm install -D @storybook/addon-essentials @storybook/addon-controls",
  "description": "Install Storybook addon packages",
  "timeout": 120000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm view @storybook/addon-essentials@latest version",
  "description": "Check latest addon-essentials version",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm view @storybook/addon-essentials versions --json | tail -10",
  "description": "Check all addon-essentials versions",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\.storybook\\main.ts",
  "newString": "import type { StorybookConfig } from \"@storybook/react-vite\";\n\nconst config: StorybookConfig = {\n  stories: [\"../src/**/*.mdx\", \"../src/**/*.stories.@(js|jsx|mjs|ts|tsx)\"],\n  addons: [],\n  framework: {\n    name: \"@storybook/react-vite\",\n    options: {},\n  },\n  docs: {\n    autodocs: \"tag\",\n  },\n  typescript: {\n    check: false,\n  },\n};\n\nexport default config;",
  "oldString": "import type { StorybookConfig } from \"@storybook/react-vite\";\n\nconst config: StorybookConfig = {\n  stories: [\"../src/**/*.mdx\", \"../src/**/*.stories.@(js|jsx|mjs|ts|tsx)\"],\n  addons: [\n    \"@storybook/addon-essentials\",\n    \"@storybook/addon-controls\",\n  ],\n  framework: {\n    name: \"@storybook/react-vite\",\n    options: {},\n  },\n  docs: {\n    autodocs: \"tag\",\n  },\n  typescript: {\n    check: false,\n  },\n};\n\nexport default config;"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\.storybook\\main.ts",
  "newString": "import type { StorybookConfig } from \"@storybook/react-vite\";\n\nconst config: StorybookConfig = {\n  stories: [\"../src/**/*.mdx\", \"../src/**/*.stories.@(js|jsx|mjs|ts|tsx)\"],\n  addons: [],\n  framework: {\n    name: \"@storybook/react-vite\",\n    options: {},\n  },\n  typescript: {\n    check: false,\n  },\n};\n\nexport default config;",
  "oldString": "import type { StorybookConfig } from \"@storybook/react-vite\";\n\nconst config: StorybookConfig = {\n  stories: [\"../src/**/*.mdx\", \"../src/**/*.stories.@(js|jsx|mjs|ts|tsx)\"],\n  addons: [],\n  framework: {\n    name: \"@storybook/react-vite\",\n    options: {},\n  },\n  docs: {\n    autodocs: \"tag\",\n  },\n  typescript: {\n    check: false,\n  },\n};\n\nexport default config;"
}

🔧 Tool: write
Input: {
  "content": "import type { Preview } from \"@storybook/react\";\nimport \"../src/index.css\";\n\nconst preview: Preview = {\n  parameters: {\n    controls: {\n      matchers: {\n        color: /(background|color)$/i,\n        date: /Date$/i,\n      },\n    },\n    backgrounds: {\n      default: \"dark\",\n      values: [\n        { name: \"dark\", value: \"#09090b\" },\n        { name: \"light\", value: \"#ffffff\" },\n      ],\n    },\n  },\n  tags: [\"autodocs\"],\n};\n\nexport default preview;\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\.storybook\\preview.ts"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "in_progress"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "pending"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: bash
Input: {
  "command": "mkdir src/mocks",
  "description": "Create mocks directory",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "if exist src\\mocks echo Directory exists || mkdir src\\mocks",
  "description": "Create mocks directory on Windows",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "powershell -Command \"New-Item -ItemType Directory -Force -Path 'src/mocks'\"",
  "description": "Create mocks directory with PowerShell",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: write
Input: {
  "content": "import type {\n  AmbiguousTerm,\n  CorrectionRule,\n  CorrectionStat,\n  DailyStats,\n  DashboardStats,\n  DateRange,\n  DeviceInfo,\n  DictionaryEntry,\n  FillerStat,\n  FillerWord,\n  LanguageBreakdown,\n  LogEntry,\n  ModelInfo,\n  RecordingState,\n  Session,\n  SessionFilter,\n  SessionWithSegments,\n  Settings,\n  TimeseriesPoint,\n  TranscriptionResult,\n  WpmPoint,\n} from \"../types\";\n\nexport const mockDevices: DeviceInfo[] = [\n  { id: \"mic-1\", name: \"Built-in Microphone\", isDefault: true },\n  { id: \"mic-2\", name: \"USB Headset Microphone\", isDefault: false },\n];\n\nexport const mockSettings: Settings = {\n  \"transcription.default_language\": \"auto\",\n  \"transcription.default_model\": \"ggml-base\",\n  \"output.mode\": \"clipboard\",\n  \"recording.hotkey\": \"CommandOrControl+Shift+D\",\n  \"recording.auto_gain\": \"true\",\n  \"recording.noise_reduction\": \"true\",\n  \"ui.pill_opacity\": \"100\",\n};\n\nexport const mockSession: Session = {\n  id: \"session-1\",\n  startedAt: \"2026-03-18T10:00:00Z\",\n  endedAt: \"2026-03-18T10:01:30Z\",\n  durationMs: 90000,\n  language: \"de\",\n  modelId: \"ggml-base\",\n  triggerType: \"hotkey\",\n  inputDeviceId: \"mic-1\",\n  rawText: \"Das ist ein Test. Hallo Welt. Wie geht es dir?\",\n  cleanedText: \"Das ist ein Test. Hallo Welt. Wie geht es dir?\",\n  wordCount: 11,\n  charCount: 51,\n  avgConfidence: 0.92,\n  estimatedWpm: 140,\n  outputMode: \"clipboard\",\n  insertedSuccessfully: true,\n  createdAt: \"2026-03-18T10:00:00Z\",\n  reprocessedCount: 0,\n};\n\nexport const mockSessions: Session[] = [\n  mockSession,\n  {\n    ...mockSession,\n    id: \"session-2\",\n    startedAt: \"2026-03-17T14:30:00Z\",\n    endedAt: \"2026-03-17T14:31:00Z\",\n    durationMs: 60000,\n    language: \"en\",\n    rawText: \"Hello world, this is a test recording.\",\n    cleanedText: \"Hello world, this is a test recording.\",\n    wordCount: 8,\n    charCount: 44,\n    avgConfidence: 0.88,\n    estimatedWpm: 130,\n  },\n  {\n    ...mockSession,\n    id: \"session-3\",\n    startedAt: \"2026-03-16T09:15:00Z\",\n    endedAt: \"2026-03-16T09:16:45Z\",\n    durationMs: 105000,\n    language: \"de\",\n    rawText: \"Mehrsprachige Aufnahme mit verschiedenen Wörtern.\",\n    cleanedText: \"Mehrsprachige Aufnahme mit verschiedenen Wörtern.\",\n    wordCount: 6,\n    charCount: 52,\n    avgConfidence: 0.85,\n    estimatedWpm: 125,\n  },\n];\n\nexport const mockTranscription: TranscriptionResult = {\n  rawText: \"Das ist ein Test. Hallo Welt. Wie geht es dir?\",\n  cleanedText: \"Das ist ein Test. Hallo Welt. Wie geht es dir?\",\n  segments: [\n    { startMs: 0, endMs: 2000, text: \"Das ist ein Test.\", confidence: 0.95 },\n    { startMs: 2000, endMs: 4000, text: \"Hallo Welt.\", confidence: 0.92 },\n    { startMs: 4000, endMs: 6000, text: \"Wie geht es dir?\", confidence: 0.88 },\n  ],\n  language: \"de\",\n  modelId: \"ggml-base\",\n  durationMs: 6000,\n  output: { mode: \"clipboard\", success: true },\n};\n\nexport const mockDashboardStats: DashboardStats = {\n  totalWordCount: 15420,\n  totalSessionCount: 142,\n  avgWpm: 132,\n  totalDurationMs: 54000000,\n  languageCounts: [\n    { language: \"de\", count: 98 },\n    { language: \"en\", count: 44 },\n  ],\n  topModels: [\n    {\n      modelId: \"ggml-base\",\n      sessionCount: 85,\n      totalWordCount: 9800,\n      totalDurationMs: 32000000,\n      avgWpm: 135,\n    },\n    {\n      modelId: \"ggml-small\",\n      sessionCount: 42,\n      totalWordCount: 4200,\n      totalDurationMs: 15000000,\n      avgWpm: 128,\n    },\n    {\n      modelId: \"ggml-medium\",\n      sessionCount: 15,\n      totalWordCount: 1420,\n      totalDurationMs: 7000000,\n      avgWpm: 140,\n    },\n  ],\n};\n\nexport const mockTimeseries: TimeseriesPoint[] = [\n  { date: \"2026-03-11\", wordCount: 120, sessionCount: 2 },\n  { date: \"2026-03-12\", wordCount: 250, sessionCount: 4 },\n  { date: \"2026-03-13\", wordCount: 180, sessionCount: 3 },\n  { date: \"2026-03-14\", wordCount: 320, sessionCount: 5 },\n  { date: \"2026-03-15\", wordCount: 410, sessionCount: 6 },\n  { date: \"2026-03-16\", wordCount: 280, sessionCount: 4 },\n  { date: \"2026-03-17\", wordCount: 350, sessionCount: 5 },\n  { date: \"2026-03-18\", wordCount: 190, sessionCount: 3 },\n];\n\nexport const mockLanguageBreakdown: LanguageBreakdown[] = [\n  { language: \"de\", sessionCount: 85, wordCount: 10200, durationMs: 36000000 },\n  { language: \"en\", sessionCount: 57, wordCount: 5220, durationMs: 18000000 },\n];\n\nexport const mockCorrectionStats: CorrectionStat[] = [\n  { sourcePhrase: \"Sprachausgabe\", targetPhrase: \"Sprachausgabe\", usageCount: 24 },\n  { sourcePhrase: \"das\", targetPhrase: \"dass\", usageCount: 18 },\n  { sourcePhrase: \"ne\", targetPhrase: \"ne\", usageCount: 15 },\n  { sourcePhrase: \"Audio Datei\", targetPhrase: \"Audiodatei\", usageCount: 12 },\n  { sourcePhrase: \"Local Voice\", targetPhrase: \"LocalVoice\", usageCount: 10 },\n];\n\nexport const mockWpmTrend: WpmPoint[] = [\n  { date: \"2026-03-11\", avgWpm: 125, sessionCount: 2 },\n  { date: \"2026-03-12\", avgWpm: 130, sessionCount: 4 },\n  { date: \"2026-03-13\", avgWpm: 128, sessionCount: 3 },\n  { date: \"2026-03-14\", avgWpm: 135, sessionCount: 5 },\n  { date: \"2026-03-15\", avgWpm: 140, sessionCount: 6 },\n  { date: \"2026-03-16\", avgWpm: 132, sessionCount: 4 },\n  { date: \"2026-03-17\", avgWpm: 138, sessionCount: 5 },\n  { date: \"2026-03-18\", avgWpm: 135, sessionCount: 3 },\n];\n\nexport const mockModels: ModelInfo[] = [\n  {\n    key: \"ggml-base\",\n    displayName: \"Base (EN)\",\n    languageScope: \"en-only\",\n    fileSizeBytes: 140000000,\n    installed: true,\n    isDefaultForDe: false,\n    isDefaultForEn: true,\n    defaultForLanguages: [\"en\"],\n    localPath: \"~/.localvoice/models/ggml-base.bin\",\n    installedAt: \"2026-01-15T10:00:00Z\",\n    description: \"Fast and lightweight, English only\",\n    speed: \"fastest\",\n    accuracy: \"low\",\n    category: \"standard\",\n    recommendedFor: \"Quick dictation, English\",\n  },\n  {\n    key: \"ggml-small\",\n    displayName: \"Small (Multilingual)\",\n    languageScope: \"multilingual\",\n    fileSizeBytes: 480000000,\n    installed: true,\n    isDefaultForDe: true,\n    isDefaultForEn: false,\n    defaultForLanguages: [\"de\", \"fr\", \"es\"],\n    localPath: \"~/.localvoice/models/ggml-small.bin\",\n    installedAt: \"2026-02-01T10:00:00Z\",\n    description: \"Good balance of speed and accuracy\",\n    speed: \"fast\",\n    accuracy: \"medium\",\n    category: \"standard\",\n    recommendedFor: \"German, French, Spanish\",\n  },\n  {\n    key: \"ggml-medium\",\n    displayName: \"Medium (Multilingual)\",\n    languageScope: \"multilingual\",\n    fileSizeBytes: 1500000000,\n    installed: true,\n    isDefaultForDe: false,\n    isDefaultForEn: false,\n    defaultForLanguages: [],\n    localPath: \"~/.localvoice/models/ggml-medium.bin\",\n    installedAt: \"2026-02-15T10:00:00Z\",\n    description: \"Higher accuracy, slower processing\",\n    speed: \"balanced\",\n    accuracy: \"good\",\n    category: \"standard\",\n    recommendedFor: \"Accurate transcription\",\n  },\n  {\n    key: \"ggml-large-v3\",\n    displayName: \"Large v3 (Multilingual)\",\n    languageScope: \"multilingual\",\n    fileSizeBytes: 2900000000,\n    installed: false,\n    isDefaultForDe: false,\n    isDefaultForEn: false,\n    defaultForLanguages: [],\n    description: \"Best accuracy, slowest processing\",\n    speed: \"slow\",\n    accuracy: \"great\",\n    category: \"large\",\n    recommendedFor: \"Maximum accuracy\",\n  },\n];\n\nexport const mockDictionaryEntries: DictionaryEntry[] = [\n  {\n    id: \"dict-1\",\n    phrase: \"LocalVoice\",\n    normalizedPhrase: \"localvoice\",\n    language: \"en\",\n    entryType: \"product\",\n    notes: \"Product name\",\n    createdAt: \"2026-02-01T10:00:00Z\",\n    updatedAt: \"2026-02-01T10:00:00Z\",\n  },\n  {\n    id: \"dict-2\",\n    phrase: \"Audiodatei\",\n    normalizedPhrase: \"audiodatei\",\n    language: \"de\",\n    entryType: \"term\",\n    notes: \"Combined word\",\n    createdAt: \"2026-02-05T10:00:00Z\",\n    updatedAt: \"2026-02-05T10:00:00Z\",\n  },\n];\n\nexport const mockCorrectionRules: CorrectionRule[] = [\n  {\n    id: \"rule-1\",\n    sourcePhrase: \"Sprachausgabe\",\n    normalizedSourcePhrase: \"sprachausgabe\",\n    targetPhrase: \"Sprachausgabe\",\n    language: \"de\",\n    ruleMode: \"manual\",\n    isActive: true,\n    autoApply: true,\n    usageCount: 24,\n    lastUsedAt: \"2026-03-18T09:30:00Z\",\n    createdAt: \"2026-02-01T10:00:00Z\",\n    updatedAt: \"2026-02-01T10:00:00Z\",\n  },\n  {\n    id: \"rule-2\",\n    sourcePhrase: \"das\",\n    normalizedSourcePhrase: \"das\",\n    targetPhrase: \"dass\",\n    language: \"de\",\n    ruleMode: \"suggested\",\n    isActive: true,\n    autoApply: false,\n    usageCount: 18,\n    lastUsedAt: \"2026-03-17T15:00:00Z\",\n    createdAt: \"2026-02-10T10:00:00Z\",\n    updatedAt: \"2026-02-10T10:00:00Z\",\n  },\n];\n\nexport const mockAmbiguousTerms: AmbiguousTerm[] = [\n  {\n    id: \"amb-1\",\n    phrase: \"Auto\",\n    normalizedPhrase: \"auto\",\n    language: \"de\",\n    occurrences: 12,\n    avgConfidence: 0.65,\n    lastSeenAt: \"2026-03-18T09:00:00Z\",\n    suggestedTarget: \"Auto\",\n    dismissed: false,\n    createdAt: \"2026-03-01T10:00:00Z\",\n    updatedAt: \"2026-03-18T09:00:00Z\",\n  },\n  {\n    id: \"amb-2\",\n    phrase: \"Bau\",\n    normalizedPhrase: \"bau\",\n    language: \"de\",\n    occurrences: 8,\n    avgConfidence: 0.58,\n    lastSeenAt: \"2026-03-17T14:00:00Z\",\n    suggestedTarget: \"bau\",\n    dismissed: false,\n    createdAt: \"2026-03-05T10:00:00Z\",\n    updatedAt: \"2026-03-17T14:00:00Z\",\n  },\n];\n\nexport const mockFillerWords: FillerWord[] = [\n  { id: \"filler-1\", word: \"äh\", language: \"de\", isDefault: true, createdAt: \"2026-01-01T00:00:00Z\" },\n  { id: \"filler-2\", word: \"ähhm\", language: \"de\", isDefault: true, createdAt: \"2026-01-01T00:00:00Z\" },\n  { id: \"filler-3\", word: \"uh\", language: \"en\", isDefault: true, createdAt: \"2026-01-01T00:00:00Z\" },\n  { id: \"filler-4\", word: \"um\", language: \"en\", isDefault: true, createdAt: \"2026-01-01T00:00:00Z\" },\n];\n\nexport const mockFillerStats: FillerStat[] = [\n  { word: \"äh\", language: \"de\", count: 45, lastRemovedAt: \"2026-03-18T09:30:00Z\" },\n  { word: \"ähhm\", language: \"de\", count: 23, lastRemovedAt: \"2026-03-18T09:00:00Z\" },\n  { word: \"um\", language: \"en\", count: 18, lastRemovedAt: \"2026-03-17T15:00:00Z\" },\n  { word: \"uh\", language: \"en\", count: 12, lastRemovedAt: \"2026-03-17T14:00:00Z\" },\n];\n\nexport const mockLogs: LogEntry[] = [\n  { id: \"log-1\", level: \"info\", area: \"recording\", message: \"Recording started\", createdAt: \"2026-03-18T10:00:00Z\" },\n  { id: \"log-2\", level: \"info\", area: \"recording\", message: \"Recording stopped\", createdAt: \"2026-03-18T10:01:30Z\" },\n  { id: \"log-3\", level: \"info\", area: \"transcription\", message: \"Transcription completed\", createdAt: \"2026-03-18T10:01:35Z\" },\n  { id: \"log-4\", level: \"debug\", area: \"audio\", message: \"Audio level: 0.75\", createdAt: \"2026-03-18T10:00:30Z\" },\n  { id: \"log-5\", level: \"warn\", area: \"audio\", message: \"High noise detected\", createdAt: \"2026-03-18T10:00:45Z\" },\n];\n\nconst mockSessionDetail: SessionWithSegments = {\n  session: mockSession,\n  segments: [\n    { id: \"seg-1\", sessionId: \"session-1\", startMs: 0, endMs: 2000, text: \"Das ist ein Test.\", confidence: 0.95, segmentIndex: 0 },\n    { id: \"seg-2\", sessionId: \"session-1\", startMs: 2000, endMs: 4000, text: \"Hallo Welt.\", confidence: 0.92, segmentIndex: 1 },\n    { id: \"seg-3\", sessionId: \"session-1\", startMs: 4000, endMs: 6000, text: \"Wie geht es dir?\", confidence: 0.88, segmentIndex: 2 },\n  ],\n};\n\nexport const mockInvoke = async (cmd: string, args?: Record<string, unknown>): Promise<unknown> => {\n  await new Promise((resolve) => setTimeout(resolve, 50));\n\n  switch (cmd) {\n    case \"get_settings\":\n      return mockSettings;\n    case \"update_setting\":\n      return undefined;\n    case \"reset_settings\":\n      return undefined;\n    case \"update_shortcut\":\n      return undefined;\n    case \"show_pill\":\n      return undefined;\n    case \"hide_pill\":\n      return undefined;\n    case \"open_main_window\":\n      return undefined;\n    case \"expand_pill\":\n      return undefined;\n    case \"collapse_pill\":\n      return undefined;\n    case \"set_pill_position\":\n      return undefined;\n    case \"reprocess_session\":\n      return mockSessionDetail;\n    case \"start_recording\":\n      return undefined;\n    case \"stop_recording\":\n      return \"session-mock\";\n    case \"cancel_recording\":\n      return undefined;\n    case \"get_recording_state\":\n      return \"idle\" as RecordingState;\n    case \"list_input_devices\":\n      return mockDevices;\n    case \"transcribe_last_recording\":\n      return mockTranscription;\n    case \"get_last_transcription\":\n      return mockTranscription;\n    case \"list_sessions\": {\n      const filter = (args?.filter as SessionFilter) ?? {};\n      let result = [...mockSessions];\n      if (filter.query) {\n        result = result.filter((s) => s.cleanedText.toLowerCase().includes(filter.query!.toLowerCase()));\n      }\n      if (filter.language) {\n        result = result.filter((s) => s.language === filter.language);\n      }\n      return result;\n    }\n    case \"get_session\":\n      return mockSessionDetail;\n    case \"delete_session\":\n      return undefined;\n    case \"export_sessions\":\n      return \"exported/path.txt\";\n    case \"get_dashboard_stats\":\n      return mockDashboardStats;\n    case \"get_usage_timeseries\":\n      return mockTimeseries;\n    case \"get_language_breakdown\":\n      return mockLanguageBreakdown;\n    case \"get_correction_stats\":\n      return mockCorrectionStats;\n    case \"get_wpm_trend\":\n      return mockWpmTrend;\n    case \"get_daily_comparison\":\n      return [\n        { date: \"2026-03-15\", sessionCount: 6, wordCount: 410, durationMs: 18000000, avgWpm: 140 } as DailyStats,\n        { date: \"2026-03-16\", sessionCount: 4, wordCount: 280, durationMs: 10500000, avgWpm: 132 } as DailyStats,\n      ];\n    case \"list_available_models\":\n      return mockModels;\n    case \"download_model\":\n      return undefined;\n    case \"delete_model\":\n      return undefined;\n    case \"set_default_model\":\n      return undefined;\n    case \"list_dictionary_entries\":\n      return mockDictionaryEntries;\n    case \"create_dictionary_entry\":\n      return mockDictionaryEntries[0];\n    case \"update_dictionary_entry\":\n      return undefined;\n    case \"delete_dictionary_entry\":\n      return undefined;\n    case \"list_correction_rules\":\n      return mockCorrectionRules;\n    case \"create_correction_rule\":\n      return mockCorrectionRules[0];\n    case \"update_correction_rule\":\n      return undefined;\n    case \"delete_correction_rule\":\n      return undefined;\n    case \"list_ambiguous_terms\":\n      return mockAmbiguousTerms;\n    case \"accept_ambiguity_suggestion\":\n      return undefined;\n    case \"dismiss_ambiguity_suggestion\":\n      return undefined;\n    case \"check_first_run\":\n      return false;\n    case \"has_default_model\":\n      return true;\n    case \"set_autostart\":\n      return undefined;\n    case \"get_autostart\":\n      return false;\n    case \"list_logs\":\n      return mockLogs;\n    case \"export_logs\":\n      return undefined;\n    case \"clear_logs\":\n      return undefined;\n    case \"set_logging_enabled\":\n      return undefined;\n    case \"list_filler_words\":\n      return mockFillerWords;\n    case \"add_filler_word\":\n      return mockFillerWords[0];\n    case \"delete_filler_word\":\n      return undefined;\n    case \"reset_filler_words\":\n      return mockFillerWords;\n    case \"get_filler_stats\":\n      return mockFillerStats;\n    case \"get_filler_total_count\":\n      return 98;\n    default:\n      throw new Error(`Unknown mock command: ${cmd}`);\n  }\n};\n\nexport const mockTauri = {\n  invoke: mockInvoke,\n};\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\mocks\\tauri.ts"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\mocks\\tauri.ts",
  "newString": "import type {\n  AmbiguousTerm,\n  CorrectionRule,\n  CorrectionStat,\n  DailyStats,\n  DeviceInfo,\n  DictionaryEntry,\n  FillerStat,\n  FillerWord,\n  LanguageBreakdown,\n  LogEntry,\n  ModelInfo,\n  RecordingState,\n  Session,\n  SessionFilter,\n  SessionWithSegments,\n  Settings,\n  TimeseriesPoint,\n  TranscriptionResult,\n  WpmPoint,\n} from \"../types\";",
  "oldString": "import type {\n  AmbiguousTerm,\n  CorrectionRule,\n  CorrectionStat,\n  DailyStats,\n  DashboardStats,\n  DateRange,\n  DeviceInfo,\n  DictionaryEntry,\n  FillerStat,\n  FillerWord,\n  LanguageBreakdown,\n  LogEntry,\n  ModelInfo,\n  RecordingState,\n  Session,\n  SessionFilter,\n  SessionWithSegments,\n  Settings,\n  TimeseriesPoint,\n  TranscriptionResult,\n  WpmPoint,\n} from \"../types\";"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\mocks\\tauri.ts",
  "limit": 20,
  "offset": 100
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\mocks\\tauri.ts",
  "newString": "import type {\n  AmbiguousTerm,\n  CorrectionRule,\n  CorrectionStat,\n  DailyStats,\n  DashboardStats,\n  DeviceInfo,\n  DictionaryEntry,\n  FillerStat,\n  FillerWord,\n  LanguageBreakdown,\n  LogEntry,\n  ModelInfo,\n  RecordingState,\n  Session,\n  SessionFilter,\n  SessionWithSegments,\n  Settings,\n  TimeseriesPoint,\n  TranscriptionResult,\n  WpmPoint,\n} from \"../types\";",
  "oldString": "import type {\n  AmbiguousTerm,\n  CorrectionRule,\n  CorrectionStat,\n  DailyStats,\n  DeviceInfo,\n  DictionaryEntry,\n  FillerStat,\n  FillerWord,\n  LanguageBreakdown,\n  LogEntry,\n  ModelInfo,\n  RecordingState,\n  Session,\n  SessionFilter,\n  SessionWithSegments,\n  Settings,\n  TimeseriesPoint,\n  TranscriptionResult,\n  WpmPoint,\n} from \"../types\";"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "in_progress"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "pending"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: bash
Input: {
  "command": "powershell -Command \"New-Item -ItemType Directory -Force -Path 'src/stories'\"",
  "description": "Create stories directory",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: write
Input: {
  "content": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { Pill } from \"../components/pill/Pill\";\nimport type { RecordingState } from \"../types\";\nimport { mockTranscription } from \"./tauri\";\n\nconst meta: Meta<typeof Pill> = {\n  title: \"Pill\",\n  component: Pill,\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"centered\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\ntype Story = StoryObj<typeof Pill>;\n\nexport const Idle: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const IdleWithLastTranscription: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Listening: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"listening\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n        audioLevel: 0.7,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Processing: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"processing\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Success: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"success\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const SuccessInsert: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"success\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"insert\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Error: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"error\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: \"Microphone not available\",\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ErrorOutputFailed: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"error\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: false, error: \"Clipboard access denied\" },\n        recordingError: \"Output failed\",\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ExpandedIdle: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { useSettingsStore } = require(\"../stores/settings-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: true,\n      });\n      useSettingsStore.setState({\n        settings: { \"transcription.default_language\": \"auto\" },\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ExpandedListening: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { useSettingsStore } = require(\"../stores/settings-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"listening\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: true,\n        audioLevel: 0.5,\n      });\n      useSettingsStore.setState({\n        settings: { \"transcription.default_language\": \"de\" },\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const AllStates: Story = {\n  parameters: {\n    layout: \"fullscreen\",\n  },\n  render: () => {\n    const { useAppStore } = require(\"../stores/app-store\");\n    const { mockInvoke } = require(\"./tauri\");\n    require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n\n    const states: RecordingState[] = [\"idle\", \"listening\", \"processing\", \"success\", \"error\"];\n\n    return (\n      <div className=\"p-8 space-y-8 bg-zinc-950 min-h-screen\">\n        <h2 className=\"text-xl font-semibold text-white\">Pill States</h2>\n        <div className=\"grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8\">\n          {states.map((state) => {\n            useAppStore.setState({\n              recordingState: state,\n              lastTranscription: state === \"success\" ? mockTranscription : null,\n              lastOutputResult: state === \"success\" ? { mode: \"clipboard\", success: true } : null,\n              recordingError: state === \"error\" ? \"Microphone unavailable\" : null,\n              isPillExpanded: false,\n              audioLevel: state === \"listening\" ? 0.6 : 0,\n            });\n\n            return (\n              <div key={state} className=\"space-y-2\">\n                <h3 className=\"text-sm text-zinc-400 uppercase\">{state}</h3>\n                <div className=\"w-80 h-16 rounded-2xl overflow-hidden shadow-2xl\">\n                  <Pill />\n                </div>\n              </div>\n            );\n          })}\n        </div>\n      </div>\n    );\n  },\n};\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\Pill.stories.tsx"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\Pill.stories.tsx",
  "newString": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { Pill } from \"../components/pill/Pill\";\nimport type { RecordingState } from \"../types\";\nimport { mockTranscription } from \"../mocks/tauri\";\n\nconst meta: Meta<typeof Pill> = {\n  title: \"Pill\",\n  component: Pill,\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"centered\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\ntype Story = StoryObj<typeof Pill>;\n\nexport const Idle: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const IdleWithLastTranscription: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Listening: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"listening\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n        audioLevel: 0.7,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Processing: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"processing\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Success: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"success\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const SuccessInsert: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"success\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"insert\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Error: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"error\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: \"Microphone not available\",\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ErrorOutputFailed: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"error\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: false, error: \"Clipboard access denied\" },\n        recordingError: \"Output failed\",\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ExpandedIdle: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { useSettingsStore } = require(\"../stores/settings-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: true,\n      });\n      useSettingsStore.setState({\n        settings: { \"transcription.default_language\": \"auto\" },\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ExpandedListening: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { useSettingsStore } = require(\"../stores/settings-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"listening\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: true,\n        audioLevel: 0.5,\n      });\n      useSettingsStore.setState({\n        settings: { \"transcription.default_language\": \"de\" },\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const AllStates: Story = {\n  parameters: {\n    layout: \"fullscreen\",\n  },\n  render: () => {\n    const { useAppStore } = require(\"../stores/app-store\");\n    const { mockInvoke } = require(\"../mocks/tauri\");\n    require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n\n    const states: RecordingState[] = [\"idle\", \"listening\", \"processing\", \"success\", \"error\"];\n\n    return (\n      <div className=\"p-8 space-y-8 bg-zinc-950 min-h-screen\">\n        <h2 className=\"text-xl font-semibold text-white\">Pill States</h2>\n        <div className=\"grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8\">\n          {states.map((state) => {\n            useAppStore.setState({\n              recordingState: state,\n              lastTranscription: state === \"success\" ? mockTranscription : null,\n              lastOutputResult: state === \"success\" ? { mode: \"clipboard\", success: true } : null,\n              recordingError: state === \"error\" ? \"Microphone unavailable\" : null,\n              isPillExpanded: false,\n              audioLevel: state === \"listening\" ? 0.6 : 0,\n            });\n\n            return (\n              <div key={state} className=\"space-y-2\">\n                <h3 className=\"text-sm text-zinc-400 uppercase\">{state}</h3>\n                <div className=\"w-80 h-16 rounded-2xl overflow-hidden shadow-2xl\">\n                  <Pill />\n                </div>\n              </div>\n            );\n          })}\n        </div>\n      </div>\n    );\n  },\n};",
  "oldString": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { Pill } from \"../components/pill/Pill\";\nimport type { RecordingState } from \"../types\";\nimport { mockTranscription } from \"./tauri\";\n\nconst meta: Meta<typeof Pill> = {\n  title: \"Pill\",\n  component: Pill,\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"centered\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\ntype Story = StoryObj<typeof Pill>;\n\nexport const Idle: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const IdleWithLastTranscription: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Listening: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"listening\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n        audioLevel: 0.7,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Processing: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"processing\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Success: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"success\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const SuccessInsert: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"success\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"insert\", success: true },\n        recordingError: null,\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Error: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"error\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: \"Microphone not available\",\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ErrorOutputFailed: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"error\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: false, error: \"Clipboard access denied\" },\n        recordingError: \"Output failed\",\n        isPillExpanded: false,\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ExpandedIdle: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { useSettingsStore } = require(\"../stores/settings-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"idle\",\n        lastTranscription: mockTranscription,\n        lastOutputResult: { mode: \"clipboard\", success: true },\n        recordingError: null,\n        isPillExpanded: true,\n      });\n      useSettingsStore.setState({\n        settings: { \"transcription.default_language\": \"auto\" },\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const ExpandedListening: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { useSettingsStore } = require(\"../stores/settings-store\");\n      const { mockInvoke } = require(\"./tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({\n        recordingState: \"listening\",\n        lastTranscription: null,\n        lastOutputResult: null,\n        recordingError: null,\n        isPillExpanded: true,\n        audioLevel: 0.5,\n      });\n      useSettingsStore.setState({\n        settings: { \"transcription.default_language\": \"de\" },\n      });\n      return <Story />;\n    },\n  ],\n};\n\nexport const AllStates: Story = {\n  parameters: {\n    layout: \"fullscreen\",\n  },\n  render: () => {\n    const { useAppStore } = require(\"../stores/app-store\");\n    const { mockInvoke } = require(\"./tauri\");\n    require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n\n    const states: RecordingState[] = [\"idle\", \"listening\", \"processing\", \"success\", \"error\"];\n\n    return (\n      <div className=\"p-8 space-y-8 bg-zinc-950 min-h-screen\">\n        <h2 className=\"text-xl font-semibold text-white\">Pill States</h2>\n        <div className=\"grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8\">\n          {states.map((state) => {\n            useAppStore.setState({\n              recordingState: state,\n              lastTranscription: state === \"success\" ? mockTranscription : null,\n              lastOutputResult: state === \"success\" ? { mode: \"clipboard\", success: true } : null,\n              recordingError: state === \"error\" ? \"Microphone unavailable\" : null,\n              isPillExpanded: false,\n              audioLevel: state === \"listening\" ? 0.6 : 0,\n            });\n\n            return (\n              <div key={state} className=\"space-y-2\">\n                <h3 className=\"text-sm text-zinc-400 uppercase\">{state}</h3>\n                <div className=\"w-80 h-16 rounded-2xl overflow-hidden shadow-2xl\">\n                  <Pill />\n                </div>\n              </div>\n            );\n          })}\n        </div>\n      </div>\n    );\n  },\n};"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "in_progress"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "pending"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: write
Input: {
  "content": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { useEffect, useRef } from \"react\";\nimport { Waveform } from \"../components/pill/Waveform\";\n\nconst meta: Meta<typeof Waveform> = {\n  title: \"Waveform\",\n  component: Waveform,\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"centered\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\ntype Story = StoryObj<typeof Waveform>;\n\nexport const Default: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({ audioLevel: 0.5 });\n      return <Story />;\n    },\n  ],\n};\n\nexport const HighLevel: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({ audioLevel: 0.9 });\n      return <Story />;\n    },\n  ],\n};\n\nexport const LowLevel: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({ audioLevel: 0.2 });\n      return <Story />;\n    },\n  ],\n};\n\nexport const Silent: Story = {\n  decorators: [\n    (Story) => {\n      const { useAppStore } = require(\"../stores/app-store\");\n      const { mockInvoke } = require(\"../mocks/tauri\");\n      require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n      useAppStore.setState({ audioLevel: 0 });\n      return <Story />;\n    },\n  ],\n};\n\nexport const DynamicLevel: Story = {\n  parameters: {\n    layout: \"centered\",\n  },\n  render: () => {\n    const { useAppStore } = require(\"../stores/app-store\");\n    const { mockInvoke } = require(\"../mocks/tauri\");\n    require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n    const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);\n    const phaseRef = useRef(0);\n\n    useEffect(() => {\n      timerRef.current = setInterval(() => {\n        phaseRef.current += 0.1;\n        const level = (Math.sin(phaseRef.current) + 1) / 2;\n        useAppStore.setState({ audioLevel: level });\n      }, 100);\n\n      return () => {\n        if (timerRef.current) clearInterval(timerRef.current);\n      };\n    }, []);\n\n    return (\n      <div className=\"space-y-4\">\n        <h3 className=\"text-sm text-zinc-400\">Dynamic Audio Level</h3>\n        <Waveform />\n      </div>\n    );\n  },\n};\n\nexport const AllLevels: Story = {\n  parameters: {\n    layout: \"fullscreen\",\n  },\n  render: () => {\n    const { useAppStore } = require(\"../stores/app-store\");\n    const { mockInvoke } = require(\"../mocks/tauri\");\n    require(\"@tauri-apps/api/core\").invoke = mockInvoke;\n\n    const levels = [0, 0.2, 0.4, 0.6, 0.8, 1.0];\n\n    return (\n      <div className=\"p-8 space-y-8 bg-zinc-950 min-h-screen\">\n        <h2 className=\"text-xl font-semibold text-white\">Waveform Levels</h2>\n        <div className=\"grid grid-cols-2 md:grid-cols-3 gap-8\">\n          {levels.map((level) => {\n            useAppStore.setState({ audioLevel: level });\n            return (\n              <div key={level} className=\"space-y-2\">\n                <h3 className=\"text-sm text-zinc-400\">{Math.round(level * 100)}%</h3>\n                <div className=\"bg-zinc-900 p-4 rounded-xl\">\n                  <Waveform />\n                </div>\n              </div>\n            );\n          })}\n        </div>\n      </div>\n    );\n  },\n};\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\Waveform.stories.tsx"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "in_progress"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "pending"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "pending"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: write
Input: {
  "content": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { memo, useMemo } from \"react\";\nimport {\n  LineChart,\n  Line,\n  XAxis,\n  YAxis,\n  Tooltip,\n  ResponsiveContainer,\n  PieChart,\n  Pie,\n  Cell,\n  BarChart,\n  Bar,\n} from \"recharts\";\nimport type {\n  TimeseriesPoint,\n  LanguageBreakdown,\n  WpmPoint,\n  CorrectionStat,\n} from \"../types\";\nimport {\n  mockTimeseries,\n  mockLanguageBreakdown,\n  mockWpmTrend,\n  mockCorrectionStats,\n} from \"../mocks/tauri\";\n\nconst PIE_COLORS = [\"#60a5fa\", \"#34d399\", \"#fbbf24\", \"#f87171\", \"#a78bfa\", \"#fb923c\"];\nconst TOOLTIP_STYLE = {\n  background: \"var(--color-card)\",\n  border: \"1px solid var(--color-border)\",\n  borderRadius: 8,\n  color: \"var(--color-foreground)\",\n  fontSize: 12,\n};\n\nconst meta: Meta = {\n  title: \"Dashboard/Charts\",\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"padded\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\n\nexport const WordsOverTimeChart: Story = {\n  render: () => {\n    return (\n      <div className=\"bg-card rounded-xl border border-border p-5\">\n        <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n          Words over time\n        </h2>\n        <ResponsiveContainer width=\"100%\" height={200}>\n          <LineChart data={mockTimeseries} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>\n            <XAxis\n              dataKey=\"date\"\n              tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n              tickFormatter={(v) => {\n                const d = new Date(v + \"T00:00:00\");\n                return d.toLocaleDateString(undefined, { month: \"short\", day: \"numeric\" });\n              }}\n              tickLine={false}\n              axisLine={false}\n              interval=\"preserveStartEnd\"\n            />\n            <YAxis\n              tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n              tickLine={false}\n              axisLine={false}\n              allowDecimals={false}\n            />\n            <Tooltip\n              contentStyle={TOOLTIP_STYLE}\n              formatter={(v) => [(v as number).toLocaleString(), \"Words\"]}\n              labelFormatter={(l) => l}\n            />\n            <Line\n              type=\"monotone\"\n              dataKey=\"wordCount\"\n              stroke=\"#60a5fa\"\n              strokeWidth={2}\n              dot={false}\n              activeDot={{ r: 4, fill: \"#60a5fa\" }}\n            />\n          </LineChart>\n        </ResponsiveContainer>\n      </div>\n    );\n  },\n};\n\nexport const LanguagePieChart: Story = {\n  render: () => {\n    const total = useMemo(\n      () => mockLanguageBreakdown.reduce((sum, d) => sum + d.wordCount, 0),\n      []\n    );\n    const chartData = useMemo(\n      () =>\n        mockLanguageBreakdown.map((d) => ({\n          name: d.language.toUpperCase(),\n          value: d.wordCount,\n          sessions: d.sessionCount,\n        })),\n      []\n    );\n\n    return (\n      <div className=\"bg-card rounded-xl border border-border p-5\">\n        <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n          Languages\n        </h2>\n        <div className=\"flex flex-col items-center gap-3\">\n          <ResponsiveContainer width=\"100%\" height={160}>\n            <PieChart>\n              <Pie\n                data={chartData}\n                dataKey=\"value\"\n                nameKey=\"name\"\n                cx=\"50%\"\n                cy=\"50%\"\n                innerRadius={40}\n                outerRadius={70}\n                paddingAngle={2}\n                strokeWidth={0}\n              >\n                {chartData.map((_, i) => (\n                  <Cell key={i} fill={PIE_COLORS[i % PIE_COLORS.length]} />\n                ))}\n              </Pie>\n              <Tooltip\n                contentStyle={TOOLTIP_STYLE}\n                formatter={(v) => [(v as number).toLocaleString(), \"Words\"]}\n              />\n            </PieChart>\n          </ResponsiveContainer>\n          <div className=\"flex flex-wrap justify-center gap-x-4 gap-y-1\">\n            {chartData.map((d, i) => {\n              const pct = total > 0 ? Math.round((d.value / total) * 100) : 0;\n              return (\n                <span\n                  key={d.name}\n                  className=\"flex items-center gap-1.5 text-xs text-muted-foreground\"\n                >\n                  <span\n                    className=\"w-2 h-2 rounded-full shrink-0\"\n                    style={{ background: PIE_COLORS[i % PIE_COLORS.length] }}\n                  />\n                  {d.name} {pct}%\n                </span>\n              );\n            })}\n          </div>\n        </div>\n      </div>\n    );\n  },\n};\n\nexport const WpmTrendChart: Story = {\n  render: () => {\n    return (\n      <div className=\"bg-card rounded-xl border border-border p-5\">\n        <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n          WPM trend\n        </h2>\n        <ResponsiveContainer width=\"100%\" height={200}>\n          <LineChart data={mockWpmTrend} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>\n            <XAxis\n              dataKey=\"date\"\n              tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n              tickFormatter={(v) => {\n                const d = new Date(v + \"T00:00:00\");\n                return d.toLocaleDateString(undefined, { month: \"short\", day: \"numeric\" });\n              }}\n              tickLine={false}\n              axisLine={false}\n              interval=\"preserveStartEnd\"\n            />\n            <YAxis\n              tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n              tickLine={false}\n              axisLine={false}\n              allowDecimals={false}\n            />\n            <Tooltip\n              contentStyle={TOOLTIP_STYLE}\n              formatter={(v) => [`${Math.round(v as number)} wpm`, \"Avg WPM\"]}\n              labelFormatter={(l) => l}\n            />\n            <Line\n              type=\"monotone\"\n              dataKey=\"avgWpm\"\n              stroke=\"#34d399\"\n              strokeWidth={2}\n              dot={false}\n              activeDot={{ r: 4, fill: \"#34d399\" }}\n            />\n          </LineChart>\n        </ResponsiveContainer>\n      </div>\n    );\n  },\n};\n\nexport const CorrectionFrequencyChart: Story = {\n  render: () => {\n    const maxCount = useMemo(\n      () => Math.max(...mockCorrectionStats.map((d) => d.usageCount), 1),\n      []\n    );\n    const chartData = useMemo(\n      () =>\n        mockCorrectionStats.slice(0, 10).map((d) => ({\n          name: `${d.sourcePhrase} → ${d.targetPhrase}`,\n          count: d.usageCount,\n        })),\n      []\n    );\n\n    return (\n      <div className=\"bg-card rounded-xl border border-border p-5\">\n        <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n          Top corrections\n        </h2>\n        <ResponsiveContainer width=\"100%\" height={200}>\n          <BarChart data={chartData} layout=\"vertical\" margin={{ left: 0, right: 8 }}>\n            <XAxis type=\"number\" hide />\n            <YAxis\n              type=\"category\"\n              dataKey=\"name\"\n              tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 10 }}\n              width={120}\n              tickLine={false}\n              axisLine={false}\n            />\n            <Tooltip\n              contentStyle={TOOLTIP_STYLE}\n              formatter={(v) => [`${v}x`, \"Used\"]}\n            />\n            <Bar dataKey=\"count\" fill=\"#fbbf24\" radius={[0, 4, 4, 0]} barSize={14} />\n          </BarChart>\n        </ResponsiveContainer>\n      </div>\n    );\n  },\n};\n\nexport const StatCards: Story = {\n  render: () => {\n    return (\n      <div className=\"bg-card rounded-xl border border-border p-5\">\n        <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n          Stat Cards\n        </h2>\n        <div className=\"grid grid-cols-2 xl:grid-cols-4 gap-4\">\n          <StatCard label=\"Total Words\" value=\"15,420\" />\n          <StatCard label=\"Sessions\" value=\"142\" />\n          <StatCard label=\"Avg WPM\" value=\"132 wpm\" />\n          <StatCard label=\"Recording Time\" value=\"15:00\" />\n        </div>\n      </div>\n    );\n  },\n};\n\nconst StatCard = memo(function StatCard({ label, value }: { label: string; value: string }) {\n  return (\n    <div className=\"bg-card border border-border rounded-xl p-5\">\n      <p className=\"text-xs text-muted-foreground mb-1\">{label}</p>\n      <p className=\"text-2xl font-semibold text-foreground tabular-nums\">{value}</p>\n    </div>\n  );\n});\n\nexport const DashboardGrid: Story = {\n  parameters: {\n    layout: \"fullscreen\",\n  },\n  render: () => {\n    return (\n      <div className=\"p-8 space-y-6 bg-zinc-950 min-h-screen\">\n        <div className=\"flex items-center justify-between gap-4\">\n          <h1 className=\"text-2xl font-semibold text-foreground\">Dashboard</h1>\n          <RangeSelector />\n        </div>\n\n        <StatCards />\n\n        <div className=\"grid grid-cols-1 xl:grid-cols-3 gap-4\">\n          <div className=\"xl:col-span-2\">\n            <WordsOverTimeChart />\n          </div>\n          <LanguagePieChart />\n        </div>\n\n        <div className=\"grid grid-cols-1 xl:grid-cols-2 gap-4\">\n          <WpmTrendChart />\n          <CorrectionFrequencyChart />\n        </div>\n      </div>\n    );\n  },\n};\n\nconst RANGE_LABELS = {\n  \"7d\": \"7 days\",\n  \"30d\": \"30 days\",\n  all: \"All time\",\n} as const;\n\nfunction RangeSelector() {\n  return (\n    <div className=\"flex gap-1 bg-muted rounded-lg p-1\">\n      {(Object.keys(RANGE_LABELS) as Array<keyof typeof RANGE_LABELS>).map((r) => (\n        <button\n          key={r}\n          className=\"px-3 py-1 text-xs rounded-md transition-colors bg-accent text-foreground\"\n        >\n          {RANGE_LABELS[r]}\n        </button>\n      ))}\n    </div>\n  );\n}\n\ntype Story = StoryObj;\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\Dashboard.stories.tsx"
}

🔧 Tool: write
Input: {
  "content": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { memo, useMemo } from \"react\";\nimport {\n  LineChart,\n  Line,\n  XAxis,\n  YAxis,\n  Tooltip,\n  ResponsiveContainer,\n  PieChart,\n  Pie,\n  Cell,\n  BarChart,\n  Bar,\n} from \"recharts\";\nimport {\n  mockTimeseries,\n  mockLanguageBreakdown,\n  mockWpmTrend,\n  mockCorrectionStats,\n} from \"../mocks/tauri\";\n\nconst PIE_COLORS = [\"#60a5fa\", \"#34d399\", \"#fbbf24\", \"#f87171\", \"#a78bfa\", \"#fb923c\"];\nconst TOOLTIP_STYLE = {\n  background: \"var(--color-card)\",\n  border: \"1px solid var(--color-border)\",\n  borderRadius: 8,\n  color: \"var(--color-foreground)\",\n  fontSize: 12,\n};\n\nconst meta: Meta = {\n  title: \"Dashboard/Charts\",\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"padded\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\n\nfunction shortDate(iso: string): string {\n  try {\n    const d = new Date(iso + \"T00:00:00\");\n    return d.toLocaleDateString(undefined, { month: \"short\", day: \"numeric\" });\n  } catch {\n    return iso;\n  }\n}\n\nfunction WordsOverTime() {\n  return (\n    <div className=\"bg-card rounded-xl border border-border p-5\">\n      <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n        Words over time\n      </h2>\n      <ResponsiveContainer width=\"100%\" height={200}>\n        <LineChart data={mockTimeseries} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>\n          <XAxis\n            dataKey=\"date\"\n            tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n            tickFormatter={shortDate}\n            tickLine={false}\n            axisLine={false}\n            interval=\"preserveStartEnd\"\n          />\n          <YAxis\n            tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n            tickLine={false}\n            axisLine={false}\n            allowDecimals={false}\n          />\n          <Tooltip\n            contentStyle={TOOLTIP_STYLE}\n            formatter={(v) => [(v as number).toLocaleString(), \"Words\"]}\n            labelFormatter={(l) => l}\n          />\n          <Line\n            type=\"monotone\"\n            dataKey=\"wordCount\"\n            stroke=\"#60a5fa\"\n            strokeWidth={2}\n            dot={false}\n            activeDot={{ r: 4, fill: \"#60a5fa\" }}\n          />\n        </LineChart>\n      </ResponsiveContainer>\n    </div>\n  );\n}\n\nfunction LanguagePie() {\n  const total = useMemo(\n    () => mockLanguageBreakdown.reduce((sum, d) => sum + d.wordCount, 0),\n    []\n  );\n  const chartData = useMemo(\n    () =>\n      mockLanguageBreakdown.map((d) => ({\n        name: d.language.toUpperCase(),\n        value: d.wordCount,\n        sessions: d.sessionCount,\n      })),\n    []\n  );\n\n  return (\n    <div className=\"bg-card rounded-xl border border-border p-5\">\n      <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n        Languages\n      </h2>\n      <div className=\"flex flex-col items-center gap-3\">\n        <ResponsiveContainer width=\"100%\" height={160}>\n          <PieChart>\n            <Pie\n              data={chartData}\n              dataKey=\"value\"\n              nameKey=\"name\"\n              cx=\"50%\"\n              cy=\"50%\"\n              innerRadius={40}\n              outerRadius={70}\n              paddingAngle={2}\n              strokeWidth={0}\n            >\n              {chartData.map((_, i) => (\n                <Cell key={i} fill={PIE_COLORS[i % PIE_COLORS.length]} />\n              ))}\n            </Pie>\n            <Tooltip\n              contentStyle={TOOLTIP_STYLE}\n              formatter={(v) => [(v as number).toLocaleString(), \"Words\"]}\n            />\n          </PieChart>\n        </ResponsiveContainer>\n        <div className=\"flex flex-wrap justify-center gap-x-4 gap-y-1\">\n          {chartData.map((d, i) => {\n            const pct = total > 0 ? Math.round((d.value / total) * 100) : 0;\n            return (\n              <span\n                key={d.name}\n                className=\"flex items-center gap-1.5 text-xs text-muted-foreground\"\n              >\n                <span\n                  className=\"w-2 h-2 rounded-full shrink-0\"\n                  style={{ background: PIE_COLORS[i % PIE_COLORS.length] }}\n                />\n                {d.name} {pct}%\n              </span>\n            );\n          })}\n        </div>\n      </div>\n    </div>\n  );\n}\n\nfunction WpmTrend() {\n  return (\n    <div className=\"bg-card rounded-xl border border-border p-5\">\n      <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n        WPM trend\n      </h2>\n      <ResponsiveContainer width=\"100%\" height={200}>\n        <LineChart data={mockWpmTrend} margin={{ top: 4, right: 8, bottom: 0, left: -16 }}>\n          <XAxis\n            dataKey=\"date\"\n            tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n            tickFormatter={shortDate}\n            tickLine={false}\n            axisLine={false}\n            interval=\"preserveStartEnd\"\n          />\n          <YAxis\n            tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 11 }}\n            tickLine={false}\n            axisLine={false}\n            allowDecimals={false}\n          />\n          <Tooltip\n            contentStyle={TOOLTIP_STYLE}\n            formatter={(v) => [`${Math.round(v as number)} wpm`, \"Avg WPM\"]}\n            labelFormatter={(l) => l}\n          />\n          <Line\n            type=\"monotone\"\n            dataKey=\"avgWpm\"\n            stroke=\"#34d399\"\n            strokeWidth={2}\n            dot={false}\n            activeDot={{ r: 4, fill: \"#34d399\" }}\n          />\n        </LineChart>\n      </ResponsiveContainer>\n    </div>\n  );\n}\n\nfunction CorrectionFrequency() {\n  const chartData = useMemo(\n    () =>\n      mockCorrectionStats.slice(0, 10).map((d) => ({\n        name: `${d.sourcePhrase} → ${d.targetPhrase}`,\n        count: d.usageCount,\n      })),\n    []\n  );\n\n  return (\n    <div className=\"bg-card rounded-xl border border-border p-5\">\n      <h2 className=\"text-sm font-semibold text-foreground/70 mb-4\">\n        Top corrections\n      </h2>\n      <ResponsiveContainer width=\"100%\" height={200}>\n        <BarChart data={chartData} layout=\"vertical\" margin={{ left: 0, right: 8 }}>\n          <XAxis type=\"number\" hide />\n          <YAxis\n            type=\"category\"\n            dataKey=\"name\"\n            tick={{ fill: \"var(--color-muted-foreground)\", fontSize: 10 }}\n            width={120}\n            tickLine={false}\n            axisLine={false}\n          />\n          <Tooltip\n            contentStyle={TOOLTIP_STYLE}\n            formatter={(v) => [`${v}x`, \"Used\"]}\n          />\n          <Bar dataKey=\"count\" fill=\"#fbbf24\" radius={[0, 4, 4, 0]} barSize={14} />\n        </BarChart>\n      </ResponsiveContainer>\n    </div>\n  );\n}\n\nfunction StatCard({ label, value }: { label: string; value: string }) {\n  return (\n    <div className=\"bg-card border border-border rounded-xl p-5\">\n      <p className=\"text-xs text-muted-foreground mb-1\">{label}</p>\n      <p className=\"text-2xl font-semibold text-foreground tabular-nums\">{value}</p>\n    </div>\n  );\n}\n\nfunction StatCards() {\n  return (\n    <div className=\"grid grid-cols-2 xl:grid-cols-4 gap-4\">\n      <StatCard label=\"Total Words\" value=\"15,420\" />\n      <StatCard label=\"Sessions\" value=\"142\" />\n      <StatCard label=\"Avg WPM\" value=\"132 wpm\" />\n      <StatCard label=\"Recording Time\" value=\"15:00\" />\n    </div>\n  );\n}\n\nconst RANGE_LABELS = {\n  \"7d\": \"7 days\",\n  \"30d\": \"30 days\",\n  all: \"All time\",\n} as const;\n\nfunction RangeSelector() {\n  return (\n    <div className=\"flex gap-1 bg-muted rounded-lg p-1\">\n      {(Object.keys(RANGE_LABELS) as Array<keyof typeof RANGE_LABELS>).map((r) => (\n        <button\n          key={r}\n          className=\"px-3 py-1 text-xs rounded-md transition-colors bg-accent text-foreground\"\n        >\n          {RANGE_LABELS[r]}\n        </button>\n      ))}\n    </div>\n  );\n}\n\nexport const WordsOverTimeChart: StoryObj = {\n  render: () => <WordsOverTime />,\n};\n\nexport const LanguagePieChart: StoryObj = {\n  render: () => <LanguagePie />,\n};\n\nexport const WpmTrendChart: StoryObj = {\n  render: () => <WpmTrend />,\n};\n\nexport const CorrectionFrequencyChart: StoryObj = {\n  render: () => <CorrectionFrequency />,\n};\n\nexport const StatCardsStory: StoryObj = {\n  render: () => <StatCards />,\n};\n\nexport const DashboardGrid: StoryObj = {\n  parameters: {\n    layout: \"fullscreen\",\n  },\n  render: () => (\n    <div className=\"p-8 space-y-6 bg-zinc-950 min-h-screen\">\n      <div className=\"flex items-center justify-between gap-4\">\n        <h1 className=\"text-2xl font-semibold text-foreground\">Dashboard</h1>\n        <RangeSelector />\n      </div>\n\n      <StatCards />\n\n      <div className=\"grid grid-cols-1 xl:grid-cols-3 gap-4\">\n        <div className=\"xl:col-span-2\">\n          <WordsOverTime />\n        </div>\n        <LanguagePie />\n      </div>\n\n      <div className=\"grid grid-cols-1 xl:grid-cols-2 gap-4\">\n        <WpmTrend />\n        <CorrectionFrequency />\n      </div>\n    </div>\n  ),\n};\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\Dashboard.stories.tsx"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\Dashboard.stories.tsx",
  "newString": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { useMemo } from \"react\";",
  "oldString": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { memo, useMemo } from \"react\";"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "in_progress"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "pending"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: write
Input: {
  "content": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { memo, useMemo, useState } from \"react\";\nimport { Search, Copy, Check, Trash2, Upload, ChevronLeft, ChevronRight, X, RefreshCw, Calendar } from \"lucide-react\";\nimport type { Session, SessionWithSegments } from \"../types\";\nimport { mockSessions, mockSessionDetail } from \"../mocks/tauri\";\n\nconst meta: Meta = {\n  title: \"History\",\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"padded\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\n\nfunction formatDate(iso: string): string {\n  try {\n    return new Date(iso).toLocaleString(undefined, {\n      year: \"numeric\",\n      month: \"short\",\n      day: \"numeric\",\n      hour: \"2-digit\",\n      minute: \"2-digit\",\n    });\n  } catch {\n    return iso;\n  }\n}\n\nfunction LanguageBadge({ lang }: { lang: string }) {\n  return (\n    <span className=\"text-xs bg-blue-600 text-white px-1.5 py-0.5 rounded font-mono uppercase\">\n      {lang}\n    </span>\n  );\n}\n\nfunction OutputBadge({ mode, ok }: { mode: string; ok: boolean }) {\n  return (\n    <span\n      className={`text-xs px-1.5 py-0.5 rounded ${\n        ok ? \"bg-green-600 text-white\" : \"bg-red-600 text-white\"\n      }`}\n    >\n      {mode === \"insert\" ? \"inserted\" : \"copied\"}\n    </span>\n  );\n}\n\nfunction SessionRow({\n  session,\n  active,\n  onClick,\n}: {\n  session: Session;\n  active: boolean;\n  onClick: () => void;\n}) {\n  const [copied, setCopied] = useState(false);\n  const preview = useMemo(\n    () =>\n      session.cleanedText.length > 80\n        ? session.cleanedText.slice(0, 78) + \"…\"\n        : session.cleanedText,\n    [session.cleanedText]\n  );\n\n  const handleCopy = (e: React.MouseEvent) => {\n    e.stopPropagation();\n    navigator.clipboard.writeText(session.cleanedText).then(() => {\n      setCopied(true);\n      setTimeout(() => setCopied(false), 1500);\n    });\n  };\n\n  return (\n    <button\n      onClick={onClick}\n      className={`\n        w-full text-left px-4 py-3 rounded-lg border transition-colors\n        ${\n          active\n            ? \"border-neutral-500 bg-accent\"\n            : \"border-border bg-card hover:border-border hover:bg-accent\"\n        }\n      `}\n    >\n      <div className=\"flex items-center gap-2 mb-1\">\n        <span className=\"text-xs text-muted-foreground tabular-nums\">{formatDate(session.startedAt)}</span>\n        <LanguageBadge lang={session.language} />\n        <span className=\"text-xs text-muted-foreground\">{session.wordCount} words</span>\n        <OutputBadge mode={session.outputMode} ok={session.insertedSuccessfully} />\n        <span className=\"flex-1\" />\n        <button\n          onClick={handleCopy}\n          title=\"Copy to clipboard\"\n          className=\"flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors px-1.5 py-0.5 rounded hover:bg-muted\"\n        >\n          {copied ? <Check size={12} className=\"text-green-400\" /> : <Copy size={12} />}\n          {copied ? \"Copied\" : \"Copy\"}\n        </button>\n      </div>\n      <p className=\"text-sm text-foreground/70 leading-snug\">{preview}</p>\n    </button>\n  );\n}\n\nfunction SearchFilters() {\n  return (\n    <div className=\"flex flex-col gap-2\">\n      <div className=\"relative\">\n        <Search size={14} className=\"absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none\" />\n        <input\n          type=\"search\"\n          placeholder=\"Search transcriptions…\"\n          className=\"w-full bg-muted border border-border text-foreground text-sm rounded-md pl-9 pr-3 py-2 placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-neutral-500\"\n        />\n      </div>\n      <div className=\"flex flex-wrap gap-2\">\n        <select className=\"bg-muted border border-border text-foreground/70 text-xs rounded px-2 py-1.5 focus:outline-none\">\n          <option value=\"\">All languages</option>\n          <option value=\"de\">German (de)</option>\n          <option value=\"en\">English (en)</option>\n        </select>\n        <label className=\"flex items-center gap-1 text-xs text-muted-foreground\">\n          <Calendar size={12} />\n          From\n          <input\n            type=\"date\"\n            className=\"bg-muted border border-border text-foreground/70 rounded px-2 py-1 text-xs focus:outline-none\"\n          />\n        </label>\n        <label className=\"flex items-center gap-1 text-xs text-muted-foreground\">\n          <Calendar size={12} />\n          To\n          <input\n            type=\"date\"\n            className=\"bg-muted border border-border text-foreground/70 rounded px-2 py-1 text-xs focus:outline-none\"\n          />\n        </label>\n      </div>\n    </div>\n  );\n}\n\nfunction Pagination() {\n  return (\n    <div className=\"flex items-center justify-between text-xs text-muted-foreground\">\n      <span>1–3</span>\n      <div className=\"flex gap-2\">\n        <button className=\"flex items-center gap-1 px-3 py-1 rounded bg-muted hover:bg-accent disabled:opacity-40 disabled:cursor-not-allowed transition-colors\">\n          <ChevronLeft size={12} /> Previous\n        </button>\n        <button className=\"flex items-center gap-1 px-3 py-1 rounded bg-muted hover:bg-accent disabled:opacity-40 disabled:cursor-not-allowed transition-colors\">\n          Next <ChevronRight size={12} />\n        </button>\n      </div>\n    </div>\n  );\n}\n\nfunction SessionDrawer({\n  detail,\n  onClose,\n}: {\n  detail: SessionWithSegments | null;\n  onClose: () => void;\n}) {\n  const [tab, setTab] = useState<\"cleaned\" | \"raw\" | \"diff\">(\"cleaned\");\n\n  if (!detail) return null;\n\n  const { session, segments } = detail;\n  const durationSec = Math.round(session.durationMs / 1000);\n\n  return (\n    <aside className=\"w-96 shrink-0 border-l border-border flex flex-col overflow-hidden\">\n      <div className=\"flex items-center justify-between px-5 py-4 border-b border-border\">\n        <h2 className=\"text-sm font-semibold text-foreground\">Session Details</h2>\n        <button onClick={onClose} className=\"text-muted-foreground hover:text-foreground\">\n          <X size={16} />\n        </button>\n      </div>\n\n      <div className=\"px-5 py-3 border-b border-border space-y-1\">\n        <p className=\"text-xs text-muted-foreground\">{formatDate(session.startedAt)}</p>\n        <div className=\"flex flex-wrap gap-2\">\n          <LanguageBadge lang={session.language} />\n          <span className=\"text-xs bg-muted text-muted-foreground px-1.5 py-0.5 rounded\">\n            {session.modelId}\n          </span>\n          <span className=\"text-xs text-muted-foreground\">{session.wordCount} words</span>\n          {durationSec > 0 && (\n            <span className=\"text-xs text-muted-foreground\">{durationSec}s</span>\n          )}\n          {session.estimatedWpm && (\n            <span className=\"text-xs text-muted-foreground\">\n              ~{Math.round(session.estimatedWpm)} wpm\n            </span>\n          )}\n        </div>\n      </div>\n\n      <div className=\"flex border-b border-border\">\n        {([\"cleaned\", \"raw\", \"diff\"] as const).map((t) => (\n          <button\n            key={t}\n            onClick={() => setTab(t)}\n            className={`flex-1 py-2 text-xs font-medium transition-colors ${\n              tab === t\n                ? \"text-foreground border-b-2 border-foreground\"\n                : \"text-muted-foreground hover:text-foreground/70\"\n            }`}\n          >\n            {t === \"cleaned\" ? \"Cleaned\" : t === \"raw\" ? \"Raw\" : \"Diff\"}\n          </button>\n        ))}\n      </div>\n\n      <div className=\"flex-1 overflow-y-auto px-5 py-4 min-h-0\">\n        <p className=\"text-sm text-foreground/80 leading-relaxed whitespace-pre-wrap\">\n          {tab === \"cleaned\"\n            ? session.cleanedText\n            : tab === \"raw\"\n              ? session.rawText\n              : \"Diff view would show changes here\"}\n        </p>\n      </div>\n\n      <div className=\"flex items-center gap-2 px-5 py-3 border-t border-border\">\n        <button className=\"flex items-center gap-1.5 flex-1 text-xs bg-muted hover:bg-accent text-foreground/70 hover:text-foreground rounded px-3 py-1.5 transition-colors\">\n          <Copy size={12} />\n          Copy\n        </button>\n        <button className=\"flex items-center gap-1.5 text-xs rounded px-3 py-1.5 transition-colors bg-muted hover:bg-accent text-foreground/70 hover:text-foreground\">\n          <RefreshCw size={12} />\n          Reprocess\n        </button>\n        <button className=\"flex items-center gap-1.5 text-xs bg-muted hover:bg-accent text-foreground/70 hover:text-foreground rounded px-3 py-1.5 transition-colors\">\n          <Upload size={12} />\n          Export\n        </button>\n        <button className=\"flex items-center gap-1.5 text-xs bg-muted hover:bg-rose-900/30 text-rose-400 hover:text-rose-300 rounded px-3 py-1.5 transition-colors\">\n          <Trash2 size={12} />\n          Delete\n        </button>\n      </div>\n    </aside>\n  );\n}\n\nexport const Default: StoryObj = {\n  render: () => {\n    const [selected, setSelected] = useState<SessionWithSegments | null>(null);\n\n    return (\n      <div className=\"flex h-96 min-h-0 bg-zinc-950 rounded-lg overflow-hidden\">\n        <div className=\"flex flex-col flex-1 min-w-0 p-4 gap-3\">\n          <div className=\"flex items-center justify-between\">\n            <h1 className=\"text-xl font-semibold text-foreground shrink-0\">History</h1>\n            <button className=\"flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground border border-border hover:border-neutral-500 px-3 py-1.5 rounded transition-colors\">\n              <Upload size={12} />\n              Export page\n            </button>\n          </div>\n\n          <SearchFilters />\n\n          <div className=\"flex-1 min-h-0 overflow-y-auto space-y-2\">\n            {mockSessions.map((session) => (\n              <SessionRow\n                key={session.id}\n                session={session}\n                active={selected?.session.id === session.id}\n                onClick={() => setSelected(selected?.session.id === session.id ? null : mockSessionDetail)}\n              />\n            ))}\n          </div>\n\n          <Pagination />\n        </div>\n\n        {selected && (\n          <SessionDrawer detail={selected} onClose={() => setSelected(null)} />\n        )}\n      </div>\n    );\n  },\n};\n\nexport const Empty: StoryObj = {\n  render: () => (\n    <div className=\"flex flex-col h-96 min-h-0 bg-zinc-950 rounded-lg overflow-hidden\">\n      <div className=\"flex flex-col flex-1 min-w-0 p-4 gap-3\">\n        <div className=\"flex items-center justify-between\">\n          <h1 className=\"text-xl font-semibold text-foreground shrink-0\">History</h1>\n        </div>\n\n        <SearchFilters />\n\n        <div className=\"flex-1 min-h-0 overflow-y-auto space-y-2\">\n          <p className=\"text-muted-foreground text-sm py-8 text-center\">\n            No sessions found.\n          </p>\n        </div>\n      </div>\n    </div>\n  ),\n};\n\nexport const Loading: StoryObj = {\n  render: () => (\n    <div className=\"flex flex-col h-96 min-h-0 bg-zinc-950 rounded-lg overflow-hidden\">\n      <div className=\"flex flex-col flex-1 min-w-0 p-4 gap-3\">\n        <div className=\"flex items-center justify-between\">\n          <h1 className=\"text-xl font-semibold text-foreground shrink-0\">History</h1>\n        </div>\n\n        <SearchFilters />\n\n        <div className=\"flex-1 min-h-0 overflow-y-auto space-y-2\">\n          <p className=\"text-muted-foreground text-sm py-8 text-center\">\n            Loading…\n          </p>\n        </div>\n      </div>\n    </div>\n  ),\n};\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\History.stories.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\mocks\\tauri.ts",
  "limit": 50,
  "offset": 240
}

🔧 Tool: grep
Input: {
  "path": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src",
  "pattern": "mockSessionDetail"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\mocks\\tauri.ts",
  "limit": 40,
  "offset": 345
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\mocks\\tauri.ts",
  "newString": "export const mockSessionDetail: SessionWithSegments = {",
  "oldString": "const mockSessionDetail: SessionWithSegments = {"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\History.stories.tsx",
  "newString": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { useMemo, useState } from \"react\";\nimport { Search, Copy, Check, Trash2, Upload, ChevronLeft, ChevronRight, X, RefreshCw, Calendar } from \"lucide-react\";\nimport type { Session, SessionWithSegments } from \"../types\";\nimport { mockSessions, mockSessionDetail } from \"../mocks/tauri\";",
  "oldString": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { memo, useMemo, useState } from \"react\";\nimport { Search, Copy, Check, Trash2, Upload, ChevronLeft, ChevronRight, X, RefreshCw, Calendar } from \"lucide-react\";\nimport type { Session, SessionWithSegments } from \"../types\";\nimport { mockSessions, mockSessionDetail } from \"../mocks/tauri\";"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\History.stories.tsx",
  "newString": "  const { session } = detail;",
  "oldString": "  const { session, segments } = detail;"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "in_progress"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "pending"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: write
Input: {
  "content": "import type { Meta, StoryObj } from \"@storybook/react\";\nimport { Button } from \"../components/ui/button\";\nimport { Mic, Settings, Copy, Trash2, Download } from \"lucide-react\";\n\nconst meta: Meta<typeof Button> = {\n  title: \"UI/Button\",\n  component: Button,\n  tags: [\"autodocs\"],\n  parameters: {\n    layout: \"centered\",\n    backgrounds: {\n      default: \"dark\",\n    },\n  },\n};\n\nexport default meta;\n\nexport const Default: StoryObj<typeof Button> = {\n  args: {\n    children: \"Button\",\n  },\n};\n\nexport const Outline: StoryObj<typeof Button> = {\n  args: {\n    variant: \"outline\",\n    children: \"Outline Button\",\n  },\n};\n\nexport const Secondary: StoryObj<typeof Button> = {\n  args: {\n    variant: \"secondary\",\n    children: \"Secondary Button\",\n  },\n};\n\nexport const Ghost: StoryObj<typeof Button> = {\n  args: {\n    variant: \"ghost\",\n    children: \"Ghost Button\",\n  },\n};\n\nexport const Destructive: StoryObj<typeof Button> = {\n  args: {\n    variant: \"destructive\",\n    children: \"Destructive Button\",\n  },\n};\n\nexport const Link: StoryObj<typeof Button> = {\n  args: {\n    variant: \"link\",\n    children: \"Link Button\",\n  },\n};\n\nexport const Small: StoryObj<typeof Button> = {\n  args: {\n    size: \"sm\",\n    children: \"Small Button\",\n  },\n};\n\nexport const Large: StoryObj<typeof Button> = {\n  args: {\n    size: \"lg\",\n    children: \"Large Button\",\n  },\n};\n\nexport const Icon: StoryObj<typeof Button> = {\n  args: {\n    size: \"icon\",\n    children: <Mic size={16} />,\n  },\n};\n\nexport const IconSmall: StoryObj<typeof Button> = {\n  args: {\n    size: \"icon-sm\",\n    children: <Settings size={14} />,\n  },\n};\n\nexport const WithIcon: StoryObj<typeof Button> = {\n  args: {\n    children: (\n      <>\n        <Copy size={14} data-slot=\"icon\" />\n        Copy\n      </>\n    ),\n  },\n};\n\nexport const Loading: StoryObj<typeof Button> = {\n  args: {\n    children: \"Loading...\",\n    disabled: true,\n  },\n};\n\nexport const AllVariants: StoryObj = {\n  parameters: {\n    layout: \"padded\",\n  },\n  render: () => (\n    <div className=\"flex flex-wrap gap-4 items-center\">\n      <Button variant=\"default\">Default</Button>\n      <Button variant=\"outline\">Outline</Button>\n      <Button variant=\"secondary\">Secondary</Button>\n      <Button variant=\"ghost\">Ghost</Button>\n      <Button variant=\"destructive\">Destructive</Button>\n      <Button variant=\"link\">Link</Button>\n    </div>\n  ),\n};\n\nexport const AllSizes: StoryObj = {\n  parameters: {\n    layout: \"padded\",\n  },\n  render: () => (\n    <div className=\"flex flex-wrap gap-4 items-center\">\n      <Button size=\"xs\">XS</Button>\n      <Button size=\"sm\">SM</Button>\n      <Button size=\"default\">Default</Button>\n      <Button size=\"lg\">LG</Button>\n      <Button size=\"icon\"><Mic size={16} /></Button>\n      <Button size=\"icon-sm\"><Settings size={14} /></Button>\n      <Button size=\"icon-xs\"><Trash2 size={12} /></Button>\n      <Button size=\"icon-lg\"><Download size={18} /></Button>\n    </div>\n  ),\n};\n\nexport const Disabled: StoryObj = {\n  parameters: {\n    layout: \"padded\",\n  },\n  render: () => (\n    <div className=\"flex flex-wrap gap-4 items-center\">\n      <Button disabled>Default Disabled</Button>\n      <Button variant=\"outline\" disabled>Outline Disabled</Button>\n      <Button variant=\"secondary\" disabled>Secondary Disabled</Button>\n      <Button variant=\"destructive\" disabled>Destructive Disabled</Button>\n    </div>\n  ),\n};\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stories\\Button.stories.tsx"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "completed"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "in_progress"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "pending"
    }
  ]
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\package.json"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\package.json",
  "newString": "  \"scripts\": {\n    \"dev\": \"vite\",\n    \"build\": \"tsc && vite build\",\n    \"preview\": \"vite preview\",\n    \"tauri\": \"tauri\",\n    \"storybook\": \"storybook dev -p 6006\",\n    \"build-storybook\": \"storybook build\"\n  },",
  "oldString": "  \"scripts\": {\n    \"dev\": \"vite\",\n    \"build\": \"tsc && vite build\",\n    \"preview\": \"vite preview\",\n    \"tauri\": \"tauri\"\n  },"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "completed"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "in_progress"
    }
  ]
}

🔧 Tool: glob
Input: {
  "pattern": "src/index.css"
}

🔧 Tool: bash
Input: {
  "command": "npm run build-storybook 2>&1 | head -100",
  "description": "Build Storybook",
  "timeout": 300000,
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: todowrite
Input: {
  "todos": [
    {
      "content": "Install Storybook and dependencies",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create .storybook configuration (main.ts, preview.ts, decorators)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create mock Tauri invoke wrappers in src/mocks/",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Pill component stories (all states)",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Create Waveform component story",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Dashboard chart stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create History list item stories",
      "priority": "medium",
      "status": "completed"
    },
    {
      "content": "Create Button and other UI component stories",
      "priority": "low",
      "status": "completed"
    },
    {
      "content": "Update package.json with Storybook scripts",
      "priority": "high",
      "status": "completed"
    },
    {
      "content": "Verify Storybook builds and runs correctly",
      "priority": "high",
      "status": "completed"
    }
  ]
}
<summary>
## Summary: Component Storybook with Tauri Mocking

### Changes Implemented
- Installed Storybook 10.3.0 with React/Vite framework
- Created `.storybook/` configuration (main.ts, preview.ts)
- Created comprehensive mock Tauri invoke system in `src/mocks/tauri.ts` with:
  - Fixture data for all types (sessions, models, dashboard stats, etc.)
  - `mockInvoke()` function that returns realistic mock data for all 50+ Tauri commands
- Created stories for all major UI components:
  - **Pill**: Idle, IdleWithLastTranscription, Listening, Processing, Success, SuccessInsert, Error, ErrorOutputFailed, ExpandedIdle, ExpandedListening, AllStates
  - **Waveform**: Default, HighLevel, LowLevel, Silent, DynamicLevel, AllLevels
  - **Dashboard/Charts**: WordsOverTimeChart, LanguagePieChart, WpmTrendChart, CorrectionFrequencyChart, StatCards, DashboardGrid
  - **History**: Default, Empty, Loading (with SessionRow, SessionDrawer, SearchFilters, Pagination)
  - **UI/Button**: All variants (default, outline, secondary, ghost, destructive), sizes, with icons, disabled states
- Updated `package.json` with Storybook scripts

### Files Modified
- `package.json` - Added storybook and build-storybook scripts
- `.storybook/main.ts` - Storybook configuration
- `.storybook/preview.ts` - Preview configuration with dark background default
- `src/mocks/tauri.ts` - Complete mock Tauri invoke system with all fixtures
- `src/stories/Pill.stories.tsx` - Pill component stories
- `src/stories/Waveform.stories.tsx` - Waveform component stories
- `src/stories/Dashboard.stories.tsx` - Dashboard chart stories
- `src/stories/History.stories.tsx` - History page stories
- `src/stories/Button.stories.tsx` - Button UI component stories

### Notes for Developer
- Run `npm run storybook` to start the Storybook dev server on port 6006
- Run `npm run build-storybook` to build static Storybook output
- Mock Tauri commands are automatically used in stories via decorators that override `window.invoke`
- Stories use realistic fixture data that matches production API shapes
- All components can be viewed in isolation without running the Tauri backend
</summary>