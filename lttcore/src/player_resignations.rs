use crate::Player;

/// High performance player resignation abstraction designd to be O(1) for
/// Add/Remove/Lookup and to only use a fixed 32 bytes of memory. Is also
/// `Copy` which makes it super ergonomic to use
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
    /// use lttcore::{Player, PlayerResignations};
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
    /// use lttcore::{Player, PlayerResignations};
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
    /// use lttcore::{Player, PlayerResignations};
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
    /// use lttcore::{Player, PlayerResignations};
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
