//! Pity / UP / version-grouping analytics shared by the UI.
//! Port of `src/lib/stats.ts` (SolidJS frontend, now removed).

use crate::catalog::is_standard_5_star;
use crate::{GachaRecord, QualityLevel, VERSIONS, version_of};
use chrono::NaiveDateTime;

pub const SOFT_PITY: u32 = 66;
pub const HARD_PITY: u32 = 80;
pub const ASTRITE_PER_PULL: u32 = 160;

#[derive(Debug, Clone)]
pub struct EnrichedPull {
    pub record: GachaRecord,
    /// 1-based position in chronological order (oldest first).
    pub index: usize,
    /// Populated only for 5★ rows: how many pulls since the previous 5★.
    pub pity_at_pull: Option<u32>,
    /// Populated only for 5★ rows: true if not a standard (banner) character/weapon.
    pub is_up: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct BannerStats {
    pub total: usize,
    pub up_count: usize,
    pub stray_count: usize,
    pub r5: Vec<EnrichedPull>,
    pub r4: Vec<EnrichedPull>,
    pub avg_pity_5: f64,
    pub rate_5: f64,
}

#[derive(Debug, Clone)]
pub struct FiveStarSegment {
    pub end: Option<EnrichedPull>,
    pub items: Vec<EnrichedPull>,
    pub pity: u32,
    pub is_up: bool,
    pub pad: bool,
}

#[derive(Debug, Clone)]
pub struct VersionGroup {
    pub version: String,
    pub start: Option<NaiveDateTime>,
    pub pulls: Vec<EnrichedPull>,
    pub r5: Vec<EnrichedPull>,
    pub r4: Vec<EnrichedPull>,
    pub ups: usize,
    pub stray: usize,
    pub up_names: Vec<String>,
}

/// Convert API records (newest-first) into chronological order and annotate
/// 5★ rows with their pity counter and UP/standard classification.
pub fn enrich_pulls(mut records: Vec<GachaRecord>) -> Vec<EnrichedPull> {
    records.reverse(); // oldest first
    let mut counter: u32 = 0;
    let mut out = Vec::with_capacity(records.len());
    for (i, r) in records.into_iter().enumerate() {
        counter += 1;
        if r.quality_level == QualityLevel::FiveStar {
            let pity = counter;
            counter = 0;
            let is_up = !is_standard_5_star(&r.name);
            out.push(EnrichedPull {
                record: r,
                index: i + 1,
                pity_at_pull: Some(pity),
                is_up: Some(is_up),
            });
        } else {
            out.push(EnrichedPull {
                record: r,
                index: i + 1,
                pity_at_pull: None,
                is_up: None,
            });
        }
    }
    out
}

pub fn banner_stats(chrono: &[EnrichedPull]) -> BannerStats {
    let r5: Vec<EnrichedPull> = chrono
        .iter()
        .filter(|p| p.record.quality_level == QualityLevel::FiveStar)
        .cloned()
        .collect();
    let r4: Vec<EnrichedPull> = chrono
        .iter()
        .filter(|p| p.record.quality_level == QualityLevel::FourStar)
        .cloned()
        .collect();
    let up_count = r5.iter().filter(|p| p.is_up == Some(true)).count();
    let avg_pity_5 = if r5.is_empty() {
        0.0
    } else {
        r5.iter().map(|p| p.pity_at_pull.unwrap_or(0) as f64).sum::<f64>() / r5.len() as f64
    };
    let total = chrono.len();
    let rate_5 = if total == 0 {
        0.0
    } else {
        r5.len() as f64 / total as f64
    };
    BannerStats {
        total,
        up_count,
        stray_count: r5.len() - up_count,
        r5,
        r4,
        avg_pity_5,
        rate_5,
    }
}

/// Split chronological pulls into segments ending at each 5★; trailing pulls
/// with no 5★ become a `pad: true` segment at the end.
pub fn segments_by_five(chrono: &[EnrichedPull]) -> Vec<FiveStarSegment> {
    let mut segs = Vec::new();
    let mut buf: Vec<EnrichedPull> = Vec::new();
    for p in chrono.iter().cloned() {
        let is_five = p.record.quality_level == QualityLevel::FiveStar;
        buf.push(p.clone());
        if is_five {
            let pity = buf.len() as u32;
            segs.push(FiveStarSegment {
                end: Some(p.clone()),
                items: std::mem::take(&mut buf),
                pity,
                is_up: p.is_up.unwrap_or(false),
                pad: false,
            });
        }
    }
    if !buf.is_empty() {
        let pity = buf.len() as u32;
        segs.push(FiveStarSegment {
            end: None,
            items: buf,
            pity,
            is_up: false,
            pad: true,
        });
    }
    segs
}

pub fn group_by_version(chrono: &[EnrichedPull]) -> Vec<VersionGroup> {
    use std::collections::BTreeMap;
    let mut buckets: BTreeMap<String, Vec<EnrichedPull>> = BTreeMap::new();
    for p in chrono.iter().cloned() {
        let v = if p.record.version.is_empty() {
            let iso = p.record.time.format("%Y-%m-%dT%H:%M:%S").to_string();
            version_of(&iso).to_string()
        } else {
            p.record.version.clone()
        };
        buckets.entry(v).or_default().push(p);
    }
    let mut result: Vec<VersionGroup> = buckets
        .into_iter()
        .map(|(version, pulls)| {
            let r5: Vec<EnrichedPull> = pulls
                .iter()
                .filter(|p| p.record.quality_level == QualityLevel::FiveStar)
                .cloned()
                .collect();
            let r4: Vec<EnrichedPull> = pulls
                .iter()
                .filter(|p| p.record.quality_level == QualityLevel::FourStar)
                .cloned()
                .collect();
            let ups = r5.iter().filter(|p| p.is_up == Some(true)).count();
            let mut up_names: Vec<String> = Vec::new();
            for p in &r5 {
                if p.is_up == Some(true) && !up_names.contains(&p.record.name) {
                    up_names.push(p.record.name.clone());
                }
            }
            let start = VERSIONS
                .iter()
                .find(|vr| vr.version == version)
                .and_then(|vr| {
                    NaiveDateTime::parse_from_str(vr.start, "%Y-%m-%dT%H:%M:%S").ok()
                });
            VersionGroup {
                version,
                start,
                pulls,
                r5,
                r4,
                ups,
                stray: 0, // filled below
                up_names,
            }
        })
        .collect();
    for g in &mut result {
        g.stray = g.r5.len() - g.ups;
    }
    // Newest version first. Assumes semantic "major.minor" with numeric compare.
    result.sort_by(|a, b| natural_version_cmp(&b.version, &a.version));
    result
}

pub fn pity_tier(n: u32) -> PityTier {
    if n <= 40 {
        PityTier::Good
    } else if n <= SOFT_PITY {
        PityTier::Warn
    } else {
        PityTier::Bad
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PityTier {
    Good,
    Warn,
    Bad,
}

fn natural_version_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    fn key(s: &str) -> (u32, u32) {
        let mut it = s.split('.');
        let major: u32 = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        let minor: u32 = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        (major, minor)
    }
    key(a).cmp(&key(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CardPool;
    use chrono::NaiveDate;

    fn mk(id: u64, name: &str, q: QualityLevel, days_ago: i64) -> GachaRecord {
        GachaRecord {
            id,
            server_id: "s".into(),
            card_pool: CardPool::FeaturedResonatorConvene,
            language_code: "zh".into(),
            record_id: format!("r{id}"),
            quality_level: q,
            name: name.into(),
            time: NaiveDate::from_ymd_opt(2025, 6, 12)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                - chrono::Duration::days(days_ago),
            version: String::new(),
        }
    }

    #[test]
    fn enrich_sets_pity_and_reverses_order() {
        // Records are newest-first from API; enrich reverses.
        let raw = vec![
            mk(3, "Jinhsi", QualityLevel::FiveStar, 0),     // newest, 5★
            mk(2, "Yuanwu", QualityLevel::FourStar, 1),
            mk(1, "Yuanwu", QualityLevel::FourStar, 2),     // oldest
        ];
        let enriched = enrich_pulls(raw);
        assert_eq!(enriched.len(), 3);
        // After reverse, oldest (id=1) is index 0, newest (id=3, 5★) is last.
        assert_eq!(enriched[0].record.id, 1);
        assert_eq!(enriched[2].record.id, 3);
        // Pity for the only 5★ should be 3 (third pull since "start").
        assert_eq!(enriched[2].pity_at_pull, Some(3));
        // Non-5★ rows: no pity.
        assert_eq!(enriched[0].pity_at_pull, None);
    }

    #[test]
    fn stats_counts_up_vs_stray() {
        let raw = vec![
            mk(1, "Jinhsi", QualityLevel::FiveStar, 0),  // UP
            mk(2, "Calcharo", QualityLevel::FiveStar, 1), // standard
        ];
        let s = banner_stats(&enrich_pulls(raw));
        assert_eq!(s.up_count, 1);
        assert_eq!(s.stray_count, 1);
        assert_eq!(s.r5.len(), 2);
    }

    #[test]
    fn segments_flush_trailing_pad() {
        let raw = vec![
            mk(1, "Yuanwu", QualityLevel::FourStar, 0),
            mk(2, "Jinhsi", QualityLevel::FiveStar, 1),
            mk(3, "Yuanwu", QualityLevel::FourStar, 2),
        ];
        let segs = segments_by_five(&enrich_pulls(raw));
        assert_eq!(segs.len(), 2);
        assert!(segs[0].end.is_some());
        assert!(segs[1].pad);
    }

    #[test]
    fn pity_tier_boundaries() {
        assert_eq!(pity_tier(1), PityTier::Good);
        assert_eq!(pity_tier(40), PityTier::Good);
        assert_eq!(pity_tier(41), PityTier::Warn);
        assert_eq!(pity_tier(66), PityTier::Warn);
        assert_eq!(pity_tier(67), PityTier::Bad);
    }
}
