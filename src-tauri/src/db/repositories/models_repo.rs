use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::DbConn;
use crate::errors::{AppError, CmdResult};

/// A row from the `model_installations` table, with install state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInstallation {
    pub id: String,
    pub model_key: String,
    pub display_name: String,
    pub language_scope: String,
    pub local_path: String,
    pub file_size_bytes: Option<i64>,
    pub checksum: Option<String>,
    pub installed: bool,
    pub installed_at: Option<String>,
    pub is_default_for_de: bool,
    pub is_default_for_en: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub fn upsert(
    db: &DbConn,
    model_key: &str,
    display_name: &str,
    language_scope: &str,
    local_path: &str,
    file_size_bytes: Option<u64>,
    checksum: Option<&str>,
    installed: bool,
) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO model_installations
            (id, model_key, display_name, language_scope, local_path,
             file_size_bytes, checksum, installed, installed_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
         ON CONFLICT(model_key) DO UPDATE SET
            display_name    = excluded.display_name,
            language_scope  = excluded.language_scope,
            local_path      = excluded.local_path,
            file_size_bytes = excluded.file_size_bytes,
            checksum        = excluded.checksum,
            installed       = excluded.installed,
            installed_at    = excluded.installed_at,
            updated_at      = excluded.updated_at",
        params![
            uuid::Uuid::new_v4().to_string(),
            model_key,
            display_name,
            language_scope,
            local_path,
            file_size_bytes.map(|v| v as i64),
            checksum,
            installed as i64,
            if installed { Some(now.clone()) } else { None },
            now.clone(),
            now,
        ],
    )?;
    Ok(())
}

pub fn get(db: &DbConn, model_key: &str) -> Result<Option<ModelInstallation>, AppError> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, model_key, display_name, language_scope, local_path, file_size_bytes,
                checksum, installed, installed_at, is_default_for_de, is_default_for_en,
                created_at, updated_at
         FROM model_installations WHERE model_key = ?1",
    )?;
    let mut rows = stmt.query(params![model_key])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row_to_model(row)?))
    } else {
        Ok(None)
    }
}

pub fn list_installed(db: &DbConn) -> Result<Vec<ModelInstallation>, AppError> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, model_key, display_name, language_scope, local_path, file_size_bytes,
                checksum, installed, installed_at, is_default_for_de, is_default_for_en,
                created_at, updated_at
         FROM model_installations WHERE installed = 1",
    )?;
    let rows = stmt.query_map([], |row| Ok(row_to_model_unchecked(row)))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row??);
    }
    Ok(out)
}

/// Sets the installed flag to false and clears the default flags if applicable.
pub fn mark_uninstalled(db: &DbConn, model_key: &str) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE model_installations
         SET installed = 0, installed_at = NULL,
             is_default_for_de = 0, is_default_for_en = 0, updated_at = ?1
         WHERE model_key = ?2",
        params![now, model_key],
    )?;
    Ok(())
}

/// Sets the default model for a given language ("de" | "en").
/// Clears any existing default for that language first.
pub fn set_default_for_language(
    db: &DbConn,
    language: &str,
    model_key: &str,
) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let col = match language {
        "de" => "is_default_for_de",
        "en" => "is_default_for_en",
        other => return Err(AppError(format!("Unknown language: {other}"))),
    };

    // Clear existing default for this language.
    conn.execute(
        &format!(
            "UPDATE model_installations SET {col} = 0, updated_at = ?1 WHERE {col} = 1"
        ),
        params![now],
    )?;

    // Set new default.
    conn.execute(
        &format!(
            "UPDATE model_installations SET {col} = 1, updated_at = ?1
             WHERE model_key = ?2 AND installed = 1"
        ),
        params![now, model_key],
    )?;

    Ok(())
}

/// Returns the `local_path` of the default installed model for `language`.
pub fn get_default_path(db: &DbConn, language: &str) -> Result<Option<String>, AppError> {
    let conn = db.lock().unwrap();
    let col = match language {
        "de" => "is_default_for_de",
        "en" => "is_default_for_en",
        _ => return Ok(None),
    };
    let mut stmt = conn.prepare(&format!(
        "SELECT local_path FROM model_installations
         WHERE {col} = 1 AND installed = 1 LIMIT 1"
    ))?;
    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        let path: String = row.get(0)?;
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

/// Returns the `local_path` of an installed model identified by `model_key`.
pub fn get_model_path(db: &DbConn, model_key: &str) -> CmdResult<Option<String>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT local_path FROM model_installations WHERE model_key = ?1 AND installed = 1 LIMIT 1")
        .map_err(AppError::from)?;
    let mut rows = stmt.query(rusqlite::params![model_key]).map_err(AppError::from)?;
    if let Some(row) = rows.next().map_err(AppError::from)? {
        let path: String = row.get(0).map_err(AppError::from)?;
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn row_to_model(row: &rusqlite::Row<'_>) -> rusqlite::Result<ModelInstallation> {
    Ok(ModelInstallation {
        id: row.get(0)?,
        model_key: row.get(1)?,
        display_name: row.get(2)?,
        language_scope: row.get(3)?,
        local_path: row.get(4)?,
        file_size_bytes: row.get(5)?,
        checksum: row.get(6)?,
        installed: row.get::<_, i64>(7)? != 0,
        installed_at: row.get(8)?,
        is_default_for_de: row.get::<_, i64>(9)? != 0,
        is_default_for_en: row.get::<_, i64>(10)? != 0,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}

fn row_to_model_unchecked(row: &rusqlite::Row<'_>) -> rusqlite::Result<ModelInstallation> {
    row_to_model(row)
}
