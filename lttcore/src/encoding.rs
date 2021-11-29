//! Serialize/Deserialize strategies
//!
//! This module includes the [`Encoding`] enum which can be used to serialize/deserialize
//! messages. Tooling working with storage/transmission of `LibTableTop` things should do so
//! through this module

use bytes::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

/// A {de}serialization format for wire/disk uses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Encoding {
    /// [Bincode encoding](https://docs.rs/bincode/1.3.3/bincode/)
    Bincode,
    /// [JSON](https://en.wikipedia.org/wiki/JSON), but pretty
    PrettyJson,
    /// [JSON](https://en.wikipedia.org/wiki/JSON)
    Json,
}

/// An error produced while trying to {de}serialize a value
#[allow(missing_docs)]
#[derive(Debug)]
pub enum EncodingError {
    Bincode(bincode::Error),
    Json(serde_json::error::Error),
}

impl Encoding {
    /// Turn some value `T` into [`Bytes`]
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

    /// Try to turn some [`Bytes`] into a `T`
    pub fn deserialize<T: DeserializeOwned>(&self, bytes: &Bytes) -> Result<T, EncodingError> {
        match self {
            Encoding::Bincode => bincode::deserialize(bytes).map_err(EncodingError::Bincode),
            Encoding::Json | Encoding::PrettyJson => {
                serde_json::from_slice(bytes).map_err(EncodingError::Json)
            }
        }
    }
}
