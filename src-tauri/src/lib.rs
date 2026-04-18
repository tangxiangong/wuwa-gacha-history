mod log_reader;
mod sniffer;

use std::path::PathBuf;

use serde::Deserialize;
use tauri::Manager;
use wuwa_gacha_history::{
    add_records, export_to_file, list_users as list_users_impl, query_records, CardPool,
    GachaFilter, GachaHistoryClient, GachaRecord, RequestParams,
};

use sniffer::SnifferState;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchParams {
    player_id: String,
    server_id: String,
    language_code: String,
    record_id: String,
}

fn db_path(app: &tauri::AppHandle) -> Result<String, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("gacha.db").to_string_lossy().into_owned())
}

#[tauri::command]
async fn fetch_gacha_records(
    app: tauri::AppHandle,
    params: FetchParams,
    pool_types: Vec<CardPool>,
) -> Result<u64, String> {
    let db_path = db_path(&app)?;

    let request_params = RequestParams {
        player_id: params.player_id.clone(),
        server_id: params.server_id.clone(),
        card_pool_id: String::new(),
        card_pool_type: CardPool::FeaturedResonatorConvene,
        language_code: params.language_code.clone(),
        record_id: params.record_id,
    };

    let mut client = GachaHistoryClient::new(request_params).map_err(|e| e.to_string())?;
    let mut total = 0u64;

    for pool_type in pool_types {
        let records = client
            .fetch_all(pool_type)
            .await
            .map_err(|e| e.to_string())?;
        total += records.len() as u64;
        add_records(
            &db_path,
            &params.player_id,
            &params.server_id,
            &params.language_code,
            pool_type,
            records,
        )
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(total)
}

#[tauri::command]
async fn query_gacha_records(
    app: tauri::AppHandle,
    player_id: String,
    filter: GachaFilter,
) -> Result<Vec<GachaRecord>, String> {
    let db_path = db_path(&app)?;
    let records = query_records(&db_path, &player_id, &filter)
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}

#[tauri::command]
async fn export_gacha_records(
    app: tauri::AppHandle,
    player_id: String,
    filter: GachaFilter,
    path: String,
) -> Result<(), String> {
    let db_path = db_path(&app)?;
    let records = query_records(&db_path, &player_id, &filter)
        .await
        .map_err(|e| e.to_string())?;
    export_to_file(&records, &path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn list_users(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let db_path = db_path(&app)?;
    list_users_impl(&db_path).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_sniffer(
    app: tauri::AppHandle,
    state: tauri::State<'_, SnifferState>,
) -> Result<u16, String> {
    let ca_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("mitm");
    state.start(app.clone(), ca_dir).await
}

#[tauri::command]
async fn stop_sniffer(
    app: tauri::AppHandle,
    state: tauri::State<'_, SnifferState>,
) -> Result<(), String> {
    state.stop(app).await
}

#[tauri::command]
async fn read_params_from_log(
    path: Option<String>,
    game_dir: Option<String>,
) -> Result<log_reader::LogParams, String> {
    log_reader::read_params(path.map(PathBuf::from), game_dir.map(PathBuf::from)).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(SnifferState::default())
        .invoke_handler(tauri::generate_handler![
            fetch_gacha_records,
            query_gacha_records,
            export_gacha_records,
            list_users,
            start_sniffer,
            stop_sniffer,
            read_params_from_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
