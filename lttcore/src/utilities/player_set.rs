use crate::Player;
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

/// High performance player set abstraction designd to be O(1) for
/// Add/Remove/Lookup and to only use a fixed 32 bytes of memory. Is also
/// `Copy` which makes it super ergonomic to use
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlayerSet([u64; 4]);

fn section(player: Player) -> usize {
    player.as_usize().checked_div(64).unwrap()
}

fn offset(player: Player) -> usize {
    player.as_usize() % 64
}

impl PlayerSet {
    /// Returns a new, empty player set
    pub fn new() -> Self {
        Default::default()
    }

    /// The same as `new` or `Default::default` but declares intent that the programmer wants this
    /// to be empty
    pub fn empty() -> Self {
        Default::default()
    }

    /// Returns the offset of the player relative to the playerset
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let ps: PlayerSet = [2, 4, 6].into_iter().map(Player::new).collect();
    /// assert_eq!(ps.player_offset(2), Some(0));
    /// assert_eq!(ps.player_offset(4), Some(1));
    /// assert_eq!(ps.player_offset(6), Some(2));
    ///
    /// assert_eq!(ps.player_offset(42), None);
    /// ```
    pub fn player_offset(&self, player: impl Into<Player>) -> Option<u8> {
        let player = player.into();

        self.players()
            .enumerate()
            .filter(|&(_offset, p)| p == player)
            .map(|(offset, _p)| offset)
            .map(|n| n.try_into().unwrap())
            .next()
    }

    /// Return the count of players in the set
    /// ```
    /// use lttcore::PlayerSet;
    ///
    /// let mut set: PlayerSet = Default::default();
    /// assert_eq!(set.count(), 0);
    /// set.insert(0);
    /// assert_eq!(set.count(), 1);
    /// set.insert(1);
    /// assert_eq!(set.count(), 2);
    /// set.insert(1);
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

    /// Alias for `count`
    pub fn len(&self) -> u8 {
        self.count()
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
    /// set.insert(player);
    /// assert!(set.contains(player));
    /// ```
    pub fn contains(&self, player: impl Into<Player>) -> bool {
        let player = player.into();
        (self.0[section(player)] & (1usize << offset(player)) as u64) > 0
    }

    /// If a PlayerSet is empty
    ///
    /// ```
    /// use lttcore::PlayerSet;
    ///
    /// let mut set: PlayerSet = Default::default();
    /// assert!(set.is_empty());
    /// set.insert(1);
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
    /// set.insert(player);
    ///
    /// assert_eq!(
    ///   set.players().collect::<Vec<_>>(),
    ///   vec![player]
    /// );
    /// ```
    pub fn players(&self) -> impl Iterator<Item = Player> {
        self.clone().into_iter()
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
    /// set.insert(player);
    /// assert!(set.contains(player));
    /// ```
    pub fn insert(&mut self, player: impl Into<Player>) {
        let player = player.into();
        self.0[section(player)] |= (1usize << offset(player)) as u64
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
    /// set.insert(player);
    /// assert!(set.contains(player));
    /// set.remove(player);
    /// assert!(!set.contains(player));
    /// ```
    pub fn remove(&mut self, player: impl Into<Player>) {
        let player = player.into();
        self.0[section(player)] &= !(1usize << offset(player)) as u64
    }

    /// The PlayerSet representing the union, i.e. the players that are in self, other, or both
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let set1: PlayerSet = [1,2,3].into_iter().map(Player::new).collect();
    /// let set2: PlayerSet = [2,3,4].into_iter().map(Player::new).collect();
    ///
    /// let result = set1.union(set2);
    /// let expected: PlayerSet = [1, 2, 3, 4].into_iter().map(Player::new).collect();
    /// assert_eq!(result, expected);
    /// ```
    pub fn union(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x | y))
    }

    /// The PlayerSet representing the intersection, i.e. the players that are in self and also in other
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let set1: PlayerSet = [1,2,3].into_iter().map(Player::new).collect();
    /// let set2: PlayerSet = [2,3,4].into_iter().map(Player::new).collect();
    ///
    /// let result = set1.intersection(set2);
    /// let expected: PlayerSet = [2,3].into_iter().map(Player::new).collect();
    /// assert_eq!(result, expected);
    /// ```
    pub fn intersection(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x & y))
    }

    /// The PlayerSet representing the difference, i.e., the players that are in self but not in other.
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let set1: PlayerSet = [1,2,3].into_iter().map(Player::new).collect();
    /// let set2: PlayerSet = [2,3,4].into_iter().map(Player::new).collect();
    ///
    /// let result = set1.difference(set2);
    /// let expected: PlayerSet = [1].into_iter().map(Player::new).collect();
    /// assert_eq!(result, expected);
    /// ```
    pub fn difference(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x & !y))
    }

    /// The PlayerSet representing the symmetric difference, i.e., the players in self or other but
    /// not both
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet};
    ///
    /// let set1: PlayerSet = [1,2,3].into_iter().map(Player::new).collect();
    /// let set2: PlayerSet = [2,3,4].into_iter().map(Player::new).collect();
    ///
    /// let result = set1.symmetric_difference(set2);
    /// let expected: PlayerSet = [1, 4].into_iter().map(Player::new).collect();
    /// assert_eq!(result, expected);
    ///
    pub fn symmetric_difference(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x ^ y))
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter {
    index: Option<u8>,
    set: PlayerSet,
}

impl IntoIterator for PlayerSet {
    type Item = Player;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            index: Some(0),
            set: self,
        }
    }
}

impl Iterator for IntoIter {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index.take() {
            None => None,
            Some(idx) => {
                for i in idx..=u8::MAX {
                    if self.set.contains(i) {
                        self.index = i.checked_add(1);
                        return Some(i.into());
                    }
                }

                None
            }
        }
    }
}

impl From<Player> for PlayerSet {
    fn from(p: Player) -> Self {
        Some(p).into_iter().collect()
    }
}

impl FromIterator<Player> for PlayerSet {
    fn from_iter<I: IntoIterator<Item = Player>>(iter: I) -> Self {
        let mut set = PlayerSet::new();

        for player in iter {
            set.insert(player);
        }

        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iter_for_player_set() {
        let set: PlayerSet = [Player::new(0), Player::new(1)].into_iter().collect();
        assert!(set.contains(0));
        assert!(set.contains(1));
        assert!(!set.contains(2));
    }

    #[test]
    fn test_into_iter_for_player_set() {
        let players: Vec<Player> = [0, 1, 2, u8::MAX].into_iter().map(Player::new).collect();
        let player_set: PlayerSet = players.iter().cloned().collect();
        let mut result: Vec<Player> = Default::default();

        for player in player_set {
            result.push(player);
        }

        assert_eq!(result, players);
    }

    #[test]
    fn test_set_works_for_all_players() {
        for player in Player::all() {
            let mut set = PlayerSet::new();
            assert!(!set.contains(player));
            set.insert(player);
            assert!(set.contains(player));
            set.remove(player);
            assert!(!set.contains(player));
        }

        let mut set = PlayerSet::new();

        for player in Player::all() {
            set.insert(player);
        }

        for player in Player::all() {
            assert!(set.contains(player));
        }
    }
}
