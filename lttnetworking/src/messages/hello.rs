use crate::Token;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientHello {
    pub version: u64,
    pub credentials: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerHello {
    Unauthorized,
    UnsupportedVersion,
    Welcome { username: String },
}
