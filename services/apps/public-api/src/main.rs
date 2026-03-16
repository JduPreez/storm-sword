mod controllers;

use lambda_http::{run, service_fn, Body, Request, Response};

async fn handler(event: Request) -> Result<Response<Body>, lambda_http::Error> {
    let method = event.method().as_str();
    // lambda_http strips the stage prefix; raw_http_path gives the plain path
    let path = event.uri().path();

    match (method, path) {
        ("GET", "/health")        => controllers::health::handler(event).await,
        ("GET", "/events")        => controllers::events::list(event).await,
        // add more routes here
        _ => {
            let body = serde_json::json!({ "error": "NotFound", "message": "Route not found" });
            Ok(Response::builder()
                .status(404)
                .header("content-type", "application/json")
                .body(Body::Text(body.to_string()))
                .map_err(Into::<lambda_http::Error>::into)?)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .without_time()
        .with_ansi(false)
        .init();

    run(service_fn(handler)).await
}