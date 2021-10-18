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
