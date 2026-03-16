use serde::{Deserialize, Serialize};

use super::events::Event;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListEventsRequest {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListEventsResponse {
    pub events: Vec<Event>,
    pub next_token: String,
}
