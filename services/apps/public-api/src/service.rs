use once_cell::sync::Lazy;
use base::utils;

#[derive(Debug)]
pub struct Config {
  pub events_lambda_arn: Option<String>,
  pub api_token: Option<String>,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
  events_lambda_arn: utils::get_env_var("EVENTS_LAMBDA_ARN").ok(),
  api_token: utils::get_env_var("API_TOKEN").ok(),
});
