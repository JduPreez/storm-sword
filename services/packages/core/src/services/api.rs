use std::future::Future;

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