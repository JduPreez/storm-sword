use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;
use tracing_subscriber;
use std::panic;

use base::models::events::Event;
use base::models::api::{
  EventRequest,
  EventResponse,
  ListEventsRequest,
  ListEventsResponse,
  SaveEventRequest,
  SaveEventResponse,
  ResponseInfo,
};

async fn list_events(req: ListEventsRequest) -> Result<ListEventsResponse, Error> {
  info!("Handler invoked, limit: {}", req.limit);

  // TODO: Query DynamoDB here
  let events = vec![
      Event {
          id: "evt_001".to_string(),
          ns: "TrailRunning/SouthAfrica/WesternCape".to_string(),
          name: "Boston Marathon 2026".to_string(),
          start_date: 1735689600,
          end_date: 1735689600,
          distance_min: 42.195,
          distance_max: 42.195,
          location: "Boston, MA".to_string(),
          metadata: serde_json::json!({}),
      },
      Event {
          id: "evt_002".to_string(),
          ns: "TrailRunning/SouthAfrica/Gauteng".to_string(),
          name: "NYC Half Marathon".to_string(),
          start_date: 1736294400,
          end_date: 1736294400,
          distance_min: 21.0975,
          distance_max: 21.0975,
          location: "New York, NY".to_string(),
          metadata: serde_json::json!({}),
      },
      Event {
          id: "evt_003".to_string(),
          ns: "TrailRunning/UnitedKingdom/London".to_string(),
          name: "Portland 5K Fun Run".to_string(),
          start_date: 1736899200,
          end_date: 1736899200,
          distance_min: 5.0,
          distance_max: 5.0,
          location: "Portland, OR".to_string(),
          metadata: serde_json::json!({}),
      },
  ];

  let response = ListEventsResponse {
      events,
      next_token: String::new(),
  };

  info!("Returning {} events", response.events.len());
  Ok(response)
}

async fn save_event(req: SaveEventRequest) -> Result<SaveEventResponse, Error> {
  Ok(SaveEventResponse { event: None, response: ResponseInfo { status: 200, message: "Event saved successfully".to_string() } })
}

async fn handler(event: LambdaEvent<EventRequest>) -> Result<EventResponse, Error> {
    match event.payload {
        EventRequest::ListEvents(req) => {
            let response = list_events(req).await?;
            Ok(EventResponse::ListEvents(response))
        }
        EventRequest::SaveEvent(req) => {
            let response = save_event(req).await?;
            Ok(EventResponse::SaveEvent(response))
        }
    }
}

/// Events service - handles all event-related operations
// async fn handler(event: LambdaEvent<ListEventsRequest>) -> Result<ListEventsResponse, Error> {
//     let request = event.payload;
//     info!("Handler invoked, limit: {}", request.limit);

//     // TODO: Query DynamoDB here
//     let events = vec![
//         Event {
//             id: "evt_001".to_string(),
//             ns: "TrailRunning/SouthAfrica/WesternCape".to_string(),
//             name: "Boston Marathon 2026".to_string(),
//             start_date: 1735689600,
//             end_date: 1735689600,
//             distance_min: 42.195,
//             distance_max: 42.195,
//             location: "Boston, MA".to_string(),
//             metadata: serde_json::json!({}),
//         },
//         Event {
//             id: "evt_002".to_string(),
//             ns: "TrailRunning/SouthAfrica/Gauteng".to_string(),
//             name: "NYC Half Marathon".to_string(),
//             start_date: 1736294400,
//             end_date: 1736294400,
//             distance_min: 21.0975,
//             distance_max: 21.0975,
//             location: "New York, NY".to_string(),
//             metadata: serde_json::json!({}),
//         },
//         Event {
//             id: "evt_003".to_string(),
//             ns: "TrailRunning/UnitedKingdom/London".to_string(),
//             name: "Portland 5K Fun Run".to_string(),
//             start_date: 1736899200,
//             end_date: 1736899200,
//             distance_min: 5.0,
//             distance_max: 5.0,
//             location: "Portland, OR".to_string(),
//             metadata: serde_json::json!({}),
//         },
//     ];

//     let response = ListEventsResponse {
//         events,
//         next_token: String::new(),
//     };

//     info!("Returning {} events", response.events.len());
//     Ok(response)
// }

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

    run(service_fn(handler)).await
}
