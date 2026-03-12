#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod backend;

use std::sync::Mutex;

use backend::{database_path, execute_result, init_index, search_index, DbState};
use backend::search::indexer::open_or_create_database;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{Builder as ShortcutBuilder, Code, Modifiers, ShortcutState};

fn main() {
  let context = tauri::generate_context!();

  let init_summary = init_index();
  println!(
    "Flux Launcher index ready: {} apps, {} files",
    init_summary.apps_indexed, init_summary.files_indexed
  );

  let db_path = database_path();
  let conn = open_or_create_database(&db_path).expect("db open failed");

  tauri::Builder::default()
    .manage(DbState {
      conn: Mutex::new(conn),
    })
    .invoke_handler(tauri::generate_handler![search_index, execute_result])
    .plugin(
      ShortcutBuilder::new()
        .with_shortcuts(["Alt+Space"])
        .expect("failed to register Alt+Space shortcut")
        .with_handler(|app, shortcut, event| {
          if event.state == ShortcutState::Pressed
            && shortcut.matches(Modifiers::ALT, Code::Space)
          {
            if let Some(window) = app.get_webview_window("main") {
              let _ = window.show();
              let _ = window.set_focus();
            }
            let _ = app.emit("show_launcher", ());
          }
        })
        .build(),
    )
    .run(context)
    .expect("error while running Flux Launcher");
}


