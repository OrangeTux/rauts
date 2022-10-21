use ocpp::call::Call;
use ocpp::call_error::CallError;
use ocpp::call_result::CallResult;

use ocpp::call_result::Payload;
use ocpp::v16::authorize_response::AuthorizeResponse;

/// Trait for generating a response to an OCPP Call.
///
/// Types that implement `IntoResponse` can be returned from handler functions.
///
pub trait IntoResponse {
    /// Create a response for an OCPP Call. The response's `unique_id` must match the one from the
    /// `Call`.
    fn into_response(&self, call: &Call) -> Result<CallResult, CallError>;
}

impl IntoResponse for CallResult {
    fn into_response(&self, _: &Call) -> Result<CallResult, CallError> {
        Ok(self.clone())
    }
}

impl IntoResponse for Result<CallResult, CallError> {
    fn into_response(&self, _: &Call) -> Result<CallResult, CallError> {
        self.clone()
    }
}

impl IntoResponse for AuthorizeResponse {
    fn into_response(&self, call: &Call) -> Result<CallResult, CallError> {
        Ok(CallResult {
            message_type_id: 3,
            unique_id: call.unique_id.clone(),
            payload: Payload::Authorize(self.clone()),
        })
    }
}
