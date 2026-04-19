"""Fetch character & weapon portraits from the Kurobbs Wuthering Waves wiki.

Wiki catalogue URLs (public):
  https://wiki.kurobbs.com/mc/catalogue/list?fid=1099&sid=1105   -> characters
  https://wiki.kurobbs.com/mc/catalogue/list?fid=1099&sid=1106   -> weapons

The page is a Vite SPA; its list data comes from the backend endpoint
  POST https://api.kurobbs.com/wiki/core/catalogue/item/getPage
with body encoded as application/x-www-form-urlencoded (NOT JSON — that
returns a misleading 500). The relevant portrait URL for each entry lives
at `content.contentUrl`, and the display name at `name`.

Usage:
  uv run --project scripts main.py                      # -> ./assets/wiki-art/
  uv run --project scripts main.py ./out                # custom dir
  uv run --project scripts main.py --only weapons       # one category
  uv run --project scripts main.py -j 16                # 16 parallel downloads

Default output lives under assets/ so Vite serves the portraits as
static assets at /wiki-art/<name>.png without bundling them.
"""

from __future__ import annotations

import argparse
import sys
import urllib.parse
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

import httpx

API_URL = "https://api.kurobbs.com/wiki/core/catalogue/item/getPage"
UA = (
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
)
LIST_HEADERS = {
    "Origin": "https://wiki.kurobbs.com",
    "Referer": "https://wiki.kurobbs.com/",
    "wiki_type": "9",
    "User-Agent": UA,
    "Content-Type": "application/x-www-form-urlencoded",
}
DOWNLOAD_HEADERS = {"User-Agent": UA}

CATALOGUES: dict[str, int] = {
    "characters": 1105,
    "weapons": 1106,
}

_ILLEGAL_FS_CHARS = set('/\\:*?"<>|\0')


def sanitize(name: str) -> str:
    cleaned = "".join("_" if c in _ILLEGAL_FS_CHARS else c for c in name)
    cleaned = cleaned.strip().strip(".")
    return cleaned or "unnamed"


def fetch_list(client: httpx.Client, catalogue_id: int, limit: int = 1000) -> list[dict]:
    resp = client.post(
        API_URL,
        headers=LIST_HEADERS,
        data={"catalogueId": catalogue_id, "page": 1, "limit": limit},
        timeout=30,
    )
    resp.raise_for_status()
    payload = resp.json()
    if payload.get("code") != 200:
        raise RuntimeError(
            f"API error {payload.get('code')}: {payload.get('msg')} (body={payload})"
        )
    results = payload["data"]["results"]
    records = results.get("records") or []
    total = results.get("total") or len(records)
    if total > len(records):
        raise RuntimeError(
            f"catalogueId={catalogue_id}: server has {total} records but only "
            f"returned {len(records)}; bump --limit or implement pagination"
        )
    return records


def download(client: httpx.Client, url: str, dest: Path) -> str:
    if dest.exists() and dest.stat().st_size > 0:
        return "skip"
    dest.parent.mkdir(parents=True, exist_ok=True)
    resp = client.get(url, headers=DOWNLOAD_HEADERS, timeout=60)
    resp.raise_for_status()
    if not resp.content:
        raise RuntimeError("empty body")
    tmp = dest.with_suffix(dest.suffix + ".part")
    tmp.write_bytes(resp.content)
    tmp.replace(dest)
    return "ok"


def collect_tasks(records: list[dict], dest_root: Path) -> list[tuple[str, str, Path]]:
    tasks: list[tuple[str, str, Path]] = []
    for r in records:
        name = (r.get("name") or r.get("content", {}).get("title") or f"id_{r.get('id')}").strip()
        url = (r.get("content") or {}).get("contentUrl")
        if not url:
            print(f"  [skip] {name!r}: no contentUrl", file=sys.stderr)
            continue
        ext = Path(urllib.parse.urlparse(url).path).suffix or ".png"
        tasks.append((name, url, dest_root / f"{sanitize(name)}{ext}"))
    return tasks


def run(out_dir: Path, only: str | None, concurrency: int) -> int:
    categories = [only] if only else list(CATALOGUES.keys())
    errors = 0
    with httpx.Client(http2=False, follow_redirects=True) as client:
        for cat in categories:
            if cat not in CATALOGUES:
                print(
                    f"unknown category: {cat}; choose from {list(CATALOGUES)}",
                    file=sys.stderr,
                )
                errors += 1
                continue
            cid = CATALOGUES[cat]
            print(f"\n=== {cat} (catalogueId={cid}) ===")
            try:
                records = fetch_list(client, cid)
            except Exception as e:
                print(f"  fetch failed: {e}", file=sys.stderr)
                errors += 1
                continue
            print(f"  {len(records)} records")

            tasks = collect_tasks(records, out_dir / cat)
            if not tasks:
                continue

            counts = {"ok": 0, "skip": 0, "err": 0}
            with ThreadPoolExecutor(max_workers=concurrency) as pool:
                futures = {
                    pool.submit(download, client, url, dest): (name, dest)
                    for name, url, dest in tasks
                }
                for fut in as_completed(futures):
                    name, dest = futures[fut]
                    try:
                        status = fut.result()
                        counts[status] += 1
                        print(f"  [{status:4}] {name} -> {dest.name}")
                    except Exception as e:
                        counts["err"] += 1
                        errors += 1
                        print(f"  [err ] {name}: {e}", file=sys.stderr)
            print(
                f"  summary: downloaded={counts['ok']} "
                f"cached={counts['skip']} failed={counts['err']}"
            )
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        "out",
        nargs="?",
        default="assets/wiki-art",
        help="output directory (default: ./assets/wiki-art)",
    )
    parser.add_argument(
        "--only", choices=list(CATALOGUES.keys()), help="fetch one category only"
    )
    parser.add_argument(
        "-j", "--jobs", type=int, default=8, help="parallel downloads (default: 8)"
    )
    args = parser.parse_args()
    return run(Path(args.out), args.only, args.jobs)


if __name__ == "__main__":
    sys.exit(main())
