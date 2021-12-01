//! Serialize/Deserialize support
//!
//! This module includes the [`Encoding`] enum which can be used to serialize/deserialize
//! messages. Tooling working with storage/transmission of `LibTableTop` things should do so
//! through this module.

use bytes::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

/// Turn `&self` to [`Bytes`]
///
/// This trait exists so it can be used behind a trait object. This has the pretty serious tradeoff
/// that we lost type information need to deserialize the output.
pub trait SerializeSelf {
    /// Turn `self` into [`Bytes`]
    ///
    /// ```
    /// use lttcore::encoding::{SerializeSelf, Encoding};
    ///
    /// assert_eq!(
    ///   [1,2,3].serialize_self(Encoding::Json).expect("could encode"),
    ///   "[1,2,3]"
    /// );
    /// ```
    fn serialize_self(&self, encoding: Encoding) -> Result<Bytes, EncodingError>;

    /// Helper function to **approximate** the type that produced the output
    ///
    /// This defaults to [`std::any::type_name`] which shouldn't be relied upon, it's also the best
    /// we have 🤷
    ///
    /// We can get bytes via the [`SerializeSelf::serialize_self`] method, but it's really
    /// difficult to go backwards to the original struct implementing [`SerializeSelf`] when we're
    /// using the trait through `dyn` (most of the time). This is a hack to provide slightly better
    /// error messages, output comes with the same tradeoffs as [`std::any::type_name`]
    fn source_hint(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<T: Serialize> SerializeSelf for T {
    fn serialize_self(&self, encoding: Encoding) -> Result<Bytes, EncodingError> {
        encoding.serialize(self)
    }
}

/// A specifier for which {de}serialization format the wire/disk uses
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
