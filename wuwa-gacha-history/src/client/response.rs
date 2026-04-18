use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GachaHistoryResponse {
    pub code: i32,
    pub message: String,
    #[serde(default)]
    pub data: Vec<ResponseRecord>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseRecord {
    pub resource_id: u32,
    pub quality_level: QualityLevel,
    pub resource_type: String,
    pub name: String,
    pub count: u32,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub time: NaiveDateTime,
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum QualityLevel {
    ThreeStar = 3,
    FourStar = 4,
    FiveStar = 5,
}

impl std::fmt::Display for QualityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}★", *self as u8)
    }
}
