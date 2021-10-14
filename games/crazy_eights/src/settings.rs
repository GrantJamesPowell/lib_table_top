use lttcore::common::deck::{
    cards::*,
    Card,
    Rank::{self, *},
};
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;

pub const DEFAULT_NUMBER_OF_STARTING_CARDS_PER_PLAYER: usize = 7;
pub const DEFAULT_TURN_LIMIT: usize = 200;

lazy_static! {
    static ref EIGHTS_WILD_POWER_RULES: HashMap<Card, SmallVec<[Power; 2]>> = {
        EIGHTS
            .iter()
            .cloned()
            .map(|eight| (eight, smallvec![Wild]))
            .collect()
    };
    static ref DOS_POWER_RULES: HashMap<Card, SmallVec<[Power; 2]>> = {
        let mut rules = HashMap::new();

        for eight in EIGHTS {
            rules.insert(eight, smallvec![Wild]);
        }

        for four in FOURS {
            rules.insert(four, smallvec![Reverse]);
        }

        for two in TWOS {
            rules.insert(two, smallvec![Skip, Draw(2)]);
        }

        rules
    };
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum UnableToPlayCardRule {
    LoseTurn,
    DrawButDontPlay {
        quantity: usize,
    },
    #[default]
    DrawUntilCanPlay,
    DrawUntilCanPlayOrUpTo {
        quantity: usize,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Power {
    Wild,
    Reverse,
    Skip,
    Draw(usize),
}

use Power::*;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum PowerRules {
    #[default]
    EightsWild,
    Dos,
    Custom {
        rules: HashMap<Card, SmallVec<[Power; 2]>>,
    },
}

impl PowerRules {
    fn powers_for_card(&self, card: Card) -> &[Power] {
        let rules: &HashMap<Card, SmallVec<[Power; 2]>> = match self {
            PowerRules::EightsWild => &EIGHTS_WILD_POWER_RULES,
            PowerRules::Dos => &DOS_POWER_RULES,
            PowerRules::Custom { rules } => rules,
        };
        rules.get(&card).map(|vec| vec.as_slice()).unwrap_or(&[])
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq)]
#[builder(setter(into, strip_option))]
pub struct Settings {
    num_players: u8,
    #[builder(default = "DEFAULT_TURN_LIMIT")]
    turn_limit: usize,
    #[builder(default)]
    custom_deck: Option<Vec<Card>>,
    #[builder(default = "DEFAULT_NUMBER_OF_STARTING_CARDS_PER_PLAYER")]
    starting_num_cards_per_player: usize,
    #[builder(default)]
    unable_to_play_card_rule: UnableToPlayCardRule,
    #[builder(default)]
    custom_power_rules: PowerRules,
}

fn validate_settings(builder: &SettingsBuilder) -> Result<(), String> {
    let num_players = builder.num_players.ok_or("number of players must be set")?;
    let starting_num_of_cards_per_player = builder
        .starting_num_cards_per_player
        .unwrap_or(DEFAULT_NUMBER_OF_STARTING_CARDS_PER_PLAYER);

    let num_cards_in_deck = match builder.custom_deck {
        Some(Some(ref deck)) => deck.len(),
        _ => 52,
    };

    if num_cards_in_deck < (starting_num_of_cards_per_player * (num_players as usize)) {
        return Err("Number of cards in deck is too few for the number of players and number of starting cards per player".to_string());
    }
    Ok(())
}
