use super::{Action, UniqueId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub message_type_id: u8,
    pub unique_id: UniqueId,
    pub action: Action,
    pub payload: serde_json::Value,
}

impl Call {
    pub fn new(unique_id: UniqueId, action: Action, payload: serde_json::Value) -> Call {
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
            payload: serde_json::to_value(payload).unwrap(),
        }
    }
}

impl From<Heartbeat> for Call {
    fn from(payload: Heartbeat) -> Self {
        Call {
            message_type_id: 2,
            unique_id: UniqueId::default(),
            action: Action::Heartbeat,
            payload: serde_json::to_value(payload).unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorize {
    pub id_tag: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {}
