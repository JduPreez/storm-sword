mod controllers;
mod service;

use base::services::api::{handler_boxed, not_found_boxed};
use base::models::api::{BoxApiHandler, ApiResult};
use base::{partial};

use lambda_http::{run, service_fn, Request, Body, http::Response};
use std::future::Future;
use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;
use controllers::health_controller::handler as health_handler;
use controllers::events_controller::{list_events, save_event};

#[macro_use]
extern crate http_router;

struct ControllerHandlers {
  pub get_health: BoxApiHandler,
  pub list_events: BoxApiHandler,
  pub save_event: BoxApiHandler,
}

async fn main_handler<R>(router: Arc<R>, request: Request) -> ApiResult
where
    R: Fn(Request, http_router::Method, &str) ->
      Pin<Box<dyn Future<Output = Result<Response<Body>, Box<dyn Error + Send + Sync + 'static>>> + Send>>,
{
    let method = request.method().clone().into();
    let path = request.uri().path().to_string();
    (&*router)(request, method, path.as_str()).await
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .without_time()
        .with_ansi(false)
        .init();

    let handlers = ControllerHandlers {
        get_health: handler_boxed(health_handler),
        list_events: handler_boxed(list_events),
        save_event: handler_boxed(save_event),
    };

    let ControllerHandlers {
      get_health,
      list_events,
      save_event,
    } = handlers;

    let router = Arc::new(router!(
      GET /health => get_health,
      GET /events => list_events,
      POST /events => save_event,
      _ => not_found_boxed,
    ));

    run(service_fn(partial!(main_handler, [router]))).await
}