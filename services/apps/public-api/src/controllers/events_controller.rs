use lambda_http::{Body, Request, Response};
use serde_json::json;
use lambda_client::PrivateLambdaClient;
use base::ErrorResponse;
use base::models::api::{
  ListEventsRequest,
  ListEventsResponse,
  SaveEventRequest,
  SaveEventResponse,
  ApiResult, 
};
use crate::service::CONFIG;

pub async fn list_events(_req: Request) -> ApiResult {
  let events_lambda_arn =
    match &CONFIG.events_lambda_arn {
      Some(arn) => arn.clone(),
      None => {
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
  let lambda_req = ListEventsRequest { start_date: None, end_date: None, limit: 10 };

  match client.invoke::<_, ListEventsResponse>(lambda_req).await {
    Ok(response) => {
      let events: Vec<_> = response.events.iter()
        .map(|e| serde_json::to_value(e).unwrap())
        .collect();

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

pub async fn save_event(req: Request) -> ApiResult {
  let events_lambda_arn =
    match &CONFIG.events_lambda_arn {
      Some(arn) => arn.clone(),
      None => {
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

  let lambda_event: Option<SaveEventRequest> = match serde_json::from_slice(req.body().as_ref()) {
    Ok(save_event_req) => Some(save_event_req),
    Err(e) => {
      let body = serde_json::to_string(&ErrorResponse::new("BadRequest", format!("Invalid request body: {}", e)))?;

      let response = Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::Text(body))?;

      return Ok(response);
    }
  };

  let lambda_req = SaveEventRequest { event: lambda_event.unwrap().event };

  match client.invoke::<_, SaveEventResponse>(lambda_req).await {
    Ok(response) => {
      let body = serde_json::to_string(&response)?;

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
