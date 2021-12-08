use std::{num::NonZeroU8, ops::RangeInclusive};

use crate::{
    common::deck::{cards::STANDARD_DECK, Card},
    play::{
        number_of_players::ONE_PLAYER,
        settings::{Builtin, BuiltinGameModes, NumPlayers},
        NumberOfPlayers,
    },
};
use im::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    PlayerWins,
    DealerWins,
    Push,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Surrender {
    Late,
    Early,
    NoSurrender,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DeckConfig {
    Standard(NonZeroU8),
    Custom(Vec<Card>),
}

impl DeckConfig {
    pub(crate) fn cards(&self) -> Vec<Card> {
        match self {
            DeckConfig::Custom(cards) => cards.clone(),
            DeckConfig::Standard(num) => {
                let mut vec = Vec::with_capacity((num.get() as usize) * STANDARD_DECK.len());

                for _ in 0..=num.get() {
                    vec.extend_from_slice(&STANDARD_DECK);
                }

                vec
            }
        }
    }

    pub(crate) fn cards_without(&self, remove: impl Iterator<Item = Card>) -> Vec<Card> {
        let mut to_remove: HashMap<Card, usize> = HashMap::new();

        for card in remove {
            let count = to_remove.entry(card).or_insert(0);
            *count += 1;
        }

        let mut cards = self.cards();

        cards.retain(|card| match to_remove.get_mut(card) {
            None => true,
            Some(0) => true,
            Some(n) => {
                *n -= 1;
                false
            }
        });

        cards
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub(super) number_of_players: NumberOfPlayers,
    pub(super) deck: DeckConfig,
    pub(super) surrender: Surrender,
    pub(super) maximum_number_of_splits: u8,
    pub(super) dealer_hits_on_hard: RangeInclusive<u8>,
    pub(super) dealer_hits_on_soft: RangeInclusive<u8>,
    pub(super) both_player_and_dealer_have_black_jack: Outcome,
    pub(super) black_jack_payout_ratio: (u8, u8),
}

impl Settings {
    pub fn dealer_will_hit_on_hard(&self, n: u8) -> bool {
        self.dealer_hits_on_hard.contains(&n)
    }

    pub fn dealer_will_hit_on_soft(&self, n: u8) -> bool {
        self.dealer_hits_on_soft.contains(&n)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            number_of_players: ONE_PLAYER,
            deck: DeckConfig::Standard(NonZeroU8::new(6).expect("6 > 0")),
            surrender: Surrender::Late,
            dealer_hits_on_hard: 0..=16,
            dealer_hits_on_soft: 0..=17,
            maximum_number_of_splits: 3,
            both_player_and_dealer_have_black_jack: Outcome::Push,
            black_jack_payout_ratio: (2, 1),
        }
    }
}

impl NumPlayers for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        self.number_of_players
    }
}

impl BuiltinGameModes for Settings {
    fn builtins() -> &'static [Builtin<Self>] {
        &[]
    }
}
