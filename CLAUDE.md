# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Wuthering Waves (鸣潮) gacha/convene history tracker. A Tauri v2 desktop app with a SolidJS frontend and Rust backend, using the game's undocumented API to fetch and store pull history locally.

## Build & Development Commands

```bash
# Prerequisites: bun, Rust toolchain (edition 2024)

# Install frontend dependencies
bun install

# Development (launches both Vite dev server and Tauri window)
bun run tauri dev

# Production build
bun run tauri build

# Format frontend code
bun run format

# Build only the Rust library crate
cargo build -p wuwa-gacha-history

# Run Rust tests
cargo test -p wuwa-gacha-history

# Check all Rust code
cargo check --workspace
```

## Architecture

### Workspace Layout

- **`wuwa-gacha-history/`** — Core Rust library crate. Contains the API client (`client/`), database layer (`db.rs`), export logic (`export.rs`), and error types (`error.rs`). This is the domain logic, independent of Tauri.
- **`src-tauri/`** — Tauri v2 backend crate (`wuwa_gacha_history_backend`). Wires the core library into Tauri commands and manages the app lifecycle. DB stored at `app_data_dir()/gacha.db`.
- **`src/`** — SolidJS + TypeScript frontend. Bundled by Vite, served at `localhost:1420` during dev.

### Core Library (`wuwa-gacha-history/`)

- **`client/mod.rs`** — `GachaHistoryClient`: HTTP client that POSTs to `https://aki-game2.com/gacha/record/query` to fetch gacha records. Handles pagination via `lastId` cursor with 500ms delay between pages. Uses `reqwest`.
- **`client/utils.rs`** — `CardPool` enum mapping the 7 convene/banner types (featured resonator/weapon, standard, novice, etc.) to their fixed UUIDs.
- **`client/response.rs`** — Response deserialization types. `QualityLevel` (3/4/5 star) uses `serde_repr` for integer enum deserialization.
- **`client/request.rs`** — `RequestParams` serialized as camelCase JSON for the API.
- **`db.rs`** — SQLite persistence via `sqlx` with raw SQL queries. Uses a global `OnceCell<SqlitePool>` singleton. Records are stored in per-user tables named `gacha_{player_id}` where `player_id` is validated as exactly 9 ASCII digits (`validate_player_id`) before interpolation into SQL — SQL-injection defense. `ensure_user_table` creates the table + unique `record_id` index on demand. `GachaRecord` fields: id, server_id, card_pool, language_code, record_id (UNIQUE), quality_level, name, time. `list_users` enumerates `sqlite_master` for tables matching `gacha_<9-digit-id>` and returns the id list. Dynamic query building via `sqlx::QueryBuilder`. `GachaFilter` supports: card_pool, quality_level, name, time_from, time_to, limit, offset.
- **`export.rs`** — File export via `export_to_file()` which detects format from extension. Supports CSV (`csv` crate), XLSX (`rust_xlsxwriter`), and JSON. Headers are Chinese: 时间, 名称, 星级, 卡池类型.
- **`error.rs`** — `Error` enum via `thiserror` with variants: Http, Db, Api, Io, Csv, Xlsx, Json, Other. Custom `Result<T>` type alias.

### Tauri Commands (`src-tauri/src/lib.rs`)

Four `#[tauri::command]` functions bridge frontend to core library. All return `Result<T, String>` (errors stringified for IPC):

- **`fetch_gacha_records(params, pool_types)`** — Fetches from game API for each pool type, stores in DB. Returns total record count.
- **`query_gacha_records(player_id, filter)`** — Queries the user's `gacha_{player_id}` table with `GachaFilter`. Returns `Vec<GachaRecord>`.
- **`export_gacha_records(player_id, filter, path)`** — Queries the user's table then writes to file (format from extension).
- **`list_users()`** — Returns `Vec<String>` of known player IDs by scanning `sqlite_master` for `gacha_<9-digit-id>` tables.

### Frontend (`src/`)

- **`src/lib/types.ts`** — TypeScript mirrors of Rust types: `CardPool` enum (1–7), `QualityLevel` enum (3/4/5), `GachaRecord`, `GachaFilter`, `FetchParams`. `CARD_POOL_LABELS` maps pool types to Chinese names.
- **`src/lib/commands.ts`** — Typed wrappers around `invoke()` for the four Tauri commands (including `listUsers`).
- **`src/App.tsx`** — Root layout. Loads user list via `createResource(listUsers)`; when empty, renders `WelcomePage`; otherwise renders `Sidebar + ContentArea + ExportDialog + AddUserDialog`. Tracks `activePool`, `playerId`, `exportOpen`, `addUserOpen`.
- **`src/components/FetchForm.tsx`** — Shared JSON-paste form. Parses + validates `playerId/serverId/languageCode/recordId`, calls `fetchGachaRecords` across all 7 pools, fires `onSuccess(playerId)`.
- **`src/components/WelcomePage.tsx`** — Empty-state page wrapping `FetchForm`.
- **`src/components/AddUserDialog.tsx`** — Modal wrapping `FetchForm`. Opened from the sidebar.
- **`src/components/Sidebar.tsx`** — Left nav (160px). Top: user selector `<select>` over known player IDs. Middle: 7 pool-type items in 3 groups (限定池, 常驻池, 其他). Footer: "添加用户" + "导出" buttons.
- **`src/components/ContentArea.tsx`** — Main panel. Manages records, loading, pagination (PAGE_SIZE=20), and filter state (quality, name, time range). Re-fetches on pool/filter/page changes via SolidJS effects.
- **`src/components/FilterPanel.tsx`** — Collapsible filter: quality chips (5★/4★/3★), name search, date range.
- **`src/components/RecordTable.tsx`** — Table with columns: 名称, 星级, 时间. Rows styled by quality (star-5/4/3 CSS classes).
- **`src/components/Pagination.tsx`** — 5-page sliding window with prev/next.
- **`src/components/ExportDialog.tsx`** — Modal: format selection (CSV/XLSX/JSON), file save dialog via `@tauri-apps/plugin-dialog`, invokes export command.
- **`src/App.css`** — All styling. CSS custom properties for dark/light theme (`prefers-color-scheme`). Color tokens: `--star-5` (gold), `--star-4` (purple), `--star-3` (blue).

### Key Dependencies

- **Rust**: `reqwest` (HTTP), `sqlx` (SQLite), `chrono` (datetime), `serde`/`serde_repr` (serialization), `tokio` (async runtime), `thiserror` (errors), `csv`/`rust_xlsxwriter` (export)
- **Frontend**: `solid-js`, `@tauri-apps/api`, `@tauri-apps/plugin-dialog`, Vite, TypeScript
- **Package manager**: `bun` (configured in `tauri.conf.json` as the `beforeDevCommand`/`beforeBuildCommand` runner)

## API Reference

See `API.md` for the full gacha record query API documentation including request/response schemas and `cardPoolType` values. The `card_pool_id.json` file maps pool types to their fixed UUIDs (only needed for non-featured pools; featured pools get their IDs from the game URL).
