use super::UniqueId;
use chrono::prelude::*;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdTagInfo {
    pub expiry_date: Option<String>,
    pub parent_id_tag: Option<String>,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallResult {
    pub message_type_id: u8,
    pub unique_id: UniqueId,
    pub payload: serde_json::Value,
}

impl Serialize for CallResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.message_type_id);
        seq.serialize_element(&Into::<String>::into(self.unique_id.clone()));
        seq.serialize_element(&self.payload);

        seq.end()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorize {
    pub id_tag_info: IdTagInfo,
}

/// Payload of Heartbeat response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    /// The current time of the Central System. The charger should adjust it's clock to this
    /// timestamp.
    pub current_time: DateTime<Utc>,
}
