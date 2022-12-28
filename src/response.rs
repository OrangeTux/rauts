use crate::ocpp::v16::{
    call::Call,
    call_error::CallError,
    call_result::{Authorize, CallResult, Heartbeat},
};

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

impl IntoResponse for Authorize {
    fn into_response(&self, call: &Call) -> Result<CallResult, CallError> {
        Ok(CallResult {
            message_type_id: 3,
            unique_id: call.unique_id.clone(),
            payload: serde_json::to_value(self).unwrap(),
        })
    }
}

impl IntoResponse for Heartbeat {
    fn into_response(&self, call: &Call) -> Result<CallResult, CallError> {
        Ok(CallResult {
            message_type_id: 3,
            unique_id: call.unique_id.clone(),
            payload: serde_json::to_value(self).unwrap(),
        })
    }
}
