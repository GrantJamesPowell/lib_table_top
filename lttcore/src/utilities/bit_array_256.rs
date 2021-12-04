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
        let BitArray256 {
            bits: [a1, a2, a3, a4],
        } = $bf1;
        let BitArray256 {
            bits: [b1, b2, b3, b4],
        } = $bf2;

        BitArray256 {
            bits: [
                $func((a1, b1)),
                $func((a2, b2)),
                $func((a3, b3)),
                $func((a4, b4)),
            ],
        }
    }};
}

fn section(num: u8) -> usize {
    (num as usize).checked_div(64).unwrap()
}

fn offset(num: u8) -> usize {
    (num as usize) % 64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BitArray256 {
    bits: [u64; 4],
}

impl Default for BitArray256 {
    fn default() -> Self {
        BitArray256 { bits: [0; 4] }
    }
}

impl BitArray256 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter<'_> {
        self.iter_starting_from_number(0)
    }

    pub fn is_empty(&self) -> bool {
        self.bits == [0_u64; 4]
    }

    pub fn count(&self) -> u16 {
        // We use a u16 instead of a u32 because there are 257 possible numbers of players 0-256
        // inclusive on both sides
        self.bits
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
        (self.bits[section(num)] & (1_usize << offset(num)) as u64) > 0
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
        self.bits[section(num)] |= (1_usize << offset(num)) as u64;
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
        self.bits[section(num)] &= !(1_usize << offset(num)) as u64;
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
