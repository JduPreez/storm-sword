use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use prost::Message;
use tracing::{info, error};
use tracing_subscriber;
use std::panic;

use protos::{ListEventsRequest, ListEventsResponse, Event};

/// Events service - handles all event-related operations
async fn handler(event: LambdaEvent<Vec<u8>>) -> Result<Vec<u8>, Error> {
    info!("Handler invoked, payload size: {} bytes", event.payload.len());
    
    // Decode the protobuf request
    let request = ListEventsRequest::decode(event.payload.as_slice())
        .map_err(|e| {
            error!("Failed to decode request: {}", e);
            format!("Failed to decode request: {}", e)
        })?;

    info!("Request decoded successfully, limit: {}", request.limit);

    // TODO: Query DynamoDB here
    // For now, return mock data
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
            metadata: Default::default(),
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
            metadata: Default::default(),
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
            metadata: Default::default(),
        },
    ];

    let response = ListEventsResponse {
        events,
        next_token: String::new(),
    };

    info!("Returning {} events", response.events.len());

    // Encode response to protobuf bytes
    let mut buf = Vec::new();
    response.encode(&mut buf)
        .map_err(|e| {
            error!("Failed to encode response: {}", e);
            format!("Failed to encode response: {}", e)
        })?;

    info!("Response encoded successfully, size: {} bytes", buf.len());
    Ok(buf)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set panic handler to log panics
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {:?}", panic_info);
    }));

    // Initialize tracing - use env-filter to respect RUST_LOG
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