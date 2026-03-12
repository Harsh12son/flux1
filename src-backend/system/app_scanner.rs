use std::env;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct DiscoveredApp {
    pub id: String,
    pub name: String,
    pub exec_path: PathBuf,
    pub icon_path: Option<PathBuf>,
}

fn start_menu_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(program_data) = env::var("PROGRAMDATA") {
        paths.push(
            Path::new(&program_data)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs"),
        );
    }

    if let Ok(appdata) = env::var("APPDATA") {
        paths.push(
            Path::new(&appdata)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs"),
        );
    }

    paths
}

pub fn scan_apps() -> Vec<DiscoveredApp> {
    let mut apps = Vec::new();

    for root in start_menu_paths() {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(&root)
            .follow_links(true)
            .into_iter()
            .flatten()
        {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("lnk") || ext.eq_ignore_ascii_case("url") {
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Application")
                        .to_string();

                    // For performance and portability, treat the .lnk itself as the executable path.
                    // Windows will resolve the shortcut when launched via "cmd /C start".
                    let exec_path = path.to_path_buf();
                    let id = format!("app:{}", path.to_string_lossy());

                    apps.push(DiscoveredApp {
                        id,
                        name,
                        exec_path,
                        icon_path: None,
                    });
                }
            }
        }
    }

    apps
}

