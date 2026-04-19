use crate::version::{sql_case_expression, version_of};
use crate::{CardPool, QualityLevel, ResponseRecord, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tokio::sync::OnceCell;

static DB: OnceCell<SqlitePool> = OnceCell::const_new();

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

async fn pool(path: &str) -> Result<&'static SqlitePool> {
    DB.get_or_try_init(|| async {
        let pool = init(path).await?;
        Ok::<_, crate::Error>(pool)
    })
    .await
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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
    /// WuWa version whose window contains `time` (e.g. "2.4"). "pre" if
    /// earlier than 1.0 launch.
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaFilter {
    pub card_pool: Option<CardPool>,
    pub quality_level: Option<QualityLevel>,
    pub name: Option<String>,
    pub time_from: Option<NaiveDateTime>,
    pub time_to: Option<NaiveDateTime>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

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
            time TEXT NOT NULL,
            seq INTEGER NOT NULL DEFAULT 0,
            version TEXT NOT NULL DEFAULT ''
        )"
    ))
    .execute(pool)
    .await?;

    let _ = sqlx::query(&format!(
        "ALTER TABLE {table} ADD COLUMN seq INTEGER NOT NULL DEFAULT 0"
    ))
    .execute(pool)
    .await;

    let _ = sqlx::query(&format!(
        "ALTER TABLE {table} ADD COLUMN version TEXT NOT NULL DEFAULT ''"
    ))
    .execute(pool)
    .await;

    // Backfill version for rows still carrying the empty default.
    let case = sql_case_expression();
    sqlx::query(&format!(
        "UPDATE {table} SET version = ({case}) WHERE version = '' OR version IS NULL"
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

fn record_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<GachaRecord> {
    let card_pool = match row.try_get::<i32, _>("card_pool")? {
        1 => CardPool::FeaturedResonatorConvene,
        2 => CardPool::FeaturedWeaponConvene,
        3 => CardPool::StandardResonatorConvene,
        4 => CardPool::StandardWeaponConvene,
        5 => CardPool::NoviceConvene,
        6 => CardPool::BeginnerChoiceConvene,
        7 => CardPool::GivebackCustomConvene,
        v => return Err(crate::Error::Other(format!("invalid card_pool: {v}"))),
    };

    let quality_level = match row.try_get::<i32, _>("quality_level")? {
        3 => QualityLevel::ThreeStar,
        4 => QualityLevel::FourStar,
        5 => QualityLevel::FiveStar,
        v => return Err(crate::Error::Other(format!("invalid quality_level: {v}"))),
    };

    let time_str = row.try_get::<String, _>("time")?;
    let time = NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S"))
        .map_err(|e| crate::Error::Other(format!("invalid time: {e}")))?;

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
        version: row.try_get("version")?,
    })
}

pub async fn add_records(
    path: &str,
    player_id: &str,
    server_id: &str,
    language_code: &str,
    card_pool: CardPool,
    records: Vec<ResponseRecord>,
) -> Result<()> {
    let pool = pool(path).await?;
    let table = ensure_user_table(pool, player_id).await?;
    let mut tx = pool.begin().await?;

    let card_pool_int = card_pool as i32;

    sqlx::query(&format!("DELETE FROM {table} WHERE card_pool = ?"))
        .bind(card_pool_int)
        .execute(&mut *tx)
        .await?;

    let sql = format!(
        "INSERT INTO {table}
            (server_id, card_pool, language_code, record_id, quality_level, name, time, seq, version)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    );

    // Preserve the response's exact ordering. The API returns records with the
    // game UI's top-first (newest time first, oldest-first within the same
    // second). seq = index in the response, so ORDER BY seq ASC reproduces
    // exactly what the player sees in-game.
    for (seq, record) in records.into_iter().enumerate() {
        let synthetic_id = format!("{card_pool_int}-{seq}");
        let time_iso = record.time.format("%Y-%m-%dT%H:%M:%S").to_string();
        let version = version_of(&time_iso);
        sqlx::query(&sql)
            .bind(server_id)
            .bind(card_pool_int)
            .bind(language_code)
            .bind(&synthetic_id)
            .bind(record.quality_level as i32)
            .bind(&record.name)
            .bind(&time_iso)
            .bind(seq as i64)
            .bind(version)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn query_records(
    path: &str,
    player_id: &str,
    filter: &GachaFilter,
) -> Result<Vec<GachaRecord>> {
    let pool = pool(path).await?;
    let table = ensure_user_table(pool, player_id).await?;

    let mut qb: sqlx::QueryBuilder<'_, sqlx::Sqlite> = sqlx::QueryBuilder::new(format!(
        "SELECT id, server_id, card_pool, language_code, record_id, quality_level, name, time, version FROM {table} WHERE 1=1"
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

    qb.push(" ORDER BY time DESC, seq ASC");

    if let Some(limit) = filter.limit {
        qb.push(" LIMIT ").push_bind(limit as i64);
        if let Some(offset) = filter.offset {
            qb.push(" OFFSET ").push_bind(offset as i64);
        }
    }

    let rows = qb.build().fetch_all(pool).await?;
    rows.iter().map(record_from_row).collect()
}

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
        if let Some(id) = name.strip_prefix("gacha_")
            && validate_player_id(id).is_ok()
        {
            ids.push(id.to_string());
        }
    }

    Ok(ids)
}

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
        // Regression-safety: reject control chars and unicode digits
        assert!(validate_player_id("12345678\n").is_err());
        assert!(validate_player_id("12345678\0").is_err());
        assert!(validate_player_id("\u{FF10}2345678").is_err()); // fullwidth 0
        assert!(validate_player_id("١٢٣٤٥٦٧٨٩").is_err()); // Arabic-Indic digits
    }

    #[test]
    fn user_table_returns_prefixed_name() {
        assert_eq!(user_table("123456789").unwrap(), "gacha_123456789");
    }

    #[test]
    fn user_table_rejects_invalid_id() {
        assert!(user_table("bad").is_err());
    }

    use std::sync::OnceLock;

    static TEST_DIR: OnceLock<tempfile::TempDir> = OnceLock::new();

    fn test_db_path() -> String {
        let dir = TEST_DIR.get_or_init(|| tempfile::tempdir().unwrap());
        dir.path().join("gacha.db").to_string_lossy().into_owned()
    }

    use crate::{CardPool, QualityLevel, ResponseRecord};
    use chrono::NaiveDate;

    fn sample_record(name: &str) -> ResponseRecord {
        ResponseRecord {
            card_pool_type: "角色精准调谐".to_string(),
            quality_level: QualityLevel::FiveStar,
            name: name.to_string(),
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
        let path = test_db_path();
        let player_id = "123456789";

        add_records(
            &path,
            player_id,
            "76402e5b",
            "zh-Hans",
            CardPool::FeaturedResonatorConvene,
            vec![sample_record("安可")],
        )
        .await
        .unwrap();

        let records = query_records(&path, player_id, &GachaFilter::default())
            .await
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "安可");
    }

    #[tokio::test]
    async fn add_records_replaces_previous_pool_data() {
        let path = test_db_path();
        let player_id = "555555555";
        let pool = CardPool::FeaturedResonatorConvene;

        add_records(
            &path,
            player_id,
            "s",
            "zh-Hans",
            pool,
            vec![sample_record("a"), sample_record("b")],
        )
        .await
        .unwrap();
        add_records(
            &path,
            player_id,
            "s",
            "zh-Hans",
            pool,
            vec![sample_record("c")],
        )
        .await
        .unwrap();

        let rows = query_records(&path, player_id, &GachaFilter::default())
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "c");
    }

    #[tokio::test]
    async fn query_preserves_response_order_within_same_timestamp() {
        let path = test_db_path();
        let player_id = "777777777";
        let pool = CardPool::FeaturedResonatorConvene;
        let t = NaiveDate::from_ymd_opt(2026, 3, 19)
            .unwrap()
            .and_hms_opt(12, 38, 37)
            .unwrap();

        let mk = |name: &str| ResponseRecord {
            card_pool_type: "x".into(),
            quality_level: QualityLevel::ThreeStar,
            name: name.into(),
            time: t,
            resource_id: 0,
            resource_type: String::new(),
            count: 1,
        };

        // Response order (newest first across times, in-game order within same time):
        // game shows these 10 items top-to-bottom exactly as returned.
        let response = vec![
            mk("item0"),
            mk("item1"),
            mk("item2"),
            mk("item3"),
            mk("item4"),
            mk("item5"),
            mk("item6"),
            mk("item7"),
            mk("item8"),
            mk("item9_5star"),
        ];

        add_records(&path, player_id, "s", "zh-Hans", pool, response)
            .await
            .unwrap();

        let rows = query_records(&path, player_id, &GachaFilter::default())
            .await
            .unwrap();
        let names: Vec<String> = rows.iter().map(|r| r.name.clone()).collect();
        assert_eq!(
            names,
            vec![
                "item0",
                "item1",
                "item2",
                "item3",
                "item4",
                "item5",
                "item6",
                "item7",
                "item8",
                "item9_5star",
            ],
            "within-same-timestamp order must match response order",
        );
    }

    #[tokio::test]
    async fn add_records_scopes_wipe_to_single_pool() {
        let path = test_db_path();
        let player_id = "666666666";

        add_records(
            &path,
            player_id,
            "s",
            "zh-Hans",
            CardPool::FeaturedResonatorConvene,
            vec![sample_record("r")],
        )
        .await
        .unwrap();
        add_records(
            &path,
            player_id,
            "s",
            "zh-Hans",
            CardPool::StandardWeaponConvene,
            vec![sample_record("w")],
        )
        .await
        .unwrap();

        let rows = query_records(&path, player_id, &GachaFilter::default())
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);
    }

    #[tokio::test]
    async fn add_records_rejects_invalid_player_id() {
        let path = test_db_path();
        let err = add_records(
            &path,
            "bad",
            "s",
            "zh-Hans",
            CardPool::FeaturedResonatorConvene,
            vec![],
        )
        .await
        .unwrap_err();
        assert!(matches!(err, crate::Error::Other(_)));
    }

    #[tokio::test]
    async fn query_records_isolates_users() {
        let path = test_db_path();
        let pool = CardPool::FeaturedResonatorConvene;

        add_records(
            &path,
            "111111111",
            "s",
            "zh-Hans",
            pool,
            vec![sample_record("a")],
        )
        .await
        .unwrap();
        add_records(
            &path,
            "222222222",
            "s",
            "zh-Hans",
            pool,
            vec![sample_record("b")],
        )
        .await
        .unwrap();

        let r1 = query_records(&path, "111111111", &GachaFilter::default())
            .await
            .unwrap();
        let r2 = query_records(&path, "222222222", &GachaFilter::default())
            .await
            .unwrap();
        assert_eq!(r1.len(), 1);
        assert_eq!(r1[0].name, "a");
        assert_eq!(r2.len(), 1);
        assert_eq!(r2[0].name, "b");
    }

    #[tokio::test]
    async fn list_users_returns_player_ids() {
        let path = test_db_path();
        let pool = CardPool::FeaturedResonatorConvene;

        add_records(&path, "333333333", "s", "zh-Hans", pool, vec![])
            .await
            .unwrap();
        add_records(&path, "444444444", "s", "zh-Hans", pool, vec![])
            .await
            .unwrap();

        let users = list_users(&path).await.unwrap();
        assert!(users.contains(&"333333333".to_string()));
        assert!(users.contains(&"444444444".to_string()));
        for id in &users {
            assert!(validate_player_id(id).is_ok(), "leaked invalid id: {id}");
        }
    }

    #[tokio::test]
    async fn list_users_filters_invalid_table_names() {
        let path = test_db_path();
        let pool = pool(&path).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS gacha_foobar (id INTEGER)")
            .execute(pool)
            .await
            .unwrap();

        let users = list_users(&path).await.unwrap();
        assert!(!users.contains(&"foobar".to_string()));
    }
}
