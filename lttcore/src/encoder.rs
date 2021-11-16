use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Encoder: Debug + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + 'static {
    type Error: Debug + Send + Sync + std::error::Error;

    fn serialize<T: Serialize>(value: &T) -> Result<Bytes, Self::Error>;
    fn deserialize<T: DeserializeOwned>(bytes: Bytes) -> Result<T, Self::Error>;
}

#[cfg(feature = "bincode_encoder")]
pub mod bincode {
    use super::Encoder;
    use bytes::Bytes;
    use serde::{de::DeserializeOwned, Serialize};

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
}

#[cfg(feature = "json_encoder")]
pub mod json {
    use super::Encoder;
    use bytes::Bytes;
    use serde::{de::DeserializeOwned, Serialize};

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
}
