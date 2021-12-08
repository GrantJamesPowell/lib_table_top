use super::PlayerIndexedData;
use crate::play::{NumberOfPlayers, Player};
use crate::utilities::BitArray256;
use itertools::{EitherOrBoth, Itertools};
use smallvec::SmallVec;
use std::iter::FromIterator;

mod serialize_and_deserialize;

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
/// let ps = player_set![{ 1 + 1 }, { 4 * 5 }, u32::MAX];
/// assert!(ps.contains(2));
/// assert!(ps.contains(20));
/// assert!(ps.contains(u32::MAX));
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
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PlayerSet(SmallVec<[(u32, BitArray256); 1]>);

fn join_with<'a>(
    PlayerSet(blocks_a): &'a PlayerSet,
    PlayerSet(blocks_b): &'a PlayerSet,
    f: impl Fn(BitArray256, BitArray256) -> BitArray256 + 'static,
) -> impl Iterator<Item = Player> + 'a {
    Itertools::merge_join_by(blocks_a.iter(), blocks_b.iter(), |(a, _), (b, _)| a.cmp(b))
        .map(move |x| match x {
            EitherOrBoth::Left((i, x)) => (i, f(*x, BitArray256::empty())),
            EitherOrBoth::Right((i, x)) => (i, f(BitArray256::empty(), *x)),
            EitherOrBoth::Both((i, x), (_, y)) => (i, f(*x, *y)),
        })
        .flat_map(|(block, bit_array)| {
            bit_array
                .into_iter()
                .map(move |num| player_for_block_and_offset(*block, num))
        })
}

fn block_and_offest_for_player(player: Player) -> (u32, u8) {
    // Each `BitArray256` holds 256 players, we assign a block based on how far from the start the
    // player is and then calculate how deep in the 256 bit block the player is
    let num = u32::from(player);
    (
        num / 256,
        (num % 256).try_into().expect("we just modulo'd by 256"),
    )
}

fn player_for_block_and_offset(block: u32, offset: u8) -> Player {
    Player::from((block * 256) + (offset as u32))
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

    /// Returns the [`Player`] with the lowest id in the set
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let ps = player_set![4,5,6];
    /// assert_eq!(ps.first(), Some(4.into()));
    ///
    /// let empty = player_set![];
    /// assert_eq!(empty.first(), None);
    /// ```
    pub fn first(&self) -> Option<Player> {
        self.iter().next()
    }

    /// Returns the [`Player`] with the highest id in the set
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let ps = player_set![4,5,6];
    /// assert_eq!(ps.last(), Some(6.into()));
    ///
    /// let empty = player_set![];
    /// assert_eq!(empty.last(), None);
    /// ```
    pub fn last(&self) -> Option<Player> {
        self.iter().next_back()
    }

    /// Pop the [`Player`] with the lowest id
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let mut ps = player_set![4000, 5000, 6000];
    /// assert_eq!(ps.pop_first(), Some(4000.into()));
    /// assert_eq!(ps, player_set![5000, 6000]);
    /// ```
    pub fn pop_first(&mut self) -> Option<Player> {
        self.first().map(|player| {
            self.remove(player);
            player
        })
    }

    /// Pop the [`Player`] with the lowest id
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let mut ps = player_set![4000, 5000, 6000];
    /// assert_eq!(ps.pop_last(), Some(6000.into()));
    /// assert_eq!(ps, player_set![4000, 5000]);
    /// ```
    pub fn pop_last(&mut self) -> Option<Player> {
        self.last().map(|player| {
            self.remove(player);
            player
        })
    }

    /// Iterate over the players in the set
    pub fn iter(&self) -> impl Iterator<Item = Player> + DoubleEndedIterator + '_ {
        self.0.iter().flat_map(|(block, bit_array)| {
            bit_array
                .iter()
                .map(move |offset| player_for_block_and_offset(*block, offset))
        })
    }

    /// Returns the offset of the player relative to the playerset
    ///
    /// Note: [`PlayerSet`] is iterated in increasing order starting with [`Player`] `0`
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let ps = player_set![2, 4, 6, 3000, u32::MAX];
    /// assert_eq!(ps.player_offset(2), Some(0));
    /// assert_eq!(ps.player_offset(4), Some(1));
    /// assert_eq!(ps.player_offset(6), Some(2));
    /// assert_eq!(ps.player_offset(3000), Some(3));
    /// assert_eq!(ps.player_offset(u32::MAX), Some(4));
    ///
    /// // When a Player isn't in the set
    ///
    /// assert_eq!(ps.player_offset(42), None);
    /// ```
    pub fn player_offset(&self, player: impl Into<Player>) -> Option<u32> {
        let player = player.into();
        let (block, offset) = block_and_offest_for_player(player);

        let idx = self.0.binary_search_by_key(&block, |(i, _)| *i).ok()?;
        let (_block, bit_array) = self.0[idx];

        let offset_within_block = bit_array.num_offset(offset).map(u32::from)?;
        let preceding_count = self
            .0
            .iter()
            .map_while(|(idx, bit_array)| (*idx < block).then(|| bit_array.count() as u32))
            .sum::<u32>();

        Some(preceding_count + offset_within_block)
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
    /// set.insert(1000);
    /// assert_eq!(set.count(), 2);
    ///
    /// // The second time inserting the same number is a no-op
    /// set.insert(1);
    /// assert_eq!(set.count(), 3);
    /// set.insert(1);
    /// assert_eq!(set.count(), 3);
    /// ```
    pub fn count(&self) -> u32 {
        self.0
            .iter()
            .map(|(_idx, bit_array)| bit_array.count() as u32)
            .sum::<u32>()
    }

    /// Alias for [`PlayerSet::count`]
    pub fn len(&self) -> usize {
        self.count().try_into().unwrap()
    }

    /// Returns [`PlayerIndexedData`] using [`PlayerIndexedData::init_with`]
    ///
    /// ```
    /// use lttcore::player_set;
    /// use lttcore::utilities::PlayerIndexedData as PID;
    ///
    /// let ps = player_set![0, 1, 2, 1000];
    /// let data: PID<u64> = ps.player_indexed_data(|player| player.into());
    /// assert_eq!(data[0], 0);
    /// assert_eq!(data[1], 1);
    /// assert_eq!(data[2], 2);
    /// assert_eq!(data[1000], 1000);
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
    /// use lttcore::utilities::PlayerSet;
    ///
    /// let mut set = PlayerSet::new();
    ///
    /// assert!(!set.contains(1));
    /// set.insert(1);
    /// assert!(set.contains(1));
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

    /// Adds the [`Player`] to the set, is a noop if [`Player`] is already in set
    /// returns the player offset
    ///
    /// ```
    /// use lttcore::{player_set, play::Player};
    ///
    /// let mut set = player_set![];
    ///
    /// assert!(!set.contains(1));
    /// set.insert(1);
    /// assert!(set.contains(1));
    ///
    /// assert!(!set.contains(1000));
    /// set.insert(1000);
    /// assert!(set.contains(1000));
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
    /// let set1 = player_set![1, 2, 3, 1000, 1001];
    /// let set2 = player_set![2, 3, 4, 1000, 2000];
    ///
    /// assert!(set1.union(&set2).eq(player_set![1, 2, 3, 4, 1000, 1001, 2000]));
    /// ```
    pub fn union<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = Player> + 'a {
        join_with(self, other, |x, y| x.union(y))
    }

    /// The [`PlayerSet`] representing the intersection, i.e. the players that are in self and also in other
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1, 2, 3, 1000, 1001];
    /// let set2 = player_set![2, 3, 4, 1000, 2000];
    ///
    /// assert!(set1.intersection(&set2).eq(player_set![2, 3, 1000]));
    /// ```
    pub fn intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = Player> + 'a {
        join_with(self, other, |x, y| x.intersection(y))
    }

    /// The [`PlayerSet`] representing the difference, i.e., the players that are in self but not in other.
    ///
    /// ```
    /// use lttcore::player_set;
    /// use lttcore::utilities::PlayerSet;
    ///
    /// let set1 = player_set![1, 2, 3, 1000, 1001];
    /// let set2 = player_set![2, 3, 4, 1000, 2000];
    ///
    /// assert_eq!(set1.difference(&set2).collect::<PlayerSet>(), player_set![1, 1001]);
    ///
    /// assert!(set1.difference(&set2).eq(player_set![1, 1001]));
    /// ```
    pub fn difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = Player> + 'a {
        join_with(self, other, |x, y| x.difference(y))
    }

    /// The [`PlayerSet`] representing the symmetric difference, i.e., the players in self or other but
    /// not both
    ///
    /// ```
    /// use lttcore::player_set;
    ///
    /// let set1 = player_set![1, 2, 3, 1000, 1001];
    /// let set2 = player_set![2, 3, 4, 1000, 2000];
    ///
    /// assert!(set1.symmetric_difference(&set2).eq(player_set![1, 4, 1001, 2000]))
    /// ```
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = Player> + 'a {
        join_with(self, other, |x, y| x.symmetric_difference(y))
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter {
    remaining_blocks: smallvec::IntoIter<[(u32, BitArray256); 1]>,
    front_cursor: Option<(u32, BitArray256)>,
    back_cursor: Option<(u32, BitArray256)>,
}

impl Iterator for IntoIter {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((block, bit_array)) = self.front_cursor.as_mut() {
                if let Some(num) = bit_array.pop_lowest() {
                    return Some(player_for_block_and_offset(*block, num));
                }
            }

            if let Some(next) = self.remaining_blocks.next() {
                self.front_cursor = Some(next);
            } else {
                break;
            }
        }

        self.back_cursor.as_mut().and_then(|(block, bit_array)| {
            bit_array
                .pop_lowest()
                .map(|num| player_for_block_and_offset(*block, num))
        })
    }
}

impl std::iter::DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((block, bit_array)) = self.back_cursor.as_mut() {
                if let Some(num) = bit_array.pop_highest() {
                    return Some(player_for_block_and_offset(*block, num));
                }
            }

            if let Some(next) = self.remaining_blocks.next_back() {
                self.back_cursor = Some(next);
            } else {
                break;
            }
        }

        self.front_cursor.as_mut().and_then(|(block, bit_array)| {
            bit_array
                .pop_highest()
                .map(|num| player_for_block_and_offset(*block, num))
        })
    }
}

impl IntoIterator for PlayerSet {
    type Item = Player;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            remaining_blocks: self.0.into_iter(),
            front_cursor: None,
            back_cursor: None,
        }
    }
}

impl<'a> std::iter::FusedIterator for IntoIter {}

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

impl<T: Into<Player>> FromIterator<T> for PlayerSet {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
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
        let set: PlayerSet = [0, 1, 1000, 10000, u32::MAX].into_iter().collect();
        assert!(set.contains(0));
        assert!(set.contains(1));
        assert!(set.contains(1000));
        assert!(set.contains(10000));
        assert!(set.contains(u32::MAX));

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
        let player_set = player_set![0, 1, 2, u32::MAX];
        let mut result: Vec<Player> = Vec::new();

        for player in player_set.clone() {
            result.push(player);
        }

        assert_eq!(result, player_set.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_next_and_next_back_for_player_set_iter() {
        let set: PlayerSet = player_set![1, 2, 3, 8, 9, 10, u32::MAX];
        let mut iter = set.iter();

        assert_eq!(Some(1.into()), iter.next());
        assert_eq!(Some(2.into()), iter.next());
        assert_eq!(Some(u32::MAX.into()), iter.next_back());
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
    fn test_next_and_next_back_for_player_set_into_iter() {
        let set: PlayerSet = [1, 2, 3, 8, 9, 10, u32::MAX]
            .into_iter()
            .map(Player::new)
            .collect();

        let mut iter = set.into_iter();

        assert_eq!(Some(1.into()), iter.next());
        assert_eq!(Some(2.into()), iter.next());
        assert_eq!(Some(u32::MAX.into()), iter.next_back());
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
        for player in 0..10000 {
            let mut set = PlayerSet::new();
            assert!(!set.contains(player));
            set.insert(player);
            assert!(set.contains(player));
            set.remove(player);
            assert_eq!(set.player_offset(player), None);
            assert!(!set.contains(player));
        }

        let mut set = PlayerSet::new();

        for player in 0..10000 {
            set.insert(player);
        }

        for player in 0..10000 {
            assert!(set.contains(player));
            assert_eq!(set.player_offset(player), Some(u32::from(player)));
        }
    }

    #[test]
    fn adding_and_removing_is_the_same_as_empty() {
        let mut set = PlayerSet::new();

        for i in 0..1000 {
            set.insert(i);
            set.remove(i);
        }
        assert_eq!(set, PlayerSet::empty());

        let mut hasher = DefaultHasher::new();
        set.hash(&mut hasher);
        let roundtrip = hasher.finish();
        let mut hasher = DefaultHasher::new();
        PlayerSet::empty().hash(&mut hasher);
        let from_empty = hasher.finish();
        assert_eq!(roundtrip, from_empty);
    }

    #[test]
    fn block_offset_player_conversions() {
        for i in 1..1000 {
            let player = Player::from(i);
            let (block, offset) = block_and_offest_for_player(player);
            let roundtripped_player = player_for_block_and_offset(block, offset);
            assert_eq!(player, roundtripped_player)
        }
    }

    #[test]
    fn join_with_works_with_player_set_with_different_numbers_of_blocks() {
        let ps1 = player_set![1, 2, 3, 1000];
        let ps2 = player_set![1000, 2000, 3000, 4000];

        assert_eq!(
            join_with(&ps1, &ps2, |x, y| x.intersection(y)).collect::<PlayerSet>(),
            player_set![1000]
        );

        assert_eq!(
            join_with(&ps1, &ps2, |x, y| x.intersection(y)).collect::<PlayerSet>(),
            join_with(&ps2, &ps1, |x, y| x.intersection(y)).collect::<PlayerSet>()
        );
    }
}
