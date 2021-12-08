use super::{player_for_block_and_offset, PlayerSet};
use crate::play::Player;
use crate::utilities::BitArray256;
use itertools::{EitherOrBoth, Itertools};

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

/// [Relation Algebra](https://en.wikipedia.org/wiki/Relational_algebra#Set_operators) support for
/// [`PlayerSet`]
impl PlayerSet {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player_set;

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
