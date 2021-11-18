use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Serialize, Deserialize)]
pub enum Closed {
    #[error("connection hung up unexpectedly")]
    Hangup,
    #[error("connection closed normally")]
    Normal,
    #[error("connection sent an invalid message and we can't continue")]
    InvalidMsg,
    #[error("credentials not found")]
    InvalidCredentials,
    #[error("internal server error")]
    ServerError,
    #[error("connection is unauthorized to {0}")]
    Unauthorized(String),
    #[error("operation {0} is not supported")]
    Unsupported(String),
    #[error("client error: {0}")]
    ClientError(String),
}
