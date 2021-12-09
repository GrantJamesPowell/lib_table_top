use std::{ops::RangeInclusive, panic::RefUnwindSafe};

use super::{Action, BlackJack, Hand, Phase, PlayerStatus, Settings};
use crate::{bot::Bot, common::deck::Card, play::Seed, pov::player::PlayerPov};
use serde::{de::DeserializeOwned, Serialize};

pub trait BlackJackBot:
    RefUnwindSafe + Serialize + DeserializeOwned + Sync + Send + 'static
{
    fn bet(_chips: u32, settings: &Settings) -> u32 {
        *settings.bettings_limits.start()
    }

    fn surrender(_hand: &Hand, _dealer_card_showing: Card, _settings: &Settings) -> bool {
        false
    }

    fn split(_hand: &Hand, _dealer_card_showing: Card, _settings: &Settings) -> bool {
        false
    }

    fn double_down(_hand: &Hand, _dealer_card_showing: Card, _settings: &Settings) -> bool {
        false
    }

    fn hit(_hand: &Hand, _dealer_card_showing: Card, _settings: &Settings) -> bool;
}

impl<B: BlackJackBot> Bot for B {
    type Game = BlackJack;

    fn on_action_request(&mut self, player_pov: &PlayerPov<'_, BlackJack>, _rng: &Seed) -> Action {
        use PlayerStatus::*;

        match player_pov.public_info.phase {
            Phase::Bet => match player_pov.public_info.statuses[player_pov.player] {
                Resigned { .. } | Busted { .. } => Action::DontBet,
                InPlay { chips } => {
                    let bet = B::bet(chips, player_pov.settings);
                    Action::Bet(bet)
                }
            },
            Phase::PlayHand(player, idx) => {
                let hand = &player_pov.public_info.hands[player][idx];
                let dealer_card_showing = player_pov
                    .public_info
                    .dealer_card_showing()
                    .expect("can only play hand when dealer card is showing");

                if hand.is_able_to_split()
                    && B::split(hand, dealer_card_showing, player_pov.settings)
                {
                    return Action::Split(idx);
                }

                if hand.is_able_to_double_down()
                    && B::double_down(hand, dealer_card_showing, player_pov.settings)
                {
                    return Action::DoubleDown(idx);
                }

                if player_pov.settings.is_able_to_surrender()
                    && idx == 0 // Can only surrender if we have 1 hand
                    && hand.has_taken_additional_cards()
                    && B::surrender(hand, dealer_card_showing, player_pov.settings)
                {
                    return Action::Surrender;
                }

                if B::hit(hand, dealer_card_showing, player_pov.settings) {
                    Action::Hit(idx)
                } else {
                    Action::Stand(idx)
                }
            }
        }
    }
}
