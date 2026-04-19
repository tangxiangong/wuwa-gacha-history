# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Wuthering Waves (鸣潮) gacha/convene history tracker. A Dioxus 0.7 desktop
application (wry webview) with a Rust core library, using the game's
undocumented API to fetch and store pull history locally.

## Build & Development Commands

```bash
# Prerequisites: Rust toolchain (edition 2024), dioxus-cli v0.7
# Install dx CLI:    cargo binstall dioxus-cli   (or cargo install dioxus-cli)

# Development — dx launches a wry webview window, auto-compiles Tailwind
cd app && dx serve --platform desktop

# Production build
cd app && dx build --platform desktop --release

# Rust-only workspace checks (no dx)
cargo check --workspace
cargo clippy --workspace -- -D warnings

# Core library tests (sniffer / log_reader / stats / db / export)
cargo test -p wuwa-gacha-history

# Core library without the sniffer feature (smaller for headless use)
cargo check -p wuwa-gacha-history --no-default-features
```

## Architecture

### Workspace Layout

- **`wuwa-gacha-history/`** — Core Rust library. Contains the API client
  (`client/`), SQLite persistence (`db.rs`), CSV/XLSX/JSON export
  (`export.rs`), Resonator / Weapon catalog with zh/en names and asset
  paths (`catalog.rs`), WuWa version release windows (`version.rs`), pity
  / UP / version-grouping analytics (`stats.rs`, port of the old TS
  frontend `src/lib/stats.ts`), game-log scanner (`log_reader.rs`), and an
  opt-in local MITM HTTP proxy (`sniffer/`, feature `sniffer`, on by
  default).
- **`app/`** — Dioxus 0.7 binary crate (`bin` name `wuwa-gacha-history`).
  Renders a wry webview window with Tailwind v4 styling (dx generates
  `assets/tailwind.css` from the source at `tailwind.css`). UI state via
  `use_signal` / `use_context_provider`, subscribes to sniffer events via
  `broadcast::Receiver`, uses `rfd` for native save dialogs and
  `directories` for the per-user data directory.

### Core Library (`wuwa-gacha-history/`)

- **`client/*`** — `GachaHistoryClient` POSTs to
  `https://aki-game2.com/gacha/record/query`. Paginates via `lastId`
  cursor. See `client/utils.rs::CardPool` for the 7 pool UUIDs.
- **`db.rs`** — SQLite via `sqlx`. Per-user tables named `gacha_{player_id}`
  where `player_id` is validated as 9 ASCII digits before interpolation
  (SQL-injection defense). `GachaFilter` supports card_pool,
  quality_level, name, time range, limit, offset.
- **`export.rs`** — `export_to_file()` routes by extension to CSV / XLSX
  / JSON.
- **`catalog.rs` / `version.rs`** — Static data. `version_of(iso) ->
  &'static str` and `VERSIONS: &[VersionRelease]`.
- **`stats.rs`** — `enrich_pulls`, `banner_stats`, `segments_by_five`,
  `group_by_version`, `pity_tier`. Pure functions, used by the three view
  components.
- **`log_reader.rs`** — Scans `Client.log` / `debug.log` under the player's
  game install directory to extract the latest gacha URL and parse its
  query params into `LogParams`.
- **`sniffer/*`** (feature `sniffer`) — `SnifferHandle` runs a local MITM
  proxy via `hudsucker` + a self-signed CA (installed into the OS trust
  store on macOS / Windows) and emits
  `SnifferEvent::{Started,Stopped,Captured,Error}` via a
  `tokio::sync::broadcast` channel. UI frontends subscribe via
  `SnifferHandle::subscribe()`.

### App Crate (`app/`)

- **`main.rs`** — `dioxus::launch(App)`, loads favicon + main.css +
  tailwind.css via `asset!()`, `use_context_provider(AppCtx::init)`,
  renders `components::Root`.
- **`state.rs`** — `AppCtx { sniffer_ca_dir, sniffer: SnifferHandle }`.
- **`platform.rs`** — `data_dir()` via
  `directories::ProjectDirs::from("dev", "tangxiangong", "wuwa-gacha-history")`,
  `db_path()`, `sniffer_ca_dir()`, `pick_save_file()`, `pick_directory()`
  (all via `rfd::AsyncFileDialog`).
- **`api.rs`** — Thin wrappers over the core lib injecting
  `platform::db_path()`.
- **`build.rs`** — Scans `assets/wiki-art/{characters,weapons}/` at
  compile time and emits `OUT_DIR/wiki_art_match.rs` with two functions
  `character_asset(name) -> Option<Asset>` / `weapon_asset(name) ->
  Option<Asset>` that match on file basename to a compile-time
  `asset!("/assets/wiki-art/<dir>/<name>.png")`. Runtime lookup via
  `assets_wiki::character_asset(name)` / `weapon_asset(name)`. Also
  stubs `assets/tailwind.css` on fresh clones so `cargo check` works
  before the first `dx serve`.
- **`components/`** — `Root` (users resource loader + WelcomePage /
  MainLayout branch), `MainLayout` (Sidebar + ContentArea), `Sidebar`
  (user selector + 7 pool items in 3 groups), `ContentArea` (FilterPanel
  + ViewTabs + stats strip + view + RecordTable + Pagination),
  `FilterPanel`, `RecordTable`, `Pagination`, `BarsView`, `CardsView`,
  `SummaryView`, `WelcomePage`, `FetchForm` (JSON-paste entry),
  `AddUserDialog` (sniffer subscription + log_reader button + FetchForm),
  `ExportDialog` (format chips + rfd save-file), `labels` (Chinese
  pool/quality text + Tailwind text-color classes).

### Key Dependencies

- **Core:** `reqwest`, `sqlx` (sqlite), `chrono`, `serde`/`serde_repr`,
  `tokio`, `thiserror`, `csv`, `rust_xlsxwriter`, `regex`; `hudsucker`
  + `rcgen` + platform deps under the `sniffer` feature.
- **App:** `dioxus` 0.7 (desktop feature, wry webview), `rfd`,
  `directories`, `tokio`, `serde_json`, `chrono`.
- **Tooling:** `dx` (dioxus-cli v0.7) serves/builds and auto-generates
  Tailwind.

## API Reference

See `API.md` for the full gacha record query API documentation including
request/response schemas and `cardPoolType` values. The `cardPoolId`
field is ignored by the server — pass an empty string.
