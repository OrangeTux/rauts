use crate::ocpp::v16::Action;

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub message_type_id: u8,
    pub unique_id: String,
    pub action: Action,
    pub payload: Payload,
}

impl Call {
    pub fn new(unique_id: String, action: Action, payload: Payload) -> Call {
        Call {
            message_type_id: 2,
            unique_id,
            action,
            payload,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Payload {
    Authorize(Authorize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Authorize {
    pub id_tag: String,
}
