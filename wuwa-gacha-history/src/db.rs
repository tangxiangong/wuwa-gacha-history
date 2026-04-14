use crate::{CardPool, QualityLevel, ResponseRecord};
use jiff::civil::DateTime;
use toasty::Db;
use tokio::sync::{Mutex, OnceCell};

static DB: OnceCell<Mutex<Db>> = OnceCell::const_new();

pub async fn db(path: &str) -> tokio::sync::MutexGuard<'static, Db> {
    DB.get_or_init(|| async { Mutex::new(load(path).await) })
        .await
        .lock()
        .await
}

#[derive(Debug, Clone, toasty::Model)]
#[table = "gacha"]
pub struct GachaRecord {
    #[key]
    #[auto]
    pub id: u64,

    pub user_id: String,
    pub server_id: String,
    pub card_pool: CardPool,
    pub language_code: String,
    pub record_id: String,
    pub quality_level: QualityLevel,
    pub name: String,
    pub time: DateTime,
}

async fn load(path: &str) -> Db {
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&format!("sqlite:{}", path))
        .await
        .unwrap();

    db.push_schema().await.unwrap();

    db
}

pub async fn add_records(
    path: &str,
    user_id: &str,
    server_id: &str,
    language_code: &str,
    records: Vec<ResponseRecord>,
) {
    let mut db = db(path).await;

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

    toasty::batch(insertions).exec(&mut *db).await.unwrap();
}
