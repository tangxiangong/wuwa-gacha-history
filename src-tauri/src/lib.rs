use serde::Deserialize;
use tauri::Manager;
use wuwa_gacha_history::{
    add_records, export_to_file, query_records, CardPool, GachaFilter, GachaHistoryClient,
    GachaRecord, RequestParams,
};

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
        size: 20,
        last_id: None,
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
    user_id: String,
    filter: GachaFilter,
) -> Result<Vec<GachaRecord>, String> {
    let db_path = db_path(&app)?;
    let records = query_records(&db_path, &user_id, &filter)
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}

#[tauri::command]
async fn export_gacha_records(
    app: tauri::AppHandle,
    user_id: String,
    filter: GachaFilter,
    path: String,
) -> Result<(), String> {
    let db_path = db_path(&app)?;
    let records = query_records(&db_path, &user_id, &filter)
        .await
        .map_err(|e| e.to_string())?;
    export_to_file(&records, &path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            fetch_gacha_records,
            query_gacha_records,
            export_gacha_records,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
