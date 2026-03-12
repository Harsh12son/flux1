// Backend implementation used by Tauri.
// We expose the search, system and commands modules explicitly and then
// include the main backend logic from `src-backend/main.rs`.

pub mod commands {
    pub mod open_app {
        include!("../../src-backend/commands/open_app.rs");
    }
    pub mod open_file {
        include!("../../src-backend/commands/open_file.rs");
    }
}

pub mod system {
    pub mod app_scanner {
        include!("../../src-backend/system/app_scanner.rs");
    }
    pub mod file_scanner {
        include!("../../src-backend/system/file_scanner.rs");
    }
}

pub mod search {
    pub mod indexer {
        include!("../../src-backend/search/indexer.rs");
    }
    pub mod query {
        include!("../../src-backend/search/query.rs");
    }
}

include!("../../src-backend/main.rs");

