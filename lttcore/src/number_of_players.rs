use crate::Player;
use std::num::{NonZeroU8, TryFromIntError};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberOfPlayers(NonZeroU8);

impl NumberOfPlayers {
    pub const fn new(n: NonZeroU8) -> Self {
        Self(n)
    }

    /// Returns an iterator over the players for a NumberOfPlayers
    ///
    /// ```
    /// use lttcore::{Player, NumberOfPlayers, number_of_players::THREE_PLAYER};
    ///
    /// let expected: Vec<Player> = [0, 1, 2].iter().map(|&n| n.into()).collect();
    ///
    /// assert_eq!(
    ///   THREE_PLAYER.players().collect::<Vec<Player>>(),
    ///   expected
    /// );
    /// ```
    pub fn players(&self) -> impl Iterator<Item = Player> {
        (0..self.get()).map(|p| p.into())
    }

    /// Returns true if a player is within a certain number of players.
    ///
    /// Players are zero indexed, so 2 players would represent players 0 && 1
    /// ```
    /// use lttcore::{Player, NumberOfPlayers};
    ///
    /// let p0: Player = 0.into();
    /// let p1: Player = 1.into();
    /// let p2: Player = 2.into();
    ///
    /// let num_players: NumberOfPlayers = 2.try_into().unwrap();
    ///
    /// assert!(num_players.includes_player(p0));
    /// assert!(num_players.includes_player(p1));
    /// assert!(!num_players.includes_player(p2));
    ///
    /// ```
    pub fn includes_player(&self, player: Player) -> bool {
        player.as_u8() <= (self.get() - 1)
    }
}

pub const ONE_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(1).unwrap());
pub const TWO_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(2).unwrap());
pub const THREE_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(3).unwrap());
pub const FOUR_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(4).unwrap());
pub const FIVE_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(5).unwrap());
pub const SIX_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(6).unwrap());
pub const SEVEN_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(7).unwrap());
pub const EIGHT_PLAYER: NumberOfPlayers = NumberOfPlayers::new(NonZeroU8::new(8).unwrap());

impl TryFrom<u8> for NumberOfPlayers {
    type Error = TryFromIntError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        Ok(Self(n.try_into()?))
    }
}

impl From<NonZeroU8> for NumberOfPlayers {
    fn from(n: NonZeroU8) -> Self {
        Self(n)
    }
}

impl Deref for NumberOfPlayers {
    type Target = NonZeroU8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
