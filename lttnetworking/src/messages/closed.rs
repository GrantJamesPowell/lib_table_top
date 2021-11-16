use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Closed {
    Hangup,
    Normal,
    InvalidMsg,
    Unauthorized,
    InvalidCredentials,
    ServerError,
    Unsupported(String),
    ClientError(String),
}
