use ocpp::call_error::CallError;
use ocpp::call_result::CallResult;

pub trait IntoResponse {
    fn into_response(&self) -> Result<CallResult, CallError>;
}

impl IntoResponse for CallResult {
    fn into_response(&self) -> Result<CallResult, CallError> {
        Ok(self.clone())
    }
}

impl IntoResponse for AuthorizeResponse {
    fn into_response(&self) -> Result<CallResult, CallError> {
        Ok(CallResult {
            message_type_id: 3,
            unique_id: "123".to_string(),
            payload: Payload::Authorize(self.clone()),
        })
    }
}
use ocpp::call_result::Payload;
use ocpp::v16::authorize_response::AuthorizeResponse;
