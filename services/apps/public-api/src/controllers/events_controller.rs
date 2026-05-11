use base::services::api::json_response;
use lambda_http::Request;
use serde_json::json;
use lambda_client::SsLambdaClient;
use base::ErrorResponse;
use base::models::api::{
  ListEventsRequest,
  SaveEventRequest,
  SaveEventResponse,
  EventRequest,
  EventResponse,
  ApiResult
};
use crate::service::CONFIG;

pub async fn list_events(_req: Request) -> ApiResult {
  let events_lambda_arn =
    match &CONFIG.events_lambda_arn {
      Some(arn) => arn.clone(),
      None => {
      let err_value = &ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set");
      return json_response(500, err_value);

        // let body = serde_json::to_string(
        //     &ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set")
        // )?;

        // let response = Response::builder()
        //     .status(500)
        //     .header("content-type", "application/json")
        //     .body(Body::Text(body))?;

        // return Ok(response);
      }
    };

  let client = SsLambdaClient::new(events_lambda_arn).await;
  let lambda_req = EventRequest::ListEvents(ListEventsRequest {
    start_date: None,
    end_date: None,
    limit: 10,
  });

  match client.invoke::<_, EventResponse>(lambda_req).await {
    Ok(EventResponse::ListEvents(response)) => {
      let events: Vec<_> = response.events.iter()
        .map(|e| serde_json::to_value(e).unwrap())
        .collect();

      let result = json!({ "events": events, "nextToken": response.next_token });
      let response = json_response(200, &result);
      response
      

      // let body = json!({ "events": events, "nextToken": response.next_token }).to_string();

      // let response = Response::builder()
      //     .status(200)
      //     .header("content-type", "application/json")
      //     .body(Body::Text(body))?;

      // Ok(response)
    }
    Ok(_) => {
      let err_value = &ErrorResponse::new("InternalError", "Unexpected response from events service");
      let response = json_response(500, err_value);
      response

      // let body = json!({ "error": "Unexpected response from events service" }).to_string();
      // let response = Response::builder()
      //   .status(500)
      //   .header("content-type", "application/json")
      //   .body(Body::Text(body))?;

      // Ok(response)
    },
    Err(e) => {
      let err_value = &ErrorResponse::new("InternalError", e.to_string());
      let response = json_response(500, err_value);
      response

      // let body = serde_json::to_string(&ErrorResponse::new("InternalError", e.to_string()))?;

      // let response = Response::builder()
      //     .status(500)
      //     .header("content-type", "application/json")
      //     .body(Body::Text(body))?;

      // let x = Ok(response);
      // return x;
    }
  }
}

pub async fn save_event(req: Request) -> ApiResult {
  let events_lambda_arn =
    match &CONFIG.events_lambda_arn {
      Some(arn) => arn.clone(),
      None => {
        let err_value = &ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set");
        return json_response(500, err_value);

        // let body = serde_json::to_string(
        //     &ErrorResponse::new("ConfigError", "EVENTS_LAMBDA_ARN not set")
        // )?;

        // let response = Response::builder()
        //     .status(500)
        //     .header("content-type", "application/json")
        //     .body(Body::Text(body))?;

        // return Ok(response);
      }
    };

  let client = SsLambdaClient::new(events_lambda_arn).await;

  let save_event_req: SaveEventRequest = match serde_json::from_slice(req.body().as_ref()) {
    Ok(value) => value,
    Err(e) => {
      let err_value = &ErrorResponse::new("BadRequest", format!("Invalid request body: {}", e));
      return json_response(400, err_value);
      // let body = serde_json::to_string(err_value)?;

      // let response = Response::builder()
      //   .status(400)
      //   .header("content-type", "application/json")
      //   .body(Body::Text(body))?;

      // return Ok(response);
    }
  };

  let lambda_req = EventRequest::SaveEvent(save_event_req);

  match client.invoke::<_, EventResponse>(lambda_req).await {
    Ok(EventResponse::SaveEvent(response)) => {
      json_response(200, &response)
      // let body = serde_json::to_string(&response)?;

      // let response = Response::builder()
      //     .status(200)
      //     .header("content-type", "application/json")
      //     .body(Body::Text(body))?;

      // Ok(response)
    }
    Ok(_) => {
      let err_value = &ErrorResponse::new("InternalError", "Unexpected response from events service");
      json_response(500, err_value)
      // let body = errValue.to_string();

      // let response = Response::builder()
      //     .status(500)
      //     .header("content-type", "application/json")
      //     .body(Body::Text(body))?;

      // Ok(response)
    }
    Err(e) => {
      let err_value = ErrorResponse::new("InternalError", e.to_string());
      json_response(500, &err_value)
      // let body = serde_json::to_string(&err_value)?;

      // let response = Response::builder()
      //     .status(500)
      //     .header("content-type", "application/json")
      //     .body(Body::Text(body))?;

      // Ok(response)
    }
  }
}
