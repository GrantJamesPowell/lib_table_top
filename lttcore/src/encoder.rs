use bytes::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Encoder: Debug + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + 'static {
    type Error: Debug + Send + Sync + std::error::Error;

    fn serialize<T: Serialize>(value: &T) -> Result<Bytes, Self::Error>;
    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error>;
}

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
            Encoding::Bincode => BincodeEncoder::serialize(value).map_err(EncodingError::Bincode),
            Encoding::Json => JsonEncoder::serialize(value).map_err(EncodingError::Json),
            Encoding::PrettyJson => {
                PrettyJsonEncoder::serialize(value).map_err(EncodingError::Json)
            }
        }
    }

    pub fn deserialize<T: DeserializeOwned>(&self, bytes: Bytes) -> Result<T, EncodingError> {
        match self {
            Encoding::Bincode => BincodeEncoder::deserialize(bytes).map_err(EncodingError::Bincode),
            Encoding::Json => JsonEncoder::deserialize(bytes).map_err(EncodingError::Json),
            Encoding::PrettyJson => {
                PrettyJsonEncoder::deserialize(bytes).map_err(EncodingError::Json)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BincodeEncoder;

impl Encoder for BincodeEncoder {
    type Error = bincode::Error;

    fn serialize<T: Serialize>(value: &T) -> Result<Bytes, Self::Error> {
        bincode::serialize(value).map(|vec| vec.into())
    }

    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error> {
        bincode::deserialize(&bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsonEncoder;

impl Encoder for JsonEncoder {
    type Error = serde_json::error::Error;

    fn serialize<T: Serialize>(value: &T) -> Result<Bytes, Self::Error> {
        serde_json::to_vec(value).map(|vec| vec.into())
    }

    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error> {
        serde_json::from_slice(&bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrettyJsonEncoder;

impl Encoder for PrettyJsonEncoder {
    type Error = serde_json::error::Error;

    fn serialize<T: Serialize>(value: &T) -> Result<Bytes, Self::Error> {
        serde_json::to_vec_pretty(value).map(|vec| vec.into())
    }

    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error> {
        serde_json::from_slice(&bytes)
    }
}
