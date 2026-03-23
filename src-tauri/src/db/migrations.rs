use rusqlite::{Connection, Result};

/// Runs all pending migrations in version order.
/// Uses the `settings` table keyed "db.schema_version" as the version store,
/// but also creates a dedicated `schema_migrations` table for clarity.
pub fn run(conn: &Connection) -> Result<()> {
    // Bootstrap the migration-tracking table itself.
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version  INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        );",
    )?;

    let applied_version: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    for (version, sql) in MIGRATIONS {
        if *version > applied_version {
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, datetime('now'))",
                rusqlite::params![version],
            )?;
        }
    }

    Ok(())
}

/// Each entry is (version, SQL). Keep versions consecutive and monotonically increasing.
static MIGRATIONS: &[(i64, &str)] = &[(
    1,
    "
    CREATE TABLE IF NOT EXISTS sessions (
        id                  TEXT PRIMARY KEY,
        started_at          TEXT NOT NULL,
        ended_at            TEXT NOT NULL,
        duration_ms         INTEGER NOT NULL,
        language            TEXT NOT NULL,
        model_id            TEXT,
        trigger_type        TEXT NOT NULL,
        input_device_id     TEXT,
        raw_text            TEXT NOT NULL,
        cleaned_text        TEXT NOT NULL,
        word_count          INTEGER NOT NULL DEFAULT 0,
        char_count          INTEGER NOT NULL DEFAULT 0,
        avg_confidence      REAL,
        estimated_wpm       REAL,
        output_mode         TEXT NOT NULL,
        output_target_app   TEXT,
        inserted_successfully INTEGER NOT NULL DEFAULT 0,
        error_message       TEXT,
        created_at          TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS session_segments (
        id              TEXT PRIMARY KEY,
        session_id      TEXT NOT NULL,
        start_ms        INTEGER NOT NULL,
        end_ms          INTEGER NOT NULL,
        text            TEXT NOT NULL,
        confidence      REAL,
        segment_index   INTEGER NOT NULL,
        FOREIGN KEY(session_id) REFERENCES sessions(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS dictionary_entries (
        id                  TEXT PRIMARY KEY,
        phrase              TEXT NOT NULL,
        normalized_phrase   TEXT NOT NULL,
        language            TEXT,
        entry_type          TEXT NOT NULL,
        notes               TEXT,
        created_at          TEXT NOT NULL,
        updated_at          TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS correction_rules (
        id                      TEXT PRIMARY KEY,
        source_phrase           TEXT NOT NULL,
        normalized_source_phrase TEXT NOT NULL,
        target_phrase           TEXT NOT NULL,
        language                TEXT,
        rule_mode               TEXT NOT NULL,
        confidence_threshold    REAL,
        is_active               INTEGER NOT NULL DEFAULT 1,
        auto_apply              INTEGER NOT NULL DEFAULT 1,
        usage_count             INTEGER NOT NULL DEFAULT 0,
        last_used_at            TEXT,
        created_at              TEXT NOT NULL,
        updated_at              TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS ambiguous_terms (
        id                  TEXT PRIMARY KEY,
        phrase              TEXT NOT NULL,
        normalized_phrase   TEXT NOT NULL,
        language            TEXT,
        occurrences         INTEGER NOT NULL DEFAULT 1,
        avg_confidence      REAL,
        last_seen_at        TEXT NOT NULL,
        suggested_target    TEXT,
        dismissed           INTEGER NOT NULL DEFAULT 0,
        created_at          TEXT NOT NULL,
        updated_at          TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS model_installations (
        id                  TEXT PRIMARY KEY,
        model_key           TEXT NOT NULL UNIQUE,
        display_name        TEXT NOT NULL,
        language_scope      TEXT NOT NULL,
        local_path          TEXT NOT NULL,
        file_size_bytes     INTEGER,
        checksum            TEXT,
        installed           INTEGER NOT NULL DEFAULT 0,
        installed_at        TEXT,
        version             TEXT,
        is_default_for_de   INTEGER NOT NULL DEFAULT 0,
        is_default_for_en   INTEGER NOT NULL DEFAULT 0,
        created_at          TEXT NOT NULL,
        updated_at          TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS settings (
        key         TEXT PRIMARY KEY,
        value       TEXT NOT NULL,
        updated_at  TEXT NOT NULL
    );

    -- Seed default settings
    INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
        ('app.theme',                    'system',    datetime('now')),
        ('app.language',                 'de',        datetime('now')),
        ('app.start_hidden',             'false',     datetime('now')),
        ('app.autostart',                'false',     datetime('now')),
        ('ui.default_mode',              'pill',      datetime('now')),
        ('ui.pill.always_on_top',        'true',      datetime('now')),
        ('recording.shortcut',           'CommandOrControl+Shift+Space', datetime('now')),
        ('recording.push_to_talk',       'false',     datetime('now')),
        ('recording.silence_timeout_ms', '1500',      datetime('now')),
        ('transcription.default_language','auto',      datetime('now')),
        ('transcription.auto_punctuation','true',     datetime('now')),
        ('transcription.auto_capitalization','true',  datetime('now')),
        ('transcription.remove_fillers', 'false',     datetime('now')),
        ('output.mode',                  'clipboard', datetime('now')),
        ('output.auto_paste',            'false',     datetime('now')),
        ('dictionary.auto_apply_rules',  'true',      datetime('now')),
        ('ambiguity.confidence_threshold','0.6',      datetime('now')),
        ('ambiguity.min_occurrences',    '3',         datetime('now'));
    ",
),
(
    2,
    "
    -- Add dismissed_at_occurrences to ambiguous_terms so we can re-surface terms
    -- after they accumulate 5+ new occurrences post-dismissal.
    ALTER TABLE ambiguous_terms ADD COLUMN dismissed_at_occurrences INTEGER NOT NULL DEFAULT 0;
    ",
),
(
    3,
    "
    CREATE TABLE IF NOT EXISTS app_logs (
        id          TEXT PRIMARY KEY,
        level       TEXT NOT NULL,
        area        TEXT NOT NULL DEFAULT '',
        message     TEXT NOT NULL,
        created_at  TEXT NOT NULL
    );

    INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
        ('notifications.on_error',   'true',  datetime('now')),
        ('notifications.on_success', 'false', datetime('now'));
    ",
),
(
    4,
    "
    INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
        ('output.insert_delay_ms', '100', datetime('now'));
    ",
),
(
    5,
    "
    -- Session reprocessing columns (TASK-203)
    ALTER TABLE sessions ADD COLUMN audio_path TEXT;
    ALTER TABLE sessions ADD COLUMN original_raw_text TEXT;
    ALTER TABLE sessions ADD COLUMN reprocessed_count INTEGER NOT NULL DEFAULT 0;

    -- Audio retention and post-processing settings (TASK-210)
    INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
        ('recording.keep_audio',           'false', datetime('now')),
        ('recording.audio_retention_days', '7',     datetime('now')),
        ('recording.max_audio_storage_mb', '500',   datetime('now'));
    ",
),
(
    6,
    "
    CREATE TABLE IF NOT EXISTS filler_words (
        id          TEXT PRIMARY KEY,
        word        TEXT NOT NULL,
        language    TEXT NOT NULL,
        is_default  INTEGER NOT NULL DEFAULT 0,
        created_at  TEXT NOT NULL
    );

    -- Seed default German filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'äh',            'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ähm',           'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'öhm',           'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'hm',            'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'hmm',           'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'mhm',           'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'halt',          'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'sozusagen',     'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'quasi',         'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'irgendwie',     'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'also',          'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ja',            'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ne',            'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'naja',          'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'gewissermaßen', 'de', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'sagen wir mal', 'de', 1, datetime('now'));

    -- Seed default English filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'uh',        'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'um',        'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'uhm',       'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'hmm',       'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'hm',        'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'mhm',       'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'you know',  'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'like',      'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'basically', 'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'actually',  'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'sort of',   'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'kind of',   'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'i mean',    'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'right',     'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'well',      'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'so',        'en', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'okay so',   'en', 1, datetime('now'));
    ",
),
(
    7,
    "
    -- Filler word removal tracking
    CREATE TABLE IF NOT EXISTS filler_removal_log (
        id          TEXT PRIMARY KEY,
        session_id  TEXT,
        word        TEXT NOT NULL,
        language    TEXT NOT NULL,
        removed_at  TEXT NOT NULL
    );

    -- Seed French filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'euh',           'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'bah',           'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ben',           'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'hein',          'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'voilà',         'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'quoi',          'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'genre',         'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'du coup',       'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'en fait',       'fr', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'tu vois',       'fr', 1, datetime('now'));

    -- Seed Spanish filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'eh',            'es', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'este',          'es', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'pues',          'es', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'bueno',         'es', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'o sea',         'es', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'a ver',         'es', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'sabes',         'es', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'mira',          'es', 1, datetime('now'));

    -- Seed Italian filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'allora',        'it', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'cioè',          'it', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'quindi',        'it', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'praticamente',  'it', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'tipo',          'it', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ecco',          'it', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'vabbè',         'it', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'insomma',       'it', 1, datetime('now'));

    -- Seed Portuguese filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'né',            'pt', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'então',         'pt', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'tipo',          'pt', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'sabe',          'pt', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'assim',         'pt', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ahn',           'pt', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'bom',           'pt', 1, datetime('now'));

    -- Seed Dutch filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'eh',            'nl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'uhm',           'nl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'nou',           'nl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'eigenlijk',     'nl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'gewoon',        'nl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'zeg maar',      'nl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'weet je',       'nl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'toch',          'nl', 1, datetime('now'));

    -- Seed Polish filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'eee',           'pl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'yyy',           'pl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'no',            'pl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'właśnie',       'pl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'znaczy',        'pl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'tak jakby',     'pl', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'wiesz',         'pl', 1, datetime('now'));

    -- Seed Russian filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'э',             'ru', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ну',            'ru', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'вот',           'ru', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'значит',        'ru', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'короче',        'ru', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'типа',          'ru', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'как бы',        'ru', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'понимаешь',     'ru', 1, datetime('now'));

    -- Seed Japanese filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), 'えーと',        'ja', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'あの',          'ja', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'まあ',          'ja', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'なんか',        'ja', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'ちょっと',      'ja', 1, datetime('now')),
        (lower(hex(randomblob(16))), 'そうですね',    'ja', 1, datetime('now'));

    -- Seed Chinese filler words
    INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES
        (lower(hex(randomblob(16))), '那个',          'zh', 1, datetime('now')),
        (lower(hex(randomblob(16))), '就是',          'zh', 1, datetime('now')),
        (lower(hex(randomblob(16))), '然后',          'zh', 1, datetime('now')),
        (lower(hex(randomblob(16))), '这个',          'zh', 1, datetime('now')),
        (lower(hex(randomblob(16))), '嗯',            'zh', 1, datetime('now')),
        (lower(hex(randomblob(16))), '啊',            'zh', 1, datetime('now')),
        (lower(hex(randomblob(16))), '对对对',        'zh', 1, datetime('now'));
    ",
),
(
    8,
    "
    -- Generic per-language model defaults (replaces is_default_for_de / is_default_for_en columns).
    CREATE TABLE IF NOT EXISTS model_language_defaults (
        language    TEXT PRIMARY KEY,
        model_key   TEXT NOT NULL
    );

    -- Migrate existing de/en defaults into the new table.
    INSERT OR IGNORE INTO model_language_defaults (language, model_key)
        SELECT 'de', model_key FROM model_installations WHERE is_default_for_de = 1 AND installed = 1 LIMIT 1;
    INSERT OR IGNORE INTO model_language_defaults (language, model_key)
        SELECT 'en', model_key FROM model_installations WHERE is_default_for_en = 1 AND installed = 1 LIMIT 1;
    ",
),
(
    9,
    "
    -- Preserve original transcription metadata before first reprocess.
    ALTER TABLE sessions ADD COLUMN original_model_id TEXT;
    ALTER TABLE sessions ADD COLUMN original_language TEXT;
    ALTER TABLE sessions ADD COLUMN original_avg_confidence REAL;
    ",
)];

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn open_in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        conn
    }

    #[test]
    fn all_migrations_apply_cleanly() {
        let conn = open_in_memory();
        run(&conn).expect("migrations should apply without error");

        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        for expected in &[
            "app_logs",
            "ambiguous_terms",
            "correction_rules",
            "dictionary_entries",
            "filler_words",
            "model_installations",
            "model_language_defaults",
            "schema_migrations",
            "session_segments",
            "sessions",
            "settings",
        ] {
            assert!(
                tables.contains(&expected.to_string()),
                "table '{}' should exist after migrations",
                expected
            );
        }
    }

    #[test]
    fn migrations_are_idempotent() {
        let conn = open_in_memory();
        run(&conn).expect("first run should succeed");
        run(&conn).expect("second run should not error (idempotent)");
    }

    #[test]
    fn schema_version_tracks_all_migrations() {
        let conn = open_in_memory();
        run(&conn).unwrap();
        let version: i64 = conn
            .query_row(
                "SELECT MAX(version) FROM schema_migrations",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(version, MIGRATIONS.len() as i64);
    }

    #[test]
    fn default_settings_are_seeded() {
        let conn = open_in_memory();
        run(&conn).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM settings", [], |r| r.get(0))
            .unwrap();
        assert!(count > 0, "default settings should be seeded");
    }

    #[test]
    fn filler_words_are_seeded() {
        let conn = open_in_memory();
        run(&conn).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM filler_words", [], |r| r.get(0))
            .unwrap();
        assert!(count > 0, "default filler words should be seeded");
    }

    #[test]
    fn sessions_table_has_reprocessing_columns() {
        let conn = open_in_memory();
        run(&conn).unwrap();
        // If migration 5/9 applied correctly, these columns must exist.
        conn.execute(
            "INSERT INTO sessions
                (id, started_at, ended_at, duration_ms, language, trigger_type,
                 raw_text, cleaned_text, word_count, char_count, output_mode,
                 inserted_successfully, created_at, reprocessed_count,
                 original_model_id, original_language)
             VALUES ('test', datetime('now'), datetime('now'), 1000, 'de',
                     'hotkey', '', '', 0, 0, 'clipboard', 0, datetime('now'), 0,
                     NULL, NULL)",
            [],
        )
        .expect("session insert with reprocessing columns should succeed");
    }
}
