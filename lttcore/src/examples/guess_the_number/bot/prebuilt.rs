use super::super::GuessTheNumber;
use super::GuessTheNumberBot;
use crate::{bot::defective::panicking_bot, play::Seed};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::RangeInclusive;

macro_rules! display_name {
    ($ty:ty) => {
        impl Display for $ty {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(fmt, "{}", stringify!($ty))
            }
        }
    };
}

panicking_bot!(GuessTheNumber);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Blaze it
pub struct AlwaysHighest;

display_name!(AlwaysHighest);

impl GuessTheNumberBot for AlwaysHighest {
    fn guess(&self, range: RangeInclusive<u32>, _seed: &Seed) -> u32 {
        *range.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlwaysLowest;

display_name!(AlwaysLowest);

impl GuessTheNumberBot for AlwaysLowest {
    fn guess(&self, range: RangeInclusive<u32>, _seed: &Seed) -> u32 {
        *range.start()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PickCenterOfRange;

display_name!(PickCenterOfRange);

impl GuessTheNumberBot for PickCenterOfRange {
    fn guess(&self, range: RangeInclusive<u32>, _seed: &Seed) -> u32 {
        range.start() + ((range.end() - range.start()) / 2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PickRandomly;

display_name!(PickRandomly);

impl GuessTheNumberBot for PickRandomly {
    fn guess(&self, range: RangeInclusive<u32>, seed: &Seed) -> u32 {
        seed.rng().gen_range(range)
    }
}

#[cfg(test)]
mod tests {
    use crate::play::seed::SEED_42;

    use super::*;

    #[test]
    fn always_highest() {
        assert_eq!(AlwaysHighest.guess(1..=10, &Seed::new()), 10);
        assert_eq!(AlwaysHighest.guess(1..=1, &Seed::new()), 1);
    }

    #[test]
    fn always_lowest() {
        assert_eq!(AlwaysLowest.guess(1..=10, &Seed::new()), 1);
        assert_eq!(AlwaysLowest.guess(10..=10, &Seed::new()), 10);
    }

    #[test]
    fn always_center() {
        assert_eq!(PickCenterOfRange.guess(1..=10, &Seed::new()), 5);
        assert_eq!(PickCenterOfRange.guess(1..=1, &Seed::new()), 1);
    }

    #[test]
    fn pick_randomly() {
        assert_eq!(PickRandomly.guess(1..=10, &SEED_42), 9);
        assert_eq!(PickRandomly.guess(1..=1, &SEED_42), 1);
    }
}
