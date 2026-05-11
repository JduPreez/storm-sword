use std::future::Future;
use std::error::Error;
use crate::models::api::{ApiResult, BoxApiHandler, BoxApiResultFuture};
use lambda_http::{Body, Request, Response};

pub fn handler_boxed<F, Fut>(f: F) -> BoxApiHandler
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ApiResult> + Send + 'static,
{
    Box::new(move |req: &Request| {
        let owned = req.clone();
        Box::pin(f(owned)) as BoxApiResultFuture
    })
}

pub fn not_found_boxed(_request: &Request) -> BoxApiResultFuture {
    Box::pin(async move {
        let response = Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(Body::Text(r#"{"error":"Not Found"}"#.to_string()))?;

        Ok(response)
    })
}

pub fn json_response<T: serde::Serialize>(status: u16, obj: &T)
  -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let json = serde_json::to_string(obj)
      .unwrap_or_else(|_| r#"{"error":"Serialization Error"}"#.to_string());

    let response = Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Body::Text(json))?;

    return Ok(response);
}