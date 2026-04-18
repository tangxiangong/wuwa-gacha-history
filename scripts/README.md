# wiki-art-fetcher

Scrape character and weapon portraits from the Kurobbs Wuthering Waves wiki
and drop them into `public/wiki-art/` for the Tauri frontend.

## Requirements

- [uv](https://docs.astral.sh/uv/) (project was scaffolded with `uv init --app`)

## Usage

From the repo root:

```bash
uv run --project scripts scripts/main.py                 # -> ./public/wiki-art/
uv run --project scripts scripts/main.py ./out           # custom output dir
uv run --project scripts scripts/main.py --only weapons  # one category
uv run --project scripts scripts/main.py -j 16           # 16 parallel downloads
```

Re-running is idempotent — already-downloaded files are skipped.

## Endpoint note

The wiki SPA fetches the catalogue via

```
POST https://api.kurobbs.com/wiki/core/catalogue/item/getPage
Content-Type: application/x-www-form-urlencoded
body: catalogueId=<id>&page=1&limit=1000
```

with `catalogueId` = 1105 (共鸣者) / 1106 (武器). **Content-Type must be
form-urlencoded**, not JSON — the backend silently returns `code:500 系统异常`
for JSON bodies. Each record's portrait URL lives at `content.contentUrl`.
