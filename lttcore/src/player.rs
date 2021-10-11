use std::num::{NonZeroU128, TryFromIntError};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Player(NonZeroU128);

impl TryFrom<u128> for Player {
    type Error = TryFromIntError;

    fn try_from(n: u128) -> Result<Self, Self::Error> {
        n.try_into().map(Player)
    }
}

/// Utility function to create players from integers
///
/// # Panics
///
/// panics if n == 0 || n > [`u128::MAX`]
pub fn p<T>(n: T) -> Player
where
    T: TryInto<u128>,
    <T as TryInto<u128>>::Error: std::fmt::Debug,
{
    let num: u128 = n.try_into().unwrap();
    num.try_into().unwrap()
}
