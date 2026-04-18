# Add User Flow Design

## Overview

Add the ability for users to paste a JSON payload containing API request parameters, triggering the backend to fetch gacha records and store them in a per-user SQLite table. Support multiple users with switching.

## User Flow

1. **First launch (no users)**: Main content area shows a welcome/guide page with a JSON paste area and a "fetch records" button.
2. **Paste & fetch**: User pastes JSON (`{ playerId, serverId, languageCode, recordId }`) → frontend validates → calls `fetch_gacha_records` for all 7 pool types → records stored in DB → auto-selects the new user.
3. **Subsequent use**: Sidebar top shows a user selector dropdown. User can switch between existing users. Sidebar bottom has an "add user" button that opens a dialog with the same JSON paste form.
4. **Adding another user**: Click "add user" button in sidebar → dialog opens → paste JSON → fetch → auto-switch to new user.

## Backend Changes

### Per-User Tables

Each user gets a dedicated table named `gacha_{player_id}`. The `player_id` is validated as a 9-digit numeric string before use in table names to prevent SQL injection.

**Table schema** (same as current `gacha` table minus `user_id`):

```sql
CREATE TABLE IF NOT EXISTS gacha_{player_id} (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id TEXT NOT NULL,
    card_pool INTEGER NOT NULL,
    language_code TEXT NOT NULL,
    record_id TEXT NOT NULL UNIQUE,
    quality_level INTEGER NOT NULL,
    name TEXT NOT NULL,
    time TEXT NOT NULL
)
```

Plus indexes:
```sql
CREATE UNIQUE INDEX IF NOT EXISTS idx_gacha_{player_id}_record_id ON gacha_{player_id}(record_id)
```

### GachaRecord struct changes

Remove `user_id` field. The user is identified by which table is queried.

### Updated function signatures

- `add_records(path, player_id, server_id, language_code, records)` — creates table if not exists, inserts into `gacha_{player_id}`
- `query_records(path, player_id, filter)` — queries `gacha_{player_id}`
- `list_users(path)` — queries `sqlite_master` for tables matching `gacha_%`, extracts player IDs

### New function

```rust
pub async fn list_users(path: &str) -> Result<Vec<String>>
```

Queries `SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'gacha_%'`, strips the `gacha_` prefix, returns the list of player IDs.

### Validation

`player_id` must match `^\d{9}$` (9 digits). Reject anything else with `Error::Other("invalid player_id")`. This prevents SQL injection via table names.

### Tauri commands

- `fetch_gacha_records(params, pool_types)` — unchanged signature, already receives `FetchParams` with `player_id`
- `query_gacha_records(player_id, filter)` — parameter renamed from `user_id` to `player_id`
- `export_gacha_records(player_id, filter, path)` — parameter renamed from `user_id` to `player_id`
- **New**: `list_users()` — returns `Vec<String>` of player IDs

## Frontend Changes

### New types / commands

In `commands.ts`, add:
```typescript
function listUsers(): Promise<string[]>
```

### App.tsx state changes

- `userId` signal → `playerId` signal, initialized by loading `listUsers()` on mount
- `users` signal: `string[]` — list of known player IDs, loaded on mount and refreshed after fetch
- When `users` is empty → show WelcomePage instead of ContentArea
- When `users` is non-empty and `playerId` is set → show ContentArea as before

### New component: FetchForm

A reusable form component containing:
- A `<textarea>` for pasting JSON
- A "fetch" button
- Loading state during fetch
- Error display for validation / API errors
- On success: calls `fetchGachaRecords` with parsed params and all 7 pool types, then refreshes user list and auto-selects the new user

Used by both WelcomePage and AddUserDialog.

### New component: WelcomePage

Displayed when no users exist. Centered layout with:
- Brief instruction text explaining what to paste
- The FetchForm component

### New component: AddUserDialog

Modal dialog (same style as ExportDialog) containing the FetchForm. Opened via sidebar button.

### Sidebar changes

- **Top**: User selector dropdown showing all player IDs. Selecting one switches `playerId`.
- **Bottom**: "Add user" button alongside the existing "export" button. Opens AddUserDialog.

### ContentArea changes

- Receives `playerId` instead of `userId`
- Passes `playerId` to `queryGachaRecords`

### ExportDialog changes

- Receives `playerId` instead of `userId`

## Error Handling

- Invalid JSON → show inline error "JSON 格式错误"
- Missing fields → show inline error "缺少必要字段: ..."
- Invalid playerId (not 9 digits) → show inline error "playerId 格式错误"
- API error → show inline error with message from backend
- Network error → show inline error

## Migration

The old `gacha` table (single table for all users) will be ignored. Since this is a pre-release project, no migration from the old schema is needed.
