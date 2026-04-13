use lambda_http::{Body, Request, Response };
use base::models::api::ApiResult;

pub async fn handler(_event: Request) -> ApiResult {
  let response = Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body(Body::Text(r#"{"status":"ok"}"#.to_string()))?;

  Ok(response)
}
