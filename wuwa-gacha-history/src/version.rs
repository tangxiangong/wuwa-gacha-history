//! WuWa global version release schedule.
//!
//! Source: game8.co All Version Updates and Release Dates.
//! Updated through 3.2 (released 2026-03-19).
//!
//! Times are stored as ISO `%Y-%m-%dT%H:%M:%S` strings, matching the format
//! used by the gacha time column, so `str` comparison is chronological.

pub struct VersionRelease {
    pub version: &'static str,
    /// ISO `%Y-%m-%dT%H:%M:%S`, server time (CN): patches go live at 04:00
    /// on release day after the scheduled maintenance window.
    pub start: &'static str,
}

/// Ordered oldest-first.
pub const VERSIONS: &[VersionRelease] = &[
    VersionRelease { version: "1.0", start: "2024-05-23T04:00:00" },
    VersionRelease { version: "1.1", start: "2024-06-28T04:00:00" },
    VersionRelease { version: "1.2", start: "2024-08-15T04:00:00" },
    VersionRelease { version: "1.3", start: "2024-09-29T04:00:00" },
    VersionRelease { version: "1.4", start: "2024-11-14T04:00:00" },
    VersionRelease { version: "2.0", start: "2025-01-02T04:00:00" },
    VersionRelease { version: "2.1", start: "2025-02-13T04:00:00" },
    VersionRelease { version: "2.2", start: "2025-03-27T04:00:00" },
    VersionRelease { version: "2.3", start: "2025-04-29T04:00:00" },
    VersionRelease { version: "2.4", start: "2025-06-12T04:00:00" },
    VersionRelease { version: "2.5", start: "2025-07-24T04:00:00" },
    VersionRelease { version: "2.6", start: "2025-08-28T04:00:00" },
    VersionRelease { version: "2.7", start: "2025-10-09T04:00:00" },
    VersionRelease { version: "2.8", start: "2025-11-20T04:00:00" },
    VersionRelease { version: "3.0", start: "2025-12-25T04:00:00" },
    VersionRelease { version: "3.1", start: "2026-02-05T04:00:00" },
    VersionRelease { version: "3.2", start: "2026-03-19T04:00:00" },
];

/// Find the WuWa version whose window contains the given ISO timestamp.
/// Returns "pre" for timestamps before 1.0 launch.
pub fn version_of(iso_time: &str) -> &'static str {
    for v in VERSIONS.iter().rev() {
        if iso_time >= v.start {
            return v.version;
        }
    }
    "pre"
}

/// Build a CASE expression that maps a `time` column value to its version
/// string, suitable for use inside `UPDATE ... SET version = (...)`.
pub(crate) fn sql_case_expression() -> String {
    let mut out = String::from("CASE");
    for v in VERSIONS.iter().rev() {
        out.push_str(&format!(" WHEN time >= '{}' THEN '{}'", v.start, v.version));
    }
    out.push_str(" ELSE 'pre' END");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_of_buckets_correctly() {
        assert_eq!(version_of("2024-05-23T04:00:00"), "1.0");
        assert_eq!(version_of("2024-06-27T23:59:59"), "1.0");
        assert_eq!(version_of("2024-06-28T04:00:00"), "1.1");
        assert_eq!(version_of("2025-01-01T23:59:59"), "1.4");
        assert_eq!(version_of("2025-01-02T04:00:00"), "2.0");
        assert_eq!(version_of("2026-03-18T12:00:00"), "3.1");
        assert_eq!(version_of("2026-03-19T04:00:00"), "3.2");
        assert_eq!(version_of("2026-04-18T04:00:00"), "3.2");
    }

    #[test]
    fn version_of_pre_launch() {
        assert_eq!(version_of("2024-01-01T04:00:00"), "pre");
    }

    #[test]
    fn case_expression_starts_with_case() {
        let sql = sql_case_expression();
        assert!(sql.starts_with("CASE"));
        assert!(sql.ends_with("END"));
        assert!(sql.contains("'3.2'"));
        assert!(sql.contains("'1.0'"));
    }
}
