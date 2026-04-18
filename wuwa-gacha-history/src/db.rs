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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaRecord {
    pub id: u64,
    pub user_id: String,
    pub server_id: String,
    pub card_pool: CardPool,
    pub language_code: String,
    pub record_id: String,
    pub quality_level: QualityLevel,
    pub name: String,
    pub time: NaiveDateTime,
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

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS gacha (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            server_id TEXT NOT NULL,
            card_pool INTEGER NOT NULL,
            language_code TEXT NOT NULL,
            record_id TEXT NOT NULL UNIQUE,
            quality_level INTEGER NOT NULL,
            name TEXT NOT NULL,
            time TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_gacha_user_id ON gacha(user_id)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_gacha_record_id ON gacha(record_id)")
        .execute(&pool)
        .await?;

    Ok(pool)
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
        user_id: row.try_get("user_id")?,
        server_id: row.try_get("server_id")?,
        card_pool,
        language_code: row.try_get("language_code")?,
        record_id: row.try_get("record_id")?,
        quality_level,
        name: row.try_get("name")?,
        time,
    })
}

pub async fn add_records(
    path: &str,
    user_id: &str,
    server_id: &str,
    language_code: &str,
    records: Vec<ResponseRecord>,
) -> Result<()> {
    let pool = pool(path).await?;
    let mut tx = pool.begin().await?;

    for record in records {
        sqlx::query(
            "INSERT OR IGNORE INTO gacha
                (user_id, server_id, card_pool, language_code, record_id, quality_level, name, time)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(user_id)
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

pub async fn query_records(
    path: &str,
    user_id: &str,
    filter: &GachaFilter,
) -> Result<Vec<GachaRecord>> {
    let pool = pool(path).await?;

    let mut qb: sqlx::QueryBuilder<'_, sqlx::Sqlite> = sqlx::QueryBuilder::new(
        "SELECT id, user_id, server_id, card_pool, language_code, record_id, quality_level, name, time FROM gacha WHERE user_id = ",
    );
    qb.push_bind(user_id.to_owned());

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
}
