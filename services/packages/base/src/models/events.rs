use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: Option<String>,
    pub ns: Option<String>,
    pub name: String,
    #[serde(rename = "startDate")]
    pub start_date: Option<i64>,
    #[serde(rename = "endDate")]
    pub end_date: Option<i64>,
    #[serde(rename = "distanceMin")]
    pub distance_min: Option<f64>,
    #[serde(rename = "distanceMax")]
    pub distance_max: Option<f64>,
    pub location: Option<String>,
    pub metadata: Option<serde_json::Value>,
}