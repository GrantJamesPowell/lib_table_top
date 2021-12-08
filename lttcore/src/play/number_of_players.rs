#![warn(missing_docs)]
//! Number of players

use crate::{
    play::Player,
    utilities::{PlayerIndexedData, PlayerSet},
};
use serde::{Deserialize, Serialize};
use std::num::{NonZeroU32, TryFromIntError};
use std::ops::Deref;

/// Wrapper around [`NonZeroU32`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NumberOfPlayers(NonZeroU32);

impl NumberOfPlayers {
    /// Create a new [`NumberOfPlayers`]
    pub const fn new(n: NonZeroU32) -> Self {
        Self(n)
    }

    /// Return the "Starting Player" for a game, mostly just for readability,
    /// always returns Player 0
    pub const fn starting_player() -> Player {
        Player::new(0)
    }

    /// Returns an iterator over the players for a `NumberOfPlayers`
    ///
    /// ```
    /// use lttcore::play::{Player, NumberOfPlayers, number_of_players::THREE_PLAYER};
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

    /// Returns `PlayerIndexedData<T>` using `PlayerIndexedData::init_with`
    ///
    /// ```
    /// use lttcore::play::number_of_players::THREE_PLAYER;
    /// use lttcore::utilities::PlayerIndexedData as PID;
    ///
    /// let data: PID<u64> = THREE_PLAYER.player_indexed_data(|player| player.into());
    /// assert_eq!(data[0], 0);
    /// assert_eq!(data[1], 1);
    /// assert_eq!(data[2], 2);
    /// ```
    pub fn player_indexed_data<T>(&self, func: impl FnMut(Player) -> T) -> PlayerIndexedData<T> {
        PlayerIndexedData::init_with(self.player_set(), func)
    }

    /// Returns the `PlayerSet` containing all the players for that number of players
    ///
    /// ```
    /// use lttcore::{utilities::PlayerSet, play::{Player, NumberOfPlayers, number_of_players::THREE_PLAYER}};
    ///
    /// let mut expected = PlayerSet::new();
    /// expected.insert(0);
    /// expected.insert(1);
    /// expected.insert(2);
    ///
    /// assert_eq!(
    ///   THREE_PLAYER.player_set(),
    ///   expected
    /// );
    /// ```
    pub fn player_set(&self) -> PlayerSet {
        let mut set = PlayerSet::new();
        for player in self.players() {
            set.insert(player);
        }
        set
    }

    /// Returns true if a player is within a certain number of players.
    ///
    /// Players are zero indexed, so 2 players would represent players 0 && 1
    /// ```
    /// use lttcore::play::{Player, NumberOfPlayers, number_of_players::{TWO_PLAYER, THREE_PLAYER}};
    ///
    /// let p0: Player = 0.into();
    /// let p1: Player = 1.into();
    /// let p2: Player = 2.into();
    ///
    /// assert!(TWO_PLAYER.includes_player(p0));
    /// assert!(TWO_PLAYER.includes_player(p1));
    /// assert!(!TWO_PLAYER.includes_player(p2));
    /// assert!(THREE_PLAYER.includes_player(p2));
    /// ```
    pub fn includes_player(&self, player: impl Into<Player>) -> bool {
        let player = player.into();
        u32::from(player) <= (self.get() - 1)
    }
}

// [const_option] isn't a feature yet, so I'm using new_unchecked. It's sound to do so because none
// of the inputs are 0
/// [One](https://www.youtube.com/watch?v=d5ab8BOu4LE) Player
pub const ONE_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(1) });
/// Two Player
pub const TWO_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(2) });
/// Three Player
pub const THREE_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(3) });
/// Four Player
pub const FOUR_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(4) });
/// Five Player
pub const FIVE_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(5) });
/// Six Player
pub const SIX_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(6) });
/// Seven Player
pub const SEVEN_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(7) });
/// Eight Player
pub const EIGHT_PLAYER: NumberOfPlayers =
    NumberOfPlayers::new(unsafe { NonZeroU32::new_unchecked(8) });

impl TryFrom<u32> for NumberOfPlayers {
    type Error = TryFromIntError;

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        Ok(Self(n.try_into()?))
    }
}

impl From<NonZeroU32> for NumberOfPlayers {
    fn from(n: NonZeroU32) -> Self {
        Self(n)
    }
}

impl Deref for NumberOfPlayers {
    type Target = NonZeroU32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
