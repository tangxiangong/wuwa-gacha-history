# Frontend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the SolidJS frontend for the gacha history viewer — sidebar navigation, record table with filtering/pagination, and export dialog.

**Architecture:** Two-column layout with SolidJS reactive signals. Left sidebar for pool navigation, right content area with collapsible filter panel, record table, and pagination. All data flows through typed Tauri `invoke()` wrappers. CSS custom properties for system-following dark/light theme.

**Tech Stack:** SolidJS, TypeScript, Vite, Tauri v2 (`@tauri-apps/api`), `@tauri-apps/plugin-dialog` (for file save dialog)

**Note:** Quality level filter is single-select (one at a time or none) because the backend `GachaFilter.qualityLevel` accepts `Option<QualityLevel>`, not a list. This can be enhanced later by updating the backend.

---

### Task 1: TypeScript Types and Tauri Command Wrappers

**Files:**
- Create: `src/lib/types.ts`
- Create: `src/lib/commands.ts`

These files are the foundation — every component depends on them.

- [ ] **Step 1: Create types matching Rust structs**

```typescript
// src/lib/types.ts

export enum CardPool {
  FeaturedResonatorConvene = 1,
  FeaturedWeaponConvene = 2,
  StandardResonatorConvene = 3,
  StandardWeaponConvene = 4,
  NoviceConvene = 5,
  BeginnerChoiceConvene = 6,
  GivebackCustomConvene = 7,
}

export const CARD_POOL_LABELS: Record<CardPool, string> = {
  [CardPool.FeaturedResonatorConvene]: "限定角色",
  [CardPool.FeaturedWeaponConvene]: "限定武器",
  [CardPool.StandardResonatorConvene]: "常驻角色",
  [CardPool.StandardWeaponConvene]: "常驻武器",
  [CardPool.NoviceConvene]: "新手唤取",
  [CardPool.BeginnerChoiceConvene]: "新手自选",
  [CardPool.GivebackCustomConvene]: "感恩自选",
};

export enum QualityLevel {
  ThreeStar = 3,
  FourStar = 4,
  FiveStar = 5,
}

export interface GachaRecord {
  id: number;
  userId: string;
  serverId: string;
  cardPool: CardPool;
  languageCode: string;
  recordId: string;
  qualityLevel: QualityLevel;
  name: string;
  time: string;
}

export interface GachaFilter {
  cardPool?: CardPool | null;
  qualityLevel?: QualityLevel | null;
  name?: string | null;
  timeFrom?: string | null;
  timeTo?: string | null;
  limit?: number | null;
  offset?: number | null;
}

export interface FetchParams {
  playerId: string;
  serverId: string;
  languageCode: string;
  recordId: string;
}
```

- [ ] **Step 2: Create Tauri command wrappers**

```typescript
// src/lib/commands.ts

import { invoke } from "@tauri-apps/api/core";
import type { CardPool, FetchParams, GachaFilter, GachaRecord } from "./types";

export async function queryGachaRecords(
  userId: string,
  filter: GachaFilter,
): Promise<GachaRecord[]> {
  return invoke("query_gacha_records", { userId, filter });
}

export async function fetchGachaRecords(
  params: FetchParams,
  poolTypes: CardPool[],
): Promise<number> {
  return invoke("fetch_gacha_records", { params, poolTypes });
}

export async function exportGachaRecords(
  userId: string,
  filter: GachaFilter,
  path: string,
): Promise<void> {
  return invoke("export_gacha_records", { userId, filter, path });
}
```

- [ ] **Step 3: Verify TypeScript compiles**

Run: `cd /Users/xiaoyu/Projects/wuwa-gacha-history && npx tsc --noEmit`
Expected: No errors (or only pre-existing template errors from App.tsx which we'll replace later)

- [ ] **Step 4: Commit**

```bash
git add src/lib/types.ts src/lib/commands.ts
git commit -m "feat: add TypeScript types and Tauri command wrappers"
```

---

### Task 2: CSS Theme and Global Styles

**Files:**
- Modify: `src/App.css` (full rewrite)

Replace the Tauri template CSS with the dual-theme design system.

- [ ] **Step 1: Rewrite App.css with theme variables and layout styles**

```css
/* src/App.css */

/* ===== Light theme (default) ===== */
:root {
  --bg-primary: #f8f8fc;
  --bg-sidebar: #f0f0f6;
  --bg-card: #ffffff;
  --bg-input: #f8f8fc;
  --text-primary: #1a1a2e;
  --text-secondary: #999;
  --border: #ddd;
  --accent: #6c5ce7;
  --accent-hover: #5a4bd6;
  --star-5: #e6a817;
  --star-4: #9b59b6;
  --star-3: #5ea6e8;

  font-family: "Inter", "Segoe UI", system-ui, -apple-system, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: var(--text-primary);
  background-color: var(--bg-primary);
}

/* ===== Dark theme ===== */
@media (prefers-color-scheme: dark) {
  :root {
    --bg-primary: #12122a;
    --bg-sidebar: #0e0e22;
    --bg-card: #1a1a35;
    --bg-input: #252540;
    --text-primary: #e0e0e0;
    --text-secondary: #888;
    --border: #2a2a40;
    --accent: #4a3f8a;
    --accent-hover: #5a4f9a;
    --star-5: #ffd700;
    --star-4: #c678dd;
    --star-3: #5ea6e8;
  }
}

/* ===== Reset ===== */
*,
*::before,
*::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

/* ===== App layout ===== */
.app {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

/* ===== Sidebar ===== */
.sidebar {
  width: 160px;
  min-width: 160px;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  padding: 12px 8px;
  overflow-y: auto;
}

.nav-group-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  padding: 0 8px;
  margin-bottom: 4px;
  margin-top: 12px;
}

.nav-group-label:first-child {
  margin-top: 0;
}

.nav-item {
  padding: 6px 10px;
  margin-bottom: 2px;
  border-radius: 6px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
}

.nav-item:hover {
  background: var(--border);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--accent);
  color: #fff;
}

.nav-footer {
  margin-top: auto;
  border-top: 1px solid var(--border);
  padding-top: 8px;
}

.nav-footer .nav-item {
  color: var(--text-secondary);
}

/* ===== Content area ===== */
.content-area {
  flex: 1;
  padding: 16px 20px;
  overflow-y: auto;
}

.content-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.content-title {
  font-size: 18px;
  font-weight: 600;
}

/* ===== Filter panel ===== */
.filter-toggle {
  background: var(--bg-card);
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 4px 12px;
  font-size: 13px;
  cursor: pointer;
  transition: border-color 0.15s;
}

.filter-toggle:hover {
  border-color: var(--accent);
}

.filter-toggle.has-filter {
  border-color: var(--accent);
  color: var(--accent);
}

.filter-panel {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 12px;
}

.filter-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.filter-row:last-child {
  margin-bottom: 0;
}

.filter-label {
  font-size: 12px;
  color: var(--text-secondary);
  width: 40px;
  flex-shrink: 0;
}

.chip {
  padding: 2px 10px;
  border-radius: 12px;
  font-size: 12px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
}

.chip:hover {
  border-color: var(--text-primary);
  color: var(--text-primary);
}

.chip.active-5 {
  border-color: var(--star-5);
  color: var(--star-5);
  background: color-mix(in srgb, var(--star-5) 12%, transparent);
}

.chip.active-4 {
  border-color: var(--star-4);
  color: var(--star-4);
  background: color-mix(in srgb, var(--star-4) 12%, transparent);
}

.chip.active-3 {
  border-color: var(--star-3);
  color: var(--star-3);
  background: color-mix(in srgb, var(--star-3) 12%, transparent);
}

.filter-input {
  flex: 1;
  background: var(--bg-input);
  color: var(--text-primary);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 4px 10px;
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.filter-input:focus {
  border-color: var(--accent);
}

.filter-input-short {
  width: 120px;
  flex: none;
}

.filter-separator {
  color: var(--text-secondary);
}

/* ===== Record table ===== */
.record-table {
  width: 100%;
  font-size: 13px;
}

.record-table-header {
  display: flex;
  padding: 6px 10px;
  color: var(--text-secondary);
  font-size: 12px;
  border-bottom: 1px solid var(--border);
}

.record-row {
  display: flex;
  padding: 8px 10px;
  border-bottom: 1px solid color-mix(in srgb, var(--border) 50%, transparent);
  align-items: center;
  transition: background 0.1s;
}

.record-row:hover {
  background: color-mix(in srgb, var(--border) 30%, transparent);
}

.col-name {
  flex: 3;
  font-weight: 500;
}

.col-quality {
  flex: 1;
}

.col-time {
  flex: 2;
  color: var(--text-secondary);
}

.star-5 { color: var(--star-5); }
.star-4 { color: var(--star-4); }
.star-3 { color: var(--star-3); }

.record-empty {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
}

/* ===== Pagination ===== */
.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 4px;
  margin-top: 16px;
}

.page-btn {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  border: none;
  cursor: pointer;
  background: var(--bg-card);
  color: var(--text-secondary);
  transition: all 0.15s;
}

.page-btn:hover:not(:disabled) {
  background: var(--border);
  color: var(--text-primary);
}

.page-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.page-btn.active {
  background: var(--accent);
  color: #fff;
}

/* ===== Export dialog ===== */
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.dialog {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 20px;
  min-width: 320px;
  max-width: 400px;
}

.dialog h3 {
  margin-bottom: 12px;
  font-size: 16px;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 16px;
}

.btn {
  padding: 6px 16px;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  border: 1px solid var(--border);
  background: var(--bg-card);
  color: var(--text-primary);
  transition: all 0.15s;
}

.btn:hover {
  border-color: var(--accent);
}

.btn-primary {
  background: var(--accent);
  color: #fff;
  border-color: var(--accent);
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.format-options {
  display: flex;
  gap: 8px;
}

.format-option {
  flex: 1;
  padding: 10px;
  border: 1px solid var(--border);
  border-radius: 8px;
  text-align: center;
  cursor: pointer;
  font-size: 13px;
  background: transparent;
  color: var(--text-primary);
  transition: all 0.15s;
}

.format-option:hover {
  border-color: var(--accent);
}

.format-option.active {
  border-color: var(--accent);
  background: color-mix(in srgb, var(--accent) 15%, transparent);
}
```

- [ ] **Step 2: Commit**

```bash
git add src/App.css
git commit -m "feat: add dual-theme CSS design system with all component styles"
```

---

### Task 3: Sidebar Component

**Files:**
- Create: `src/components/Sidebar.tsx`

- [ ] **Step 1: Create Sidebar component**

```tsx
// src/components/Sidebar.tsx

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
    items: [
      CardPool.StandardResonatorConvene,
      CardPool.StandardWeaponConvene,
    ],
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
  activePool: CardPool | null;
  onSelectPool: (pool: CardPool) => void;
  onExport: () => void;
}

export default function Sidebar(props: SidebarProps) {
  return (
    <nav class="sidebar">
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
        <div class="nav-item" onClick={props.onExport}>
          导出
        </div>
      </div>
    </nav>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/Sidebar.tsx
git commit -m "feat: add Sidebar navigation component"
```

---

### Task 4: RecordTable Component

**Files:**
- Create: `src/components/RecordTable.tsx`

- [ ] **Step 1: Create RecordTable component**

```tsx
// src/components/RecordTable.tsx

import { For, Show } from "solid-js";
import type { GachaRecord } from "../lib/types";
import { QualityLevel } from "../lib/types";

function qualityClass(level: QualityLevel): string {
  switch (level) {
    case QualityLevel.FiveStar:
      return "star-5";
    case QualityLevel.FourStar:
      return "star-4";
    case QualityLevel.ThreeStar:
      return "star-3";
  }
}

function qualityText(level: QualityLevel): string {
  return `${level}★`;
}

function formatTime(time: string): string {
  return time.replace("T", " ").slice(0, 16);
}

interface RecordTableProps {
  records: GachaRecord[];
  loading: boolean;
}

export default function RecordTable(props: RecordTableProps) {
  return (
    <div class="record-table">
      <div class="record-table-header">
        <span class="col-name">名称</span>
        <span class="col-quality">星级</span>
        <span class="col-time">时间</span>
      </div>
      <Show
        when={!props.loading && props.records.length > 0}
        fallback={
          <div class="record-empty">
            {props.loading ? "加载中..." : "暂无记录"}
          </div>
        }
      >
        <For each={props.records}>
          {(record) => (
            <div class={`record-row ${qualityClass(record.qualityLevel)}`}>
              <span class="col-name">{record.name}</span>
              <span class="col-quality">{qualityText(record.qualityLevel)}</span>
              <span class="col-time">{formatTime(record.time)}</span>
            </div>
          )}
        </For>
      </Show>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/RecordTable.tsx
git commit -m "feat: add RecordTable component with quality-colored rows"
```

---

### Task 5: Pagination Component

**Files:**
- Create: `src/components/Pagination.tsx`

- [ ] **Step 1: Create Pagination component**

```tsx
// src/components/Pagination.tsx

import { For, Show } from "solid-js";

interface PaginationProps {
  currentPage: number;
  totalRecords: number;
  pageSize: number;
  onPageChange: (page: number) => void;
}

export default function Pagination(props: PaginationProps) {
  const totalPages = () => Math.max(1, Math.ceil(props.totalRecords / props.pageSize));

  const pageNumbers = () => {
    const total = totalPages();
    const current = props.currentPage;
    const pages: number[] = [];

    let start = Math.max(1, current - 2);
    let end = Math.min(total, start + 4);
    start = Math.max(1, end - 4);

    for (let i = start; i <= end; i++) {
      pages.push(i);
    }
    return pages;
  };

  return (
    <Show when={totalPages() > 1}>
      <div class="pagination">
        <button
          class="page-btn"
          disabled={props.currentPage <= 1}
          onClick={() => props.onPageChange(props.currentPage - 1)}
        >
          ‹
        </button>
        <For each={pageNumbers()}>
          {(page) => (
            <button
              class={`page-btn ${page === props.currentPage ? "active" : ""}`}
              onClick={() => props.onPageChange(page)}
            >
              {page}
            </button>
          )}
        </For>
        <button
          class="page-btn"
          disabled={props.currentPage >= totalPages()}
          onClick={() => props.onPageChange(props.currentPage + 1)}
        >
          ›
        </button>
      </div>
    </Show>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/Pagination.tsx
git commit -m "feat: add Pagination component"
```

---

### Task 6: FilterPanel Component

**Files:**
- Create: `src/components/FilterPanel.tsx`

- [ ] **Step 1: Create FilterPanel component**

```tsx
// src/components/FilterPanel.tsx

import { Show } from "solid-js";
import { QualityLevel } from "../lib/types";

interface FilterPanelProps {
  open: boolean;
  qualityLevel: QualityLevel | null;
  nameQuery: string;
  timeFrom: string;
  timeTo: string;
  onQualityChange: (level: QualityLevel | null) => void;
  onNameChange: (name: string) => void;
  onTimeFromChange: (date: string) => void;
  onTimeToChange: (date: string) => void;
}

export default function FilterPanel(props: FilterPanelProps) {
  function toggleQuality(level: QualityLevel) {
    props.onQualityChange(props.qualityLevel === level ? null : level);
  }

  function chipClass(level: QualityLevel): string {
    if (props.qualityLevel !== level) return "chip";
    return `chip active-${level}`;
  }

  return (
    <Show when={props.open}>
      <div class="filter-panel">
        <div class="filter-row">
          <span class="filter-label">星级</span>
          <button class={chipClass(QualityLevel.FiveStar)} onClick={() => toggleQuality(QualityLevel.FiveStar)}>
            5★
          </button>
          <button class={chipClass(QualityLevel.FourStar)} onClick={() => toggleQuality(QualityLevel.FourStar)}>
            4★
          </button>
          <button class={chipClass(QualityLevel.ThreeStar)} onClick={() => toggleQuality(QualityLevel.ThreeStar)}>
            3★
          </button>
        </div>
        <div class="filter-row">
          <span class="filter-label">名称</span>
          <input
            class="filter-input"
            placeholder="搜索角色/武器名称..."
            value={props.nameQuery}
            onInput={(e) => props.onNameChange(e.currentTarget.value)}
          />
        </div>
        <div class="filter-row">
          <span class="filter-label">时间</span>
          <input
            type="date"
            class="filter-input filter-input-short"
            value={props.timeFrom}
            onInput={(e) => props.onTimeFromChange(e.currentTarget.value)}
          />
          <span class="filter-separator">—</span>
          <input
            type="date"
            class="filter-input filter-input-short"
            value={props.timeTo}
            onInput={(e) => props.onTimeToChange(e.currentTarget.value)}
          />
        </div>
      </div>
    </Show>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/FilterPanel.tsx
git commit -m "feat: add collapsible FilterPanel with quality chips, name search, time range"
```

---

### Task 7: ExportDialog Component

**Files:**
- Create: `src/components/ExportDialog.tsx`

Requires `@tauri-apps/plugin-dialog` for the file save dialog.

- [ ] **Step 1: Install dialog plugin**

Run:
```bash
cd /Users/xiaoyu/Projects/wuwa-gacha-history
bun add @tauri-apps/plugin-dialog
cd src-tauri && cargo add tauri-plugin-dialog
```

- [ ] **Step 2: Register dialog plugin in Tauri backend**

Modify `src-tauri/src/lib.rs` — add the plugin registration:

```rust
// Change this line:
        .plugin(tauri_plugin_opener::init())
// To:
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
```

- [ ] **Step 3: Add dialog permission to capabilities**

Modify `src-tauri/capabilities/default.json` — add `"dialog:default"` to permissions:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "opener:default",
    "dialog:default"
  ]
}
```

- [ ] **Step 4: Create ExportDialog component**

```tsx
// src/components/ExportDialog.tsx

import { createSignal, Show } from "solid-js";
import { save } from "@tauri-apps/plugin-dialog";
import { exportGachaRecords } from "../lib/commands";
import type { GachaFilter } from "../lib/types";

interface ExportDialogProps {
  open: boolean;
  userId: string;
  filter: GachaFilter;
  onClose: () => void;
}

type ExportFormat = "csv" | "xlsx" | "json";

const FORMAT_EXTENSIONS: Record<ExportFormat, string> = {
  csv: "csv",
  xlsx: "xlsx",
  json: "json",
};

const FORMAT_LABELS: Record<ExportFormat, string> = {
  csv: "CSV",
  xlsx: "Excel",
  json: "JSON",
};

export default function ExportDialog(props: ExportDialogProps) {
  const [format, setFormat] = createSignal<ExportFormat>("xlsx");
  const [exporting, setExporting] = createSignal(false);
  const [error, setError] = createSignal("");

  async function handleExport() {
    setError("");
    const ext = FORMAT_EXTENSIONS[format()];
    const filePath = await save({
      defaultPath: `gacha-history.${ext}`,
      filters: [{ name: FORMAT_LABELS[format()], extensions: [ext] }],
    });

    if (!filePath) return;

    setExporting(true);
    try {
      await exportGachaRecords(props.userId, props.filter, filePath);
      props.onClose();
    } catch (e) {
      setError(String(e));
    } finally {
      setExporting(false);
    }
  }

  return (
    <Show when={props.open}>
      <div class="dialog-overlay" onClick={() => props.onClose()}>
        <div class="dialog" onClick={(e) => e.stopPropagation()}>
          <h3>导出记录</h3>
          <div class="format-options">
            {(["csv", "xlsx", "json"] as ExportFormat[]).map((fmt) => (
              <button
                class={`format-option ${format() === fmt ? "active" : ""}`}
                onClick={() => setFormat(fmt)}
              >
                {FORMAT_LABELS[fmt]}
              </button>
            ))}
          </div>
          <Show when={error()}>
            <p style={{ color: "var(--star-5)", "font-size": "12px", "margin-top": "8px" }}>
              {error()}
            </p>
          </Show>
          <div class="dialog-actions">
            <button class="btn" onClick={() => props.onClose()}>
              取消
            </button>
            <button class="btn btn-primary" onClick={handleExport} disabled={exporting()}>
              {exporting() ? "导出中..." : "导出"}
            </button>
          </div>
        </div>
      </div>
    </Show>
  );
}
```

- [ ] **Step 5: Commit**

```bash
git add src/components/ExportDialog.tsx src-tauri/src/lib.rs src-tauri/capabilities/default.json package.json bun.lock src-tauri/Cargo.toml Cargo.lock
git commit -m "feat: add ExportDialog with file save dialog and format selection"
```

---

### Task 8: ContentArea Component

**Files:**
- Create: `src/components/ContentArea.tsx`

Wires together FilterPanel, RecordTable, and Pagination. Manages query state.

- [ ] **Step 1: Create ContentArea component**

```tsx
// src/components/ContentArea.tsx

import { createSignal, createEffect, on, Show } from "solid-js";
import { queryGachaRecords } from "../lib/commands";
import { CARD_POOL_LABELS, QualityLevel } from "../lib/types";
import type { CardPool, GachaFilter, GachaRecord } from "../lib/types";
import FilterPanel from "./FilterPanel";
import RecordTable from "./RecordTable";
import Pagination from "./Pagination";

const PAGE_SIZE = 20;

interface ContentAreaProps {
  activePool: CardPool | null;
  userId: string;
}

export default function ContentArea(props: ContentAreaProps) {
  const [records, setRecords] = createSignal<GachaRecord[]>([]);
  const [loading, setLoading] = createSignal(false);
  const [page, setPage] = createSignal(1);
  const [totalRecords, setTotalRecords] = createSignal(0);

  // Filter state
  const [filterOpen, setFilterOpen] = createSignal(false);
  const [qualityLevel, setQualityLevel] = createSignal<QualityLevel | null>(null);
  const [nameQuery, setNameQuery] = createSignal("");
  const [timeFrom, setTimeFrom] = createSignal("");
  const [timeTo, setTimeTo] = createSignal("");

  const hasActiveFilter = () =>
    qualityLevel() !== null || nameQuery() !== "" || timeFrom() !== "" || timeTo() !== "";

  function buildFilter(): GachaFilter {
    return {
      cardPool: props.activePool,
      qualityLevel: qualityLevel(),
      name: nameQuery() || null,
      timeFrom: timeFrom() ? `${timeFrom()}T00:00:00` : null,
      timeTo: timeTo() ? `${timeTo()}T23:59:59` : null,
      limit: PAGE_SIZE,
      offset: (page() - 1) * PAGE_SIZE,
    };
  }

  async function loadRecords() {
    if (!props.activePool || !props.userId) return;
    setLoading(true);
    try {
      const filter = buildFilter();
      const result = await queryGachaRecords(props.userId, filter);
      setRecords(result);

      // Fetch total count (without limit/offset) for pagination
      const countFilter = { ...filter, limit: null, offset: null };
      const allResults = await queryGachaRecords(props.userId, countFilter);
      setTotalRecords(allResults.length);
    } catch (e) {
      console.error("Failed to query records:", e);
      setRecords([]);
      setTotalRecords(0);
    } finally {
      setLoading(false);
    }
  }

  // Reset page and reload when pool or filters change
  createEffect(
    on(
      () => [props.activePool, qualityLevel(), nameQuery(), timeFrom(), timeTo()],
      () => {
        setPage(1);
        loadRecords();
      },
    ),
  );

  // Reload when page changes (but don't reset page)
  createEffect(on(() => page(), () => loadRecords(), { defer: true }));

  function handlePageChange(newPage: number) {
    setPage(newPage);
  }

  return (
    <div class="content-area">
      <Show
        when={props.activePool !== null}
        fallback={<div class="record-empty">请选择一个卡池类型</div>}
      >
        <div class="content-header">
          <span class="content-title">{CARD_POOL_LABELS[props.activePool!]}</span>
          <button
            class={`filter-toggle ${hasActiveFilter() ? "has-filter" : ""}`}
            onClick={() => setFilterOpen(!filterOpen())}
          >
            {filterOpen() ? "▲ 筛选" : "▼ 筛选"}
          </button>
        </div>
        <FilterPanel
          open={filterOpen()}
          qualityLevel={qualityLevel()}
          nameQuery={nameQuery()}
          timeFrom={timeFrom()}
          timeTo={timeTo()}
          onQualityChange={setQualityLevel}
          onNameChange={setNameQuery}
          onTimeFromChange={setTimeFrom}
          onTimeToChange={setTimeTo}
        />
        <RecordTable records={records()} loading={loading()} />
        <Pagination
          currentPage={page()}
          totalRecords={totalRecords()}
          pageSize={PAGE_SIZE}
          onPageChange={handlePageChange}
        />
      </Show>
    </div>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/ContentArea.tsx
git commit -m "feat: add ContentArea with filter state, pagination, and query logic"
```

---

### Task 9: App Root — Wire Everything Together

**Files:**
- Modify: `src/App.tsx` (full rewrite)
- Modify: `src/index.tsx` (no change needed, already correct)
- Delete: `src/assets/logo.svg` (unused)

- [ ] **Step 1: Rewrite App.tsx**

```tsx
// src/App.tsx

import { createSignal } from "solid-js";
import type { CardPool, GachaFilter } from "./lib/types";
import Sidebar from "./components/Sidebar";
import ContentArea from "./components/ContentArea";
import ExportDialog from "./components/ExportDialog";
import "./App.css";

function App() {
  const [activePool, setActivePool] = createSignal<CardPool | null>(null);
  const [exportOpen, setExportOpen] = createSignal(false);

  // TODO: This should come from a settings view in the future.
  // For now, hardcoded or empty — records won't load without a valid userId.
  const userId = () => "";

  const exportFilter = (): GachaFilter => ({
    cardPool: activePool(),
  });

  return (
    <div class="app">
      <Sidebar
        activePool={activePool()}
        onSelectPool={setActivePool}
        onExport={() => setExportOpen(true)}
      />
      <ContentArea activePool={activePool()} userId={userId()} />
      <ExportDialog
        open={exportOpen()}
        userId={userId()}
        filter={exportFilter()}
        onClose={() => setExportOpen(false)}
      />
    </div>
  );
}

export default App;
```

- [ ] **Step 2: Update index.html title**

Modify `index.html` — change the `<title>` tag:

```html
<title>鸣潮抽卡记录</title>
```

- [ ] **Step 3: Remove unused template files**

```bash
rm src/assets/logo.svg
rm -f public/vite.svg public/tauri.svg public/solid.svg
```

- [ ] **Step 4: Verify it compiles**

Run: `cd /Users/xiaoyu/Projects/wuwa-gacha-history && npx tsc --noEmit`
Expected: No errors

- [ ] **Step 5: Commit**

```bash
git add src/App.tsx index.html
git rm src/assets/logo.svg
git rm -f public/vite.svg public/tauri.svg public/solid.svg
git commit -m "feat: wire up App root with Sidebar, ContentArea, and ExportDialog"
```

---

### Task 10: Visual Verification

- [ ] **Step 1: Run the dev server**

Run: `cd /Users/xiaoyu/Projects/wuwa-gacha-history && bun run tauri dev`

- [ ] **Step 2: Verify visually**

Check:
- Two-column layout renders (sidebar + content area)
- Sidebar shows 3 groups with pool items
- Clicking a pool highlights it and shows the content header
- "筛选" button toggles the filter panel open/closed
- Quality chips toggle on/off with correct colors
- Empty state "暂无记录" shows when no userId is set
- Light/dark theme follows system setting (toggle in OS settings to verify)
- "导出" opens the export dialog with 3 format options
- Export dialog close works (cancel button and overlay click)

- [ ] **Step 3: Fix any visual issues found**

Address any CSS or layout problems discovered during verification.

- [ ] **Step 4: Final commit if changes were made**

```bash
git add -A
git commit -m "fix: visual adjustments from manual testing"
```
