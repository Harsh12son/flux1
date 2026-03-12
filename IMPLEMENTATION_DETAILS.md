# Flux Launcher – Full Implementation Details and Rationale ✅

This document explains **every major file and directory** that was created for Flux Launcher and **why** it exists. It is meant as a developer-facing deep dive so you can quickly understand, modify, or extend the project.

---

## Root Project (JavaScript / TypeScript side)

### `package.json`

- **What it is**: Node project manifest.
- **Why**:
  - Defines dependencies for the **frontend** (React, Tailwind, Tauri JS API).
  - Adds scripts for:
    - `dev` – run Vite dev server.
    - `build` – produce a production build of the frontend.
    - `preview` – preview static build.
    - `lint` – lint TypeScript/React.
    - `tauri:dev` – run Tauri app in dev mode (starts Vite, then Tauri).
    - `tauri:build` – build a production-ready desktop app.

### `tsconfig.json`

- **What**: TypeScript compiler configuration.
- **Why**:
  - Enables strict typing (`strict: true`) to catch errors early.
  - Sets module, target, JSX mode, and module resolution for a Vite + React app.
  - Configures `baseUrl` and path aliases:
    - `@frontend/*` → `src/frontend/*`
    - `@plugins/*` → `plugins/*`
  - Ensures only relevant directories are included in type-checking (`src/frontend`, `plugins`).

### `vite.config.ts`

- **What**: Vite bundler configuration.
- **Why**:
  - Uses `@vitejs/plugin-react-swc` for fast TypeScript/JSX transformation.
  - Defines path aliases that match `tsconfig.json`.
  - Configures dev server port `5173` (used in `tauri.conf.json`).
  - Sets build output directory to `dist` (consumed by Tauri in production).

### `tailwind.config.cjs`

- **What**: Tailwind CSS configuration.
- **Why**:
  - Tells Tailwind which files to scan for class names (`index.html`, `src/frontend/**/*.{ts,tsx}`).
  - Defines a **dark theme palette** that matches the Raycast-style UI:
    - `background`, `surface`, `surfaceAlt`, `accent`, `textPrimary`, `textSecondary`.
  - Adds `shadow.launcher` for a soft drop shadow around the launcher window.

### `postcss.config.cjs`

- **What**: PostCSS pipeline configuration.
- **Why**:
  - Wires `tailwindcss` and `autoprefixer` so Tailwind classes compile correctly and CSS is vendor-prefixed for browsers.

### `index.html`

- **What**: Vite entry HTML file.
- **Why**:
  - Defines the base HTML page into which React renders.
  - Sets `<html class="dark">` for dark mode by default.
  - Creates `<div id="root"></div>` – target for `ReactDOM.createRoot`.
  - Loads the React entry script `src/frontend/main.tsx`.

### `Cargo.toml` (root workspace)

- **What**: Rust workspace definition.
- **Why**:
  - Declares the Rust workspace so that `src-tauri` is the actual Rust/Tauri crate.
  - Keeps frontend (Node) and backend (Rust) in one cohesive project.

---

## Frontend (React + TypeScript) – `src/frontend`

This is the UI that the user sees when the launcher is opened.

### `src/frontend/styles/global.css`

- **What**: Global stylesheet (Tailwind-based).
- **Why**:
  - Imports Tailwind directives (`@tailwind base;`, `components;`, `utilities;`).
  - Sets global body styles: dark background, anti-aliasing.
  - Defines custom utility classes:
    - `.flux-launcher-backdrop` – full-screen translucent background behind launcher.
    - `.flux-launcher-shell` – centered, rounded container for the launcher.
    - `.flux-command-input` – styling for the search input.
    - `.flux-result-row` / `.flux-result-row-active` – result row states.
    - `.flux-kbd` – small keyboard-hint badges (`Esc`, `Enter`, etc.).

### `src/frontend/main.tsx`

- **What**: React entry point.
- **Why**:
  - Bootstraps the React application into the `#root` element from `index.html`.
  - Wraps `App` in `React.StrictMode` to catch common issues.
  - Imports `global.css` so Tailwind and custom styles apply globally.

### `src/frontend/App.tsx`

- **What**: Top-level React component.
- **Why**:
  - Simple wrapper that currently renders the `Launcher` page.
  - Acts as a future expansion point (e.g. multiple pages, settings view) while keeping the main launch UI isolated in `Launcher.tsx`.

---

## Frontend – Pages

### `src/frontend/pages/Launcher.tsx`

- **What**: The **main launcher UI** component (Raycast-style panel).
- **Why**:
  - Implements the core behavior:
    - Shows the command bar (input).
    - Shows search results (apps, files, plugin-provided items).
    - Handles keyboard navigation and execution.
    - Listens for Tauri events to show/hide the window.
  - Key responsibilities:
    - Uses `useSearch` to talk to the Rust backend’s `search_index` command.
    - Uses `usePlugins` to merge plugin-generated results with backend search results.
    - Uses `useKeyboardNav` to track which result is currently active.
    - Listens for `show_launcher` event: resets state and focuses the window when triggered by the Alt+Space global hotkey.
    - On `Enter`, it:
      - Calls `executePluginResult` if the result’s `id` starts with `plugin:`.
      - Otherwise calls `executeResult` (Tauri) to execute apps/files/system commands.
    - On `Esc`, hides the window via `appWindow.hide()`.

---

## Frontend – Components

### `src/frontend/components/CommandBar.tsx`

- **What**: The search input at the top of the launcher.
- **Why**:
  - Provides a minimal, keyboard-focused input area with:
    - Left label (⌘) as a small visual marker.
    - A text input that auto-focuses and captures key events (up/down/enter/esc).
    - Right-hand side keyboard hint badges (`↑↓`, `Enter`, `Esc`).
  - The component is intentionally “dumb”: it gets `value`, `onChange`, and `onKeyDown` from the parent so the parent controls the entire behavior.

### `src/frontend/components/ResultItem.tsx`

- **What**: Renders an individual search result row.
- **Why**:
  - Presents the information you asked for per result:
    - **Icon** – derived from the first character or a provided emoji/letter.
    - **Title** – name of the app/file/command.
    - **Subtitle** – path, web search description, expression, etc.
    - **Shortcut hint** – typically `Enter`, can be extended.
    - **Kind badge** – app/file/command/plugin.
  - Uses `classnames` (as `clsx`) to toggle active styling when selected via keyboard.
  - Handles `onMouseDown` instead of `onClick` to avoid losing input focus during rapid keyboard interaction.

### `src/frontend/components/ResultList.tsx`

- **What**: Renders a scrollable list of results.
- **Why**:
  - Encapsulates the mapping of result data to `ResultItem` components.
  - Handles the “empty state” message:
    - Shows instructions when there are no results yet.
  - Limits max height and enables vertical scrolling for long result lists.

---

## Frontend – Hooks

### `src/frontend/hooks/useKeyboardNav.ts`

- **What**: Manages keyboard-based selection among results.
- **Why**:
  - Abstracts the index tracking and wrap-around logic.
  - Exposes:
    - `activeIndex`
    - `moveUp()`, `moveDown()` – with wrap-around when at top or bottom.
    - `reset()` – sets active index back to 0 when needed.
  - Keeps navigation logic separate from UI components so it’s easier to test/extend.

### `src/frontend/hooks/useSearch.ts`

- **What**: Talks to the Rust backend’s `search_index` and `execute_result` commands.
- **Why**:
  - Encapsulates:
    - `query` state and its setter.
    - `results` – typed search results.
    - `isSearching` – flags active searches (handy for spinners, debug).
    - A tiny debounce (~35 ms) to avoid flooding the backend while keeping latency <50 ms.
  - Uses Tauri’s `invoke`:
    - `search_index` – for live search over SQLite FTS5.
    - `execute_result` – to trigger execution of non-plugin results.
  - Designed to be focused on **backend-based** search only; plugins are layered via a separate hook (`usePlugins`).

### `src/frontend/hooks/usePlugins.ts`

- **What**: Client-side plugin system integration.
- **Why**:
  - Loads and manages all plugins (currently statically imported: calculator, web-search, clipboard).
  - For each query:
    - Filters plugins whose `trigger` matches the start of the query.
    - Calls the plugin’s `provideResults(query)` method.
    - Returns combined plugin results to the page in a typed form (`SearchResult`).
  - Exposes `executePluginResult(id)`:
    - Locates the plugin by `id` prefix (e.g. `plugin:calculator:...`).
    - Delegates execution to the plugin’s `execute` function.
  - Separates plugin logic from `Launcher.tsx`, keeping the page lean.

---

## Plugin System – `plugins/`

All plugins share a common contract defined in `plugins/types.ts`.

### `plugins/types.ts`

- **What**: Shared TypeScript types for plugins.
- **Why**:
  - Defines `PluginDefinition`:
    - `name`, `description`, `trigger`
    - `provideResults(query): SearchResult[]`
    - `execute(id: string): void | Promise<void>`
  - Ensures every plugin returns search results in a format compatible with the main search result list.

---

### `plugins/calculator/plugin.json`

- **What**: Metadata for the calculator plugin.
- **Why**:
  - States:
    - Name: `calculator`
    - Description: “Evaluate math expressions”
    - Trigger: `=`
  - Mirrors the plugin system design where each plugin declares its trigger and purpose.

### `plugins/calculator/index.ts`

- **What**: Implementation of the calculator plugin.
- **Why**:
  - Allows queries like `=2+2` to produce an instant calculation result.
  - Uses a **small, controlled evaluator**:
    - Cleans the expression, allowing only digits, operators, parentheses, and decimal points.
    - Uses `new Function` to safely evaluate the sanitized expression.
  - `provideResults`:
    - Returns a single high-score `SearchResult` with the computed value as title and expression as subtitle.
  - `execute`:
    - Copies the resulting value to the clipboard for quick reuse.

---

### `plugins/web-search/plugin.json`

- **What**: Metadata for the web search plugin.
- **Why**:
  - States:
    - Name: `web-search`
    - Description: “Search the web via browser”
    - Trigger: `g `
  - Aligns closely with your requirement `g react tutorial → opens Google`.

### `plugins/web-search/index.ts`

- **What**: Implementation of the web search plugin.
- **Why**:
  - Allows queries like `g react tutorial`:
    - `provideResults` returns a single result labeled “Search Google for 'react tutorial'”.
  - `execute`:
    - Constructs the Google search URL.
    - Opens the default browser via `window.open` (the OS will route to the default browser).

---

### `plugins/clipboard/plugin.json`

- **What**: Metadata for clipboard history plugin.
- **Why**:
  - States:
    - Name: `clipboard`
    - Description: “Session clipboard history”
    - Trigger: `clip `
  - Matches your requirement for clipboard history as a built-in feature, implemented as a plugin.

### `plugins/clipboard/index.ts`

- **What**: Implementation of clipboard history plugin.
- **Why**:
  - Maintains an in-memory `history` array of unique clipboard entries (up to 20).
  - `provideResults`:
    - When the query starts with `clip`, exposes existing history items as results.
  - `execute`:
    - On first trigger or invalid id: attempts to read from `navigator.clipboard` and populate history.
    - On valid result selection: writes the selected string back to the clipboard.
  - Operates entirely on the frontend to keep latency low and avoid cross-platform clipboard complexity in Rust.

---

## Backend Logic – `src-backend`

This folder holds the Rust backend logic reused by Tauri.

### `src-backend/search/indexer.rs`

- **What**: SQLite + FTS5 schema management and indexing functions.
- **Why**:
  - `ensure_schema`:
    - Creates `meta` table for simple metadata (reserved).
    - Creates **FTS5 virtual tables**:
      - `apps(id, name, exec_path, icon_path)`
      - `files(id, name, path, extension, last_opened)`
    - Uses `unicode61` tokenizer for better Unicode support.
  - `index_apps`:
    - Clears existing app entries.
    - Inserts all discovered apps in a transaction for speed.
  - `index_files`:
    - Inserts discovered file metadata in a transaction.
    - Stores `last_opened` as RFC3339 for potential future sorting.
  - `open_or_create_database`:
    - Opens a SQLite DB at the given path and ensures schema is created.

### `src-backend/search/query.rs`

- **What**: High-performance query logic for apps and files.
- **Why**:
  - Defines `SearchResult` struct serialized back to JS.
  - `search(conn, query, limit)`:
    - Executes FTS5 queries on both `apps` and `files` tables.
    - Uses a small `like_score` function to prioritize:
      - Exact match: +100
      - Prefix: +60
      - Substring: +30
    - Combines FTS scores with these heuristics to approximate:
      1. Exact match
      2. Prefix match
      3. Fuzzy-ish match
    - Sorts results by score descending and truncates to `limit`.
  - Designed for **fast lookup** with simple SQL + FTS operations.

### `src-backend/system/app_scanner.rs`

- **What**: Windows Start Menu app discovery.
- **Why**:
  - Implements requirement: search installed apps by scanning:
    - `C:\ProgramData\Microsoft\Windows\Start Menu\Programs`
    - `%APPDATA%\Microsoft\Windows\Start Menu\Programs`
  - Uses `walkdir` to walk directories and find `.lnk` and `.url` files.
  - For each found file:
    - Derives `name` from file stem.
    - Uses the `.lnk`/`.url` path itself as `exec_path` (Windows resolves these through `start`).
    - Computes a stable `id` of the form `app:<path>`.
  - `scan_apps` returns a vector of `DiscoveredApp` ready to be indexed.

### `src-backend/system/file_scanner.rs`

- **What**: Desktop/Documents/Downloads file discovery.
- **Why**:
  - Implements requirement: index files from:
    - `<USER>\Desktop`
    - `<USER>\Documents`
    - `<USER>\Downloads`
  - `user_dirs`:
    - Resolves `%USERPROFILE%` and builds the three target directories.
  - `scan_files`:
    - Walks the directories (max depth 5 for performance).
    - Skips non-files.
    - For each file:
      - Gets `name`, `extension`, full `path`.
      - Computes `last_opened` as modified time from filesystem metadata.
      - Builds a `DiscoveredFile` with id `file:<path>`.
  - Returns a list ready to be inserted into the `files` FTS table.

### `src-backend/system/mod.rs`

- **What**: Module declaration for `system` namespace.
- **Why**:
  - Exposes `app_scanner` and `file_scanner` modules to the rest of the backend.

---

### `src-backend/commands/open_app.rs`

- **What**: App launching logic.
- **Why**:
  - Uses Windows `cmd /C start "" <target>` to:
    - Execute `.lnk` and `.url` shortcuts properly.
    - Delegate path resolution to the shell.
  - Allows launching apps from search results with minimal extra logic.

### `src-backend/commands/open_file.rs`

- **What**: File opening logic.
- **Why**:
  - Similar to `open_app`, uses `cmd /C start "" <path>` to open a file with its default associated application.
  - Satisfies the requirement for opening files via the launcher.

### `src-backend/commands/mod.rs`

- **What**: Commands module aggregator.
- **Why**:
  - Exposes `open_app` and `open_file` modules as a cohesive `commands` namespace within the backend.

---

### `src-backend/main.rs`

- **What**: Core backend logic used by Tauri commands.
- **Why**:
  - Provides:
    - **Global database connection state** (via `once_cell` + `parking_lot::Mutex`).
    - `database_path()` helper:
      - Ensures `database/` directory exists at project root.
      - Targets `database/index.db` (matching your required layout).
    - `DbState`:
      - Managed by Tauri as application state.
      - Provides a clonable `Connection` to SQLite DB for each command.
  - Tauri commands:
    - `search_index(query: String, state: State<DbState>)`:
      - Opens/clones the DB connection.
      - Calls `search_index_impl` (from `search/query.rs`).
      - Returns `Vec<SearchResult>` to the frontend.
    - `execute_result(id: String, state: State<DbState>)`:
      - Decodes the `id` prefix:
        - `app:` → queries app exec path in DB and calls `open_app`.
        - `file:` → calls `open_file` with real path.
        - `cmd:` → calls `execute_builtin_command`.
        - `plugin:` → currently a no-op (plugins execute on frontend).
  - `execute_builtin_command`:
    - Implements built-in system actions:
      - `shutdown` → `shutdown /s /t 0`
      - `restart` → `shutdown /r /t 0`
  - `InitSummary` and `init_index()`:
    - On app startup, scans apps and files.
    - Indexes them into SQLite (`index_apps`, `index_files`).
    - Returns basic counts for logging / debugging.

---

## Tauri Shell – `src-tauri`

### `src-tauri/Cargo.toml`

- **What**: Tauri Rust crate manifest.
- **Why**:
  - Declares the `flux-launcher` binary crate for Tauri.
  - Adds dependencies:
    - `tauri` – desktop framework.
    - `rusqlite`, `walkdir`, `chrono`, `once_cell`, `parking_lot`, `regex`, `arboard` – backend utilities.
    - `serde`, `serde_json` – serialization for command arguments/returns.
    - `tokio` – allows async tasks if needed (not heavily used yet).
  - Enables Tauri features:
    - `shell-open`, `global-shortcut`, `window-all`.

### `src-tauri/tauri.conf.json`

- **What**: Tauri application configuration.
- **Why**:
  - `build`:
    - `beforeDevCommand`: `npm run dev` – start Vite before Tauri dev.
    - `beforeBuildCommand`: `npm run build` – build frontend assets before bundling.
    - `devPath`: `http://localhost:5173` – where Tauri loads content during dev.
    - `distDir`: `../dist` – where Tauri loads assets in production.
  - `package`:
    - Specifies product name and version shown in Windows metadata.
  - `tauri.windows[0]`:
    - Main window is:
      - Borderless (`decorations: false`).
      - Transparent.
      - Always on top.
      - Not visible at start (`visible: false`) – it is shown via the global hotkey.
      - Fixed size (width: 800, height: 520).
  - `allowlist`:
    - Grants permission only to the APIs needed:
      - `shell.open` – open system shell/URLs.
      - `window.*` – control the Tauri window.
      - `globalShortcut.*` – register the Alt+Space global hotkey.

### `src-tauri/src/main.rs`

- **What**: Tauri entry point.
- **Why**:
  - Initializes the backend index on startup by calling `backend::init_index()`.
  - Sets up Tauri with:
    - Managed `DbState` for DB access.
    - Invoke handlers for `search_index` and `execute_result` (from backend).
  - In `.setup`:
    - Gets the main window and centers it.
    - Registers the **global shortcut Alt+Space** using `global_shortcut_manager`:
      - On trigger:
        - Shows the window.
        - Brings it to front (`set_focus`).
        - Emits `show_launcher` event so frontend can reset state/focus.
  - Finally calls `.run(context)` to start the Tauri event loop.

### `src-tauri/src/backend.rs`

- **What**: Small module that re-exports backend symbols for Tauri.
- **Why**:
  - Tauri’s entry point expects to import backend functions from `src-tauri`.
  - This file exposes everything from `root_backend` (which includes the actual logic from `src-backend/main.rs`), making the backend implementation shareable without duplication.

### `src-tauri/src/root_backend.rs`

- **What**: Adapter that includes the backend implemented in `src-backend/main.rs`.
- **Why**:
  - Uses `include!("../../src-backend/main.rs");`:
    - This compiles the actual backend code as part of the Tauri crate, while the source logically “lives” in `src-backend`.
  - Satisfies your requested project layout while keeping Tauri’s structure happy.

---

## Database – `database/index.db`

- **What**: SQLite database file (initially empty, then populated by backend).
- **Why**:
  - Central storage for:
    - `apps` FTS5 index (installed applications).
    - `files` FTS5 index (user files).
    - `meta` table (future metadata).
  - Chosen to satisfy:
    - Local indexed search with FTS5.
    - Low-latency lookups (<50 ms).
    - No external DB server dependency – portable and easy to ship.

The file is created (if missing) and managed by `src-backend/search/indexer.rs` via `open_or_create_database`.

---

## README and Documentation

### `README.md`

- **What**: High-level project overview and usage guide.
- **Why**:
  - Summarizes:
    - Tech stack: Tauri + Rust + React + SQLite FTS5.
    - Project structure.
    - Setup instructions (Node, Rust/Tauri, npm scripts).
    - How indexing works and where it scans.
    - Core features and how to trigger them (e.g. Alt+Space).
  - Designed as user + developer entry documentation.

### `IMPLEMENTATION_DETAILS.md` (this file)

- **What**: Deep internal documentation of *everything created and why*.
- **Why**:
  - Serves as the “map” of the entire implementation:
    - If you want to modify something (UI, search, plugins, indexing, commands), you can quickly locate the relevant file(s).
    - Captures architectural choices and their reasoning (e.g., FTS5 for search, `.lnk` handling via `start`, plugin system on frontend).
  - Helps future contributors or your future self understand the system without reverse-engineering the codebase.

---

## How These Pieces Work Together (High-Level Flow)

1. **Startup**
   - Tauri starts, `main.rs` runs.
   - `backend::init_index()`:
     - Opens/creates `database/index.db`.
     - Scans apps and files.
     - Indexes them into FTS5 tables.
   - Tauri registers **Alt+Space** global hotkey and keeps window hidden by default.

2. **User presses Alt+Space**
   - Global shortcut callback:
     - Shows the transparent, borderless launcher window.
     - Focuses it.
     - Emits `show_launcher` event.
   - Frontend `Launcher` listens for this event:
     - Resets query and selection.
     - Ensures focus is on the command input.

3. **User types a query**
   - Query entered in `CommandBar` updates `useSearch` hook.
   - `useSearch`:
     - Waits ~35ms (tiny debounce).
     - Calls `invoke("search_index", { query })`.
   - Rust backend `search_index`:
     - Uses SQLite FTS5 to search `apps` and `files` with ranking.
     - Returns results to frontend.
   - In parallel, `usePlugins`:
     - Checks each plugin whose trigger matches the query.
     - Provides plugin-specific results (calculator, web-search, clipboard).
   - `Launcher` merges plugin and backend results and shows them via `ResultList`.

4. **Keyboard navigation and execution**
   - `ArrowUp/ArrowDown` update `activeIndex` via `useKeyboardNav`.
   - `Enter`:
     - If `plugin:` id → delegated to `executePluginResult` → plugin `execute`.
     - Otherwise → `useSearch.executeResult` → `invoke("execute_result", { id })`:
       - `app:` → `open_app` (cmd /C start).
       - `file:` → `open_file` (cmd /C start).
       - `cmd:` → built-in system commands (shutdown/restart).
   - `Esc` → hides the window (via `appWindow.hide()`).

5. **Plugins & Built-ins**
   - Calculator: inline math evaluation.
   - Web search: delegates to browser/Google.
   - Clipboard history: session-based recall.

All of this is intentionally built with **no placeholders**: every file participates in the functioning app and is wired into the runtime behavior.

