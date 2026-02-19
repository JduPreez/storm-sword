use aws_config;
use aws_sdk_lambda::{Client as LambdaClient, primitives::Blob};
use prost::Message;
use thiserror::Error;
use tracing::{debug, error};

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

/// Client for invoking private Lambda functions with protobuf payloads
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

    /// Invoke a private Lambda with protobuf payload
    pub async fn invoke<Req, Resp>(&self, request: Req) -> Result<Resp, InvocationError>
    where
        Req: Message,
        Resp: Message + Default,
    {
        debug!("Invoking Lambda: {}", self.function_arn);

        // Serialize request to protobuf bytes
        let mut buf = Vec::new();
        request.encode(&mut buf)
            .map_err(|e| {
                error!("Failed to encode request: {}", e);
                InvocationError::SerializationError(e.to_string())
            })?;

        debug!("Request payload size: {} bytes", buf.len());

        // Invoke Lambda
        let result = self.lambda_client
            .invoke()
            .function_name(&self.function_arn)
            .invocation_type(aws_sdk_lambda::types::InvocationType::RequestResponse)
            .payload(Blob::new(buf))
            .send()
            .await
            .map_err(|e| {
              eprintln!("===== Lambda invocation error details =====");
              eprintln!("Error type: {}", std::any::type_name_of_val(&e));
              eprintln!("Error Display: {}", e);
              eprintln!("Error Debug: {:?}", e);
              eprintln!("Error Pretty Debug: {:#?}", e);
              
              // Try to match on specific error types
              use aws_sdk_lambda::error::SdkError;
              match &e {
                  SdkError::ServiceError(service_err) => {
                      eprintln!(">>> This is a ServiceError");
                      eprintln!(">>> Inner error: {:?}", service_err.err());
                      eprintln!(">>> Raw response: {:?}", service_err.raw());
                  }
                  SdkError::ConstructionFailure(err) => {
                      eprintln!(">>> This is a ConstructionFailure: {:?}", err);
                  }
                  SdkError::TimeoutError(err) => {
                      eprintln!(">>> This is a TimeoutError: {:?}", err);
                  }
                  SdkError::DispatchFailure(err) => {
                      eprintln!(">>> This is a DispatchFailure: {:?}", err);
                  }
                  SdkError::ResponseError(err) => {
                      eprintln!(">>> This is a ResponseError: {:?}", err);
                  }
                  _ => {
                      eprintln!(">>> Unknown error variant");
                  }
              }
              eprintln!("==========================================");
              
              error!("Lambda invocation failed: {}", e);
              InvocationError::InvocationFailed(format!("{}", e))
            })?;

        // Check for function errors
        if let Some(function_error) = result.function_error() {
            error!("Lambda returned error: {}", function_error);
            return Err(InvocationError::LambdaError(function_error.to_string()));
        }

        // Get response payload
        let payload = result.payload()
            .ok_or_else(|| {
                error!("No payload returned from Lambda");
                InvocationError::DeserializationError("No payload returned".to_string())
            })?;

        debug!("Response payload size: {} bytes", payload.as_ref().len());

        // Deserialize protobuf response
        let response = Resp::decode(payload.as_ref())
            .map_err(|e| {
                error!("Failed to decode response: {}", e);
                InvocationError::DeserializationError(e.to_string())
            })?;

        Ok(response)
    }
}