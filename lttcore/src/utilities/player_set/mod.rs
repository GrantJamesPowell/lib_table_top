use super::PlayerIndexedData;
use crate::common::direction::LeftOrRight::{self, Left, Right};
use crate::play::{NumberOfPlayers, Player};
use crate::utilities::BitArray256;
use core::ops::{Range, RangeInclusive};
use itertools::{EitherOrBoth, Itertools};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::iter::FromIterator;

/// Helper macro to define [`PlayerSet`] literals
///
/// ```
/// use lttcore::{player_set, play::Player, utilities::PlayerSet};
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
        [$($expr,)*].into_iter()
            .map($crate::play::Player::new)
            .collect::<$crate::utilities::PlayerSet>()
    };
}

/// A set of [`Player`](crate::play::Player)
///
/// # Design goals
///
/// * `O(1)-ish` Add/Remove/Lookup
/// * Serializes nicely
/// * Avoids allocating if all players are under 256
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerSet(SmallVec<[(u32, BitArray256); 1]>);

fn join_with(
    PlayerSet(blocks_a): PlayerSet,
    PlayerSet(blocks_b): PlayerSet,
    f: impl Fn(BitArray256, BitArray256) -> BitArray256,
) -> PlayerSet {
    let mut output = SmallVec::with_capacity(blocks_a.len().max(blocks_b.len()));

    let iter = blocks_a
        .into_iter()
        .merge_join_by(blocks_b.into_iter(), |(a, _), (b, _)| a.cmp(b))
        .map(|x| match x {
            EitherOrBoth::Left(x) | EitherOrBoth::Right(x) => x,
            EitherOrBoth::Both((i, x), (_, y)) => (i, f(x, y)),
        });

    output.extend(iter);
    PlayerSet(output)
}

fn block_and_offest_for_player(player: Player) -> (u32, u8) {
    // Each `BitArray256` holds 245 players, we assign a block based on how far from the start the
    // player is and then calculate how deep in the 256 bit block the player is
    let num = u32::from(player);
    (
        num / 256,
        (num % 256).try_into().expect("we just modulo'd by 256"),
    )
}

impl PlayerSet {
    /// Returns a new, empty player set
    pub fn new() -> Self {
        Self::default()
    }

    /// The same as `new` or `Default::default` but declares intent that the programmer wants this
    /// to be empty
    pub fn empty() -> Self {
        Self::default()
    }

    /// Iterate over the players in the set
    pub fn iter(&self) -> Iter<'_> {
        self.iter_starting_from_player(0)
    }

    /// Returns the offset of the player relative to the playerset
    ///
    /// Note: [`PlayerSet`] is iterated in increasing order starting with [`Player`] `0`
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let ps = player_set![2, 4, 6, u8::MAX];
    /// assert_eq!(ps.player_offset(2), Some(0));
    /// assert_eq!(ps.player_offset(4), Some(1));
    /// assert_eq!(ps.player_offset(6), Some(2));
    /// assert_eq!(ps.player_offset(u8::MAX), Some(3));
    ///
    /// // When a Player isn't in the set
    ///
    /// assert_eq!(ps.player_offset(42), None);
    /// ```
    pub fn player_offset(&self, player: impl Into<Player>) -> Option<u32> {
        let (block, offset) = block_and_offest_for_player(player.into());

        let (_idx, bit_array) = self.0.get(block as usize)?;
        let block_offset = bit_array.num_offset(offset).map(u32::from)?;
        let preceding_count = self
            .0
            .iter()
            .map_while(|(idx, bit_array)| (*idx < block).then(|| bit_array.count() as u32))
            .sum::<u32>();

        Some(preceding_count + block_offset)
    }

    /// Return the count of players in the set
    ///
    /// ```
    /// use lttcore::utilities::PlayerSet;
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
    pub fn count(&self) -> u32 {
        self.0
            .iter()
            .map(|(_idx, bit_array)| bit_array.count() as u32)
            .sum::<u32>()
    }

    /// Alias for [`PlayerSet::count`]
    pub fn len(&self) -> u32 {
        self.count()
    }

    /// Returns [`PlayerIndexedData`] using [`PlayerIndexedData::init_with`]
    ///
    /// ```
    /// use lttcore::player_set;
    /// use lttcore::utilities::PlayerIndexedData as PID;
    ///
    /// let ps = player_set![0, 1, 2];
    /// let data: PID<u64> = ps.player_indexed_data(|player| player.into());
    /// assert_eq!(data[0], 0);
    /// assert_eq!(data[1], 1);
    /// assert_eq!(data[2], 2);
    /// ```
    pub fn player_indexed_data<T>(&self, func: impl FnMut(Player) -> T) -> PlayerIndexedData<T> {
        self.clone().into_player_indexed_data(func)
    }

    /// Same as [`PlayerSet::player_indexed_data`] but consumes `Self` to use in the indexed data
    pub fn into_player_indexed_data<T>(
        self,
        func: impl FnMut(Player) -> T,
    ) -> PlayerIndexedData<T> {
        PlayerIndexedData::init_with(self, func)
    }

    /// Returns whether the [`Player`] is in [`PlayerSet`]
    ///
    /// ```
    /// use lttcore::{play::Player, utilities::PlayerSet};
    ///
    /// let mut set = PlayerSet::new();
    /// let player: Player = 1.into();
    ///
    /// assert!(!set.contains(player));
    /// set.insert(player);
    /// assert!(set.contains(player));
    /// ```
    pub fn contains(&self, player: impl Into<Player>) -> bool {
        let (block, offset) = block_and_offest_for_player(player.into());

        if let Ok(idx) = self.0.binary_search_by_key(&block, |(i, _)| *i) {
            let (_idx, bit_array) = &self.0[idx];
            bit_array.contains(offset)
        } else {
            false
        }
    }

    /// If a [`PlayerSet`] is empty
    ///
    /// ```
    /// use lttcore::utilities::PlayerSet;
    ///
    /// let mut set: PlayerSet = Default::default();
    /// assert!(set.is_empty());
    /// set.insert(1);
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterator over players in the set
    ///
    /// ```
    /// use lttcore::{play::Player, utilities::PlayerSet};
    ///
    /// let mut set = PlayerSet::new();
    ///
    /// assert!(set.players().next().is_none());
    ///
    /// set.insert(1);
    ///
    /// assert_eq!(
    ///   set.players().collect::<Vec<_>>(),
    ///   vec![Player::new(1)]
    /// );
    /// ```
    pub fn players(&self) -> impl Iterator<Item = Player> + '_ {
        self.iter()
    }

    /// Adds the [`Player`] to the set, is a noop if [`Player`] is already in set
    /// returns the player offset
    ///
    /// ```
    /// use lttcore::{player_set, play::Player};
    ///
    /// let mut set = player_set![];
    /// assert!(!set.contains(1));
    /// set.insert(1);
    /// assert!(set.contains(1));
    /// ```
    pub fn insert(&mut self, player: impl Into<Player>) {
        let (block, offset) = block_and_offest_for_player(player.into());
        match self.0.binary_search_by_key(&block, |(i, _)| *i) {
            Ok(idx) => {
                let (_idx, bit_array) = &mut self.0[idx];
                bit_array.insert(offset);
            }
            Err(idx) => {
                self.0.insert(idx, (block, BitArray256::from(offset)));
            }
        }
    }

    /// Remove a [`Player`] from the set, is a noop if [`Player`] is not in the set
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
        let (block, offset) = block_and_offest_for_player(player.into());

        if let Ok(idx) = self.0.binary_search_by_key(&block, |(i, _)| *i) {
            let (_idx, bit_array) = &mut self.0[idx];
            bit_array.remove(offset);
            if bit_array.is_empty() {
                self.0.remove(idx);
            }
        }
    }

    /// The [`PlayerSet`] representing the union, i.e. the players that are in self, other, or both
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1,2,3];
    /// let set2 = player_set![2,3,4];
    ///
    /// assert_eq!(set1.union(set2), player_set![1, 2, 3, 4]);
    /// ```
    pub fn union(self, other: Self) -> Self {
        join_with(self, other, |x, y| x.union(y))
    }

    /// The [`PlayerSet`] representing the intersection, i.e. the players that are in self and also in other
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1,2,3];
    /// let set2 = player_set![2,3,4];
    ///
    /// assert_eq!(set1.intersection(set2), player_set![2, 3]);
    /// ```
    pub fn intersection(self, other: Self) -> Self {
        join_with(self, other, |x, y| x.intersection(y))
    }

    /// The [`PlayerSet`] representing the difference, i.e., the players that are in self but not in other.
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1,2,3];
    /// let set2 = player_set![2,3,4];
    ///
    /// assert_eq!(set1.difference(set2), player_set![1]);
    /// ```
    pub fn difference(self, other: Self) -> Self {
        join_with(self, other, |x, y| x.difference(y))
    }

    /// The [`PlayerSet`] representing the symmetric difference, i.e., the players in self or other but
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
    pub fn symmetric_difference(self, other: Self) -> Self {
        join_with(self, other, |x, y| x.symmetric_difference(y))
    }

    /// Returns the next [`Player`] to the right of the given [`Player`], wrapping around if required
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
        self.iter_starting_from_player(player.next()).next()
    }

    /// Returns the next player to the left of the given [`Player`], wrapping around if required
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
        self.iter_starting_from_player(player.previous())
            .next_back()
    }

    /// see [`PlayerSet::next_player_left`] and [`PlayerSet::next_player_right`]
    pub fn next_player_in_direction(
        &self,
        player: impl Into<Player>,
        direction: LeftOrRight,
    ) -> Option<Player> {
        match direction {
            Right => self.next_player_right(player),
            Left => self.next_player_left(player),
        }
    }

    fn iter_starting_from_player(&self, player: impl Into<Player>) -> Iter<'_> {
        let player = player.into();

        let to_end = u8::from(player)..=u8::MAX;
        let from_start = 0..u8::from(player);

        Iter {
            set: Cow::Borrowed(&self),
            to_end,
            from_start,
        }
    }

    fn into_iter_starting_from_player(self, player: impl Into<Player>) -> Iter<'static> {
        let player = player.into();

        let to_end = u8::from(player)..=u8::MAX;
        let from_start = 0..u8::from(player);

        Iter {
            set: Cow::Owned(self),
            to_end,
            from_start,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
    set: Cow<'a, PlayerSet>,
    to_end: RangeInclusive<u8>,
    from_start: Range<u8>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        for player in self.to_end.by_ref() {
            if self.set.contains(player) {
                return Some(player.into());
            }
        }

        for player in self.from_start.by_ref() {
            if self.set.contains(player) {
                return Some(player.into());
            }
        }

        None
    }
}

impl<'a> std::iter::DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(player) = self.from_start.next_back() {
            if self.set.contains(player) {
                return Some(player.into());
            }
        }

        while let Some(player) = self.to_end.next_back() {
            if self.set.contains(player) {
                return Some(player.into());
            }
        }

        None
    }
}

impl IntoIterator for PlayerSet {
    type Item = Player;
    type IntoIter = Iter<'static>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter_starting_from_player(0)
    }
}

impl<'a> std::iter::FusedIterator for Iter<'a> {}

impl From<Player> for PlayerSet {
    fn from(p: Player) -> Self {
        Some(p).into_iter().collect()
    }
}

impl From<NumberOfPlayers> for PlayerSet {
    fn from(number_of_players: NumberOfPlayers) -> Self {
        number_of_players.player_set()
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
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_from_iter_for_player_set() {
        let set: PlayerSet = [Player::new(0), Player::new(1)].into_iter().collect();
        assert!(set.contains(0));
        assert!(set.contains(1));
        assert!(!set.contains(2));
    }

    #[test]
    fn test_creating_a_full_player_set() {
        let ps: PlayerSet = (0..=255).map(Player::new).collect();

        for player in ps.iter() {
            assert_eq!(ps.player_offset(player), Some(u32::from(player)))
        }
    }

    #[test]
    fn test_into_iter_for_player_set() {
        let players: Vec<Player> = [0, 1, 2, u8::MAX].into_iter().map(Player::new).collect();
        let player_set: PlayerSet = players.iter().copied().collect();
        let mut result: Vec<Player> = Vec::new();

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

        assert_eq!(Some(1.into()), iter.next());
        assert_eq!(Some(2.into()), iter.next());
        assert_eq!(Some(u8::MAX.into()), iter.next_back());
        assert_eq!(Some(10.into()), iter.next_back());
        assert_eq!(Some(9.into()), iter.next_back());
        assert_eq!(Some(3.into()), iter.next());
        assert_eq!(Some(8.into()), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }

    #[test]
    fn test_set_works_for_all_players() {
        for player in (0..=255).map(Player::new) {
            let mut set = PlayerSet::new();
            assert!(!set.contains(player));
            set.insert(player);
            assert!(set.contains(player));
            set.remove(player);
            assert_eq!(set.player_offset(player), None);
            assert!(!set.contains(player));
        }

        let mut set = PlayerSet::new();

        for player in (0..=255).map(Player::new) {
            set.insert(player);
        }

        for player in (0..=255).map(Player::new) {
            assert!(set.contains(player));
            assert_eq!(set.player_offset(player), Some(u32::from(player)));
        }
    }

    #[test]
    fn adding_and_removing_is_the_same_as_empty() {
        let mut set = PlayerSet::new();
        set.insert(1);
        set.remove(1);
        assert_eq!(set, PlayerSet::empty());

        let mut hasher = DefaultHasher::new();
        set.hash(&mut hasher);
        let roundtrip = hasher.finish();
        let mut hasher = DefaultHasher::new();
        PlayerSet::empty().hash(&mut hasher);
        let from_empty = hasher.finish();
        assert_eq!(roundtrip, from_empty);
    }
}
