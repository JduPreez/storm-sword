use crate::service::CONFIG;
use base::models::api::{
  ApiResult, EventRequest, EventResponse, ListEventsRequest, SaveEventRequest,
};
use base::services::api::json_response;
use base::services::auth;
use base::ErrorResponse;
use lambda_client::SsLambdaClient;
use lambda_http::{Request, RequestExt};
use serde_json::json;

/// `GET /events/{event_type}/{country_code}?startDate=&endDate=&limit=`
///
/// `event_type` and `country_code` are mandatory path params (they compose the
/// `Ns` partition key the events service queries on). Optional filters and
/// pagination arrive as query-string params.
pub async fn list_events(req: Request, event_type: String, country_code: String) -> ApiResult {
  let events_lambda_arn = match &CONFIG.events_lambda_arn {
    Some(arn) => arn.clone(),
    None => {
      let err_value = &ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set");
      return json_response(500, err_value);
    }
  };

  let query = req.query_string_parameters();
  let list_events_req = ListEventsRequest {
    event_type,
    country_code,
    start_date: query.first("startDate").map(str::to_string),
    end_date: query.first("endDate").map(str::to_string),
    limit: query
      .first("limit")
      .and_then(|s| s.parse::<i32>().ok())
      .unwrap_or(50),
  };

  let client = SsLambdaClient::new(events_lambda_arn).await;
  let lambda_req = EventRequest::ListEvents(list_events_req);

  match client.invoke::<_, EventResponse>(lambda_req).await {
    Ok(EventResponse::ListEvents(response)) => {
      let events: Vec<_> = response
        .events
        .iter()
        .map(|e| serde_json::to_value(e).unwrap())
        .collect();

      let result = json!({ "events": events, "nextToken": response.next_token });
      let response = json_response(200, &result);
      response
    }
    Ok(_) => {
      let err_value =
        &ErrorResponse::new("InternalError", "Unexpected response from events service");
      let response = json_response(500, err_value);
      response
    }
    Err(e) => {
      let err_value = &ErrorResponse::new("InternalError", e.to_string());
      let response = json_response(500, err_value);
      response
    }
  }
}

pub async fn save_event(req: Request) -> ApiResult {
  let api_token = match &CONFIG.api_token {
    Some(token) => token,
    None => {
      let err_value = &ErrorResponse::new("ConfigError", "API_TOKEN not set");
      return json_response(500, err_value);
    }
  };

  if !auth::is_authorized(&req, api_token) {
    return auth::unauthorized_response();
  }

  let events_lambda_arn = match &CONFIG.events_lambda_arn {
    Some(arn) => arn.clone(),
    None => {
      let err_value = &ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set");
      return json_response(500, err_value);
    }
  };

  let client = SsLambdaClient::new(events_lambda_arn).await;

  let save_event_req: SaveEventRequest = match serde_json::from_slice(req.body().as_ref()) {
    Ok(value) => value,
    Err(e) => {
      let err_value = &ErrorResponse::new("BadRequest", format!("Invalid request body: {}", e));
      return json_response(400, err_value);
    }
  };

  let lambda_req = EventRequest::SaveEvent(save_event_req);

  match client.invoke::<_, EventResponse>(lambda_req).await {
    Ok(EventResponse::SaveEvent(response)) => json_response(200, &response),
    Ok(_) => {
      let err_value =
        &ErrorResponse::new("InternalError", "Unexpected response from events service");
      json_response(500, err_value)
    }
    Err(e) => {
      let err_value = ErrorResponse::new("InternalError", e.to_string());
      json_response(500, &err_value)
    }
  }
}
