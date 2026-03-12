use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::Connection;
use serde::Serialize;
use tauri::State;

use crate::backend::search::indexer::{index_apps, index_files, open_or_create_database};
use crate::backend::search::query::{search as search_index_impl, SearchResult};
use crate::backend::system::app_scanner::scan_apps;
use crate::backend::system::file_scanner::scan_files;

pub struct DbState {
    pub conn: Mutex<Connection>,
}

pub fn database_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push("database");
    std::fs::create_dir_all(&path).ok();
    path.push("index.db");
    path
}

#[tauri::command]
pub fn search_index(query: String, state: State<'_, DbState>) -> Result<Vec<SearchResult>, String> {
    let conn = state.conn.lock().unwrap();
    search_index_impl(&conn, &query, 50).map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
pub fn execute_result(id: String, state: State<'_, DbState>) -> Result<(), String> {
    // lock to ensure we keep a single connection alive while commands execute
    let _conn = state.conn.lock().unwrap();

    if let Some(path) = id.strip_prefix("app:") {
        crate::backend::commands::open_app::open_app(path)
            .map_err(|e: std::io::Error| e.to_string())?;
        Ok(())
    } else if let Some(path) = id.strip_prefix("file:") {
        crate::backend::commands::open_file::open_file(path)
            .map_err(|e: std::io::Error| e.to_string())?;
        Ok(())
    } else if let Some(rest) = id.strip_prefix("cmd:") {
        execute_builtin_command(rest)
    } else if let Some(rest) = id.strip_prefix("plugin:") {
        // plugin execution is delegated to front-end logic; nothing to do here for now
        let _ = rest;
        Ok(())
    } else {
        Err("Unknown result id".into())
    }
}

fn execute_builtin_command(name: &str) -> Result<(), String> {
    match name {
        "shutdown" => std::process::Command::new("shutdown")
            .args(["/s", "/t", "0"])
            .spawn()
            .map(|_| ())
            .map_err(|e: std::io::Error| e.to_string()),
        "restart" => std::process::Command::new("shutdown")
            .args(["/r", "/t", "0"])
            .spawn()
            .map(|_| ())
            .map_err(|e: std::io::Error| e.to_string()),
        _ => Err(format!("Unknown builtin command: {name}")),
    }
}

#[derive(Serialize)]
pub struct InitSummary {
    pub apps_indexed: usize,
    pub files_indexed: usize,
}

pub fn init_index() -> InitSummary {
    let db_path = database_path();
    let mut conn = open_or_create_database(&db_path).expect("failed to open database");

    let apps = scan_apps();
    let files = scan_files();
    index_apps(&mut conn, &apps).expect("failed to index apps");
    index_files(&mut conn, &files).expect("failed to index files");

    InitSummary {
        apps_indexed: apps.len(),
        files_indexed: files.len(),
    }
}

