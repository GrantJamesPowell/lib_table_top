use super::super::GuessTheNumber;
use super::GuessTheNumberBot;
use crate::bot::{defective::panicking_bot, BotContext, BotError};
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
    fn guess(
        &self,
        range: RangeInclusive<u32>,
        _context: &BotContext<'_, GuessTheNumber>,
    ) -> Result<u32, BotError<GuessTheNumber>> {
        Ok(*range.end())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlwaysLowest;

display_name!(AlwaysLowest);

impl GuessTheNumberBot for AlwaysLowest {
    fn guess(
        &self,
        range: RangeInclusive<u32>,
        _context: &BotContext<'_, GuessTheNumber>,
    ) -> Result<u32, BotError<GuessTheNumber>> {
        Ok(*range.start())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PickCenterOfRange;

display_name!(PickCenterOfRange);

impl GuessTheNumberBot for PickCenterOfRange {
    fn guess(
        &self,
        range: RangeInclusive<u32>,
        _context: &BotContext<'_, GuessTheNumber>,
    ) -> Result<u32, BotError<GuessTheNumber>> {
        Ok(range.start() + ((range.end() - range.start()) / 2))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PickRandomly;

display_name!(PickRandomly);

impl GuessTheNumberBot for PickRandomly {
    fn guess(
        &self,
        range: RangeInclusive<u32>,
        context: &BotContext<'_, GuessTheNumber>,
    ) -> Result<u32, BotError<GuessTheNumber>> {
        let _rng = context.rng_for_turn();
        Ok(context.rng_for_turn().gen_range(range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::play::{seed::SEED_42, Seed};

    #[test]
    fn always_highest() {
        let seed = Seed::new();
        let context = (&seed).into();
        assert_eq!(AlwaysHighest.guess(1..=10, &context), Ok(10));
        assert_eq!(AlwaysHighest.guess(1..=1, &context), Ok(1));
    }

    #[test]
    fn always_lowest() {
        let seed = Seed::new();
        let context = (&seed).into();
        assert_eq!(AlwaysLowest.guess(1..=10, &context), Ok(1));
        assert_eq!(AlwaysLowest.guess(10..=10, &context), Ok(10));
    }

    #[test]
    fn always_center() {
        let seed = Seed::new();
        let context = (&seed).into();
        assert_eq!(PickCenterOfRange.guess(1..=10, &context), Ok(5));
        assert_eq!(PickCenterOfRange.guess(1..=1, &context), Ok(1));
    }

    #[test]
    fn pick_randomly() {
        let context = (&SEED_42).into();
        assert_eq!(PickRandomly.guess(1..=10, &context), Ok(2));
        assert_eq!(PickRandomly.guess(1..=1, &context), Ok(1));
    }
}
