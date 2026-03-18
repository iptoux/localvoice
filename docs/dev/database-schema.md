# SQLite Database Schema

LocalVoice uses SQLite as its single-file relational database. All persistence lives in one `localvoice.db` file located in the platform app-data directory.

## Versioned Migrations

Schema changes are applied via a **versioned migration system** defined in `src-tauri/src/db/migrations.rs`.

The `schema_migrations` table tracks applied versions:

```sql
CREATE TABLE schema_migrations (
    version    INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);
```

The `run()` function queries `MAX(version)` and executes any pending migrations in order. Keep versions consecutive and monotonically increasing.

---

## Entity-Relationship Diagram

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                              SCHEMA OVERVIEW                                  │
└──────────────────────────────────────────────────────────────────────────────┘

  ┌─────────────┐       ┌─────────────────────┐       ┌──────────────────────┐
  │  sessions   │ 1───N │  session_segments   │       │  model_installations │
  │             │──────>│                     │       │                      │
  │ id (PK)     │       │ id (PK)             │       │ id (PK)              │
  │ started_at  │       │ session_id (FK)     │       │ model_key (UNIQUE)   │
  │ ended_at    │       │ start_ms            │       │ display_name         │
  │ duration_ms │       │ end_ms              │       │ language_scope       │
  │ language    │       │ text                │       │ local_path           │
  │ raw_text    │       │ confidence          │       │ installed            │
  │ cleaned_text│       │ segment_index       │       │ created_at           │
  │ word_count  │       └─────────────────────┘       │ updated_at           │
  │ ...         │                                        └──────────┬─────────┘
  └──────┬──────┘                                                   │
         │ N                                                        │ N
         │                                                          │
         ▼                                                          ▼
  ┌─────────────────────┐       ┌──────────────────────┐   ┌────────────────────────┐
  │ filler_removal_log  │       │  model_language_     │   │  dictionary_entries     │
  │                     │       │  defaults            │   │                        │
  │ id (PK)             │       │                      │   │ id (PK)                │
  │ session_id (FK,NULL)│       │ language (PK)        │   │ phrase                 │
  │ word                │       │ model_key            │   │ normalized_phrase      │
  │ language            │       └──────────────────────┘   │ language               │
  │ removed_at          │                                    │ entry_type             │
  └─────────────────────┘                                    │ created_at             │
                                                               │ updated_at             │
  ┌────────────────────────────────────────────────────────┐   └───────────┬──────────┘
  │                 correction_rules                       │              │ 1───N
  │                                                         │              │
  │ id (PK)              is_active (BOOL)                   │              ▼
  │ source_phrase        auto_apply (BOOL)                  │   ┌──────────────────────┐
  │ normalized_source_phrase usage_count                    │   │  ambiguous_terms     │
  │ target_phrase        last_used_at                       │   │                      │
  │ language             created_at                        │   │ id (PK)              │
  │ rule_mode            updated_at                        │   │ phrase               │
  └────────────────────────────────────────────────────────┘   │ normalized_phrase     │
                                                               │ occurrences           │
  ┌────────────────────────────────────────────────────────┐   │ avg_confidence        │
  │                      settings                           │   │ last_seen_at          │
  │                                                         │   │ suggested_target      │
  │ key (PK)           Dot-notation key (e.g. app.theme)     │   │ dismissed             │
  │ value              JSON-encoded value string             │   │ created_at            │
  │ updated_at         ISO 8601 timestamp                    │   │ updated_at            │
  └────────────────────────────────────────────────────────┘   └──────────────────────┘

  ┌─────────────────────────────────────────────────────────────┐
  │                      filler_words                           │
  │                                                              │
  │ id (PK)            word                language              │
  │ is_default         created_at                               │
  └─────────────────────────────────────────────────────────────┘

  ┌─────────────────────────────────────────────────────────────┐
  │                       app_logs                              │
  │                                                              │
  │ id (PK)            level (debug|info|warn|error)             │
  │ area               message                                  │
  │ created_at                                                 │
  └─────────────────────────────────────────────────────────────┘
```

---

## Table Reference

### sessions

Primary dictation session record.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| started_at | TEXT | NOT NULL | ISO 8601 |
| ended_at | TEXT | NOT NULL | ISO 8601 |
| duration_ms | INTEGER | NOT NULL | Duration in ms |
| language | TEXT | NOT NULL | ISO 639-1 (de, en, …) |
| model_id | TEXT | NULL | Whisper model key |
| trigger_type | TEXT | NOT NULL | shortcut / button / tray |
| input_device_id | TEXT | NULL | Audio device ID |
| raw_text | TEXT | NOT NULL | Original transcription |
| cleaned_text | TEXT | NOT NULL | Post-processed text |
| word_count | INTEGER | NOT NULL DEFAULT 0 | |
| char_count | INTEGER | NOT NULL DEFAULT 0 | |
| avg_confidence | REAL | NULL | 0–1 |
| estimated_wpm | REAL | NULL | Words per minute |
| output_mode | TEXT | NOT NULL | insert / clipboard / preview |
| output_target_app | TEXT | NULL | Target app (if detectable) |
| inserted_successfully | INTEGER | NOT NULL DEFAULT 0 | Boolean |
| error_message | TEXT | NULL | |
| created_at | TEXT | NOT NULL | ISO 8601 |
| audio_path | TEXT | NULL | Added v5 — path to audio file |
| original_raw_text | TEXT | NULL | Added v5 — pre-reprocessing text |
| reprocessed_count | INTEGER | NOT NULL DEFAULT 0 | Added v5 |

**Indexes:** PRIMARY KEY on `id`; consider indexes on `started_at`, `language`.

**Relationships:** 1 session → N session_segments (ON DELETE CASCADE); N sessions → N filler_removal_log entries.

---

### session_segments

Timestamped transcription segments within a session.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| session_id | TEXT | NOT NULL, FK | sessions(id) ON DELETE CASCADE |
| start_ms | INTEGER | NOT NULL | |
| end_ms | INTEGER | NOT NULL | |
| text | TEXT | NOT NULL | |
| confidence | REAL | NULL | 0–1 |
| segment_index | INTEGER | NOT NULL | Order |

**Relationships:** N segments → 1 session.

---

### dictionary_entries

Custom vocabulary entries for domain-specific terms.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| phrase | TEXT | NOT NULL | Original phrase |
| normalized_phrase | TEXT | NOT NULL | Lowercase, whitespace-collapsed |
| language | TEXT | NULL | ISO 639-1 |
| entry_type | TEXT | NOT NULL | term / name / acronym / product / custom |
| notes | TEXT | NULL | |
| created_at | TEXT | NOT NULL | ISO 8601 |
| updated_at | TEXT | NOT NULL | ISO 8601 |

**Indexes:** UNIQUE on `(normalized_phrase, language)` recommended.

---

### correction_rules

Find-and-replace text correction rules.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| source_phrase | TEXT | NOT NULL | Text to match |
| normalized_source_phrase | TEXT | NOT NULL | Lowercase version |
| target_phrase | TEXT | NOT NULL | Replacement |
| language | TEXT | NULL | ISO 639-1 |
| rule_mode | TEXT | NOT NULL | manual / suggested / learned |
| confidence_threshold | REAL | NULL | Min confidence to auto-apply |
| is_active | INTEGER | NOT NULL DEFAULT 1 | Boolean |
| auto_apply | INTEGER | NOT NULL DEFAULT 1 | Apply on transcription |
| usage_count | INTEGER | NOT NULL DEFAULT 0 | |
| last_used_at | TEXT | NULL | ISO 8601 |
| created_at | TEXT | NOT NULL | ISO 8601 |
| updated_at | TEXT | NOT NULL | ISO 8601 |

**Indexes:** INDEX on `(normalized_source_phrase, language, is_active)`.

---

### ambiguous_terms

Low-confidence transcribed phrases flagged for user review.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| phrase | TEXT | NOT NULL | Original phrase |
| normalized_phrase | TEXT | NOT NULL | Lowercase |
| language | TEXT | NULL | ISO 639-1 |
| occurrences | INTEGER | NOT NULL DEFAULT 1 | |
| avg_confidence | REAL | NULL | Rolling average |
| last_seen_at | TEXT | NOT NULL | ISO 8601 |
| suggested_target | TEXT | NULL | Suggested replacement |
| dismissed | INTEGER | NOT NULL DEFAULT 0 | |
| created_at | TEXT | NOT NULL | ISO 8601 |
| updated_at | TEXT | NOT NULL | ISO 8601 |
| dismissed_at_occurrences | INTEGER | NOT NULL DEFAULT 0 | Added v2 — re-surface after N new occurrences |

**Indexes:** INDEX on `(normalized_phrase, language, dismissed)`.

---

### model_installations

Registry of whisper.cpp models.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| model_key | TEXT | NOT NULL UNIQUE | e.g. ggml-base |
| display_name | TEXT | NOT NULL | Human-readable name |
| language_scope | TEXT | NOT NULL | multilingual / en / de / custom |
| local_path | TEXT | NOT NULL | Filesystem path |
| file_size_bytes | INTEGER | NULL | |
| checksum | TEXT | NULL | SHA-256 |
| installed | INTEGER | NOT NULL DEFAULT 0 | |
| installed_at | TEXT | NULL | ISO 8601 |
| version | TEXT | NULL | |
| is_default_for_de | INTEGER | NOT NULL DEFAULT 0 | Deprecated v8 |
| is_default_for_en | INTEGER | NOT NULL DEFAULT 0 | Deprecated v8 |
| created_at | TEXT | NOT NULL | ISO 8601 |
| updated_at | TEXT | NOT NULL | ISO 8601 |

**Note:** `is_default_for_de`/`is_default_for_en` are deprecated; use `model_language_defaults` (v8) instead.

---

### model_language_defaults

Per-language default model assignments.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| language | TEXT | PK | ISO 639-1 |
| model_key | TEXT | NOT NULL | Default model for this language |

**Added:** v8. Replaces deprecated `is_default_for_de`/`is_default_for_en` columns.

---

### settings

Key-value store for all application settings.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| key | TEXT | PK | Dot-notation key (e.g. app.theme) |
| value | TEXT | NOT NULL | JSON-encoded string |
| updated_at | TEXT | NOT NULL | ISO 8601 |

**Seed keys (v1):**

| Key | Default |
|-----|---------|
| app.theme | system |
| app.language | de |
| app.start_hidden | false |
| app.autostart | false |
| ui.default_mode | pill |
| ui.pill.always_on_top | true |
| recording.shortcut | CommandOrControl+Shift+Space |
| recording.push_to_talk | false |
| recording.silence_timeout_ms | 1500 |
| recording.keep_audio | false (v5) |
| recording.audio_retention_days | 7 (v5) |
| recording.max_audio_storage_mb | 500 (v5) |
| transcription.default_language | auto |
| transcription.auto_punctuation | true |
| transcription.auto_capitalization | true |
| transcription.remove_fillers | false |
| output.mode | clipboard |
| output.auto_paste | false |
| output.insert_delay_ms | 100 (v4) |
| dictionary.auto_apply_rules | true |
| ambiguity.confidence_threshold | 0.6 |
| ambiguity.min_occurrences | 3 |
| notifications.on_error | true (v3) |
| notifications.on_success | false (v3) |

---

### filler_words

Language-specific filler words (äh, uh, um, …).

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| word | TEXT | NOT NULL | |
| language | TEXT | NOT NULL | ISO 639-1 |
| is_default | INTEGER | NOT NULL DEFAULT 0 | System-provided |
| created_at | TEXT | NOT NULL | ISO 8601 |

**Seeded languages:** de, en, fr, es, it, pt, nl, pl, ru, ja, zh (v6–v7).

**Indexes:** INDEX on `(language, is_default)`.

---

### filler_removal_log

Audit log of removed filler words.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| session_id | TEXT | NULL, FK | sessions(id) ON DELETE SET NULL |
| word | TEXT | NOT NULL | |
| language | TEXT | NOT NULL | ISO 639-1 |
| removed_at | TEXT | NOT NULL | ISO 8601 |

**Added:** v7.

---

### app_logs

Application logging (future SQLite-backed implementation).

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PK | UUID |
| level | TEXT | NOT NULL | debug / info / warn / error |
| area | TEXT | NOT NULL DEFAULT '' | Log component |
| message | TEXT | NOT NULL | |
| created_at | TEXT | NOT NULL | ISO 8601 |

**Added:** v3.

---

## Migration Guide

### How to Add a New Table

1. **Pick the next version number.** Check `MIGRATIONS` in `migrations.rs`; the highest key is the current version. Add version `N+1`.

2. **Write the CREATE TABLE SQL.** Use `IF NOT EXISTS` for safety on re-runs.

3. **Add seed data if needed.** Use `INSERT OR IGNORE` so re-runs are idempotent.

4. **Update this document.** Add the new table to the ERD and table reference.

5. **Add corresponding Rust models and repository.** See `src-tauri/src/db/models.rs` and `src-tauri/src/db/repositories/`.

Example:

```rust
// migrations.rs
(
    9,
    "
    CREATE TABLE IF NOT EXISTS new_feature (
        id         TEXT PRIMARY KEY,
        name       TEXT NOT NULL,
        created_at TEXT NOT NULL
    );

    INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
        ('new_feature.enabled', 'false', datetime('now'));
    ",
),
```

---

### How to Add a Column to an Existing Table

**Rule: Never use `DROP` or destructive operations on user data.**

1. **Pick the next version number.**

2. **Use `ALTER TABLE ... ADD COLUMN`.** This is safe — SQLite adds the column with NULL (or DEFAULT) for existing rows.

3. **Set a DEFAULT value if the column is NOT NULL.** SQLite requires a default for new NOT NULL columns so existing rows don't violate the constraint.

4. **Add the column to Rust models.** Update the corresponding struct in `models.rs`.

5. **Update this document.** Add the column to the table reference.

Example:

```rust
// migrations.rs
(
    9,
    "
    ALTER TABLE sessions ADD COLUMN new_field TEXT NOT NULL DEFAULT '';
    ",
),
```

```rust
// models.rs
pub struct Session {
    // ... existing fields ...
    pub new_field: String,
}
```

---

### How to Deprecate a Column

If a column becomes obsolete (like `is_default_for_de` in v8):

1. **Do NOT drop the column immediately.** It may still exist in user databases.

2. **Create a replacement table or column** (e.g., `model_language_defaults`).

3. **Add a migration to copy data** from old to new.

4. **Mark the old column as deprecated** in comments and docs.

5. **Drop the column in a future major version** after a reasonable grace period (e.g., 6–12 months).

---

### Migration Best Practices

| Do | Don't |
|----|-------|
| Use `IF NOT EXISTS` on CREATE TABLE | Drop tables or columns in migrations |
| Use `INSERT OR IGNORE` for seed data | Assume columns exist without checking |
| Keep migrations additive | Reorder or remove past migrations |
| Test migrations on existing DB | Use `SELECT *` in production queries |
| Increment version numbers consecutively | Skip version numbers |

---

### Verifying Migrations

After writing a migration, test it:

```bash
# Run the app fresh — migrations run automatically on startup
npm run tauri dev

# Or verify via the Rust test suite
cargo test --package localvoice -- migrations
```

Check the `schema_migrations` table after startup:

```sql
SELECT * FROM schema_migrations ORDER BY version;
```

---

## Relationship Summary

| Relationship | Type | ON DELETE |
|-------------|-------|-----------|
| sessions → session_segments | 1:N | CASCADE |
| sessions → filler_removal_log | 1:N | SET NULL |
| model_installations → model_language_defaults | 1:N | — |

All other tables (`settings`, `dictionary_entries`, `correction_rules`, `ambiguous_terms`, `filler_words`, `app_logs`) are standalone with no foreign keys.

---

## Seed Data

Default data is seeded via `INSERT OR IGNORE INTO` statements within the relevant migration. This pattern ensures:
- First-run: data is inserted
- Subsequent runs: rows with the same PK are skipped (idempotent)
- No errors on re-run
