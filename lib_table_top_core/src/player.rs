use std::num::{NonZeroU16, TryFromIntError};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Player(NonZeroU16);

impl TryFrom<u16> for Player {
    type Error = TryFromIntError;

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        n.try_into().map(Player)
    }
}
