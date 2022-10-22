use super::{Action, UniqueId};

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub message_type_id: u8,
    pub unique_id: UniqueId,
    pub action: Action,
    pub payload: Payload,
}

impl Call {
    pub fn new(unique_id: UniqueId, action: Action, payload: Payload) -> Call {
        Call {
            message_type_id: 2,
            unique_id,
            action,
            payload,
        }
    }
}

impl From<Authorize> for Call {
    fn from(payload: Authorize) -> Self {
        Call {
            message_type_id: 2,
            unique_id: UniqueId::default(),
            action: Action::Authorize,
            payload: Payload::Authorize(payload),
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
