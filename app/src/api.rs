//! Thin wrappers around the core library, injecting the platform-resolved
//! db path so UI components don't have to thread it manually.

use std::path::{Path, PathBuf};

use wuwa_gacha_history::{
    CardPool, GachaFilter, GachaHistoryClient, GachaRecord, LogParams, RequestParams,
    ResponseRecord, Result, add_records, export_to_file, list_users as core_list_users,
    query_records, read_params as core_read_params,
};

use crate::platform::db_path;

/// Fetch all records across the given pool types and persist them.
/// Returns the total number of records returned by the API across all pools
/// (new or existing — DB dedupes on `record_id`).
pub async fn fetch_all_pools(
    player_id: String,
    server_id: String,
    language_code: String,
    record_id: String,
    pools: Vec<CardPool>,
) -> Result<u64> {
    let db = db_path();
    let db_str = db.to_string_lossy().to_string();

    let params = RequestParams {
        player_id: player_id.clone(),
        server_id: server_id.clone(),
        card_pool_id: String::new(),
        card_pool_type: CardPool::FeaturedResonatorConvene, // overwritten per pool
        language_code: language_code.clone(),
        record_id,
    };

    let mut client = GachaHistoryClient::new(params)?;
    let mut total = 0u64;

    for pool_type in pools {
        let records: Vec<ResponseRecord> = client.fetch_all(pool_type).await?;
        total += records.len() as u64;
        add_records(
            &db_str,
            &player_id,
            &server_id,
            &language_code,
            pool_type,
            records,
        )
        .await?;
    }
    Ok(total)
}

pub async fn query(player_id: &str, filter: &GachaFilter) -> Result<Vec<GachaRecord>> {
    let db = db_path();
    query_records(&db.to_string_lossy(), player_id, filter).await
}

pub async fn export(player_id: &str, filter: &GachaFilter, out: &Path) -> Result<()> {
    let records = query(player_id, filter).await?;
    export_to_file(&records, &out.to_string_lossy())?;
    Ok(())
}

pub async fn list_users() -> Result<Vec<String>> {
    let db = db_path();
    core_list_users(&db.to_string_lossy()).await
}

/// Read convene URL params from the game log file.
/// `explicit_path` is an explicit path to the log file; `game_dir` is the
/// game installation directory to search under. Returns `Err(String)` on
/// failure (the core function is stringly-typed).
pub async fn read_params(
    explicit_path: Option<PathBuf>,
    game_dir: Option<PathBuf>,
) -> std::result::Result<LogParams, String> {
    core_read_params(explicit_path, game_dir).await
}
