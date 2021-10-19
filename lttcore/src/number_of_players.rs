use crate::Player;
use std::num::{NonZeroU8, TryFromIntError};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberOfPlayers(NonZeroU8);

impl NumberOfPlayers {
    /// Returns an iterator over the players for a NumberOfPlayers
    ///
    /// ```
    /// use lttcore::{Player, NumberOfPlayers};
    ///
    /// let num_players: NumberOfPlayers = 3.try_into().unwrap();
    /// let expected: Vec<Player> = [0, 1, 2].iter().map(|&n| n.into()).collect();
    ///
    /// assert_eq!(
    ///   num_players.players().collect::<Vec<Player>>(),
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
