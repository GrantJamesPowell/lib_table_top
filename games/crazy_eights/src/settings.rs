use lttcore::common::deck::Card;
use std::collections::HashMap;

pub const DEFAULT_NUMBER_OF_STARTING_CARDS_PER_PLAYER: usize = 7;
pub const DEFAULT_TURN_LIMIT: usize = 200;

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
    CardIsWild,
    TurnOrderIsReversed,
    NextPlayerIsSkipped,
    NextPlayerMustDraw { quantity: usize },
    NextPlayerMustDrawAndIsSkipped { quantity: usize },
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum CardPowerRuleSet {
    #[default]
    Standard,
    Custom {
        powers: HashMap<Card, Power>,
    },
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Eq)]
#[builder(setter(into, strip_option))]
pub struct Settings {
    num_players: u8,
    #[builder(default = "DEFAULT_TURN_LIMIT")]
    turn_limit: usize,
    #[builder(default)]
    custom_card_power_rule_set: Option<CardPowerRuleSet>,
    #[builder(default)]
    custom_deck: Option<Vec<Card>>,
    #[builder(default = "DEFAULT_NUMBER_OF_STARTING_CARDS_PER_PLAYER")]
    starting_num_cards_per_player: usize,
    #[builder(default)]
    unable_to_play_card_rule: Option<UnableToPlayCardRule>,
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
