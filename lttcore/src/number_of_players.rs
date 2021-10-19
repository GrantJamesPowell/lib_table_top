use std::num::{NonZeroU8, TryFromIntError};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberOfPlayers(NonZeroU8);

impl TryFrom<u8> for NumberOfPlayers {
    type Error = TryFromIntError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        Ok(Self(n.try_into()?))
    }
}

impl Deref for NumberOfPlayers {
    type Target = NonZeroU8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
