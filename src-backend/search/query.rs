use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub kind: String,
    pub icon: String,
    pub hint: String,
    pub score: f64,
}

fn like_score(query: &str, text: &str) -> f64 {
    let q = query.to_lowercase();
    let t = text.to_lowercase();
    if t == q {
        100.0
    } else if t.starts_with(&q) {
        60.0
    } else if t.contains(&q) {
        30.0
    } else {
        0.0
    }
}

pub fn search(conn: &Connection, query: &str, limit: i64) -> rusqlite::Result<Vec<SearchResult>> {
    let mut results: Vec<SearchResult> = Vec::new();

    {
        let mut stmt = conn.prepare(
            "SELECT id, name, exec_path FROM apps WHERE apps MATCH ?1 LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![query, limit], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let exec_path: String = row.get(2)?;
            let score = 50.0 + like_score(query, &name);

            Ok(SearchResult {
                id,
                title: name.clone(),
                subtitle: exec_path,
                kind: "app".into(),
                icon: name.chars().next().unwrap_or('A').to_string(),
                hint: "Enter".into(),
                score,
            })
        })?;
        for r in rows.flatten() {
            results.push(r);
        }
    }

    {
        let mut stmt = conn.prepare(
            "SELECT id, name, path, extension FROM files WHERE files MATCH ?1 LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![query, limit], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let path: String = row.get(2)?;
            let ext: String = row.get(3).unwrap_or_else(|_| "".to_string());
            let score = 40.0 + like_score(query, &name);

            Ok(SearchResult {
                id,
                title: name.clone(),
                subtitle: path,
                kind: "file".into(),
                icon: ext.chars().next().unwrap_or('F').to_string(),
                hint: "Enter".into(),
                score,
            })
        })?;
        for r in rows.flatten() {
            results.push(r);
        }
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(limit as usize);
    Ok(results)
}

