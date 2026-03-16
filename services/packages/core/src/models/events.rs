use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub ns: String,
    pub name: String,
    pub start_date: i64,
    pub end_date: i64,
    pub distance_min: f64,
    pub distance_max: f64,
    pub location: String,
    pub metadata: serde_json::Value,
}