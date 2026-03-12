use std::env;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Local};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct DiscoveredFile {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub extension: Option<String>,
    pub last_opened: Option<DateTime<Local>>,
}

fn user_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(user_profile) = env::var("USERPROFILE") {
        let base = Path::new(&user_profile);
        dirs.push(base.join("Desktop"));
        dirs.push(base.join("Documents"));
        dirs.push(base.join("Downloads"));
    }
    dirs
}

pub fn scan_files() -> Vec<DiscoveredFile> {
    let mut results = Vec::new();

    for root in user_dirs() {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(&root)
            .max_depth(5)
            .into_iter()
            .flatten()
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            if let Some(name_os) = path.file_name() {
                let name = name_os.to_string_lossy().to_string();
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase());
                let id = format!("file:{}", path.to_string_lossy());

                let last_opened = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .map(|st: SystemTime| DateTime::<Local>::from(st));

                results.push(DiscoveredFile {
                    id,
                    name,
                    path: path.to_path_buf(),
                    extension: ext,
                    last_opened,
                });
            }
        }
    }

    results
}

