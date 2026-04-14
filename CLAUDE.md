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

- **`wuwa-gacha-history/`** — Core Rust library crate. Contains the API client (`client/`) and database layer (`db.rs`). This is the domain logic, independent of Tauri.
- **`src-tauri/`** — Tauri v2 backend crate (`wuwa_gacha_history_backend`). Wires the core library into Tauri commands and manages the app lifecycle.
- **`src/`** — SolidJS + TypeScript frontend. Bundled by Vite, served at `localhost:1420` during dev.

### Core Library (`wuwa-gacha-history/`)

- **`client/mod.rs`** — `GachaHistoryClient`: HTTP client that POSTs to `https://aki-game2.com/gacha/record/query` to fetch gacha records. Handles pagination via `lastId` cursor. Uses `reqwest`.
- **`client/utils.rs`** — `CardPool` enum mapping the 7 convene/banner types (featured resonator/weapon, standard, novice, etc.) to their fixed UUIDs.
- **`client/response.rs`** — Response deserialization types. `QualityLevel` (3/4/5 star) uses `serde_repr` for integer enum deserialization.
- **`client/request.rs`** — `RequestParams` serialized as camelCase JSON for the API.
- **`db.rs`** — SQLite persistence via `toasty` ORM with `jiff` datetime support. Uses a global `OnceCell<Mutex<Db>>` singleton.

### Key Dependencies

- **Rust**: `reqwest` (HTTP), `toasty` (ORM/SQLite), `jiff` (datetime), `serde`/`serde_repr` (serialization), `tokio` (async runtime)
- **Frontend**: `solid-js`, `@tauri-apps/api`, Vite, TypeScript
- **Package manager**: `bun` (configured in `tauri.conf.json` as the `beforeDevCommand`/`beforeBuildCommand` runner)

## API Reference

See `API.md` for the full gacha record query API documentation including request/response schemas and `cardPoolType` values. The `card_pool_id.json` file maps pool types to their fixed UUIDs (only needed for non-featured pools; featured pools get their IDs from the game URL).
