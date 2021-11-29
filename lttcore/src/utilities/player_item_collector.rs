use crate::utilities::PlayerIndexedData;
use crate::{play::Player, PlayerSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerItemCollector<Item> {
    data: PlayerIndexedData<Option<Item>>,
}

impl<Item> From<PlayerSet> for PlayerItemCollector<Item> {
    fn from(players: PlayerSet) -> Self {
        Self::new(players)
    }
}

impl<Item> PlayerItemCollector<Item> {
    /// Returns a new [`PlayerItemCollector`] for a [`PlayerSet`]
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerItemCollector};
    ///
    /// let ps = player_set![2,3,4];
    /// let _: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// ```
    pub fn new(players: impl Into<PlayerSet>) -> Self {
        Self {
            data: PlayerIndexedData::init_with(players.into(), |_| None),
        }
    }

    /// Returns all the players for this collector whether they've submitted or not
    ///
    /// ```
    /// use lttcore::{player_set, utilities::PlayerItemCollector};
    ///
    /// let ps = player_set![2,3,4];
    /// let pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// assert_eq!(pic.players(), ps);
    /// ```
    pub fn players(&self) -> PlayerSet {
        self.data.players()
    }

    /// Returns all the players who have submitted items
    ///
    /// ```
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// assert!(pic.players_who_have_submitted().is_empty());
    ///
    /// let p2: Player = 2.into();
    /// pic.add(p2, 42);
    /// assert_eq!(pic.players_who_have_submitted(), p2.into());
    /// ```
    pub fn players_who_have_submitted(&self) -> PlayerSet {
        self.players()
            .into_iter()
            .filter(|&player| self.data[player].is_some())
            .collect()
    }

    /// Returns all the players who haven't submitted yet
    ///
    /// ```
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// assert_eq!(pic.unaccounted_for_players(), ps);
    ///
    /// let p3: Player = 3.into();
    /// pic.add(p3, 42);
    /// let expected = [2, 4].into_iter().map(Player::new).collect();
    /// assert_eq!(pic.unaccounted_for_players(), expected);
    /// ```
    pub fn unaccounted_for_players(&self) -> PlayerSet {
        self.players()
            .into_iter()
            .filter(|&player| self.data[player].is_none())
            .collect()
    }

    /// Returns whether all players have submitted
    ///
    /// ```
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// assert!(!pic.are_all_players_accounted_for());
    ///
    /// pic.add(2, 42);
    /// pic.add(3, 42);
    /// pic.add(4, 42);
    /// assert!(pic.are_all_players_accounted_for());
    /// ```
    pub fn are_all_players_accounted_for(&self) -> bool {
        self.unaccounted_for_players().count() == 0
    }

    /// Yields the items from the collector
    ///
    /// ```
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    ///
    /// pic.add(2, 42);
    /// pic.add(3, 43);
    /// pic.add(4, 44);
    ///
    /// assert_eq!(
    ///   pic.into_items().collect::<Vec<_>>(),
    ///   vec![
    ///      (Player::new(2), 42),
    ///      (Player::new(3), 43),
    ///      (Player::new(4), 44)
    ///   ]
    /// )
    /// ```
    pub fn into_items(self) -> impl Iterator<Item = (Player, Item)> {
        self.data
            .into_iter()
            .filter_map(|(player, item)| item.map(|x| (player, x)))
    }

    /// Adds a [`Player`] and associated item to the [`PlayerItemCollector`], returning the old value
    ///
    /// ```
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    ///
    /// let existing = pic.add(2, 123);
    /// assert!(existing.is_none());
    /// let new = pic.add(2, 124);
    /// assert_eq!(new, Some(123));
    /// ```
    ///
    /// # Panics
    ///
    /// panics if the [`Player`] isn't in the [`PlayerSet`]
    ///
    /// ```should_panic
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    ///
    /// pic.add(42, 123);
    /// ```
    pub fn add(&mut self, player: impl Into<Player>, item: Item) -> Option<Item> {
        std::mem::replace(&mut self.data[player.into()], Some(item))
    }

    /// Removes a [`Player`] from the [`PlayerItemCollector`], returning the associated item if it's there
    ///
    /// ```
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// assert!(pic.players_who_have_submitted().is_empty());
    ///
    /// pic.add(2, 42);
    /// assert!(!pic.players_who_have_submitted().is_empty());
    /// let removed = pic.remove(2);
    /// assert_eq!(removed, Some(42));
    /// assert!(pic.players_who_have_submitted().is_empty());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the [`Player`] isn't in the [`PlayerSet`]
    ///
    /// ```should_panic
    /// use lttcore::{play::Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    ///
    /// pic.remove(42);
    /// ```
    pub fn remove(&mut self, player: impl Into<Player>) -> Option<Item> {
        self.data[player.into()].take()
    }
}
