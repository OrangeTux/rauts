use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CallError {
    pub message_type_id: u8,
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: HashMap<String, String>,
}

impl CallError {
    pub fn new(
        unique_id: String,
        error_code: String,
        error_description: String,
        error_details: HashMap<String, String>,
    ) -> CallError {
        CallError {
            message_type_id: 4,
            unique_id,
            error_code,
            error_description,
            error_details,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    NotImplemented,
    NotSupported,
    InternalError,
    ProtocolError,
    SecurityError,
    FormationViolation,
    PropertyConstraintViolation,
    OccurenceConstraintViolation,
    TypeConstraintViolation,
    GenericError,
}
