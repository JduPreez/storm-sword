use lambda_http::{Body, Request, Response};

pub async fn handler(_event: Request) -> Result<Response<Body>, lambda_http::Error> {
  let response = Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body(Body::Text(r#"{"status":"ok"}"#.to_string()))?;

  Ok(response)
}
