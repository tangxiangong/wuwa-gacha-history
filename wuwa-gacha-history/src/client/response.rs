use crate::CardPool;
use jiff::civil::DateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GachaHistoryResponse {
    pub code: u32,
    pub message: String,
    pub data: Vec<ResponseRecord>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseRecord {
    pub id: String,
    pub card_pool_type: CardPool,
    pub resource_id: u32,
    pub quality_level: QualityLevel,
    pub resource_name: String,
    pub name: String,
    pub count: u32,
    pub time: DateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde_repr::Deserialize_repr, toasty::Embed)]
#[repr(u8)]
pub enum QualityLevel {
    #[column(variant = 3)]
    ThreeStar = 3,
    #[column(variant = 4)]
    FourStar = 4,
    #[column(variant = 5)]
    FiveStar = 5,
}
