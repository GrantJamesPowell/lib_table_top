use serde::{Serialize, de::DeserializeOwned};
use bytes::Bytes;
use std::fmt::Debug;

pub trait Encoder {
    type Error: Debug + std::error::Error;

    fn serialize<T: Serialize>(value: &T) -> Bytes;
    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error>;
}

pub struct BincodeEncoder;

impl Encoder for BincodeEncoder {
    type Error = bincode::Error;

    fn serialize<T: Serialize>(value: &T) -> Bytes {
        bincode::serialize(value)
            .expect("should be able to serialize anything")
            .into()
    }
    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error> {
        bincode::deserialize(&bytes)
    }
}

pub struct JsonEncoder;

impl Encoder for JsonEncoder {
    type Error = serde_json::error::Error;

    fn serialize<T: Serialize>(value: &T) -> Bytes {
        serde_json::to_vec(value)
            .expect("should be able to serialize anything")
            .into()
    }

    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error> {
        serde_json::from_slice(&bytes)
    }

}
