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
- **`db.rs`** — SQLite persistence via `toasty` ORM with `jiff` datetime support. Uses a global `OnceCell<Mutex<Db>>` singleton. `GachaRecord` model (table "gacha") with fields: id, user_id, server_id, card_pool, language_code, record_id, quality_level, name, time. `GachaFilter` supports: card_pool, quality_level, name, time_from, time_to, limit, offset.
- **`export.rs`** — File export via `export_to_file()` which detects format from extension. Supports CSV (`csv` crate), XLSX (`rust_xlsxwriter`), and JSON. Headers are Chinese: 时间, 名称, 星级, 卡池类型.
- **`error.rs`** — `Error` enum via `thiserror` with variants: Http, Db, Api, Io, Csv, Xlsx, Json, Other. Custom `Result<T>` type alias.

### Tauri Commands (`src-tauri/src/lib.rs`)

Three `#[tauri::command]` functions bridge frontend to core library. All return `Result<T, String>` (errors stringified for IPC):

- **`fetch_gacha_records(params, pool_types)`** — Fetches from game API for each pool type, stores in DB. Returns total record count.
- **`query_gacha_records(user_id, filter)`** — Queries DB with `GachaFilter`. Returns `Vec<GachaRecord>`.
- **`export_gacha_records(user_id, filter, path)`** — Queries DB then writes to file (format from extension).

### Frontend (`src/`)

- **`src/lib/types.ts`** — TypeScript mirrors of Rust types: `CardPool` enum (1–7), `QualityLevel` enum (3/4/5), `GachaRecord`, `GachaFilter`, `FetchParams`. `CARD_POOL_LABELS` maps pool types to Chinese names.
- **`src/lib/commands.ts`** — Typed wrappers around `invoke()` for the three Tauri commands.
- **`src/App.tsx`** — Root layout: Sidebar + ContentArea + ExportDialog. Tracks `activePool` and export dialog state.
- **`src/components/Sidebar.tsx`** — Left nav (160px) with 7 pool type items in 3 groups (限定池, 常驻池, 其他). Footer has export button.
- **`src/components/ContentArea.tsx`** — Main panel. Manages records, loading, pagination (PAGE_SIZE=20), and filter state (quality, name, time range). Re-fetches on pool/filter/page changes via SolidJS effects.
- **`src/components/FilterPanel.tsx`** — Collapsible filter: quality chips (5★/4★/3★), name search, date range.
- **`src/components/RecordTable.tsx`** — Table with columns: 名称, 星级, 时间. Rows styled by quality (star-5/4/3 CSS classes).
- **`src/components/Pagination.tsx`** — 5-page sliding window with prev/next.
- **`src/components/ExportDialog.tsx`** — Modal: format selection (CSV/XLSX/JSON), file save dialog via `@tauri-apps/plugin-dialog`, invokes export command.
- **`src/App.css`** — All styling. CSS custom properties for dark/light theme (`prefers-color-scheme`). Color tokens: `--star-5` (gold), `--star-4` (purple), `--star-3` (blue).

### Key Dependencies

- **Rust**: `reqwest` (HTTP), `toasty` (ORM/SQLite), `jiff` (datetime), `serde`/`serde_repr` (serialization), `tokio` (async runtime), `thiserror` (errors), `csv`/`rust_xlsxwriter` (export)
- **Frontend**: `solid-js`, `@tauri-apps/api`, `@tauri-apps/plugin-dialog`, Vite, TypeScript
- **Package manager**: `bun` (configured in `tauri.conf.json` as the `beforeDevCommand`/`beforeBuildCommand` runner)

## API Reference

See `API.md` for the full gacha record query API documentation including request/response schemas and `cardPoolType` values. The `card_pool_id.json` file maps pool types to their fixed UUIDs (only needed for non-featured pools; featured pools get their IDs from the game URL).
