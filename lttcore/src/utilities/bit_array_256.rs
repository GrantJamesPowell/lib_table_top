#![allow(missing_docs)]

use core::ops::{Range, RangeInclusive};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[macro_export]
macro_rules! bit_array_256 {
    ( $( $expr:expr ),* ) => {
        [$($expr,)*].into_iter()
            .collect::<$crate::utilities::BitArray256>()
    };
}

// [T; N].zip isn't stable yet, so working around
// https://github.com/rust-lang/rust/issues/80094
macro_rules! zip_with {
    ($bf1:expr, $bf2:expr, $func:expr) => {{
        let BitArray256([a1, a2, a3, a4]) = $bf1;
        let BitArray256([b1, b2, b3, b4]) = $bf2;

        BitArray256([
            $func((a1, b1)),
            $func((a2, b2)),
            $func((a3, b3)),
            $func((a4, b4)),
        ])
    }};
}

fn section(num: u8) -> usize {
    (num as usize).checked_div(64).unwrap()
}

fn offset(num: u8) -> usize {
    (num as usize) % 64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BitArray256([u64; 4]);

impl Default for BitArray256 {
    fn default() -> Self {
        BitArray256([0; 4])
    }
}

impl BitArray256 {
    /// Returns a new empty [`BitArray256`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new empty [`BitArray256`]
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter<'_> {
        self.iter_starting_from_number(0)
    }

    /// Returns whether the [`BitArray256`] is empty
    pub fn is_empty(&self) -> bool {
        self.0 == [0_u64; 4]
    }

    pub fn count(&self) -> u16 {
        // We use a u16 instead of a u8 because there are 257 possibilites
        //
        // u8 => [0, 255]
        // count all empty => 0
        // count all full => **256**
        self.0
            .iter()
            .map(|&x| x.count_ones())
            .sum::<u32>()
            .try_into()
            .expect("there are between 0-256 numbers in the bit field")
    }

    /// Alias for `count`
    pub fn len(&self) -> u16 {
        self.count()
    }

    /// Returns the lowest number in the set
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let ba = bit_array_256![4,5,6];
    /// assert_eq!(ba.lowest(), Some(4));
    ///
    /// let empty = bit_array_256![];
    /// assert_eq!(empty.lowest(), None);
    /// ```
    pub fn lowest(&self) -> Option<u8> {
        self.iter().next()
    }

    /// Returns the highest number in the set
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let ba = bit_array_256![4,5,6];
    /// assert_eq!(ba.highest(), Some(6));
    ///
    /// let empty = bit_array_256![];
    /// assert_eq!(empty.highest(), None);
    /// ```
    pub fn highest(&self) -> Option<u8> {
        self.iter().next_back()
    }

    /// Pops the lowest number in the set, returning [`None`] if empty
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let mut ba = bit_array_256![2, 4, 6, 8];
    /// assert_eq!(ba.pop_lowest(), Some(2));
    /// assert_eq!(ba.pop_lowest(), Some(4));
    /// assert_eq!(ba.pop_lowest(), Some(6));
    /// assert_eq!(ba.pop_lowest(), Some(8));
    /// assert_eq!(ba.pop_lowest(), None);
    /// ```
    pub fn pop_lowest(&mut self) -> Option<u8> {
        self.lowest().map(|num| {
            self.remove(num);
            num
        })
    }

    /// Pops the highest number in the set, returning [`None`] if empty
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let mut ba = bit_array_256![2, 4, 6, 8];
    /// assert_eq!(ba.pop_highest(), Some(8));
    /// assert_eq!(ba.pop_highest(), Some(6));
    /// assert_eq!(ba.pop_highest(), Some(4));
    /// assert_eq!(ba.pop_highest(), Some(2));
    /// assert_eq!(ba.pop_highest(), None);
    /// ```
    pub fn pop_highest(&mut self) -> Option<u8> {
        self.highest().map(|num| {
            self.remove(num);
            num
        })
    }

    /// Returns the offset of the number relative to the contents of the [`BitArray256`]
    ///
    /// Note: [`BitArray256`] is iterated in increasing order starting with `0`
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let ba = bit_array_256![2, 4, 6, u8::MAX];
    /// assert_eq!(ba.num_offset(2), Some(0));
    /// assert_eq!(ba.num_offset(4), Some(1));
    /// assert_eq!(ba.num_offset(6), Some(2));
    /// assert_eq!(ba.num_offset(u8::MAX), Some(3));
    ///
    /// // When a number isn't in the set
    ///
    /// assert_eq!(ba.num_offset(42), None);
    /// ```
    pub fn num_offset(&self, num: u8) -> Option<u8> {
        self.contains(num).then(|| {
            let initial_sections_sum = self.0[0..section(num)]
                .iter()
                .map(|x| x.count_ones())
                .sum::<u32>();

            let section = self.0[section(num)];
            let mask: u64 = !(u64::MAX << offset(num));
            let section_ones = (mask & section).count_ones();
            (initial_sections_sum + section_ones)
                .try_into()
                .expect("offset is always 0-255")
        })
    }

    /// Returns whether the number is in the [`BitArray256`]
    ///
    /// ```
    /// use lttcore::utilities::BitArray256;
    ///
    /// let mut set = BitArray256::new();
    /// assert!(!set.contains(1));
    /// set.insert(1);
    /// assert!(set.contains(1));
    /// ```
    pub fn contains(&self, num: u8) -> bool {
        (self.0[section(num)] & (1_usize << offset(num)) as u64) > 0
    }

    /// Adds the number to the set, is a noop if number is already in set
    ///
    /// ```
    /// use lttcore::{bit_array_256, utilities::BitArray256};
    ///
    /// let mut set = bit_array_256![];
    /// assert!(!set.contains(1));
    /// set.insert(1);
    /// assert!(set.contains(1));
    /// ```
    pub fn insert(&mut self, num: u8) {
        self.0[section(num)] |= (1_usize << offset(num)) as u64;
    }

    /// Remove a number from the set, is a noop if number is not in the set
    ///
    /// ```
    /// use lttcore::{bit_array_256, utilities::BitArray256};
    ///
    /// let mut set = bit_array_256![1];
    ///
    /// assert!(set.contains(1));
    /// set.remove(1);
    /// assert!(!set.contains(1));
    /// ```
    pub fn remove(&mut self, num: u8) {
        self.0[section(num)] &= !(1_usize << offset(num)) as u64;
    }

    /// The [`BitArray256`] representing the union, i.e. the numbers that are in self, other, or
    /// both
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let set1 = bit_array_256![1,2,3];
    /// let set2 = bit_array_256![2,3,4];
    ///
    /// assert_eq!(set1.union(set2), bit_array_256![1, 2, 3, 4]);
    /// ```
    pub fn union(self, other: Self) -> Self {
        zip_with!(self, other, |(x, y)| { x | y })
    }

    /// The [`BitArray256`] representing the intersection, i.e. the numbers that are in self and
    /// also in other
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let set1 = bit_array_256![1,2,3];
    /// let set2 = bit_array_256![2,3,4];
    ///
    /// assert_eq!(set1.intersection(set2), bit_array_256![2, 3]);
    /// ```
    pub fn intersection(self, other: Self) -> Self {
        zip_with!(self, other, |(x, y)| { x & y })
    }

    /// The [`BitArray256`] representing the difference, i.e., the numbers that are in self but not
    /// in other.
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let set1 = bit_array_256![1,2,3];
    /// let set2 = bit_array_256![2,3,4];
    ///
    /// assert_eq!(set1.difference(set2), bit_array_256![1]);
    /// ```
    pub fn difference(self, other: Self) -> Self {
        zip_with!(self, other, |(x, y): (u64, u64)| { x & !y })
    }

    /// The [`BitArray256`] representing the symmetric difference, i.e., the numbers in self or
    /// other but not both
    ///
    /// ```
    /// use lttcore::bit_array_256;
    ///
    /// let set1 = bit_array_256![1,2,3];
    /// let set2 = bit_array_256![2,3,4];
    ///
    /// assert_eq!(set1.symmetric_difference(set2), bit_array_256![1, 4])
    /// ```
    pub fn symmetric_difference(self, other: Self) -> Self {
        zip_with!(self, other, |(x, y)| { x ^ y })
    }

    fn iter_starting_from_number(&self, num: u8) -> Iter<'_> {
        let to_end = u8::from(num)..=u8::MAX;
        let from_start = 0..u8::from(num);

        Iter {
            set: Cow::Borrowed(&self),
            to_end,
            from_start,
        }
    }

    fn into_iter_starting_from_number(self, num: u8) -> Iter<'static> {
        let to_end = u8::from(num)..=u8::MAX;
        let from_start = 0..u8::from(num);

        Iter {
            set: Cow::Owned(self),
            to_end,
            from_start,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
    set: Cow<'a, BitArray256>,
    to_end: RangeInclusive<u8>,
    from_start: Range<u8>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        for num in self.to_end.by_ref() {
            if self.set.contains(num) {
                return Some(num);
            }
        }

        for num in self.from_start.by_ref() {
            if self.set.contains(num) {
                return Some(num);
            }
        }

        None
    }
}

impl<'a> std::iter::DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(num) = self.from_start.next_back() {
            if self.set.contains(num) {
                return Some(num);
            }
        }

        while let Some(num) = self.to_end.next_back() {
            if self.set.contains(num) {
                return Some(num);
            }
        }

        None
    }
}

impl IntoIterator for BitArray256 {
    type Item = u8;
    type IntoIter = Iter<'static>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter_starting_from_number(0)
    }
}

impl FromIterator<u8> for BitArray256 {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut set = BitArray256::empty();

        for num in iter {
            set.insert(num);
        }

        set
    }
}

impl From<u8> for BitArray256 {
    fn from(n: u8) -> BitArray256 {
        Some(n).into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_functionality() {
        let mut ba = BitArray256::new();
        assert!(ba.is_empty());
        ba.insert(4);
        assert!(!ba.is_empty());
        ba.remove(4);
        assert!(ba.is_empty());
    }

    #[test]
    fn from_iter() {
        let mut ba = BitArray256::new();
        ba.insert(1);
        ba.insert(2);
        ba.insert(3);

        let from_iter: BitArray256 = (1..=3).collect();
        assert_eq!(from_iter, ba);
    }

    #[test]
    fn iter_and_into_iter() {
        let ba = bit_array_256![1, 2, 3, 4];
        let collected = ba.iter().collect();
        assert_eq!(ba, collected);

        let mut for_looped = BitArray256::new();
        for num in collected {
            for_looped.insert(num);
        }

        assert_eq!(ba, for_looped);

        let mut backwarded = BitArray256::new();
        for num in for_looped.iter().rev() {
            backwarded.insert(num);
        }

        assert_eq!(ba, backwarded);
    }
}
