use serde::{Deserialize, Serialize};

/// An identifier for a "player" within Lib Table Top
///
/// Conceptually a player is someone who can act upon a the game. A player may or may not have
/// secret information and will have between 0 and many turns over the course of the game.
///
/// # Implementation notes:
///
/// [`Player`] is a wrapper around a [`u8`], letting games have a maximum of 256 players.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Player(u32);

macro_rules! from_player_for_num {
    ($num:ty) => {
        impl From<Player> for $num {
            fn from(player: Player) -> Self {
                player.0 as $num
            }
        }
    };
}

from_player_for_num!(usize);
from_player_for_num!(u64);
from_player_for_num!(u32);
from_player_for_num!(u16);
from_player_for_num!(u8);

impl Player {
    pub const MAX: Player = Player(u32::MAX);
    pub const STARTING: Player = Player(0);

    /// Create a new [`Player`]
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    /// Return the previous [`Player`], wrapping around from Player::STARTING => Player::MAX
    ///
    /// ```
    /// use lttcore::play::Player;
    ///
    /// let p_starting = Player::STARTING;
    /// let p1: Player = 1.into();
    /// let p2: Player = 2.into();
    /// let p_max = Player::MAX;
    ///
    /// assert_eq!(p_starting.previous(), p_max);
    /// assert_eq!(p1.previous(), p_starting);
    /// assert_eq!(p2.previous(), p1);
    /// ```
    pub fn previous(&self) -> Self {
        Self(self.0.wrapping_sub(1))
    }

    /// Return the next [`Player`], wrapping around from Player::MAX => Player::STARTING
    ///
    /// ```
    /// use lttcore::play::Player;
    ///
    /// let p_starting = Player::STARTING;
    /// let p1: Player = 1.into();
    /// let p2: Player = 2.into();
    /// let p_max = Player::MAX;
    ///
    /// assert_eq!(p_starting.next(), p1);
    /// assert_eq!(p1.next(), p2);
    /// assert_eq!(p_max.next(), p_starting);
    /// ```
    pub fn next(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

impl From<u32> for Player {
    fn from(n: u32) -> Self {
        Self(n)
    }
}
