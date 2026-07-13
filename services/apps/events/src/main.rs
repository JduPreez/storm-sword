use aws_sdk_dynamodb::{types::AttributeValue, Client as DynamoClient};
use cuid2::cuid;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::panic;
use tracing::info;
use tracing_subscriber;

use base::models::api::{
  EventRequest, EventResponse, ListEventsRequest, ListEventsResponse, ResponseInfo,
  SaveEventRequest, SaveEventResponse,
};
use base::models::events::{Address, Event};

fn normalize(s: &str) -> String {
  s.chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>()
    .to_lowercase()
}

/// DynamoDB storage representation of an `Event`.
///
/// `Event` is the public API wire type (camelCase JSON), so it can't double as
/// the persistence schema. This DAO carries the PascalCase attribute names the
/// `Events` table indexes on (see `sst.config.ts`) and includes
/// `AddressAdministrativeAreaIdx`, which `Event` marks `#[serde(skip_serializing)]`.
/// `serde_dynamo` serializes it directly to an item, so `Address`/`Metadata`
/// land as native Maps rather than opaque JSON strings.
#[derive(Serialize, Deserialize)]
struct EventDao {
  #[serde(rename = "Id")]
  id: String,
  #[serde(rename = "Ns")]
  ns: String,
  #[serde(rename = "Name")]
  name: String,
  #[serde(rename = "EventType", skip_serializing_if = "Option::is_none")]
  event_type: Option<String>,
  #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
  start_date: Option<i64>,
  #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
  end_date: Option<i64>,
  #[serde(rename = "DistanceMin", skip_serializing_if = "Option::is_none")]
  distance_min: Option<f64>,
  #[serde(rename = "DistanceMax", skip_serializing_if = "Option::is_none")]
  distance_max: Option<f64>,
  #[serde(rename = "Address")]
  address: Address,
  #[serde(rename = "AddressAdministrativeAreaIdx")]
  address_administrative_area_idx: String,
  #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
  metadata: Option<serde_json::Value>,
}

impl From<EventDao> for Event {
  fn from(dao: EventDao) -> Self {
    Event {
      id: Some(dao.id),
      ns: Some(dao.ns),
      event_type: dao.event_type,
      name: dao.name,
      start_date: dao.start_date,
      end_date: dao.end_date,
      distance_min: dao.distance_min,
      distance_max: dao.distance_max,
      address: dao.address,
      description: dao.description,
      address_administrative_area_idx: Some(dao.address_administrative_area_idx),
      metadata: dao.metadata,
    }
  }
}

async fn list_events(
  req: ListEventsRequest,
  client: &DynamoClient,
  table_name: &str,
) -> Result<ListEventsResponse, Error> {
  // `Ns` is a composite partition key of event_type + country_code, built the
  // same way `save_event` writes it, so we can Query the base table's primary
  // index directly. Other filters (dates, distances) will layer on later.
  let ns = normalize(&format!("{}~~{}", req.event_type, req.country_code));
  info!("Querying events for ns={} (limit {})", ns, req.limit);

  let mut query = client
    .query()
    .table_name(table_name)
    .key_condition_expression("Ns = :ns")
    .expression_attribute_values(":ns", AttributeValue::S(ns));

  if req.limit > 0 {
    query = query.limit(req.limit);
  }

  let output = query.send().await?;

  let items = output.items.unwrap_or_default();
  let events: Vec<Event> = serde_dynamo::from_items::<_, EventDao>(items)?
    .into_iter()
    .map(Event::from)
    .collect();

  info!("Returning {} events", events.len());
  Ok(ListEventsResponse {
    events,
    // TODO: paginate via output.last_evaluated_key once clients send next_token.
    next_token: String::new(),
  })
}

async fn save_event(
  req: SaveEventRequest,
  client: &DynamoClient,
  table_name: &str,
) -> Result<SaveEventResponse, Error> {
  let mut event = req.event;

  if event.id.is_none() {
    event.id = Some(cuid());
  }

  let event_type = event.event_type.clone().unwrap_or_default();
  event.ns = Some(normalize(&format!(
    "{}~~{}",
    event_type, event.address.country_code
  )));
  event.address_administrative_area_idx = Some(normalize(&format!(
    "{}~~{}~~{}",
    event_type, event.address.country_code, event.address.administrative_area
  )));

  let item = serde_dynamo::to_item(EventDao {
    id: event.id.clone().unwrap(),
    ns: event.ns.clone().unwrap_or_default(),
    name: event.name.clone(),
    event_type: event.event_type.clone(),
    start_date: event.start_date,
    end_date: event.end_date,
    distance_min: event.distance_min,
    distance_max: event.distance_max,
    address: event.address.clone(),
    address_administrative_area_idx: event
      .address_administrative_area_idx
      .clone()
      .unwrap_or_default(),
    description: event.description.clone(),
    metadata: event.metadata.clone(),
  })?;

  client
    .put_item()
    .table_name(table_name)
    .set_item(Some(item))
    .send()
    .await?;

  info!("Saved event id={}", event.id.as_deref().unwrap_or("?"));

  Ok(SaveEventResponse {
    event: Some(event),
    response: ResponseInfo {
      status: 200,
      message: "Event saved successfully".to_string(),
    },
  })
}

async fn handler(
  event: LambdaEvent<EventRequest>,
  client: &DynamoClient,
  table_name: &str,
) -> Result<EventResponse, Error> {
  match event.payload {
    EventRequest::ListEvents(req) => {
      let response = list_events(req, client, table_name).await?;
      Ok(EventResponse::ListEvents(response))
    }
    EventRequest::SaveEvent(req) => {
      let response = save_event(req, client, table_name).await?;
      Ok(EventResponse::SaveEvent(response))
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  panic::set_hook(Box::new(|panic_info| {
    eprintln!("PANIC: {:?}", panic_info);
  }));

  tracing_subscriber::fmt()
    .with_env_filter(
      tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse().unwrap()),
    )
    .with_target(false)
    .without_time()
    .with_ansi(false)
    .init();

  info!("Starting events service Lambda");

  let config = aws_config::load_from_env().await;
  let client = DynamoClient::new(&config);
  let table_name =
    std::env::var("EVENTS_TABLE_NAME").expect("EVENTS_TABLE_NAME env var must be set");

  run(service_fn(|event: LambdaEvent<EventRequest>| {
    let client = client.clone();
    let table_name = table_name.clone();
    async move { handler(event, &client, &table_name).await }
  }))
  .await
}
