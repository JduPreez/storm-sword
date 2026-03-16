use lambda_http::{Body, Request, Response};
use serde_json::json;
use lambda_client::PrivateLambdaClient;
use core::{ErrorResponse, utils};
use core::models::api::{ListEventsRequest, ListEventsResponse};

pub async fn list(_event: Request) -> Result<Response<Body>, lambda_http::Error> {
    let events_lambda_arn = match utils::get_env_var("EVENTS_LAMBDA_ARN") {
        Ok(arn) => arn,
        Err(_) => {
            let body = serde_json::to_string(
                &ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set")
            )?;

            let response = Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(Body::Text(body))?;

            return Ok(response);
        }
    };

    let client = PrivateLambdaClient::new(events_lambda_arn).await;
    let request = ListEventsRequest { start_date: None, end_date: None, limit: 10 };

    match client.invoke::<_, ListEventsResponse>(request).await {
        Ok(response) => {
            let events: Vec<_> = response.events.iter().map(|e| json!({
                "id": e.id,
                "ns": e.ns,
                "name": e.name,
                "startDate": e.start_date,
                "endDate": e.end_date,
                "distanceMin": e.distance_min,
                "distanceMax": e.distance_max,
                "location": e.location,
            })).collect();

            let body = json!({ "events": events, "nextToken": response.next_token }).to_string();

            let response = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::Text(body))?;

            Ok(response)
        }
        Err(e) => {
            let body = serde_json::to_string(&ErrorResponse::new("InternalError", e.to_string()))?;

            let response = Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(Body::Text(body))?;

            Ok(response)
        }
    }
}