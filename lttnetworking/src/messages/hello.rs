use crate::Token;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientHello {
    pub version: u64,
    pub credentials: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerHello {
    Unauthorized,
    UnsupportedVersion,
    Welcome { username: String, user_id: Uuid },
}
