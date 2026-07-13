use lambda_http::{Body, Response};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

use super::events::Event;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseInfo {
  pub status: u16,
  pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListEventsRequest {
  pub event_type: String,
  pub country_code: String,
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
  pub event: Option<Event>,
  pub response: ResponseInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "ssMethod")]
pub enum EventRequest {
  ListEvents(ListEventsRequest),
  SaveEvent(SaveEventRequest),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "ssMethod")]
pub enum EventResponse {
  ListEvents(ListEventsResponse),
  SaveEvent(SaveEventResponse),
}

pub type ApiResult = Result<Response<Body>, lambda_http::Error>;

pub type BoxApiResultFuture = Pin<Box<dyn Future<Output = ApiResult> + Send>>;

pub type BoxApiHandler = Box<dyn Fn(&lambda_http::Request) -> BoxApiResultFuture + Send + Sync>;

/// A route handler that receives two typed path params (e.g.
/// `GET /events/{eventType}/{countryCode}`). `http_router` passes captured
/// path segments to the handler positionally as `String`s.
pub type BoxApiHandler2 =
  Box<dyn Fn(&lambda_http::Request, String, String) -> BoxApiResultFuture + Send + Sync>;
