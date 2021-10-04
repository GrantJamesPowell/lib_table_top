use std::num::{NonZeroU16, TryFromIntError};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Player(NonZeroU16);

impl TryFrom<u16> for Player {
    type Error = TryFromIntError;

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        n.try_into().map(Player)
    }
}

/// Utility function to create players from integers
///
/// # Panics
///
/// panics if n == 0 || n > [`u16::MAX`]
pub fn p<T>(n: T) -> Player
where
    T: TryInto<u16>,
    <T as TryInto<u16>>::Error: std::fmt::Debug,
{
    let num: u16 = n.try_into().unwrap();
    num.try_into().unwrap()
}
