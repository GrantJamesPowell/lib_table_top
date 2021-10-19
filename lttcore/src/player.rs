use crate::common::direction::LeftOrRight::{self, *};
use crate::{NumberOfPlayers, PlayerSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Player(u8);

impl Player {
    pub fn new(n: u8) -> Self {
        n.into()
    }

    pub fn all() -> impl Iterator<Item = Player> {
        (0..=u8::MAX).map(Self::new)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl From<u8> for Player {
    fn from(n: u8) -> Self {
        Self(n)
    }
}

impl Player {
    /// Returns the "next" player, taking into account direction, number of players, and
    /// resignations. Will return `None` if all players are resigned, including the current player.
    /// ```
    /// use lttcore::{Player, PlayerSet, common::direction::LeftOrRight::*};
    ///
    /// let player: Player = 1.into();
    /// let num_players = 3.try_into().unwrap();
    /// let mut resignations: PlayerSet = Default::default();
    ///
    /// // The 0th player is to the left of the first player
    /// assert_eq!(
    ///   player.next_player_for_direction(Left, num_players, &resignations),
    ///   Some(0.into())
    /// );
    ///
    /// // The 2nd Player is to the right of the first player
    /// ```
    ///
    /// # Panics
    ///
    /// This will panic if the player is outside of the `num_players` range.
    /// This will likely represent  a logic bug in game code.
    ///
    /// ```should_panic
    /// use lttcore::{Player, PlayerSet, common::direction::LeftOrRight::*};
    /// let player: Player = 42.into();
    /// let num_players = 3.try_into().unwrap();
    ///
    /// player.next_player_for_direction(Left, num_players, &Default::default());
    /// ```
    pub fn next_player_for_direction(
        &self,
        direction: LeftOrRight,
        num_players: NumberOfPlayers,
        resignations: &PlayerSet,
    ) -> Option<Player> {
        assert!(
            num_players.includes_player(*self),
            "current_player is not in num_players"
        );

        let next = |p: Player| -> Player {
            match direction {
                Left => match p.as_u8() {
                    0 => (num_players.get() - 1).into(),
                    n => (n - 1).into(),
                },
                Right => match p.as_u8() {
                    n if n == (num_players.get() - 1) => 0.into(),
                    n => (n + 1).into(),
                },
            }
        };

        let mut curr: Player = next(*self);

        loop {
            if curr == *self {
                return Some(*self).filter(|&p| !resignations.contains(p));
            } else if !resignations.contains(curr) {
                return Some(curr);
            } else {
                curr = next(curr)
            }
        }
    }
}
