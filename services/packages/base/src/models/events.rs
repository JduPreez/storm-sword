use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub ns: String,
    pub name: String,
    #[serde(rename = "startDate")]
    pub start_date: i64,
    #[serde(rename = "endDate")]
    pub end_date: i64,
    #[serde(rename = "distanceMin")]
    pub distance_min: f64,
    #[serde(rename = "distanceMax")]
    pub distance_max: f64,
    pub location: String,
    pub metadata: serde_json::Value,
}