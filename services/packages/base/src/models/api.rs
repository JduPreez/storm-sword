use serde::{Deserialize, Serialize};
use lambda_http::{Body, Response};
use std::future::Future;
use std::pin::Pin;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SaveEventRequest {
    pub event: Event,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SaveEventResponse {
    pub event: Event,
}

pub type ApiResult = Result<Response<Body>, lambda_http::Error>;

pub type BoxApiResultFuture = Pin<Box<dyn Future<Output = ApiResult> + Send>>;

pub type BoxApiHandler = Box<dyn Fn(&lambda_http::Request) -> BoxApiResultFuture + Send + Sync>;
