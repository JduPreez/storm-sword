use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;
use tracing_subscriber;
use std::panic;
use aws_sdk_dynamodb::Client as DynamoClient;
use serde::Serialize;
use cuid2::cuid;

use base::models::events::{ Event, Address };
use base::models::api::{
  EventRequest,
  EventResponse,
  ListEventsRequest,
  ListEventsResponse,
  SaveEventRequest,
  SaveEventResponse,
  ResponseInfo,
};

fn normalize(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase()
}

/// DynamoDB storage representation of an `Event`.
///
/// `Event` is the public API wire type (camelCase JSON), so it can't double as
/// the persistence schema. This DTO carries the PascalCase attribute names the
/// `Events` table indexes on (see `sst.config.ts`) and includes
/// `AddressAdministrativeAreaIdx`, which `Event` marks `#[serde(skip_serializing)]`.
/// `serde_dynamo` serializes it directly to an item, so `Address`/`Metadata`
/// land as native Maps rather than opaque JSON strings.
#[derive(Serialize)]
struct EventItem {
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

async fn list_events(req: ListEventsRequest) -> Result<ListEventsResponse, Error> {
  info!("Handler invoked, limit: {}", req.limit);

  // TODO: Query DynamoDB here
  let events = vec![
      Event {
          id: Some("evt_001".to_string()),
          ns: Some("TrailRun~~ZA".to_string()),
          event_type: Some("Trail Run".to_string()),
          name: "Boston Marathon 2026".to_string(),
          start_date: Some(1735689600),
          end_date: Some(1735689600),
          distance_min: Some(42.195),
          distance_max: Some(42.195),
          description: None,
          address: Address {
            country_code: "US".to_string(),
            address_line1: Some("123 Main St".to_string()),
            address_line2: Some("Apt 4B".to_string()),
            city_town_locality: Some("Boston".to_string()),
            sublocality: Some("Back Bay".to_string()),
            administrative_area: "MA".to_string(),
            postal_code: Some("02116".to_string()),
            latitude: Some(42.3496),
            longitude: Some(-71.0762),
          },
          metadata: Some(serde_json::json!({})),
          address_administrative_area_idx: Some("TrailRun~~ZA~~Massachusetts".to_string()),
      },
      Event {
          id: Some("evt_002".to_string()),
          ns: Some("TrailRun~~ZA".to_string()),
          event_type: Some("Trail Run".to_string()),
          name: "NYC Half Marathon".to_string(),
          start_date: Some(1736294400),
          end_date: Some(1736294400),
          distance_min: Some(21.0975),
          distance_max: Some(21.0975),
          description: None,
          address: Address {
            country_code: "US".to_string(),
            address_line1: Some("456 Elm St".to_string()),
            address_line2: Some("Apt 12C".to_string()),
            city_town_locality: Some("New York".to_string()),
            sublocality: Some("Manhattan".to_string()),
            administrative_area: "NY".to_string(),
            postal_code: Some("10001".to_string()),
            latitude: Some(40.7128),
            longitude: Some(-74.0060),
          },
          metadata: Some(serde_json::json!({})),
          address_administrative_area_idx: Some("TrailRun~~ZA~~WesternCape".to_string()),
      },
      Event {
          id: Some("evt_003".to_string()),
          ns: Some("TrailRun~~UK".to_string()),
          event_type: Some("Trail Run".to_string()),
          name: "Portland 5K Fun Run".to_string(),
          start_date: Some(1736899200),
          end_date: Some(1736899200),
          distance_min: Some(5.0),
          distance_max: Some(5.0),
          description: None,
          address: Address {
            country_code: "UK".to_string(),
            address_line1: Some("789 Oak St".to_string()),
            address_line2: Some("Apt 5D".to_string()),
            city_town_locality: Some("Portland".to_string()),
            sublocality: Some("Downtown".to_string()),
            administrative_area: "OR".to_string(),
            postal_code: Some("97205".to_string()),
            latitude: Some(45.5152),
            longitude: Some(-122.6784),
          },
          metadata: Some(serde_json::json!({})),
          address_administrative_area_idx: Some("TrailRun~~UK~~London".to_string()),
      },
  ];

  let response = ListEventsResponse {
      events,
      next_token: String::new(),
  };

  info!("Returning {} events", response.events.len());
  Ok(response)
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
    event.ns = Some(
      normalize(
        &format!("{}~~{}",
          event_type,
          event.address.country_code
        )
      )
    );
    event.address_administrative_area_idx = Some(
      normalize(
        &format!("{}~~{}~~{}",
          event_type,
          event.address.country_code,
          event.address.administrative_area
        )
      )
    );

    let item = serde_dynamo::to_item(EventItem {
        id: event.id.clone().unwrap(),
        ns: event.ns.clone().unwrap_or_default(),
        name: event.name.clone(),
        event_type: event.event_type.clone(),
        start_date: event.start_date,
        end_date: event.end_date,
        distance_min: event.distance_min,
        distance_max: event.distance_max,
        address: event.address.clone(),
        address_administrative_area_idx: event.address_administrative_area_idx.clone().unwrap_or_default(),
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
            let response = list_events(req).await?;
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
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap())
        )
        .with_target(false)
        .without_time()
        .with_ansi(false)
        .init();

    info!("Starting events service Lambda");

    let config = aws_config::load_from_env().await;
    let client = DynamoClient::new(&config);
    let table_name = std::env::var("EVENTS_TABLE_NAME")
        .expect("EVENTS_TABLE_NAME env var must be set");

    run(service_fn(|event: LambdaEvent<EventRequest>| {
        let client = client.clone();
        let table_name = table_name.clone();
        async move { handler(event, &client, &table_name).await }
    })).await
}
