use crate::{Player, PlayerSet};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerIndexedData<T> {
    players: PlayerSet,
    data: SmallVec<[T; 6]>,
}

impl<T> Default for PlayerIndexedData<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PlayerIndexedData<T> {
    /// Returns a new PlayerIndexData
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Return an iterator over (Player, &Item)
    pub fn iter(&self) -> impl Iterator<Item = (Player, &T)> + '_ {
        self.players.into_iter().zip(&self.data)
    }

    /// Returns a new PlayerIndexData with pre allocated capacity `n`
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
    /// assert_eq!(data.players(), player_set![]);
    /// let old = data.insert(1, 42);
    /// assert_eq!(data.players(), player_set![1]);
    /// ```
    pub fn players(&self) -> PlayerSet {
        self.players
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
    /// assert_eq!(data.players(), player_set![]);
    /// let old = data.insert(1, 42);
    /// assert_eq!(data.players(), player_set![1]);
    /// assert_eq!(old, None);
    /// let old = data.insert(1, 69420);
    /// assert_eq!(old, Some(42))
    /// ```
    pub fn insert(&mut self, player: impl Into<Player>, item: T) -> Option<T> {
        let player = player.into();

        match self.players.player_offset(player) {
            Some(idx) => Some(std::mem::replace(&mut self.data[idx as usize], item)),
            None => {
                let idx = self.players.insert(player);
                self.data.insert(idx.into(), item);
                None
            }
        }
    }

    /// Removes a Player and returns the item if present
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerIndexedData};
    ///
    /// let mut data: PlayerIndexedData<u64> = Default::default();
    /// assert_eq!(data.remove(1), None);
    /// assert_eq!(data.players(), player_set![]);
    /// data.insert(1, 42);
    /// assert_eq!(data.players(), player_set![1]);
    /// assert_eq!(data.remove(1), Some(42));
    /// assert_eq!(data.players(), player_set![]);
    /// ```
    pub fn remove(&mut self, player: impl Into<Player>) -> Option<T> {
        let player = player.into();

        self.players.player_offset(player).map(|idx| {
            self.players.remove(player);
            self.data.remove(idx.into())
        })
    }
}

impl<T> IntoIterator for PlayerIndexedData<T> {
    type Item = (Player, T);
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.players.into_iter().zip(self.data.into_iter())
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

impl<T> std::ops::Index<Player> for PlayerIndexedData<T> {
    type Output = T;

    fn index(&self, player: Player) -> &Self::Output {
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
    use crate::Player;

    #[test]
    fn test_player_indexing() {
        let mut data: PlayerIndexedData<String> = Default::default();

        let p1: Player = 1.into();
        let p2: Player = 2.into();

        data.insert(p1, "foo".to_string());
        data.insert(p2, "bar".to_string());

        assert_eq!(data[p1], "foo");
        assert_eq!(data[p2], "bar");

        let _ = std::mem::replace(&mut data[p1], String::from("baz"));
        assert_eq!(data[p1], "baz");
    }

    #[test]
    #[should_panic]
    fn test_player_indexing_with_invalid_player_panics() {
        let data: PlayerIndexedData<String> = Default::default();
        let p42: Player = 42.into();
        let _ = &data[p42];
    }
}
