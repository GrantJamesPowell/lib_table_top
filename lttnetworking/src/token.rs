pub use hex::{FromHex, FromHexError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token(#[serde(with = "hex")] [u8; 32]);

impl std::str::FromStr for Token {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <[u8; 32]>::from_hex(s).map(Token)
    }
}
