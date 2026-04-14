# Frontend Design Spec — Gacha History Viewer

## Overview

SolidJS + TypeScript frontend for the Wuthering Waves gacha history Tauri desktop app. Displays gacha records fetched from the game API and stored in local SQLite, with filtering, pagination, and export.

## Layout

Two-column layout: fixed left sidebar + scrollable right content area.

### Left Sidebar

Fixed-width navigation panel (~160px), divided into three groups with section headers:

- **限定池**: 限定角色, 限定武器
- **常驻池**: 常驻角色, 常驻武器
- **其他**: 新手唤取, 新手自选, 感恩自选

Bottom section (pinned):
- **设置**: Opens a settings view (UID, language config)
- **导出**: Opens export dialog (CSV / Excel / JSON, invokes `export_gacha_records` Tauri command)

Active item is visually highlighted. Clicking a pool type updates the right content area.

### Right Content Area

Header row: pool type title on the left, "筛选" toggle button on the right.

#### Collapsible Filter Panel

Hidden by default. Toggled by the "筛选" button. When any filter is active, the button shows a visual indicator (dot or badge).

Filter controls:
- **星级**: Chip-style multi-select toggles for 3★ / 4★ / 5★
- **名称**: Text input for searching character/weapon name
- **时间**: Date range picker (start date — end date)

All filter values map to the `GachaFilter` struct fields sent to `query_gacha_records`.

#### Records Table

Columns: 名称, 星级, 时间.

Row styling by quality level:
- 5★: gold text (#ffd700 dark / #e6a817 light)
- 4★: purple text (#c678dd dark / #9b59b6 light)
- 3★: blue text (#5ea6e8 dark / #5ea6e8 light), default weight

Bottom pagination: page number buttons with prev/next arrows. Page size: 20 records. Uses `GachaFilter.limit` and `GachaFilter.offset` for server-side pagination.

## Theme

System-following via `prefers-color-scheme` media query. All colors defined as CSS custom properties on `:root`.

Key color tokens:
| Token | Dark | Light |
|---|---|---|
| --bg-primary | #12122a | #f8f8fc |
| --bg-sidebar | #0e0e22 | #f0f0f6 |
| --bg-card | #1a1a35 | #ffffff |
| --bg-input | #252540 | #f8f8fc |
| --text-primary | #e0e0e0 | #1a1a2e |
| --text-secondary | #888 | #999 |
| --border | #2a2a40 | #ddd |
| --accent | #4a3f8a | #6c5ce7 |
| --star-5 | #ffd700 | #e6a817 |
| --star-4 | #c678dd | #9b59b6 |
| --star-3 | #5ea6e8 | #5ea6e8 |

## Component Structure

```
App
├── Sidebar
│   ├── NavGroup (label, items[])
│   │   └── NavItem (pool type, active state)
│   └── NavFooter (settings, export)
├── ContentArea
│   ├── ContentHeader (title, filter toggle)
│   ├── FilterPanel (collapsed/expanded)
│   │   ├── QualityChips (multi-select)
│   │   ├── NameSearch (text input)
│   │   └── TimeRange (date inputs)
│   ├── RecordTable
│   │   └── RecordRow (name, quality, time)
│   └── Pagination (page, pageSize, total)
└── ExportDialog (format select, file path via Tauri save dialog)
```

## Data Flow

1. App startup: no data loaded. User navigates to a pool type.
2. Selecting a pool type sets `cardPool` filter and calls `query_gacha_records(userId, filter)` via `invoke()`.
3. Filter changes (quality, name, time) update the filter object and re-query.
4. Pagination changes update `limit`/`offset` in filter and re-query.
5. Export: user clicks 导出, picks format and file path, calls `export_gacha_records(userId, filter, path)`.

## Tauri Commands Used

- `query_gacha_records(userId: string, filter: GachaFilter)` → `GachaRecord[]`
- `fetch_gacha_records(params: FetchParams, poolTypes: number[])` → `number` (total fetched)
- `export_gacha_records(userId: string, filter: GachaFilter, path: string)` → `void`

## File Structure

```
src/
├── index.tsx              # Mount App
├── App.tsx                # Root layout (Sidebar + ContentArea)
├── App.css                # CSS variables, global styles, theme
├── components/
│   ├── Sidebar.tsx        # Navigation sidebar
│   ├── ContentArea.tsx    # Right panel container
│   ├── FilterPanel.tsx    # Collapsible filter controls
│   ├── RecordTable.tsx    # Records table + rows
│   ├── Pagination.tsx     # Page navigation
│   └── ExportDialog.tsx   # Export format/path picker
├── lib/
│   ├── commands.ts        # Typed wrappers around invoke()
│   └── types.ts           # TypeScript types matching Rust structs
```

## Non-Goals

- Statistics / pity counter (future work)
- URL parsing / auto packet capture
- Multi-language UI (Chinese only for now)
- Mobile / responsive layout (desktop Tauri app, fixed 800x600 minimum)
