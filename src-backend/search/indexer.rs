use std::path::Path;

use chrono::{DateTime, Local};
use rusqlite::{params, Connection};

use crate::backend::system::{app_scanner::DiscoveredApp, file_scanner::DiscoveredFile};

pub fn ensure_schema(conn: &Connection) -> rusqlite::Result<()> {
    // FTS5 tables for apps and files plus a small metadata table
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS meta (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS apps
        USING fts5(id UNINDEXED, name, exec_path, icon_path, tokenize = "unicode61");

        CREATE VIRTUAL TABLE IF NOT EXISTS files
        USING fts5(id UNINDEXED, name, path, extension, last_opened, tokenize = "unicode61");
        "#,
    )?;

    Ok(())
}

pub fn index_apps(conn: &mut Connection, apps: &[DiscoveredApp]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;
    {
        let mut stmt_clear = tx.prepare("DELETE FROM apps")?;
        stmt_clear.execute([])?;

        let mut insert = tx.prepare(
            "INSERT INTO apps (id, name, exec_path, icon_path) VALUES (?1, ?2, ?3, ?4)",
        )?;
        for app in apps {
            insert.execute(params![
                app.id,
                app.name,
                app.exec_path.to_string_lossy(),
                app.icon_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
            ])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn index_files(conn: &mut Connection, files: &[DiscoveredFile]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;
    {
        let mut insert = tx.prepare(
            "INSERT INTO files (id, name, path, extension, last_opened)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;

        for file in files {
            let last_opened: DateTime<Local> = file
                .last_opened
                .unwrap_or_else(|| DateTime::<Local>::from(std::time::SystemTime::now()));

            insert.execute(params![
                file.id,
                file.name,
                file.path.to_string_lossy(),
                file.extension.as_deref().unwrap_or(""),
                last_opened.to_rfc3339()
            ])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn open_or_create_database(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    ensure_schema(&conn)?;
    Ok(conn)
}

