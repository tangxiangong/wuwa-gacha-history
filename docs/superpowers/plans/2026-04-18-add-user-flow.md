# Add User Flow Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the hardcoded empty-userId stub with a real multi-user flow — paste JSON, fetch records into a per-user SQLite table, switch between users, and show a welcome page when no users exist.

**Architecture:** Each user gets a dedicated `gacha_{player_id}` table (9-digit numeric id validated at every boundary to prevent SQL injection). The old `gacha` table is abandoned (pre-release, no migration). The frontend loads the user list on mount; empty → WelcomePage with FetchForm, non-empty → Sidebar user selector + ContentArea. The FetchForm component is shared by WelcomePage and AddUserDialog.

**Tech Stack:** Rust (sqlx, tokio, thiserror), Tauri v2, SolidJS, TypeScript, Vite.

**Testing approach:**
- Rust: add `tempfile` as dev-dependency and write `#[tokio::test]` integration tests against a tmp SQLite file. The existing codebase has no tests; we add them for the per-user table logic where correctness matters (validation, table creation, list_users).
- Frontend + Tauri IPC: no existing JS test infrastructure — verify manually via `bun run tauri dev` with explicit smoke-test checklists at each UI task.

**Key decisions locked in by the spec:**
- `player_id` MUST be validated as `^\d{9}$` at every layer that constructs a table name. Interpolating an un-validated string into `CREATE TABLE gacha_{id}` is SQL injection.
- `add_records`, `query_records`, `export_gacha_records`, and `list_users` all take `player_id` (no more `user_id`). `GachaRecord` loses its `user_id` field — the table *is* the user.
- The pool singleton still caches the sqlx pool per DB file. `ensure_user_table` is idempotent (`CREATE TABLE IF NOT EXISTS`) so calling it per `add_records` is fine.

---

## File Structure

**Backend — `wuwa-gacha-history/` crate:**
- Modify `wuwa-gacha-history/src/db.rs` — remove `user_id` from `GachaRecord`, replace the bootstrapped `gacha` table with per-user `gacha_{player_id}` tables, add `validate_player_id`, `user_table`, `ensure_user_table`, and `list_users`.
- Modify `wuwa-gacha-history/Cargo.toml` — add `tempfile` as dev-dependency.

**Backend — Tauri crate:**
- Modify `src-tauri/src/lib.rs` — rename `user_id` → `player_id` in `query_gacha_records` and `export_gacha_records`, add new `list_users` command, register it in the invoke handler.

**Frontend:**
- Modify `src/lib/types.ts` — drop `userId` from `GachaRecord`.
- Modify `src/lib/commands.ts` — rename `userId` → `playerId`, add `listUsers()`.
- Create `src/components/FetchForm.tsx` — shared JSON-paste + fetch form.
- Create `src/components/WelcomePage.tsx` — empty-state page wrapping `FetchForm`.
- Create `src/components/AddUserDialog.tsx` — modal wrapping `FetchForm`.
- Modify `src/components/Sidebar.tsx` — add user selector dropdown at top, add-user button at bottom.
- Modify `src/components/ContentArea.tsx` — prop `userId` → `playerId`.
- Modify `src/components/ExportDialog.tsx` — prop `userId` → `playerId`.
- Modify `src/App.tsx` — load user list on mount, track `playerId`, switch between Welcome and Content.
- Modify `src/App.css` — append styles for `.user-selector`, `.welcome-page`, `.fetch-form`.

---

### Task 1: Backend — Add `tempfile` dev-dependency

**Files:**
- Modify: `wuwa-gacha-history/Cargo.toml`

- [ ] **Step 1: Add tempfile to `[dev-dependencies]`**

Edit `wuwa-gacha-history/Cargo.toml`, replace the `[dev-dependencies]` block with:

```toml
[dev-dependencies]
tempfile = "3"
tokio = { workspace = true, features = ["macros"] }
```

- [ ] **Step 2: Verify build**

Run: `cargo check -p wuwa-gacha-history --tests`
Expected: compiles cleanly (no test code yet, just ensures the dep resolves).

- [ ] **Step 3: Commit**

```bash
git add wuwa-gacha-history/Cargo.toml Cargo.lock
git commit -m "chore: add tempfile as dev-dependency for db tests"
```

---

### Task 2: Backend — `validate_player_id` + `user_table` helpers

**Files:**
- Modify: `wuwa-gacha-history/src/db.rs`
- Test: `wuwa-gacha-history/src/db.rs` (inline `#[cfg(test)] mod tests`)

These are pure functions — perfect for TDD. They MUST reject any player_id that isn't exactly 9 ASCII digits, because the value is interpolated into a SQL identifier later.

- [ ] **Step 1: Write failing tests**

Append to `wuwa-gacha-history/src/db.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_player_id_accepts_9_digits() {
        assert!(validate_player_id("123456789").is_ok());
        assert!(validate_player_id("000000000").is_ok());
    }

    #[test]
    fn validate_player_id_rejects_wrong_length() {
        assert!(validate_player_id("12345678").is_err());
        assert!(validate_player_id("1234567890").is_err());
        assert!(validate_player_id("").is_err());
    }

    #[test]
    fn validate_player_id_rejects_non_digits() {
        assert!(validate_player_id("12345678a").is_err());
        assert!(validate_player_id("123 45678").is_err());
        assert!(validate_player_id("12345678;").is_err());
        assert!(validate_player_id("12345678'").is_err());
    }

    #[test]
    fn user_table_returns_prefixed_name() {
        assert_eq!(user_table("123456789").unwrap(), "gacha_123456789");
    }

    #[test]
    fn user_table_rejects_invalid_id() {
        assert!(user_table("bad").is_err());
    }
}
```

- [ ] **Step 2: Run tests, verify they fail**

Run: `cargo test -p wuwa-gacha-history`
Expected: compilation errors — `validate_player_id` and `user_table` not defined.

- [ ] **Step 3: Implement the helpers**

Add to `wuwa-gacha-history/src/db.rs` (above `pool()`):

```rust
pub fn validate_player_id(player_id: &str) -> Result<()> {
    if player_id.len() == 9 && player_id.bytes().all(|b| b.is_ascii_digit()) {
        Ok(())
    } else {
        Err(crate::Error::Other("invalid player_id".to_string()))
    }
}

fn user_table(player_id: &str) -> Result<String> {
    validate_player_id(player_id)?;
    Ok(format!("gacha_{player_id}"))
}
```

- [ ] **Step 4: Run tests, verify they pass**

Run: `cargo test -p wuwa-gacha-history`
Expected: 5 tests pass.

- [ ] **Step 5: Commit**

```bash
git add wuwa-gacha-history/src/db.rs
git commit -m "feat(db): add player_id validation and table name helper"
```

---

### Task 3: Backend — Per-user table creation + `GachaRecord` refactor

**Files:**
- Modify: `wuwa-gacha-history/src/db.rs`

Drops `user_id` from `GachaRecord`, replaces the boot-time `gacha` table creation with a lazy `ensure_user_table`, and rewrites `add_records` / `query_records` to use the per-user table.

- [ ] **Step 1: Write failing integration test**

Append to the `#[cfg(test)] mod tests` block in `wuwa-gacha-history/src/db.rs`:

```rust
    use crate::{CardPool, QualityLevel, ResponseRecord};
    use chrono::NaiveDate;

    fn sample_record(id: &str) -> ResponseRecord {
        ResponseRecord {
            card_pool_type: CardPool::FeaturedResonatorConvene,
            id: id.to_string(),
            quality_level: QualityLevel::FiveStar,
            name: "安可".to_string(),
            time: NaiveDate::from_ymd_opt(2026, 4, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
            resource_id: 0,
            resource_type: String::new(),
            count: 1,
        }
    }

    #[tokio::test]
    async fn add_and_query_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("gacha.db").to_string_lossy().into_owned();
        let player_id = "123456789";

        add_records(&path, player_id, "76402e5b", "zh-Hans", vec![sample_record("r1")])
            .await
            .unwrap();

        let records = query_records(&path, player_id, &GachaFilter::default())
            .await
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].record_id, "r1");
        assert_eq!(records[0].name, "安可");
    }

    #[tokio::test]
    async fn add_records_rejects_invalid_player_id() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("gacha.db").to_string_lossy().into_owned();

        let err = add_records(&path, "bad", "s", "zh-Hans", vec![])
            .await
            .unwrap_err();
        assert!(matches!(err, crate::Error::Other(_)));
    }

    #[tokio::test]
    async fn query_records_isolates_users() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("gacha.db").to_string_lossy().into_owned();

        add_records(&path, "111111111", "s", "zh-Hans", vec![sample_record("a")])
            .await
            .unwrap();
        add_records(&path, "222222222", "s", "zh-Hans", vec![sample_record("b")])
            .await
            .unwrap();

        let r1 = query_records(&path, "111111111", &GachaFilter::default())
            .await
            .unwrap();
        let r2 = query_records(&path, "222222222", &GachaFilter::default())
            .await
            .unwrap();
        assert_eq!(r1.len(), 1);
        assert_eq!(r1[0].record_id, "a");
        assert_eq!(r2.len(), 1);
        assert_eq!(r2[0].record_id, "b");
    }
```

Note: the `ResponseRecord` struct is defined in `client/response.rs`. Open it first and mirror every required field in `sample_record` — the stub above lists the fields the struct is expected to have. If field names differ, adjust the test literal to match.

- [ ] **Step 2: Check `ResponseRecord` fields and adjust the test**

Run: `cat wuwa-gacha-history/src/client/response.rs`
Update `sample_record` above so every non-`Option` field of `ResponseRecord` is populated. Keep `card_pool_type`, `id`, `quality_level`, `name`, `time` as shown.

- [ ] **Step 3: Run tests, verify they fail**

Run: `cargo test -p wuwa-gacha-history`
Expected: compile errors referencing the old `user_id` on `GachaRecord`, or runtime errors because the `gacha` table no longer exists.

- [ ] **Step 4: Update `GachaRecord` struct**

In `wuwa-gacha-history/src/db.rs`, remove the `user_id` field from `GachaRecord`:

```rust
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaRecord {
    pub id: u64,
    pub server_id: String,
    pub card_pool: CardPool,
    pub language_code: String,
    pub record_id: String,
    pub quality_level: QualityLevel,
    pub name: String,
    pub time: NaiveDateTime,
}
```

- [ ] **Step 5: Replace `init()` with bare pool connect + add `ensure_user_table`**

Replace the existing `init` function with:

```rust
async fn init(path: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", path)).await?;
    Ok(pool)
}

async fn ensure_user_table(pool: &SqlitePool, player_id: &str) -> Result<String> {
    let table = user_table(player_id)?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {table} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            server_id TEXT NOT NULL,
            card_pool INTEGER NOT NULL,
            language_code TEXT NOT NULL,
            record_id TEXT NOT NULL UNIQUE,
            quality_level INTEGER NOT NULL,
            name TEXT NOT NULL,
            time TEXT NOT NULL
        )"
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_{table}_record_id ON {table}(record_id)"
    ))
    .execute(pool)
    .await?;

    Ok(table)
}
```

The `{table}` interpolation is safe because `user_table` calls `validate_player_id`, which enforces exactly 9 ASCII digits.

- [ ] **Step 6: Update `record_from_row` to drop `user_id`**

Replace the `Ok(GachaRecord { ... })` block in `record_from_row` with:

```rust
    Ok(GachaRecord {
        id: u64::try_from(row.try_get::<i64, _>("id")?)
            .map_err(|e| crate::Error::Other(format!("invalid id: {e}")))?,
        server_id: row.try_get("server_id")?,
        card_pool,
        language_code: row.try_get("language_code")?,
        record_id: row.try_get("record_id")?,
        quality_level,
        name: row.try_get("name")?,
        time,
    })
```

- [ ] **Step 7: Rewrite `add_records` to use per-user table**

Replace the existing `add_records` function with:

```rust
pub async fn add_records(
    path: &str,
    player_id: &str,
    server_id: &str,
    language_code: &str,
    records: Vec<ResponseRecord>,
) -> Result<()> {
    let pool = pool(path).await?;
    let table = ensure_user_table(pool, player_id).await?;
    let mut tx = pool.begin().await?;

    let sql = format!(
        "INSERT OR IGNORE INTO {table}
            (server_id, card_pool, language_code, record_id, quality_level, name, time)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    );

    for record in records {
        sqlx::query(&sql)
            .bind(server_id)
            .bind(record.card_pool_type as i32)
            .bind(language_code)
            .bind(&record.id)
            .bind(record.quality_level as i32)
            .bind(&record.name)
            .bind(record.time.format("%Y-%m-%dT%H:%M:%S").to_string())
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}
```

- [ ] **Step 8: Rewrite `query_records` to use per-user table**

Replace the existing `query_records` function with:

```rust
pub async fn query_records(
    path: &str,
    player_id: &str,
    filter: &GachaFilter,
) -> Result<Vec<GachaRecord>> {
    let pool = pool(path).await?;
    let table = ensure_user_table(pool, player_id).await?;

    let mut qb: sqlx::QueryBuilder<'_, sqlx::Sqlite> = sqlx::QueryBuilder::new(format!(
        "SELECT id, server_id, card_pool, language_code, record_id, quality_level, name, time FROM {table} WHERE 1=1"
    ));

    if let Some(card_pool) = filter.card_pool {
        qb.push(" AND card_pool = ").push_bind(card_pool as i32);
    }
    if let Some(quality_level) = filter.quality_level {
        qb.push(" AND quality_level = ")
            .push_bind(quality_level as i32);
    }
    if let Some(ref name) = filter.name {
        qb.push(" AND name = ").push_bind(name.clone());
    }
    if let Some(time_from) = filter.time_from {
        qb.push(" AND time >= ")
            .push_bind(time_from.format("%Y-%m-%dT%H:%M:%S").to_string());
    }
    if let Some(time_to) = filter.time_to {
        qb.push(" AND time <= ")
            .push_bind(time_to.format("%Y-%m-%dT%H:%M:%S").to_string());
    }

    qb.push(" ORDER BY time DESC");

    if let Some(limit) = filter.limit {
        qb.push(" LIMIT ").push_bind(limit as i64);
        if let Some(offset) = filter.offset {
            qb.push(" OFFSET ").push_bind(offset as i64);
        }
    }

    let rows = qb.build().fetch_all(pool).await?;
    rows.iter().map(record_from_row).collect()
}
```

- [ ] **Step 9: Run all tests, verify they pass**

Run: `cargo test -p wuwa-gacha-history`
Expected: all tests (validation + roundtrip + rejection + isolation) pass.

- [ ] **Step 10: Commit**

```bash
git add wuwa-gacha-history/src/db.rs
git commit -m "refactor(db): use per-user tables, drop user_id from GachaRecord"
```

---

### Task 4: Backend — `list_users` function

**Files:**
- Modify: `wuwa-gacha-history/src/db.rs`

- [ ] **Step 1: Write failing test**

Append to `#[cfg(test)] mod tests`:

```rust
    #[tokio::test]
    async fn list_users_returns_player_ids() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("gacha.db").to_string_lossy().into_owned();

        assert!(list_users(&path).await.unwrap().is_empty());

        add_records(&path, "111111111", "s", "zh-Hans", vec![])
            .await
            .unwrap();
        add_records(&path, "222222222", "s", "zh-Hans", vec![])
            .await
            .unwrap();

        let mut users = list_users(&path).await.unwrap();
        users.sort();
        assert_eq!(users, vec!["111111111".to_string(), "222222222".to_string()]);
    }
```

- [ ] **Step 2: Run tests, verify it fails**

Run: `cargo test -p wuwa-gacha-history list_users_returns_player_ids`
Expected: compile error, `list_users` not defined.

- [ ] **Step 3: Implement `list_users`**

Add to `wuwa-gacha-history/src/db.rs` (above the `#[cfg(test)]` block):

```rust
pub async fn list_users(path: &str) -> Result<Vec<String>> {
    let pool = pool(path).await?;

    let rows = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'gacha\\_%' ESCAPE '\\'",
    )
    .fetch_all(pool)
    .await?;

    let mut ids = Vec::with_capacity(rows.len());
    for row in rows {
        let name: String = row.try_get("name")?;
        if let Some(id) = name.strip_prefix("gacha_") {
            if validate_player_id(id).is_ok() {
                ids.push(id.to_string());
            }
        }
    }

    Ok(ids)
}
```

The `ESCAPE '\\'` clause makes the `_` literal (SQLite `LIKE` treats bare `_` as "any single char"). `validate_player_id` filters out any stray `gacha_*` tables that don't match the 9-digit shape.

- [ ] **Step 4: Run tests, verify pass**

Run: `cargo test -p wuwa-gacha-history`
Expected: all tests pass.

- [ ] **Step 5: Commit**

```bash
git add wuwa-gacha-history/src/db.rs
git commit -m "feat(db): add list_users to enumerate player ids"
```

---

### Task 5: Tauri commands — rename params + add `list_users`

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Update imports**

Change the `use wuwa_gacha_history::...` line in `src-tauri/src/lib.rs` to:

```rust
use wuwa_gacha_history::{
    add_records, export_to_file, list_users as list_users_impl, query_records, CardPool,
    GachaFilter, GachaHistoryClient, GachaRecord, RequestParams,
};
```

- [ ] **Step 2: Rename `user_id` → `player_id` in `query_gacha_records`**

Replace the existing `query_gacha_records` function with:

```rust
#[tauri::command]
async fn query_gacha_records(
    app: tauri::AppHandle,
    player_id: String,
    filter: GachaFilter,
) -> Result<Vec<GachaRecord>, String> {
    let db_path = db_path(&app)?;
    let records = query_records(&db_path, &player_id, &filter)
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}
```

- [ ] **Step 3: Rename `user_id` → `player_id` in `export_gacha_records`**

Replace the existing `export_gacha_records` function with:

```rust
#[tauri::command]
async fn export_gacha_records(
    app: tauri::AppHandle,
    player_id: String,
    filter: GachaFilter,
    path: String,
) -> Result<(), String> {
    let db_path = db_path(&app)?;
    let records = query_records(&db_path, &player_id, &filter)
        .await
        .map_err(|e| e.to_string())?;
    export_to_file(&records, &path).map_err(|e| e.to_string())?;
    Ok(())
}
```

- [ ] **Step 4: Add `list_users` command**

Add below `export_gacha_records`:

```rust
#[tauri::command]
async fn list_users(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let db_path = db_path(&app)?;
    list_users_impl(&db_path).await.map_err(|e| e.to_string())
}
```

- [ ] **Step 5: Register the command in `invoke_handler`**

Replace the `invoke_handler` call in `run()` with:

```rust
        .invoke_handler(tauri::generate_handler![
            fetch_gacha_records,
            query_gacha_records,
            export_gacha_records,
            list_users,
        ])
```

- [ ] **Step 6: Verify the backend compiles**

Run: `cargo check --workspace`
Expected: no errors.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat(tauri): rename user_id to player_id, add list_users command"
```

---

### Task 6: Frontend — types and command wrappers

**Files:**
- Modify: `src/lib/types.ts`
- Modify: `src/lib/commands.ts`

- [ ] **Step 1: Remove `userId` from `GachaRecord`**

In `src/lib/types.ts`, replace the `GachaRecord` interface with:

```typescript
export interface GachaRecord {
  id: number;
  serverId: string;
  cardPool: CardPool;
  languageCode: string;
  recordId: string;
  qualityLevel: QualityLevel;
  name: string;
  time: string;
}
```

- [ ] **Step 2: Rename params and add `listUsers`**

Replace the contents of `src/lib/commands.ts` with:

```typescript
import { invoke } from "@tauri-apps/api/core";
import type { CardPool, FetchParams, GachaFilter, GachaRecord } from "./types";

export async function queryGachaRecords(
  playerId: string,
  filter: GachaFilter,
): Promise<GachaRecord[]> {
  return invoke("query_gacha_records", { playerId, filter });
}

export async function fetchGachaRecords(
  params: FetchParams,
  poolTypes: CardPool[],
): Promise<number> {
  return invoke("fetch_gacha_records", { params, poolTypes });
}

export async function exportGachaRecords(
  playerId: string,
  filter: GachaFilter,
  path: string,
): Promise<void> {
  return invoke("export_gacha_records", { playerId, filter, path });
}

export async function listUsers(): Promise<string[]> {
  return invoke("list_users");
}
```

- [ ] **Step 3: Verify typecheck**

Run: `bunx tsc --noEmit`
Expected: errors only in the components we're about to update (`ContentArea.tsx`, `ExportDialog.tsx`, `App.tsx`) — that's expected; they'll be fixed in later tasks.

- [ ] **Step 4: Commit**

```bash
git add src/lib/types.ts src/lib/commands.ts
git commit -m "feat(frontend): rename userId to playerId, add listUsers wrapper"
```

---

### Task 7: Frontend — `FetchForm` component

**Files:**
- Create: `src/components/FetchForm.tsx`

Shared form used by WelcomePage and AddUserDialog. Parses pasted JSON, validates `playerId`, invokes `fetchGachaRecords` for all 7 pools, then notifies the parent.

- [ ] **Step 1: Create the component file**

Create `src/components/FetchForm.tsx` with:

```tsx
import { createSignal } from "solid-js";
import { fetchGachaRecords } from "../lib/commands";
import { CardPool } from "../lib/types";
import type { FetchParams } from "../lib/types";

const ALL_POOLS: CardPool[] = [
  CardPool.FeaturedResonatorConvene,
  CardPool.FeaturedWeaponConvene,
  CardPool.StandardResonatorConvene,
  CardPool.StandardWeaponConvene,
  CardPool.NoviceConvene,
  CardPool.BeginnerChoiceConvene,
  CardPool.GivebackCustomConvene,
];

const REQUIRED_FIELDS: (keyof FetchParams)[] = [
  "playerId",
  "serverId",
  "languageCode",
  "recordId",
];

interface FetchFormProps {
  onSuccess: (playerId: string) => void;
}

export default function FetchForm(props: FetchFormProps) {
  const [json, setJson] = createSignal("");
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");

  function parseParams(raw: string): FetchParams {
    let parsed: unknown;
    try {
      parsed = JSON.parse(raw);
    } catch {
      throw new Error("JSON 格式错误");
    }
    if (typeof parsed !== "object" || parsed === null) {
      throw new Error("JSON 格式错误");
    }
    const obj = parsed as Record<string, unknown>;
    const missing = REQUIRED_FIELDS.filter(
      (k) => typeof obj[k] !== "string" || (obj[k] as string) === "",
    );
    if (missing.length > 0) {
      throw new Error(`缺少必要字段: ${missing.join(", ")}`);
    }
    const playerId = obj.playerId as string;
    if (!/^\d{9}$/.test(playerId)) {
      throw new Error("playerId 格式错误");
    }
    return {
      playerId,
      serverId: obj.serverId as string,
      languageCode: obj.languageCode as string,
      recordId: obj.recordId as string,
    };
  }

  async function handleFetch() {
    setError("");
    let params: FetchParams;
    try {
      params = parseParams(json());
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      return;
    }

    setLoading(true);
    try {
      await fetchGachaRecords(params, ALL_POOLS);
      props.onSuccess(params.playerId);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div class="fetch-form">
      <textarea
        class="fetch-form-input"
        placeholder='粘贴 JSON，例如 {"playerId":"123456789","serverId":"...","languageCode":"zh-Hans","recordId":"..."}'
        value={json()}
        onInput={(e) => setJson(e.currentTarget.value)}
        rows={6}
        disabled={loading()}
      />
      {error() && <p class="fetch-form-error">{error()}</p>}
      <button
        class="btn btn-primary"
        onClick={handleFetch}
        disabled={loading() || json().trim() === ""}
      >
        {loading() ? "获取中..." : "获取记录"}
      </button>
    </div>
  );
}
```

- [ ] **Step 2: Append styles to `src/App.css`**

Append to the end of `src/App.css`:

```css
/* ===== Fetch form ===== */
.fetch-form {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.fetch-form-input {
  width: 100%;
  padding: 8px 10px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg-input);
  color: var(--text-primary);
  font-family: ui-monospace, "SF Mono", Menlo, monospace;
  font-size: 12px;
  resize: vertical;
}

.fetch-form-input:focus {
  outline: none;
  border-color: var(--accent);
}

.fetch-form-error {
  color: var(--star-5);
  font-size: 12px;
}
```

- [ ] **Step 3: Verify typecheck**

Run: `bunx tsc --noEmit`
Expected: no new errors in `FetchForm.tsx`.

- [ ] **Step 4: Commit**

```bash
git add src/components/FetchForm.tsx src/App.css
git commit -m "feat(frontend): add FetchForm component"
```

---

### Task 8: Frontend — `WelcomePage` component

**Files:**
- Create: `src/components/WelcomePage.tsx`

- [ ] **Step 1: Create the component**

Create `src/components/WelcomePage.tsx` with:

```tsx
import FetchForm from "./FetchForm";

interface WelcomePageProps {
  onUserAdded: (playerId: string) => void;
}

export default function WelcomePage(props: WelcomePageProps) {
  return (
    <div class="welcome-page">
      <div class="welcome-card">
        <h2>欢迎使用鸣潮抽卡记录</h2>
        <p class="welcome-hint">
          粘贴从游戏抽卡记录页面抓取的 JSON 参数（包含 playerId、serverId、
          languageCode、recordId），系统会自动拉取所有卡池的记录。
        </p>
        <FetchForm onSuccess={props.onUserAdded} />
      </div>
    </div>
  );
}
```

- [ ] **Step 2: Append styles to `src/App.css`**

Append to the end of `src/App.css`:

```css
/* ===== Welcome page ===== */
.welcome-page {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.welcome-card {
  max-width: 520px;
  width: 100%;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.welcome-card h2 {
  font-size: 18px;
  font-weight: 600;
}

.welcome-hint {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/WelcomePage.tsx src/App.css
git commit -m "feat(frontend): add WelcomePage for empty state"
```

---

### Task 9: Frontend — `AddUserDialog` component

**Files:**
- Create: `src/components/AddUserDialog.tsx`

Reuses the existing `.dialog-overlay` / `.dialog` / `.dialog-actions` styles already used by `ExportDialog`.

- [ ] **Step 1: Create the component**

Create `src/components/AddUserDialog.tsx` with:

```tsx
import { Show } from "solid-js";
import FetchForm from "./FetchForm";

interface AddUserDialogProps {
  open: boolean;
  onClose: () => void;
  onUserAdded: (playerId: string) => void;
}

export default function AddUserDialog(props: AddUserDialogProps) {
  function handleSuccess(playerId: string) {
    props.onUserAdded(playerId);
    props.onClose();
  }

  return (
    <Show when={props.open}>
      <div class="dialog-overlay" onClick={() => props.onClose()}>
        <div class="dialog" onClick={(e) => e.stopPropagation()}>
          <h3>添加用户</h3>
          <FetchForm onSuccess={handleSuccess} />
          <div class="dialog-actions">
            <button class="btn" onClick={() => props.onClose()}>
              取消
            </button>
          </div>
        </div>
      </div>
    </Show>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/AddUserDialog.tsx
git commit -m "feat(frontend): add AddUserDialog component"
```

---

### Task 10: Frontend — `Sidebar` with user selector and add-user button

**Files:**
- Modify: `src/components/Sidebar.tsx`
- Modify: `src/App.css`

- [ ] **Step 1: Rewrite `Sidebar.tsx`**

Replace the full contents of `src/components/Sidebar.tsx` with:

```tsx
import { For } from "solid-js";
import { CardPool, CARD_POOL_LABELS } from "../lib/types";

interface NavGroup {
  label: string;
  items: CardPool[];
}

const NAV_GROUPS: NavGroup[] = [
  {
    label: "限定池",
    items: [CardPool.FeaturedResonatorConvene, CardPool.FeaturedWeaponConvene],
  },
  {
    label: "常驻池",
    items: [CardPool.StandardResonatorConvene, CardPool.StandardWeaponConvene],
  },
  {
    label: "其他",
    items: [
      CardPool.NoviceConvene,
      CardPool.BeginnerChoiceConvene,
      CardPool.GivebackCustomConvene,
    ],
  },
];

interface SidebarProps {
  users: string[];
  playerId: string | null;
  activePool: CardPool | null;
  onSelectUser: (playerId: string) => void;
  onSelectPool: (pool: CardPool) => void;
  onAddUser: () => void;
  onExport: () => void;
}

export default function Sidebar(props: SidebarProps) {
  return (
    <nav class="sidebar">
      <div class="user-selector">
        <label class="user-selector-label">当前用户</label>
        <select
          class="user-selector-input"
          value={props.playerId ?? ""}
          onChange={(e) => props.onSelectUser(e.currentTarget.value)}
        >
          <For each={props.users}>
            {(id) => <option value={id}>{id}</option>}
          </For>
        </select>
      </div>
      <For each={NAV_GROUPS}>
        {(group) => (
          <>
            <div class="nav-group-label">{group.label}</div>
            <For each={group.items}>
              {(pool) => (
                <div
                  class={`nav-item ${props.activePool === pool ? "active" : ""}`}
                  onClick={() => props.onSelectPool(pool)}
                >
                  {CARD_POOL_LABELS[pool]}
                </div>
              )}
            </For>
          </>
        )}
      </For>
      <div class="nav-footer">
        <div class="nav-item" onClick={props.onAddUser}>
          添加用户
        </div>
        <div class="nav-item" onClick={props.onExport}>
          导出
        </div>
      </div>
    </nav>
  );
}
```

- [ ] **Step 2: Append styles to `src/App.css`**

Append to the end of `src/App.css`:

```css
/* ===== User selector ===== */
.user-selector {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 0 4px 12px;
  border-bottom: 1px solid var(--border);
  margin-bottom: 8px;
}

.user-selector-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.user-selector-input {
  padding: 4px 8px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg-input);
  color: var(--text-primary);
  font-size: 13px;
}

.user-selector-input:focus {
  outline: none;
  border-color: var(--accent);
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/Sidebar.tsx src/App.css
git commit -m "feat(frontend): add user selector and add-user button to Sidebar"
```

---

### Task 11: Frontend — rename `userId` prop in `ContentArea` and `ExportDialog`

**Files:**
- Modify: `src/components/ContentArea.tsx`
- Modify: `src/components/ExportDialog.tsx`

- [ ] **Step 1: Update `ContentArea` props and usage**

In `src/components/ContentArea.tsx`, replace the `ContentAreaProps` interface and its usages:

```tsx
interface ContentAreaProps {
  activePool: CardPool | null;
  playerId: string;
}
```

Then inside the component, replace every `props.userId` with `props.playerId`:

- In `loadRecords()`: `if (!props.activePool || !props.playerId) return;`
- In `loadRecords()`: `const result = await queryGachaRecords(props.playerId, filter);`
- In `loadRecords()`: `const allResults = await queryGachaRecords(props.playerId, countFilter);`

- [ ] **Step 2: Update `ExportDialog` props and usage**

In `src/components/ExportDialog.tsx`, replace the `ExportDialogProps` interface:

```tsx
interface ExportDialogProps {
  open: boolean;
  playerId: string;
  filter: GachaFilter;
  onClose: () => void;
}
```

Inside `handleExport`, replace:

```tsx
      await exportGachaRecords(props.userId, props.filter, filePath);
```

with:

```tsx
      await exportGachaRecords(props.playerId, props.filter, filePath);
```

- [ ] **Step 3: Commit**

```bash
git add src/components/ContentArea.tsx src/components/ExportDialog.tsx
git commit -m "refactor(frontend): rename userId prop to playerId"
```

---

### Task 12: Frontend — wire it all together in `App.tsx`

**Files:**
- Modify: `src/App.tsx`

- [ ] **Step 1: Replace `App.tsx` with the integrated shell**

Replace the full contents of `src/App.tsx` with:

```tsx
import { createSignal, createResource, Show } from "solid-js";
import type { CardPool, GachaFilter } from "./lib/types";
import { listUsers } from "./lib/commands";
import Sidebar from "./components/Sidebar";
import ContentArea from "./components/ContentArea";
import ExportDialog from "./components/ExportDialog";
import WelcomePage from "./components/WelcomePage";
import AddUserDialog from "./components/AddUserDialog";
import "./App.css";

function App() {
  const [activePool, setActivePool] = createSignal<CardPool | null>(null);
  const [exportOpen, setExportOpen] = createSignal(false);
  const [addUserOpen, setAddUserOpen] = createSignal(false);
  const [playerId, setPlayerId] = createSignal<string | null>(null);

  const [users, { refetch: refetchUsers }] = createResource(async () => {
    const list = await listUsers();
    if (list.length > 0 && playerId() === null) {
      setPlayerId(list[0]);
    }
    return list;
  });

  async function handleUserAdded(newPlayerId: string) {
    await refetchUsers();
    setPlayerId(newPlayerId);
  }

  const exportFilter = (): GachaFilter => ({
    cardPool: activePool(),
  });

  return (
    <div class="app">
      <Show
        when={(users() ?? []).length > 0 && playerId() !== null}
        fallback={
          <Show when={!users.loading}>
            <WelcomePage onUserAdded={handleUserAdded} />
          </Show>
        }
      >
        <Sidebar
          users={users() ?? []}
          playerId={playerId()}
          activePool={activePool()}
          onSelectUser={setPlayerId}
          onSelectPool={setActivePool}
          onAddUser={() => setAddUserOpen(true)}
          onExport={() => setExportOpen(true)}
        />
        <ContentArea activePool={activePool()} playerId={playerId()!} />
        <ExportDialog
          open={exportOpen()}
          playerId={playerId()!}
          filter={exportFilter()}
          onClose={() => setExportOpen(false)}
        />
        <AddUserDialog
          open={addUserOpen()}
          onClose={() => setAddUserOpen(false)}
          onUserAdded={handleUserAdded}
        />
      </Show>
    </div>
  );
}

export default App;
```

- [ ] **Step 2: Verify typecheck**

Run: `bunx tsc --noEmit`
Expected: clean — no errors in any file.

- [ ] **Step 3: Verify dev build boots**

Run: `bun run tauri dev` in a separate terminal (or background).
Expected:
- App window opens.
- No console errors.
- With an empty DB (fresh `app_data_dir`), the WelcomePage is shown.
- With a populated DB, the Sidebar + ContentArea are shown.

If the current dev DB has the old `gacha` table, it will be ignored (per the spec). You can test with a clean state by deleting the DB file at `~/Library/Application Support/<bundleId>/gacha.db` (macOS) — confirm the path with the user before rm.

- [ ] **Step 4: Manual smoke test — empty state**

With an empty DB:
1. App opens to WelcomePage.
2. Paste a valid JSON payload. Click "获取记录".
3. Loading state shows.
4. On success, WelcomePage disappears; Sidebar + ContentArea render; the new playerId appears as the selected user.
5. Click a pool in the sidebar — records render.

- [ ] **Step 5: Manual smoke test — populated state**

With at least one user already in DB:
1. App opens to Sidebar + ContentArea (no welcome).
2. The user selector shows all known playerIds; the first is selected.
3. Click "添加用户" in the sidebar footer — AddUserDialog opens.
4. Paste a second valid JSON payload. Click "获取记录".
5. Dialog closes; Sidebar shows both users; the new one is selected.
6. Change the selector back to the first user — records reload for that user.
7. Invalid inputs tested: `{}`, `not json`, `{"playerId":"abc"}` — each shows the appropriate inline error without crashing.

- [ ] **Step 6: Commit**

```bash
git add src/App.tsx
git commit -m "feat(frontend): wire welcome/add-user flow in App root"
```

---

## Verification checklist (run before marking the plan complete)

- [ ] `cargo test -p wuwa-gacha-history` — all tests pass.
- [ ] `cargo check --workspace` — clean.
- [ ] `bunx tsc --noEmit` — clean.
- [ ] `bun run format` — no diff.
- [ ] Manual: WelcomePage → fetch → user appears (Task 12 Step 4).
- [ ] Manual: AddUserDialog → fetch → selected (Task 12 Step 5).
- [ ] Manual: switching the user selector reloads records correctly.
- [ ] Manual: invalid-JSON / missing-field / bad-playerId / API-error paths each display the documented inline error and do not crash.
