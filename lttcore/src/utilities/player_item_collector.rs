use crate::{Player, PlayerSet};
use smallvec::SmallVec;

pub struct PlayerItemCollector<Item> {
    players: PlayerSet,
    items: SmallVec<[Option<Item>; 4]>,
}

impl<Item> From<PlayerSet> for PlayerItemCollector<Item> {
    fn from(players: PlayerSet) -> Self {
        players.into()
    }
}

impl<Item> PlayerItemCollector<Item> {
    /// Returns a new PlayerItemCollector for a PlayerSet
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// ```
    pub fn new(players: impl Into<PlayerSet>) -> Self {
        let players = players.into();
        let mut items: SmallVec<[Option<Item>; 4]> = Default::default();
        items.resize_with(players.count().into(), || None);
        Self { players, items }
    }

    /// Returns all the players for this collector whether they've submitted or not
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    /// assert_eq!(pic.players(), ps);
    /// ```
    pub fn players(&self) -> PlayerSet {
        self.players
    }

    /// Returns all the players who have submitted items
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
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
        self.items
            .iter()
            .zip(self.players.into_iter())
            .filter_map(|(item, player)| item.as_ref().map(|_| player))
            .collect()
    }

    /// Returns all the players who haven't submitted yet
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
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
        self.players.difference(self.players_who_have_submitted())
    }

    /// Returns all the players who haven't submitted yet
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
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
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
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
        self.items
            .into_iter()
            .zip(self.players.into_iter())
            .filter_map(|(item, player)| item.map(|item| (player, item)))
    }

    /// Adds a player and associated item to the collector. Returning the old value
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
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
    /// panics if the player isn't in the PlayerSet
    ///
    /// ```should_panic
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    ///
    /// pic.add(42, 123);
    /// ```
    pub fn add(&mut self, player: impl Into<Player>, item: Item) -> Option<Item> {
        let offset = self
            .players
            .player_offset(player)
            .expect("player was not in PlayerSet");

        std::mem::replace(&mut self.items[offset as usize], Some(item))
    }

    /// Removes a player from the collector, returning the associated item if it's there
    ///
    /// ```
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
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
    /// Panics if the player isn't in the playerset
    ///
    /// ```should_panic
    /// use lttcore::{Player, PlayerSet, utilities::PlayerItemCollector};
    ///
    /// let ps = PlayerSet::from_iter([2,3,4].into_iter().map(Player::new));
    /// let mut pic: PlayerItemCollector<u64> = PlayerItemCollector::new(ps);
    ///
    /// pic.remove(42);
    /// ```
    pub fn remove(&mut self, player: impl Into<Player>) -> Option<Item> {
        let offset = self
            .players
            .player_offset(player)
            .expect("player was not in PlayerSet");
        self.items[offset as usize].take()
    }
}
