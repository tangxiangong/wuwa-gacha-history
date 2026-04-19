import { QualityLevel } from "./types";
import type { GachaRecord } from "./types";
import { isStandard5Star } from "./catalog";

export const SOFT_PITY = 66;
export const HARD_PITY = 80;
export const ASTRITE_PER_PULL = 160;

/** WuWa version release dates (global). Source: game8.co version list. */
export interface VersionRelease {
  version: string;
  start: string; // ISO date (00:00 local)
}

export const VERSIONS: readonly VersionRelease[] = [
  { version: "1.0", start: "2024-05-23T04:00:00" },
  { version: "1.1", start: "2024-06-28T04:00:00" },
  { version: "1.2", start: "2024-08-15T04:00:00" },
  { version: "1.3", start: "2024-09-29T04:00:00" },
  { version: "1.4", start: "2024-11-14T04:00:00" },
  { version: "2.0", start: "2025-01-02T04:00:00" },
  { version: "2.1", start: "2025-02-13T04:00:00" },
  { version: "2.2", start: "2025-03-27T04:00:00" },
  { version: "2.3", start: "2025-04-29T04:00:00" },
  { version: "2.4", start: "2025-06-12T04:00:00" },
  { version: "2.5", start: "2025-07-24T04:00:00" },
  { version: "2.6", start: "2025-08-28T04:00:00" },
  { version: "2.7", start: "2025-10-09T04:00:00" },
  { version: "2.8", start: "2025-11-20T04:00:00" },
  { version: "3.0", start: "2025-12-25T04:00:00" },
  { version: "3.1", start: "2026-02-05T04:00:00" },
  { version: "3.2", start: "2026-03-19T04:00:00" },
] as const;

/**
 * Return the version string whose window contains the given ISO timestamp.
 * Mirrors the backend `version_of`; use this only when a record lacks the
 * persisted `version` field (shouldn't happen after migration).
 */
export function versionOf(iso: string): string {
  const t = new Date(iso).getTime();
  for (let i = VERSIONS.length - 1; i >= 0; i--) {
    if (t >= new Date(VERSIONS[i].start).getTime()) return VERSIONS[i].version;
  }
  return "pre";
}

export interface EnrichedPull {
  record: GachaRecord;
  index: number;
  pityAtPull?: number;
  isUp?: boolean;
}

export interface FiveStarSegment {
  end: EnrichedPull | null;
  items: EnrichedPull[];
  pity: number;
  isUp: boolean;
  pad: boolean;
}

export interface Version {
  version: string;           // "2.4"
  start: string;             // ISO date
  pulls: EnrichedPull[];
  total: number;
  r5: EnrichedPull[];
  r4: EnrichedPull[];
  ups: number;
  stray: number;
  upNames: string[];         // distinct UP 5★ names in release order
}

export interface BannerStats {
  total: number;
  upCount: number;
  strayCount: number;
  r5: EnrichedPull[];
  r4: EnrichedPull[];
  avgPity5: number;
  rate5: number;
  pulls: EnrichedPull[];
}

/** Build chronological, pity-annotated list from API records (newest-first). */
export function enrichPulls(records: GachaRecord[]): EnrichedPull[] {
  const chrono = [...records].reverse();
  let counter = 0;
  return chrono.map((r, i) => {
    counter += 1;
    if (r.qualityLevel === QualityLevel.FiveStar) {
      const entry: EnrichedPull = {
        record: r,
        index: i + 1,
        pityAtPull: counter,
        isUp: !isStandard5Star(r.name),
      };
      counter = 0;
      return entry;
    }
    return { record: r, index: i + 1 };
  });
}

export function bannerStats(chrono: EnrichedPull[]): BannerStats {
  const r5 = chrono.filter((p) => p.record.qualityLevel === QualityLevel.FiveStar);
  const r4 = chrono.filter((p) => p.record.qualityLevel === QualityLevel.FourStar);
  const upCount = r5.filter((p) => p.isUp).length;
  const avgPity5 = r5.length
    ? r5.reduce((a, p) => a + (p.pityAtPull ?? 0), 0) / r5.length
    : 0;
  return {
    total: chrono.length,
    upCount,
    strayCount: r5.length - upCount,
    r5,
    r4,
    avgPity5,
    rate5: chrono.length ? r5.length / chrono.length : 0,
    pulls: chrono,
  };
}

/** Split chronological pulls into segments ending at each 5★ (trailing pulls form a "pad" segment). */
export function segmentsByFive(chrono: EnrichedPull[]): FiveStarSegment[] {
  const segs: FiveStarSegment[] = [];
  let buf: EnrichedPull[] = [];
  for (const p of chrono) {
    buf.push(p);
    if (p.record.qualityLevel === QualityLevel.FiveStar) {
      segs.push({
        end: p,
        items: buf,
        pity: buf.length,
        isUp: !!p.isUp,
        pad: false,
      });
      buf = [];
    }
  }
  if (buf.length) {
    segs.push({ end: null, items: buf, pity: buf.length, isUp: false, pad: true });
  }
  return segs;
}

/** Bucket pulls by the WuWa version window they fall in, newest first.
 *  Uses the persisted `version` field; falls back to client-side compute. */
export function groupByVersion(chrono: EnrichedPull[]): Version[] {
  const buckets = new Map<string, EnrichedPull[]>();
  for (const p of chrono) {
    const v = p.record.version || versionOf(p.record.time);
    if (!buckets.has(v)) buckets.set(v, []);
    buckets.get(v)!.push(p);
  }

  const versionStart = new Map(VERSIONS.map((v) => [v.version, v.start]));
  const result: Version[] = [];
  for (const [version, pulls] of buckets) {
    const r5 = pulls.filter(
      (p) => p.record.qualityLevel === QualityLevel.FiveStar,
    );
    const r4 = pulls.filter(
      (p) => p.record.qualityLevel === QualityLevel.FourStar,
    );
    const ups = r5.filter((p) => p.isUp).length;
    const upNames: string[] = [];
    for (const p of r5) {
      if (p.isUp && !upNames.includes(p.record.name)) {
        upNames.push(p.record.name);
      }
    }
    result.push({
      version,
      start: versionStart.get(version) ?? pulls[0].record.time,
      pulls,
      total: pulls.length,
      r5,
      r4,
      ups,
      stray: r5.length - ups,
      upNames,
    });
  }
  result.sort((a, b) =>
    b.version.localeCompare(a.version, undefined, { numeric: true }),
  );
  return result;
}

export function pityColorClass(n: number): "good" | "warn" | "bad" {
  if (n <= 40) return "good";
  if (n <= SOFT_PITY) return "warn";
  return "bad";
}

export function fmtDay(iso: string): string {
  const d = new Date(iso);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}
