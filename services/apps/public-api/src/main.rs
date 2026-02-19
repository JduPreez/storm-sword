use lambda_http::{run, service_fn, Body, Request, Response, Error};
use serde_json::json;
use tracing::{info, error, debug};
use tracing_subscriber;

use protos::{ListEventsRequest, ListEventsResponse};
use lambda_client::PrivateLambdaClient;
use core::{ErrorResponse, utils};

async fn handler(_event: Request) -> Result<Response<Body>, lambda_http::Error> {
  eprintln!(">>>>>> Public API handler invoked");
  
  let body = json!({
    "message": "Hello from the public API Lambda!"
  }).to_string();

  // Get events service Lambda ARN from environment
  let events_lambda_arn = match utils::get_env_var("EVENTS_LAMBDA_ARN") {
    Ok(arn) => {
        eprintln!("Using EVENTS_LAMBDA_ARN: {}", arn);
        arn
    },
    Err(e) => {
        eprintln!("Failed to get EVENTS_LAMBDA_ARN: {}", e);
        let error_response = ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set".to_string());
        let body = serde_json::to_string(&error_response)?;
        
        return Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(Body::Text(body))
            .map_err(Into::into);
    }
  };

  eprintln!(">>>>>> Creating Lambda client");
  let client = PrivateLambdaClient::new(events_lambda_arn).await;

  let request = ListEventsRequest {
        start_date: None,
        end_date: None,
        limit: 10,
    };

  // Response::builder()
  //   .status(200)
  //   .header("content-type", "application/json")
  //   .body(Body::Text(body))
  //   .map_err(Into::into)

  eprintln!(">>>>>> Invoking events Lambda");
  
  match client.invoke::<_, ListEventsResponse>(request).await {
        Ok(response) => {
            eprintln!("Successfully retrieved {} events", response.events.len());

            // Convert protobuf response to JSON
            let events: Vec<_> = response.events.iter().map(|e| {
                json!({
                    "id": e.id,
                    "ns": e.ns,
                    "name": e.name,
                    "startDate": e.start_date,
                    "endDate": e.end_date,
                    "distanceMin": e.distance_min,
                    "distanceMax": e.distance_max,
                    "location": e.location,
                })
            }).collect();

            let body = json!({
                "events": events,
                "nextToken": response.next_token
            }).to_string();

            Response::builder()
              .status(200)
              .header("content-type", "application/json")
              .body(Body::Text(body))
              .map_err(Into::into)
        }
        Err(e) => {
            eprintln!(">>>>>> Failed to invoke events service: {}", e);

            let error_response = ErrorResponse::new("InternalError", e.to_string());
            let body = serde_json::to_string(&error_response)?;

            Response::builder()
              .status(500)
              .header("content-type", "application/json")
              .body(Body::Text(body))
              .map_err(Into::into)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
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

  eprintln!("Starting public API Lambda");

  run(service_fn(handler)).await
}