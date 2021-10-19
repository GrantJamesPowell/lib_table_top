use crate::common::direction::LeftOrRight::{self, *};
use crate::NumberOfPlayers;

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
    /// use lttcore::{Player, player::PlayerResignations, common::direction::LeftOrRight::*};
    ///
    /// let player: Player = 1.into();
    /// let num_players = 3.try_into().unwrap();
    /// let mut resignations: PlayerResignations = Default::default();
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
    /// use lttcore::{Player, player::PlayerResignations, common::direction::LeftOrRight::*};
    /// let player: Player = 42.into();
    /// let num_players = 3.try_into().unwrap();
    ///
    /// player.next_player_for_direction(Left, num_players, &Default::default());
    /// ```
    pub fn next_player_for_direction(
        &self,
        direction: LeftOrRight,
        num_players: NumberOfPlayers,
        resignations: &PlayerResignations,
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
                return Some(*self).filter(|&p| !resignations.is_resigned(p));
            } else if !resignations.is_resigned(curr) {
                return Some(curr);
            } else {
                curr = next(curr)
            }
        }
    }
}

/// High performance player resignation abstraction. Designed to be fast and cheap to copy
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct PlayerResignations([u64; 4]);

impl PlayerResignations {
    /// Returns a new, empty player resignation set
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns if a player has resigned
    ///
    /// ```
    /// use lttcore::{Player, player::PlayerResignations};
    ///
    /// let mut resignations = PlayerResignations::new();
    ///
    ///
    /// let player: Player = 1.into();
    ///
    /// assert!(!resignations.is_resigned(player));
    /// resignations.resign(player);
    /// assert!(resignations.is_resigned(player));
    /// ```
    pub fn is_resigned(&self, player: Player) -> bool {
        (self.0[Self::section(player)] & (1usize << Self::offset(player)) as u64) > 0
    }

    /// Iterator over resigned players
    ///
    /// ```
    /// use lttcore::{Player, player::PlayerResignations};
    ///
    /// let mut resignations = PlayerResignations::new();
    ///
    /// assert!(resignations.resigned_players().next().is_none());
    ///
    /// let player: Player = 1.into();
    /// resignations.resign(player);
    ///
    /// assert_eq!(
    ///   resignations.resigned_players().collect::<Vec<_>>(),
    ///   vec![player]
    /// );
    /// ```
    pub fn resigned_players(&self) -> impl Iterator<Item = Player> + '_ {
        Player::all().filter(|&player| self.is_resigned(player))
    }

    /// Adds the player to the resigned set, is a noop if player is already in set
    ///
    /// ```
    /// use lttcore::{Player, player::PlayerResignations};
    ///
    /// let mut resignations = PlayerResignations::new();
    /// let player: Player = 1.into();
    ///
    /// assert!(!resignations.is_resigned(player));
    /// resignations.resign(player);
    /// assert!(resignations.is_resigned(player));
    /// ```
    pub fn resign(&mut self, player: Player) {
        self.0[Self::section(player)] |= (1usize << Self::offset(player)) as u64
    }

    /// Remove a player from the set, is a noop if player is not in the set
    ///
    /// ```
    /// use lttcore::{Player, player::PlayerResignations};
    ///
    /// let mut resignations = PlayerResignations::new();
    /// let player: Player = 1.into();
    ///
    /// assert!(!resignations.is_resigned(player));
    /// resignations.resign(player);
    /// assert!(resignations.is_resigned(player));
    /// resignations.unresign(player);
    /// assert!(!resignations.is_resigned(player));
    /// ```
    pub fn unresign(&mut self, player: Player) {
        self.0[Self::section(player)] &= !(1usize << Self::offset(player)) as u64
    }

    fn section(player: Player) -> usize {
        player.as_usize().checked_div(64).unwrap()
    }

    fn offset(player: Player) -> usize {
        player.as_usize() % 64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resignation_works_for_all_players() {
        for player in Player::all() {
            let mut resignations = PlayerResignations::new();
            assert!(!resignations.is_resigned(player));
            resignations.resign(player);
            assert!(resignations.is_resigned(player));
            resignations.unresign(player);
            assert!(!resignations.is_resigned(player));
        }

        let mut resignations = PlayerResignations::new();

        for player in Player::all() {
            resignations.resign(player);
        }

        for player in Player::all() {
            assert!(resignations.is_resigned(player));
        }
    }
}
