use super::{Action, UniqueId};
use serde::de::{Deserializer, Error, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub message_type_id: u8,
    pub unique_id: UniqueId,
    pub action: String,
    pub payload: serde_json::Value,
}

impl Call {
    pub fn new<T: Into<String>>(
        unique_id: UniqueId,
        action: T,
        payload: serde_json::Value,
    ) -> Call {
        Call {
            message_type_id: 2,
            unique_id,
            action: action.into(),
            payload,
        }
    }
}

impl<'de> Deserialize<'de> for Call {
    fn deserialize<D>(deserializer: D) -> Result<Call, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CallVisitor;
        impl<'de> Visitor<'de> for CallVisitor {
            type Value = Call;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string representing an OCPP call message")
            }

            fn visit_seq<M>(self, mut seq: M) -> Result<Self::Value, M::Error>
            where
                M: SeqAccess<'de>,
            {
                let message_type_id = seq
                    .next_element::<u8>()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                match message_type_id {
                    2 => {
                        let unique_id = seq
                            .next_element::<String>()?
                            .ok_or_else(|| Error::invalid_length(1, &self))?;

                        let action = seq
                            .next_element::<String>()?
                            .ok_or_else(|| Error::invalid_length(2, &self))?;

                        let payload = seq
                            .next_element::<serde_json::Value>()?
                            .ok_or_else(|| Error::invalid_length(3, &self))?;

                        Ok(Call::new(UniqueId(unique_id), action, payload))
                    }
                    n => Err(Error::invalid_value(
                        serde::de::Unexpected::Unsigned(n.into()),
                        &self,
                    )),
                }
            }
        }

        deserializer.deserialize_seq(CallVisitor {})
    }
}

impl From<Authorize> for Call {
    fn from(payload: Authorize) -> Self {
        Call {
            message_type_id: 2,
            unique_id: UniqueId::default(),
            action: "Authorize".to_string(),
            payload: serde_json::to_value(payload).unwrap(),
        }
    }
}

impl From<Heartbeat> for Call {
    fn from(payload: Heartbeat) -> Self {
        Call {
            message_type_id: 2,
            unique_id: UniqueId::default(),
            action: "Heartbeat".to_string(),
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
