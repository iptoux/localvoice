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
        ('transcription.default_language','de',       datetime('now')),
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
)];
