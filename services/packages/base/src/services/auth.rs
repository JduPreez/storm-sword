use crate::models::api::ApiResult;
use crate::services::api::json_response;
use crate::ErrorResponse;
use lambda_http::Request;

// Header carrying the shared-secret token checked by `is_authorized`.
pub const API_TOKEN_HEADER: &str = "x-api-token";

// Checks the request's `API_TOKEN_HEADER` against `expected_token`, using a
// constant-time comparison so a mismatch can't be timed to leak the secret.
pub fn is_authorized(req: &Request, expected_token: &str) -> bool {
  req
    .headers()
    .get(API_TOKEN_HEADER)
    .and_then(|value| value.to_str().ok())
    .is_some_and(|token| constant_time_eq(token, expected_token))
}

// The standard response to return when `is_authorized` fails.
pub fn unauthorized_response() -> ApiResult {
  json_response(
    401,
    &ErrorResponse::new("Unauthorized", "Missing or invalid API token"),
  )
}

// Time-attack resistant string comparison, guarding against an attacker guessing the
// token one byte at a time by measuring response latency.
fn constant_time_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}
