use crate::Token;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMsg {
    TokenAuth(Token),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthResultMsg {
    Authorized { user_name: String, user_id: Uuid },
    Unauthorized { msg: String },
    ConnectionLimitExceeded,
}
