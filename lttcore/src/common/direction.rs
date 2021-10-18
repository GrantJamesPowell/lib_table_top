
use crate::{player::PlayerResignations, Player};
use std::num::NonZeroU8;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LeftOrRight {
    #[default]
    Left,
    Right,
}

use LeftOrRight::*;

impl LeftOrRight {
    /// Reverse the direction
    ///
    /// ```
    /// use lttcore::common::direction::LeftOrRight::*;
    ///
    /// assert_eq!(Left.reverse(), Right);
    /// assert_eq!(Right.reverse(), Left);
    /// ```
    pub fn reverse(&self) -> Self {
        match self {
            Left => Right,
            Right => Left,
        }
    }

    /// Returns the "next" player, taking into account direction, number of players, and
    /// resignations. Will return `None` if all players are resigned, including the current player.
    /// ```
    /// use lttcore::{Player, player::PlayerResignations, common::direction::LeftOrRight::*};
    ///
    /// let num_players = 3.try_into().unwrap();
    /// let mut resignations: PlayerResignations = Default::default();
    ///
    /// // The 0th player is to the left of the first player
    /// assert_eq!(
    ///   Left.next_player(1.into(), num_players, &resignations),
    ///   Some(0.into())
    /// );
    ///
    /// // The 2nd Player is to the right of the first player
    /// ```
    ///
    /// # Panics
    ///
    /// This will panic if `current_player` is outside of the `num_players` range.
    /// This will likely represent  a logic bug in game code.
    ///
    /// ```should_panic
    /// use lttcore::{Player, player::PlayerResignations, common::direction::LeftOrRight::*};
    /// let num_players = 3.try_into().unwrap();
    ///
    /// Left.next_player(42.into(), num_players, &Default::default());
    /// ```
    pub fn next_player(
        &self,
        current_player: Player,
        num_players: NonZeroU8,
        resignations: &PlayerResignations,
    ) -> Option<Player> {
        assert!(current_player.as_u8() <= (num_players.get() - 1), "current_player is outside of num_players range");

        let next = |p: Player| -> Player {
            match self {
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

        let mut curr = next(current_player);

        loop {
            if curr == current_player {
                return Some(current_player).filter(|p| !resignations.is_resigned(*p));
            } else if !resignations.is_resigned(curr) {
                return Some(curr);
            } else {
                curr = next(curr)
            }
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compass {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowKey {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
