use super::UniqueId;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizeResponse {
    pub id_tag_info: IdTagInfo,
}

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
    Authorize(AuthorizeResponse),
}
