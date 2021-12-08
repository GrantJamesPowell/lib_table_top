use super::{player_for_block_and_offset, PlayerSet};
use crate::play::Player;
use crate::utilities::BitArray256;

/// Iteration Support
impl PlayerSet {
    /// Iterate over the players in the set
    pub fn iter(&self) -> impl Iterator<Item = Player> + DoubleEndedIterator + '_ {
        self.0.iter().flat_map(|(block, bit_array)| {
            bit_array
                .iter()
                .map(move |offset| player_for_block_and_offset(*block, offset))
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player_set;

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
}
