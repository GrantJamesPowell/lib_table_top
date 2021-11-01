use crate::common::direction::LeftOrRight::{self, *};
use crate::Player;
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

/// Helper function to define `PlayerSet` literals
///
/// ```
/// use lttcore::{player_set, Player, PlayerSet};
///
/// let my_empty_ps = player_set![];
/// assert_eq!(my_empty_ps, PlayerSet::new());
///
/// let expected: PlayerSet = [4,5,6].into_iter().map(Player::new).collect();
/// assert_eq!(expected, player_set![4, 5, 6]);
///
/// let ps = player_set![{ 1 + 1 }, { 4 * 5 }, u8::MAX];
/// assert!(ps.contains(2));
/// assert!(ps.contains(20));
/// assert!(ps.contains(255));
/// ```
#[macro_export]
macro_rules! player_set {
    ( $( $expr:expr ),* ) => {
        [$($expr,)*].into_iter().map(::lttcore::Player::new).collect::<::lttcore::PlayerSet>()
    };
}

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
    /// Note: PlayerSet is iterated in increasing order starting with Player 0
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let ps = player_set![2, 4, 6];
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
    /// use lttcore::player_set;
    ///
    /// let mut set = player_set![1];
    ///
    /// assert!(set.contains(1));
    /// set.remove(1);
    /// assert!(!set.contains(1));
    /// ```
    pub fn remove(&mut self, player: impl Into<Player>) {
        let player = player.into();
        self.0[section(player)] &= !(1usize << offset(player)) as u64
    }

    /// The PlayerSet representing the union, i.e. the players that are in self, other, or both
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1,2,3];
    /// let set2 = player_set![2,3,4];
    ///
    /// assert_eq!(set1.union(set2), player_set![1, 2, 3, 4]);
    /// ```
    pub fn union(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x | y))
    }

    /// The PlayerSet representing the intersection, i.e. the players that are in self and also in other
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1,2,3];
    /// let set2 = player_set![2,3,4];
    ///
    /// assert_eq!(set1.intersection(set2), player_set![2, 3]);
    /// ```
    pub fn intersection(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x & y))
    }

    /// The PlayerSet representing the difference, i.e., the players that are in self but not in other.
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1,2,3];
    /// let set2 = player_set![2,3,4];
    ///
    /// assert_eq!(set1.difference(set2), player_set![1]);
    /// ```
    pub fn difference(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x & !y))
    }

    /// The PlayerSet representing the symmetric difference, i.e., the players in self or other but
    /// not both
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1,2,3];
    /// let set2 = player_set![2,3,4];
    ///
    /// assert_eq!(set1.symmetric_difference(set2), player_set![1, 4])
    /// ```
    pub fn symmetric_difference(&self, other: Self) -> Self {
        Self(self.0.zip(other.0).map(|(x, y)| x ^ y))
    }

    /// Returns the next player to the right of the given player, wrapping around if required
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set = player_set![10, 20, 30];
    /// assert_eq!(set.next_player_right(20), Some(30.into()));
    /// assert_eq!(set.next_player_right(30), Some(10.into()));
    /// assert_eq!(set.next_player_right(10), Some(20.into()));
    ///
    /// // It the player isn't in the set it will find the next player right as if the player was
    ///
    /// assert_eq!(set.next_player_right(25), Some(30.into()));
    ///
    /// // A PlayerSet with only one player will yield that player
    ///
    /// let set = player_set![42];
    /// assert_eq!(set.next_player_right(0), Some(42.into()));
    /// assert_eq!(set.next_player_right(42), Some(42.into()));
    /// assert_eq!(set.next_player_right(42), Some(42.into()));
    /// assert_eq!(set.next_player_right(u8::MAX), Some(42.into()));
    ///
    /// // An empty set will yield `None`
    ///
    /// let set = player_set![];
    /// assert!(set.next_player_right(0).is_none());
    /// ```
    ///
    pub fn next_player_right(&self, player: impl Into<Player>) -> Option<Player> {
        let player = player.into();
        let mut iter = IntoIter {
            start: player.as_u8().wrapping_add(1),
            end: player.as_u8(),
            set: *self,
        };
        iter.next()
    }

    /// Returns the next player to the left of the given player, wrapping around if required
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set = player_set![10, 20, 30];
    /// assert_eq!(set.next_player_left(20), Some(10.into()));
    /// assert_eq!(set.next_player_left(30), Some(20.into()));
    /// assert_eq!(set.next_player_left(10), Some(30.into()));
    ///
    /// // It the player isn't in the set it will find the next player right as if the player was
    ///
    /// assert_eq!(set.next_player_left(25), Some(20.into()));
    ///
    /// // A PlayerSet with only one player will yield that player
    ///
    /// let set = player_set![42];
    /// assert_eq!(set.next_player_left(0), Some(42.into()));
    /// assert_eq!(set.next_player_left(42), Some(42.into()));
    /// assert_eq!(set.next_player_left(42), Some(42.into()));
    /// assert_eq!(set.next_player_left(u8::MAX), Some(42.into()));
    ///
    /// // An empty set will yield `None`
    ///
    /// let set = player_set![];
    /// assert!(set.next_player_left(0).is_none());
    /// ```
    pub fn next_player_left(&self, player: impl Into<Player>) -> Option<Player> {
        let player = player.into();
        let mut iter = IntoIter {
            start: player.as_u8(),
            end: player.as_u8().wrapping_sub(1),
            set: *self,
        };
        iter.next_back()
    }

    /// Convenience wrapper around `next_player_left` and `next_player_right`
    pub fn next_player_in_direction(
        &self,
        player: impl Into<Player>,
        direction: LeftOrRight,
    ) -> Option<Player> {
        match direction {
            Left => self.next_player_left(player),
            Right => self.next_player_left(player),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter {
    start: u8,
    end: u8,
    set: PlayerSet,
}

impl IntoIterator for PlayerSet {
    type Item = Player;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            set: self,
            start: 0,
            end: u8::MAX,
        }
    }
}

impl Iterator for IntoIter {
    type Item = Player;

    // It's not pretty... but it works
    fn next(&mut self) -> Option<Self::Item> {
        while self.start != self.end {
            if self.set.contains(self.start) {
                let player: Player = self.start.into();
                self.start = self.start.wrapping_add(1);
                return Some(player);
            } else {
                self.start = self.start.wrapping_add(1);
                if self.start == self.end && self.set.contains(self.end) {
                    return Some(self.end.into());
                }
            }
        }

        return None;
    }
}

impl std::iter::DoubleEndedIterator for IntoIter {
    // It's not pretty... but it works
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.end != self.start {
            if self.set.contains(self.end) {
                let player: Player = self.end.into();
                self.end = self.end.wrapping_sub(1);
                return Some(player);
            } else {
                self.end = self.end.wrapping_sub(1);
                if self.end == self.start && self.set.contains(self.start) {
                    return Some(self.start.into());
                }
            }
        }

        return None;
    }
}

impl std::iter::FusedIterator for IntoIter {}

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
    fn test_next_and_next_back_for_player_set_into_iter() {
        let set: PlayerSet = [1, 2, 3, 8, 9, 10, u8::MAX]
            .into_iter()
            .map(Player::new)
            .collect();

        let mut iter = set.into_iter();

        assert_eq!(iter.next(), Some(1.into()));
        assert_eq!(iter.next(), Some(2.into()));
        assert_eq!(iter.next_back(), Some(u8::MAX.into()));
        assert_eq!(iter.next_back(), Some(10.into()));
        assert_eq!(iter.next_back(), Some(9.into()));
        assert_eq!(iter.next(), Some(3.into()));
        assert_eq!(iter.next(), Some(8.into()));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
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
