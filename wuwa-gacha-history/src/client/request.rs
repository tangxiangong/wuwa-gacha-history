use crate::CardPool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestParams {
    pub player_id: String,
    pub server_id: String,
    pub card_pool_id: String,
    pub card_pool_type: CardPool,
    pub language_code: String,
    pub record_id: String,
    pub size: u8,
    pub last_id: Option<String>,
}
