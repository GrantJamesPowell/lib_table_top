use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Closed {
    Hangup,
    InvalidMsg,
    Unauthorized,
    InvalidCredentials,
}
