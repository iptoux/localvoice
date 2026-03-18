
🔧 Tool: task
Input: {
  "description": "Explore codebase for schema",
  "prompt": "Explore the codebase to find all SQLite database schema definitions. Look for:\n1. SQL files that define tables (sessions, segments, dictionary entries, settings, models)\n2. Rust source files in src-tauri/src/db/ that define schema or use rusqlite/sqlx\n3. Any migration files or schema version files\n4. The PRD or plan files that document the intended schema\n\nSearch thoroughly in these locations:\n- src-tauri/src/db/\n- src-tauri/src/commands/\n- Any .sql files anywhere\n- plan/ directory for schema docs\n\nReturn a comprehensive list of all table definitions found with their columns, types, and relationships.",
  "subagent_type": "explore"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\db\\migrations.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev"
}

🔧 Tool: grep
Input: {
  "path": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs",
  "pattern": "ERD|schema|database|sqlite"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md"
}

🔧 Tool: glob
Input: {
  "pattern": "docs/dev/database-schema.md"
}
Now I'll create the database schema documentation with ERD diagrams and a migration guide.

🔧 Tool: write
Input: {
  "content": "# SQLite Database Schema\n\nLocalVoice uses SQLite as its single-file relational database. All persistence lives in one `localvoice.db` file located in the platform app-data directory.\n\n## Versioned Migrations\n\nSchema changes are applied via a **versioned migration system** defined in `src-tauri/src/db/migrations.rs`.\n\nThe `schema_migrations` table tracks applied versions:\n\n```sql\nCREATE TABLE schema_migrations (\n    version    INTEGER PRIMARY KEY,\n    applied_at TEXT NOT NULL\n);\n```\n\nThe `run()` function queries `MAX(version)` and executes any pending migrations in order. Keep versions consecutive and monotonically increasing.\n\n---\n\n## Entity-Relationship Diagram\n\n```\n┌──────────────────────────────────────────────────────────────────────────────┐\n│                              SCHEMA OVERVIEW                                  │\n└──────────────────────────────────────────────────────────────────────────────┘\n\n  ┌─────────────┐       ┌─────────────────────┐       ┌──────────────────────┐\n  │  sessions   │ 1───N │  session_segments   │       │  model_installations │\n  │             │──────>│                     │       │                      │\n  │ id (PK)     │       │ id (PK)             │       │ id (PK)              │\n  │ started_at  │       │ session_id (FK)     │       │ model_key (UNIQUE)   │\n  │ ended_at    │       │ start_ms            │       │ display_name         │\n  │ duration_ms │       │ end_ms              │       │ language_scope       │\n  │ language    │       │ text                │       │ local_path           │\n  │ raw_text    │       │ confidence          │       │ installed            │\n  │ cleaned_text│       │ segment_index       │       │ created_at           │\n  │ word_count  │       └─────────────────────┘       │ updated_at           │\n  │ ...         │                                        └──────────┬─────────┘\n  └──────┬──────┘                                                   │\n         │ N                                                        │ N\n         │                                                          │\n         ▼                                                          ▼\n  ┌─────────────────────┐       ┌──────────────────────┐   ┌────────────────────────┐\n  │ filler_removal_log  │       │  model_language_     │   │  dictionary_entries     │\n  │                     │       │  defaults            │   │                        │\n  │ id (PK)             │       │                      │   │ id (PK)                │\n  │ session_id (FK,NULL)│       │ language (PK)        │   │ phrase                 │\n  │ word                │       │ model_key            │   │ normalized_phrase      │\n  │ language            │       └──────────────────────┘   │ language               │\n  │ removed_at          │                                    │ entry_type             │\n  └─────────────────────┘                                    │ created_at             │\n                                                               │ updated_at             │\n  ┌────────────────────────────────────────────────────────┐   └───────────┬──────────┘\n  │                 correction_rules                       │              │ 1───N\n  │                                                         │              │\n  │ id (PK)              is_active (BOOL)                   │              ▼\n  │ source_phrase        auto_apply (BOOL)                  │   ┌──────────────────────┐\n  │ normalized_source_phrase usage_count                    │   │  ambiguous_terms     │\n  │ target_phrase        last_used_at                       │   │                      │\n  │ language             created_at                        │   │ id (PK)              │\n  │ rule_mode            updated_at                        │   │ phrase               │\n  └────────────────────────────────────────────────────────┘   │ normalized_phrase     │\n                                                               │ occurrences           │\n  ┌────────────────────────────────────────────────────────┐   │ avg_confidence        │\n  │                      settings                           │   │ last_seen_at          │\n  │                                                         │   │ suggested_target      │\n  │ key (PK)           Dot-notation key (e.g. app.theme)     │   │ dismissed             │\n  │ value              JSON-encoded value string             │   │ created_at            │\n  │ updated_at         ISO 8601 timestamp                    │   │ updated_at            │\n  └────────────────────────────────────────────────────────┘   └──────────────────────┘\n\n  ┌─────────────────────────────────────────────────────────────┐\n  │                      filler_words                           │\n  │                                                              │\n  │ id (PK)            word                language              │\n  │ is_default         created_at                               │\n  └─────────────────────────────────────────────────────────────┘\n\n  ┌─────────────────────────────────────────────────────────────┐\n  │                       app_logs                              │\n  │                                                              │\n  │ id (PK)            level (debug|info|warn|error)             │\n  │ area               message                                  │\n  │ created_at                                                 │\n  └─────────────────────────────────────────────────────────────┘\n```\n\n---\n\n## Table Reference\n\n### sessions\n\nPrimary dictation session record.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| started_at | TEXT | NOT NULL | ISO 8601 |\n| ended_at | TEXT | NOT NULL | ISO 8601 |\n| duration_ms | INTEGER | NOT NULL | Duration in ms |\n| language | TEXT | NOT NULL | ISO 639-1 (de, en, …) |\n| model_id | TEXT | NULL | Whisper model key |\n| trigger_type | TEXT | NOT NULL | shortcut / button / tray |\n| input_device_id | TEXT | NULL | Audio device ID |\n| raw_text | TEXT | NOT NULL | Original transcription |\n| cleaned_text | TEXT | NOT NULL | Post-processed text |\n| word_count | INTEGER | NOT NULL DEFAULT 0 | |\n| char_count | INTEGER | NOT NULL DEFAULT 0 | |\n| avg_confidence | REAL | NULL | 0–1 |\n| estimated_wpm | REAL | NULL | Words per minute |\n| output_mode | TEXT | NOT NULL | insert / clipboard / preview |\n| output_target_app | TEXT | NULL | Target app (if detectable) |\n| inserted_successfully | INTEGER | NOT NULL DEFAULT 0 | Boolean |\n| error_message | TEXT | NULL | |\n| created_at | TEXT | NOT NULL | ISO 8601 |\n| audio_path | TEXT | NULL | Added v5 — path to audio file |\n| original_raw_text | TEXT | NULL | Added v5 — pre-reprocessing text |\n| reprocessed_count | INTEGER | NOT NULL DEFAULT 0 | Added v5 |\n\n**Indexes:** PRIMARY KEY on `id`; consider indexes on `started_at`, `language`.\n\n**Relationships:** 1 session → N session_segments (ON DELETE CASCADE); N sessions → N filler_removal_log entries.\n\n---\n\n### session_segments\n\nTimestamped transcription segments within a session.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| session_id | TEXT | NOT NULL, FK | sessions(id) ON DELETE CASCADE |\n| start_ms | INTEGER | NOT NULL | |\n| end_ms | INTEGER | NOT NULL | |\n| text | TEXT | NOT NULL | |\n| confidence | REAL | NULL | 0–1 |\n| segment_index | INTEGER | NOT NULL | Order |\n\n**Relationships:** N segments → 1 session.\n\n---\n\n### dictionary_entries\n\nCustom vocabulary entries for domain-specific terms.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| phrase | TEXT | NOT NULL | Original phrase |\n| normalized_phrase | TEXT | NOT NULL | Lowercase, whitespace-collapsed |\n| language | TEXT | NULL | ISO 639-1 |\n| entry_type | TEXT | NOT NULL | term / name / acronym / product / custom |\n| notes | TEXT | NULL | |\n| created_at | TEXT | NOT NULL | ISO 8601 |\n| updated_at | TEXT | NOT NULL | ISO 8601 |\n\n**Indexes:** UNIQUE on `(normalized_phrase, language)` recommended.\n\n---\n\n### correction_rules\n\nFind-and-replace text correction rules.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| source_phrase | TEXT | NOT NULL | Text to match |\n| normalized_source_phrase | TEXT | NOT NULL | Lowercase version |\n| target_phrase | TEXT | NOT NULL | Replacement |\n| language | TEXT | NULL | ISO 639-1 |\n| rule_mode | TEXT | NOT NULL | manual / suggested / learned |\n| confidence_threshold | REAL | NULL | Min confidence to auto-apply |\n| is_active | INTEGER | NOT NULL DEFAULT 1 | Boolean |\n| auto_apply | INTEGER | NOT NULL DEFAULT 1 | Apply on transcription |\n| usage_count | INTEGER | NOT NULL DEFAULT 0 | |\n| last_used_at | TEXT | NULL | ISO 8601 |\n| created_at | TEXT | NOT NULL | ISO 8601 |\n| updated_at | TEXT | NOT NULL | ISO 8601 |\n\n**Indexes:** INDEX on `(normalized_source_phrase, language, is_active)`.\n\n---\n\n### ambiguous_terms\n\nLow-confidence transcribed phrases flagged for user review.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| phrase | TEXT | NOT NULL | Original phrase |\n| normalized_phrase | TEXT | NOT NULL | Lowercase |\n| language | TEXT | NULL | ISO 639-1 |\n| occurrences | INTEGER | NOT NULL DEFAULT 1 | |\n| avg_confidence | REAL | NULL | Rolling average |\n| last_seen_at | TEXT | NOT NULL | ISO 8601 |\n| suggested_target | TEXT | NULL | Suggested replacement |\n| dismissed | INTEGER | NOT NULL DEFAULT 0 | |\n| created_at | TEXT | NOT NULL | ISO 8601 |\n| updated_at | TEXT | NOT NULL | ISO 8601 |\n| dismissed_at_occurrences | INTEGER | NOT NULL DEFAULT 0 | Added v2 — re-surface after N new occurrences |\n\n**Indexes:** INDEX on `(normalized_phrase, language, dismissed)`.\n\n---\n\n### model_installations\n\nRegistry of whisper.cpp models.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| model_key | TEXT | NOT NULL UNIQUE | e.g. ggml-base |\n| display_name | TEXT | NOT NULL | Human-readable name |\n| language_scope | TEXT | NOT NULL | multilingual / en / de / custom |\n| local_path | TEXT | NOT NULL | Filesystem path |\n| file_size_bytes | INTEGER | NULL | |\n| checksum | TEXT | NULL | SHA-256 |\n| installed | INTEGER | NOT NULL DEFAULT 0 | |\n| installed_at | TEXT | NULL | ISO 8601 |\n| version | TEXT | NULL | |\n| is_default_for_de | INTEGER | NOT NULL DEFAULT 0 | Deprecated v8 |\n| is_default_for_en | INTEGER | NOT NULL DEFAULT 0 | Deprecated v8 |\n| created_at | TEXT | NOT NULL | ISO 8601 |\n| updated_at | TEXT | NOT NULL | ISO 8601 |\n\n**Note:** `is_default_for_de`/`is_default_for_en` are deprecated; use `model_language_defaults` (v8) instead.\n\n---\n\n### model_language_defaults\n\nPer-language default model assignments.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| language | TEXT | PK | ISO 639-1 |\n| model_key | TEXT | NOT NULL | Default model for this language |\n\n**Added:** v8. Replaces deprecated `is_default_for_de`/`is_default_for_en` columns.\n\n---\n\n### settings\n\nKey-value store for all application settings.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| key | TEXT | PK | Dot-notation key (e.g. app.theme) |\n| value | TEXT | NOT NULL | JSON-encoded string |\n| updated_at | TEXT | NOT NULL | ISO 8601 |\n\n**Seed keys (v1):**\n\n| Key | Default |\n|-----|---------|\n| app.theme | system |\n| app.language | de |\n| app.start_hidden | false |\n| app.autostart | false |\n| ui.default_mode | pill |\n| ui.pill.always_on_top | true |\n| recording.shortcut | CommandOrControl+Shift+Space |\n| recording.push_to_talk | false |\n| recording.silence_timeout_ms | 1500 |\n| recording.keep_audio | false (v5) |\n| recording.audio_retention_days | 7 (v5) |\n| recording.max_audio_storage_mb | 500 (v5) |\n| transcription.default_language | auto |\n| transcription.auto_punctuation | true |\n| transcription.auto_capitalization | true |\n| transcription.remove_fillers | false |\n| output.mode | clipboard |\n| output.auto_paste | false |\n| output.insert_delay_ms | 100 (v4) |\n| dictionary.auto_apply_rules | true |\n| ambiguity.confidence_threshold | 0.6 |\n| ambiguity.min_occurrences | 3 |\n| notifications.on_error | true (v3) |\n| notifications.on_success | false (v3) |\n\n---\n\n### filler_words\n\nLanguage-specific filler words (äh, uh, um, …).\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| word | TEXT | NOT NULL | |\n| language | TEXT | NOT NULL | ISO 639-1 |\n| is_default | INTEGER | NOT NULL DEFAULT 0 | System-provided |\n| created_at | TEXT | NOT NULL | ISO 8601 |\n\n**Seeded languages:** de, en, fr, es, it, pt, nl, pl, ru, ja, zh (v6–v7).\n\n**Indexes:** INDEX on `(language, is_default)`.\n\n---\n\n### filler_removal_log\n\nAudit log of removed filler words.\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| session_id | TEXT | NULL, FK | sessions(id) ON DELETE SET NULL |\n| word | TEXT | NOT NULL | |\n| language | TEXT | NOT NULL | ISO 639-1 |\n| removed_at | TEXT | NOT NULL | ISO 8601 |\n\n**Added:** v7.\n\n---\n\n### app_logs\n\nApplication logging (future SQLite-backed implementation).\n\n| Column | Type | Constraints | Description |\n|--------|------|-------------|-------------|\n| id | TEXT | PK | UUID |\n| level | TEXT | NOT NULL | debug / info / warn / error |\n| area | TEXT | NOT NULL DEFAULT '' | Log component |\n| message | TEXT | NOT NULL | |\n| created_at | TEXT | NOT NULL | ISO 8601 |\n\n**Added:** v3.\n\n---\n\n## Migration Guide\n\n### How to Add a New Table\n\n1. **Pick the next version number.** Check `MIGRATIONS` in `migrations.rs`; the highest key is the current version. Add version `N+1`.\n\n2. **Write the CREATE TABLE SQL.** Use `IF NOT EXISTS` for safety on re-runs.\n\n3. **Add seed data if needed.** Use `INSERT OR IGNORE` so re-runs are idempotent.\n\n4. **Update this document.** Add the new table to the ERD and table reference.\n\n5. **Add corresponding Rust models and repository.** See `src-tauri/src/db/models.rs` and `src-tauri/src/db/repositories/`.\n\nExample:\n\n```rust\n// migrations.rs\n(\n    9,\n    \"\n    CREATE TABLE IF NOT EXISTS new_feature (\n        id         TEXT PRIMARY KEY,\n        name       TEXT NOT NULL,\n        created_at TEXT NOT NULL\n    );\n\n    INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES\n        ('new_feature.enabled', 'false', datetime('now'));\n    \",\n),\n```\n\n---\n\n### How to Add a Column to an Existing Table\n\n**Rule: Never use `DROP` or destructive operations on user data.**\n\n1. **Pick the next version number.**\n\n2. **Use `ALTER TABLE ... ADD COLUMN`.** This is safe — SQLite adds the column with NULL (or DEFAULT) for existing rows.\n\n3. **Set a DEFAULT value if the column is NOT NULL.** SQLite requires a default for new NOT NULL columns so existing rows don't violate the constraint.\n\n4. **Add the column to Rust models.** Update the corresponding struct in `models.rs`.\n\n5. **Update this document.** Add the column to the table reference.\n\nExample:\n\n```rust\n// migrations.rs\n(\n    9,\n    \"\n    ALTER TABLE sessions ADD COLUMN new_field TEXT NOT NULL DEFAULT '';\n    \",\n),\n```\n\n```rust\n// models.rs\npub struct Session {\n    // ... existing fields ...\n    pub new_field: String,\n}\n```\n\n---\n\n### How to Deprecate a Column\n\nIf a column becomes obsolete (like `is_default_for_de` in v8):\n\n1. **Do NOT drop the column immediately.** It may still exist in user databases.\n\n2. **Create a replacement table or column** (e.g., `model_language_defaults`).\n\n3. **Add a migration to copy data** from old to new.\n\n4. **Mark the old column as deprecated** in comments and docs.\n\n5. **Drop the column in a future major version** after a reasonable grace period (e.g., 6–12 months).\n\n---\n\n### Migration Best Practices\n\n| Do | Don't |\n|----|-------|\n| Use `IF NOT EXISTS` on CREATE TABLE | Drop tables or columns in migrations |\n| Use `INSERT OR IGNORE` for seed data | Assume columns exist without checking |\n| Keep migrations additive | Reorder or remove past migrations |\n| Test migrations on existing DB | Use `SELECT *` in production queries |\n| Increment version numbers consecutively | Skip version numbers |\n\n---\n\n### Verifying Migrations\n\nAfter writing a migration, test it:\n\n```bash\n# Run the app fresh — migrations run automatically on startup\nnpm run tauri dev\n\n# Or verify via the Rust test suite\ncargo test --package localvoice -- migrations\n```\n\nCheck the `schema_migrations` table after startup:\n\n```sql\nSELECT * FROM schema_migrations ORDER BY version;\n```\n\n---\n\n## Relationship Summary\n\n| Relationship | Type | ON DELETE |\n|-------------|-------|-----------|\n| sessions → session_segments | 1:N | CASCADE |\n| sessions → filler_removal_log | 1:N | SET NULL |\n| model_installations → model_language_defaults | 1:N | — |\n\nAll other tables (`settings`, `dictionary_entries`, `correction_rules`, `ambiguous_terms`, `filler_words`, `app_logs`) are standalone with no foreign keys.\n\n---\n\n## Seed Data\n\nDefault data is seeded via `INSERT OR IGNORE INTO` statements within the relevant migration. This pattern ensures:\n- First-run: data is inserted\n- Subsequent runs: rows with the same PK are skipped (idempotent)\n- No errors on re-run\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\database-schema.md"
}
Now update the index to mark the doc as done:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md",
  "newString": "- [database-schema.md](database-schema.md) — full schema with ERD, table reference, and migration guide",
  "oldString": "- [database-schema.md](database-schema.md) — full schema with field descriptions *(todo)*"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md",
  "newString": "- [database-schema.md](database-schema.md) — full schema with ERD, table reference, and migration guide",
  "oldString": "- [database-schema.md](database-schema.md) — full schema with field descriptions *(todo)*"
}
Now let me create a Playwright test to verify the documentation:

🔧 Tool: write
Input: {
  "content": "import { test, expect } from '@playwright/test';\nimport * as fs from 'fs';\nimport * as path from 'path';\n\ntest.describe('Database Schema Documentation', () => {\n  const schemaPath = path.join(__dirname, '..', '..', 'docs', 'dev', 'database-schema.md');\n\n  test('documentation file exists', () => {\n    expect(fs.existsSync(schemaPath)).toBe(true);\n  });\n\n  test('contains ERD diagram section', () => {\n    const content = fs.readFileSync(schemaPath, 'utf-8');\n    expect(content).toContain('SCHEMA OVERVIEW');\n    expect(content).toContain('sessions');\n    expect(content).toContain('session_segments');\n    expect(content).toContain('dictionary_entries');\n    expect(content).toContain('correction_rules');\n    expect(content).toContain('ambiguous_terms');\n    expect(content).toContain('model_installations');\n    expect(content).toContain('settings');\n    expect(content).toContain('filler_words');\n    expect(content).toContain('app_logs');\n  });\n\n  test('contains table reference for all tables', () => {\n    const content = fs.readFileSync(schemaPath, 'utf-8');\n    const tables = [\n      'sessions',\n      'session_segments',\n      'dictionary_entries',\n      'correction_rules',\n      'ambiguous_terms',\n      'model_installations',\n      'model_language_defaults',\n      'settings',\n      'filler_words',\n      'filler_removal_log',\n      'app_logs'\n    ];\n    tables.forEach(table => {\n      expect(content).toContain(`### ${table}`);\n    });\n  });\n\n  test('contains migration guide', () => {\n    const content = fs.readFileSync(schemaPath, 'utf-8');\n    expect(content).toContain('## Migration Guide');\n    expect(content).toContain('How to Add a New Table');\n    expect(content).toContain('How to Add a Column');\n    expect(content).toContain('Migration Best Practices');\n    expect(content).toContain('ALTER TABLE');\n    expect(content).toContain('CREATE TABLE');\n  });\n\n  test('contains relationship summary', () => {\n    const content = fs.readFileSync(schemaPath, 'utf-8');\n    expect(content).toContain('## Relationship Summary');\n    expect(content).toContain('sessions → session_segments');\n  });\n\n  test('contains seed data section', () => {\n    const content = fs.readFileSync(schemaPath, 'utf-8');\n    expect(content).toContain('## Seed Data');\n    expect(content).toContain('INSERT OR IGNORE');\n  });\n});\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\database-schema.spec.ts"
}

🔧 Tool: bash
Input: {
  "command": "npx playwright test docs/dev/database-schema.spec.ts --reporter=line",
  "description": "Run Playwright test for schema docs",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm ls @playwright/test 2>nul || echo \"not installed\"",
  "description": "Check if Playwright is installed",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

Playwright isn't installed in this project. Since this is documentation-only, let me verify the file exists and contains the required sections:

🔧 Tool: bash
Input: {
  "command": "echo \"=== File exists ===\" && ls -la docs/dev/database-schema.md && echo \"\" && echo \"=== Sections in file ===\" && grep -E \"^## |^### \" docs/dev/database-schema.md",
  "description": "Verify schema doc structure",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "rm docs/dev/database-schema.spec.ts",
  "description": "Remove test file",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
<summary>
## Summary: Document the SQLite schema with ERD diagrams

### Changes Implemented
- Created `docs/dev/database-schema.md` with:
  - Full ASCII ERD diagram showing all 11 tables and their relationships (1:N via foreign keys, standalone tables)
  - Complete table reference for all tables with columns, types, constraints, and descriptions
  - Migration guide with step-by-step instructions for adding tables and columns safely
  - Deprecation guidelines for obsolete columns
  - Best practices table (do/don't)
  - Migration verification steps
  - Relationship summary table
  - Seed data patterns explanation
- Updated `docs/dev/index.md` to mark the documentation as complete (removed *(todo)*)

### Files Modified
- `docs/dev/database-schema.md` (new file, ~19KB)
- `docs/dev/index.md` (removed *(todo)* marker)

### Notes for Developer
- All 11 database tables are documented with full column details
- ERD shows the 1:N relationships: sessions→session_segments, sessions→filler_removal_log, model_installations→model_language_defaults
- The migration guide follows the project's existing versioned migration pattern (rusqlite, consecutive integers, INSERT OR IGNORE for seeds)
- Documentation covers migrations v1–v8

### Verification Status
- Verified file exists and contains all required sections using grep
- Playwright test not available in this project (not installed), but the documentation structure was verified via CLI to contain: ERD diagram, 11 table references, migration guide with add table/column/deprecate sections, relationship summary, and seed data patterns
</summary>