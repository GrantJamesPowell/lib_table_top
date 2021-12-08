use super::Settings;
use crate::common::deck::Card;

pub trait BlackJackBot {
    fn surrender(_hand: &[Card], _dealer_card_showing: Card, _settings: &Settings) -> bool {
        false
    }

    fn split(_hand: &[Card], _dealer_card_showing: Card, _settings: &Settings) -> bool {
        false
    }

    fn double_down(_hand: &[Card], _dealer_card_showing: Card, _settings: &Settings) -> bool {
        false
    }

    fn hit(_hand: &[Card], _dealer_card_showing: Card, _settings: &Settings) -> bool;
}
