use crate::{CardPool, QualityLevel, ResponseRecord, Result};
use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};
use toasty::Db;
use tokio::sync::{Mutex, OnceCell};

static DB: OnceCell<Mutex<Db>> = OnceCell::const_new();

pub async fn db(path: &str) -> Result<tokio::sync::MutexGuard<'static, Db>> {
    let db = DB
        .get_or_try_init(|| async {
            let db = load(path).await?;
            Ok::<_, crate::Error>(Mutex::new(db))
        })
        .await?;
    Ok(db.lock().await)
}

#[derive(Debug, Clone, Serialize, toasty::Model)]
#[serde(rename_all = "camelCase")]
#[table = "gacha"]
pub struct GachaRecord {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    pub user_id: String,
    pub server_id: String,
    pub card_pool: CardPool,
    pub language_code: String,
    pub record_id: String,
    pub quality_level: QualityLevel,
    pub name: String,
    pub time: DateTime,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GachaFilter {
    pub card_pool: Option<CardPool>,
    pub quality_level: Option<QualityLevel>,
    pub name: Option<String>,
    pub time_from: Option<DateTime>,
    pub time_to: Option<DateTime>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

async fn load(path: &str) -> Result<Db> {
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&format!("sqlite:{}", path))
        .await?;

    db.push_schema().await?;

    Ok(db)
}

pub async fn add_records(
    path: &str,
    user_id: &str,
    server_id: &str,
    language_code: &str,
    records: Vec<ResponseRecord>,
) -> Result<()> {
    let mut db = db(path).await?;

    let insertions: Vec<_> = records
        .into_iter()
        .map(|record| {
            GachaRecord::create()
                .user_id(user_id)
                .server_id(server_id)
                .language_code(language_code)
                .card_pool(record.card_pool_type)
                .record_id(record.id)
                .quality_level(record.quality_level)
                .name(record.name)
                .time(record.time)
        })
        .collect();

    toasty::batch(insertions).exec(&mut *db).await?;

    Ok(())
}

pub async fn query_records(
    path: &str,
    user_id: &str,
    filter: &GachaFilter,
) -> Result<Vec<GachaRecord>> {
    let mut db = db(path).await?;

    let mut query = GachaRecord::filter(GachaRecord::fields().user_id().eq(user_id));

    if let Some(card_pool) = filter.card_pool {
        query = query.filter(GachaRecord::fields().card_pool().eq(card_pool));
    }
    if let Some(quality_level) = filter.quality_level {
        query = query.filter(GachaRecord::fields().quality_level().eq(quality_level));
    }
    if let Some(ref name) = filter.name {
        query = query.filter(GachaRecord::fields().name().eq(name));
    }
    if let Some(time_from) = filter.time_from {
        query = query.filter(GachaRecord::fields().time().ge(time_from));
    }
    if let Some(time_to) = filter.time_to {
        query = query.filter(GachaRecord::fields().time().le(time_to));
    }

    let mut query = query.order_by(GachaRecord::fields().time().desc());

    if let Some(limit) = filter.limit {
        query = query.limit(limit);

        if let Some(offset) = filter.offset {
            query = query.offset(offset);
        }
    }

    let records = query.exec(&mut *db).await?;
    Ok(records)
}
