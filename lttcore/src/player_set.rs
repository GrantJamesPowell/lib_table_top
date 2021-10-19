use crate::Player;

/// High performance player set abstraction designd to be O(1) for
/// Add/Remove/Lookup and to only use a fixed 32 bytes of memory. Is also
/// `Copy` which makes it super ergonomic to use
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct PlayerSet([u64; 4]);

impl PlayerSet {
    /// Returns a new, empty player set
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns if a player is in set
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let mut set = PlayerSet::new();
    /// let player: Player = 1.into();
    ///
    /// assert!(!set.contains(player));
    /// set.add(player);
    /// assert!(set.contains(player));
    /// ```
    pub fn contains(&self, player: Player) -> bool {
        (self.0[Self::section(player)] & (1usize << Self::offset(player)) as u64) > 0
    }

    /// Iterator over players in the set
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let mut set = PlayerSet::new();
    ///
    /// assert!(set.players().next().is_none());
    ///
    /// let player: Player = 1.into();
    /// set.add(player);
    ///
    /// assert_eq!(
    ///   set.players().collect::<Vec<_>>(),
    ///   vec![player]
    /// );
    /// ```
    pub fn players(&self) -> impl Iterator<Item = Player> + '_ {
        Player::all().filter(|&player| self.contains(player))
    }

    /// Adds the player to the set, is a noop if player is already in set
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let mut set = PlayerSet::new();
    /// let player: Player = 1.into();
    ///
    /// assert!(!set.contains(player));
    /// set.add(player);
    /// assert!(set.contains(player));
    /// ```
    pub fn add(&mut self, player: Player) {
        self.0[Self::section(player)] |= (1usize << Self::offset(player)) as u64
    }

    /// Remove a player from the set, is a noop if player is not in the set
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let mut set = PlayerSet::new();
    /// let player: Player = 1.into();
    ///
    /// assert!(!set.contains(player));
    /// set.add(player);
    /// assert!(set.contains(player));
    /// set.remove(player);
    /// assert!(!set.contains(player));
    /// ```
    pub fn remove(&mut self, player: Player) {
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
    fn test_set_works_for_all_players() {
        for player in Player::all() {
            let mut set = PlayerSet::new();
            assert!(!set.contains(player));
            set.add(player);
            assert!(set.contains(player));
            set.remove(player);
            assert!(!set.contains(player));
        }

        let mut set = PlayerSet::new();

        for player in Player::all() {
            set.add(player);
        }

        for player in Player::all() {
            assert!(set.contains(player));
        }
    }
}
