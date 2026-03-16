use aws_config;
use aws_sdk_lambda::{Client as LambdaClient, primitives::Blob};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvocationError {
    #[error("Lambda invocation failed: {0}")]
    InvocationFailed(String),
    #[error("Failed to serialize request: {0}")]
    SerializationError(String),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),
    #[error("Lambda returned error: {0}")]
    LambdaError(String),
}

/// Client for invoking private Lambda functions with JSON payloads
pub struct PrivateLambdaClient {
    lambda_client: LambdaClient,
    function_arn: String,
}

impl PrivateLambdaClient {
    /// Create a new client for a specific Lambda function
    pub async fn new(function_arn: String) -> Self {
        let config = aws_config::load_from_env().await;
        let lambda_client = LambdaClient::new(&config);
        
        Self {
            lambda_client,
            function_arn,
        }
    }

       /// Invoke a private Lambda with JSON payload
    pub async fn invoke<Req, Resp>(&self, request: Req) -> Result<Resp, InvocationError>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        println!("Invoking Lambda: {}", self.function_arn);

        let payload = serde_json::to_vec(&request)
            .map_err(|e| InvocationError::SerializationError(e.to_string()))?;

        println!("Request payload size: {} bytes", payload.len());

        let result = self.lambda_client
            .invoke()
            .function_name(&self.function_arn)
            .invocation_type(aws_sdk_lambda::types::InvocationType::RequestResponse)
            .payload(Blob::new(payload))
            .send()
            .await
            .map_err(|e| InvocationError::InvocationFailed(e.to_string()))?;

        if let Some(function_error) = result.function_error() {
            let payload_text = result
                .payload()
                .map(|p| String::from_utf8_lossy(p.as_ref()).to_string())
                .unwrap_or_else(|| "<no error payload>".to_string());

            return Err(InvocationError::LambdaError(format!(
                "{}: {}",
                function_error,
                payload_text
            )));
        }

        let payload = result.payload()
            .ok_or_else(|| InvocationError::DeserializationError("No payload returned".to_string()))?;

        let response = serde_json::from_slice::<Resp>(payload.as_ref())
            .map_err(|e| InvocationError::DeserializationError(e.to_string()))?;

        Ok(response)
    }
}