use crate::{play::Player, utilities::PlayerSet};
use smallvec::SmallVec;
use std::hash::Hash;

use super::PlayerItemCollector;

mod serialize_and_deserialize;

/// A mapping from [`Player`] and some item `T`
///
/// # Implementation notes:
///
/// This uses a [`PlayerSet`] and [`SmallVec`] under the hood to represent the mapping, making
/// lookups an array index and optimizing for the case where the number of items <= 4 by
/// being able to place 4 items directly on the stack
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerIndexedData<T> {
    players: PlayerSet,
    data: SmallVec<[T; 4]>,
}

impl<T> Default for PlayerIndexedData<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PlayerIndexedData<T> {
    /// Returns a new, empty [`PlayerIndexedData`]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// alias for [`PlayerIndexedData::new`]
    pub fn empty() -> Self {
        Self::with_capacity(0)
    }

    /// Map the [`PlayerIndexedData`]
    ///
    /// A monad is *just* a burrito https://blog.plover.com/prog/burritos.html
    pub fn map<U>(&self, mut func: impl FnMut(&T) -> U) -> PlayerIndexedData<U> {
        self.iter()
            .map(|(player, item)| (player, func(item)))
            .collect()
    }

    /// Returns if the `PlayerIndexedData` is empty
    ///
    /// ```
    /// use lttcore::utilities::PlayerIndexedData;
    ///
    /// let mut pid = PlayerIndexedData::new();
    /// assert!(pid.is_empty());
    ///
    /// pid.insert(1, "foo");
    /// assert!(!pid.is_empty());
    ///
    ///
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Init the player indexed data from a playerset and function
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let ps = player_set![1, 2, 3];
    ///
    /// let data = PlayerIndexedData::init_with(ps, |player| u8::from(player));
    /// assert_eq!(data.len(), 3);
    ///
    /// assert_eq!(data.get(1), Some(&1));
    /// assert_eq!(data.get(2), Some(&2));
    /// assert_eq!(data.get(3), Some(&3));
    /// ```
    pub fn init_with(players: PlayerSet, mut func: impl FnMut(Player) -> T) -> Self {
        let mut data = SmallVec::with_capacity(players.count().try_into().unwrap());

        for player in players.iter() {
            data.push(func(player));
        }

        Self { players, data }
    }

    /// Returns the number of elements in the `PlayerIndexedData`
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let ps = player_set![1, 2, 3];
    ///
    /// let data = PlayerIndexedData::init_with(ps, |player| u8::from(player));
    /// assert_eq!(data.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.players
            .count()
            .try_into()
            .expect("player sets can hold up to 256 players, we don't support 16 bit platforms")
    }

    /// Iterate over (Player, &Item)
    pub fn iter(&self) -> impl Iterator<Item = (Player, &T)> + '_ {
        self.players.iter().zip(&self.data)
    }

    /// Iterate over (Player, &mut Item)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Player, &mut T)> + '_ {
        self.players.iter().zip(self.data.iter_mut())
    }

    /// Returns a new `PlayerIndexedData` with pre allocated capacity `n`
    pub fn with_capacity(n: usize) -> Self {
        Self {
            players: PlayerSet::empty(),
            data: SmallVec::with_capacity(n),
        }
    }

    /// Returns the players in the data set
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// assert!(data.players().eq(player_set![]));
    /// let old = data.insert(1, 42);
    /// assert!(data.players().eq(player_set![1]));
    /// ```
    pub fn players(&self) -> impl Iterator<Item = Player> + '_ {
        self.players.iter()
    }

    /// Returns a reference to the value corresponding to the Player.
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// data.insert(1, 42);
    /// assert_eq!(data.get(1), Some(&42));
    /// assert_eq!(data.get(2), None);
    /// ```
    pub fn get(&self, player: impl Into<Player>) -> Option<&T> {
        let player = player.into();
        self.players.contains(player).then(|| &self[player])
    }

    /// Returns a mutable reference to the value corresponding to the Player.
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// assert_eq!(data.get_mut(1), None);
    /// data.insert(1, 42);
    /// assert_eq!(data.get(1), Some(&42));
    /// data.get_mut(1).map(|x| { *x += 1; });
    /// assert_eq!(data.get(1), Some(&43));
    /// ```
    pub fn get_mut(&mut self, player: impl Into<Player>) -> Option<&mut T> {
        let player = player.into();
        self.players.contains(player).then(|| &mut self[player])
    }

    /// Inserts a (Player, Item) combo into the Data, returning the existing value if present
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// assert!(data.players().eq(player_set![]));
    /// let old = data.insert(1, 42);
    /// assert!(data.players().eq(player_set![1]));
    /// assert_eq!(old, None);
    /// let old = data.insert(1, 69420);
    /// assert_eq!(old, Some(42))
    /// ```
    pub fn insert(&mut self, player: impl Into<Player>, item: T) -> Option<T> {
        let player = player.into();

        if let Some(idx) = self.players.player_offset(player) {
            Some(std::mem::replace(&mut self.data[idx as usize], item))
        } else {
            self.players.insert(player);
            let idx = self
                .players
                .player_offset(player)
                .expect("we just inserted the player");
            self.data.insert(idx as usize, item);
            None
        }
    }

    /// Pop the `(Player, T)` combo with the lowest [`Player`] value, returning [`None`] if the
    /// [`PlayerIndexedData`] is empty
    ///
    /// ```
    /// use lttcore::{play::Player, utilities::{PlayerSet, PlayerIndexedData}};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// assert_eq!(data.pop_front(), None);
    ///
    /// data.insert(1, 42);
    /// data.insert(2, 43);
    /// data.insert(3, 44);
    ///
    /// assert_eq!(data.pop_front(), Some((Player::new(1), 42)));
    /// assert_eq!(data.pop_front(), Some((Player::new(2), 43)));
    /// assert_eq!(data.pop_front(), Some((Player::new(3), 44)));
    /// assert_eq!(data.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<(Player, T)> {
        self.players
            .first()
            .and_then(|player| self.remove(player).map(|t| (player, t)))
    }

    /// Pop the `(Player, T)` combo with the highest [`Player`] value, returning [`None`] if the
    /// [`PlayerIndexedData`] is empty
    ///
    /// ```
    /// use lttcore::{play::Player, utilities::{PlayerSet, PlayerIndexedData}};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// assert_eq!(data.pop_front(), None);
    ///
    /// data.insert(1, 42);
    /// data.insert(2, 43);
    /// data.insert(3, 44);
    ///
    /// assert_eq!(data.pop_back(), Some((Player::new(3), 44)));
    /// assert_eq!(data.pop_back(), Some((Player::new(2), 43)));
    /// assert_eq!(data.pop_back(), Some((Player::new(1), 42)));
    /// assert_eq!(data.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<(Player, T)> {
        self.players
            .last()
            .and_then(|player| self.remove(player).map(|t| (player, t)))
    }

    /// Removes a Player and returns the item if present
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// assert_eq!(data.remove(1), None);
    /// assert!(data.players().eq(player_set![]));
    /// data.insert(1, 42);
    /// assert!(data.players().eq(player_set![1]));
    /// assert_eq!(data.remove(1), Some(42));
    /// assert!(data.players().eq(player_set![]));
    /// ```
    pub fn remove(&mut self, player: impl Into<Player>) -> Option<T> {
        let player = player.into();

        self.players.player_offset(player).map(|idx| {
            self.players.remove(player);
            self.data.remove(idx as usize)
        })
    }
}

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    pid: PlayerIndexedData<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = (Player, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.pid.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pid.pop_back()
    }
}

impl<T> IntoIterator for PlayerIndexedData<T> {
    type Item = (Player, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { pid: self }
    }
}

impl<T> From<PlayerItemCollector<T>> for PlayerIndexedData<T> {
    fn from(pic: PlayerItemCollector<T>) -> Self {
        pic.data
            .into_iter()
            .filter_map(|(player, item)| item.map(|x| (player, x)))
            .collect()
    }
}

impl<T> From<(Player, T)> for PlayerIndexedData<T> {
    fn from(value: (Player, T)) -> Self {
        Some(value).into_iter().collect()
    }
}

impl<T> FromIterator<(Player, T)> for PlayerIndexedData<T> {
    fn from_iter<I: IntoIterator<Item = (Player, T)>>(iter: I) -> Self {
        let mut data = PlayerIndexedData::new();

        for (player, item) in iter {
            data.insert(player, item);
        }

        data
    }
}

impl<T> Extend<(Player, T)> for PlayerIndexedData<T> {
    fn extend<I: IntoIterator<Item = (Player, T)>>(&mut self, iter: I) {
        for (player, elem) in iter {
            self.insert(player, elem);
        }
    }
}

impl<U: Into<Player>, T> std::ops::Index<U> for PlayerIndexedData<T> {
    type Output = T;

    fn index(&self, player: U) -> &Self::Output {
        let player = player.into();

        let idx = self
            .players
            .player_offset(player)
            .expect("player is in player indexed data");
        &self.data[idx as usize]
    }
}

impl<T> std::ops::IndexMut<Player> for PlayerIndexedData<T> {
    fn index_mut(&mut self, player: Player) -> &mut Self::Output {
        let idx = self
            .players
            .player_offset(player)
            .expect("player is in player indexed data");
        &mut self.data[idx as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{play::Player, player_set};
    use PlayerIndexedData as PID;
    use PlayerItemCollector as PIC;

    #[test]
    fn test_player_indexing() {
        let mut data: PID<String> = Default::default();

        let p1: Player = 1.into();
        let p2: Player = 2.into();

        data.insert(p1, "foo".to_string());
        data.insert(p2, "bar".to_string());

        assert_eq!(data[p1], "foo");
        assert_eq!(data[p2], "bar");

        let old = std::mem::replace(&mut data[p1], String::from("baz"));
        assert_eq!(old, "foo");
        assert_eq!(data[p1], "baz");
    }

    #[test]
    fn test_player_item_collector_to_player_indexed_data() {
        let data: PIC<usize> = PIC::from(player_set![1, 2, 3]);
        let pid: PID<usize> = data.into();
        assert!(pid.is_empty());

        let mut data: PIC<usize> = PIC::from(player_set![1, 2, 3]);
        data.add(1, 100);
        let pid: PID<usize> = data.into();
        assert!(!pid.is_empty());
        assert!(pid.players().eq([1].map(Player::from).into_iter()));
        assert!(pid.into_iter().eq([(Player::from(1), 100)]))
    }

    #[test]
    #[should_panic]
    fn test_player_indexing_with_invalid_player_panics() {
        let data: PlayerIndexedData<String> = Default::default();
        let p42: Player = 42.into();
        let _ = &data[p42];
    }
}
