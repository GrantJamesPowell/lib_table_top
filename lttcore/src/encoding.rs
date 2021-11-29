use bytes::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Encoding {
    Bincode,
    PrettyJson,
    Json,
}

#[derive(Debug)]
pub enum EncodingError {
    Bincode(bincode::Error),
    Json(serde_json::error::Error),
}

impl Encoding {
    pub fn serialize<T: Serialize>(&self, value: &T) -> Result<Bytes, EncodingError> {
        match self {
            Encoding::Bincode => bincode::serialize(value)
                .map(|vec| vec.into())
                .map_err(EncodingError::Bincode),

            Encoding::Json => serde_json::to_vec(value)
                .map(|vec| vec.into())
                .map_err(EncodingError::Json),

            Encoding::PrettyJson => serde_json::to_vec_pretty(value)
                .map(|vec| vec.into())
                .map_err(EncodingError::Json),
        }
    }

    pub fn deserialize<T: DeserializeOwned>(&self, bytes: &Bytes) -> Result<T, EncodingError> {
        match self {
            Encoding::Bincode => bincode::deserialize(bytes).map_err(EncodingError::Bincode),
            Encoding::Json => serde_json::from_slice(bytes).map_err(EncodingError::Json),
            Encoding::PrettyJson => serde_json::from_slice(bytes).map_err(EncodingError::Json),
        }
    }
}
