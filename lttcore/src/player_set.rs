use crate::Player;
use serde::{Deserialize, Serialize};

/// High performance player set abstraction designd to be O(1) for
/// Add/Remove/Lookup and to only use a fixed 32 bytes of memory. Is also
/// `Copy` which makes it super ergonomic to use
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlayerSet([u64; 4]);

impl PlayerSet {
    /// Returns a new, empty player set
    pub fn new() -> Self {
        Default::default()
    }

    /// Return the count of players in the set
    /// ```
    /// use lttcore::PlayerSet;
    ///
    /// let mut set: PlayerSet = Default::default();
    /// assert_eq!(set.count(), 0);
    /// set.add(0);
    /// assert_eq!(set.count(), 1);
    /// set.add(1);
    /// assert_eq!(set.count(), 2);
    /// set.add(1);
    /// assert_eq!(set.count(), 2);
    /// ```
    pub fn count(&self) -> u8 {
        self.0
            .iter()
            .map(|&x| x.count_ones())
            .sum::<u32>()
            .try_into()
            .unwrap()
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
    pub fn contains(&self, player: impl Into<Player>) -> bool {
        let player = player.into();
        (self.0[Self::section(player)] & (1usize << Self::offset(player)) as u64) > 0
    }

    /// If a PlayerSet is empty
    ///
    /// ```
    /// use lttcore::PlayerSet;
    ///
    /// let mut set: PlayerSet = Default::default();
    /// assert!(set.is_empty());
    /// set.add(1);
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0 == [0u64; 4]
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
    pub fn add(&mut self, player: impl Into<Player>) {
        let player = player.into();
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
    pub fn remove(&mut self, player: impl Into<Player>) {
        let player = player.into();
        self.0[Self::section(player)] &= !(1usize << Self::offset(player)) as u64
    }

    fn section(player: Player) -> usize {
        player.as_usize().checked_div(64).unwrap()
    }

    fn offset(player: Player) -> usize {
        player.as_usize() % 64
    }
}

impl From<Player> for PlayerSet {
    fn from(p: Player) -> Self {
        let mut set: Self = Default::default();
        set.add(p);
        set
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
