use super::UniqueId;
use chrono::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IdTagInfo {
    pub expiry_date: Option<String>,
    pub parent_id_tag: Option<String>,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub payload: Payload,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Payload {
    Authorize(Authorize),
    Heartbeat(Heartbeat),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Authorize {
    pub id_tag_info: IdTagInfo,
}

/// Payload of Heartbeat response.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Heartbeat {
    /// The current time of the Central System. The charger should adjust it's clock to this
    /// timestamp.
    pub current_time: DateTime<Utc>,
}
