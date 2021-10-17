#![allow(dead_code)]
#![feature(never_type, derive_default_enum, bool_to_option)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;

pub mod logic;
mod player_view;
mod settings;
mod spectator_view;
pub use player_view::PlayerView;
pub use settings::{Power, Settings};
pub use spectator_view::SpectatorView;

use lttcore::{
    common::deck::{Card, Suit},
    play::{ActionResponse, GameAdvance},
    Play, Player,
};
use smallvec::SmallVec;
use std::collections::HashMap;
// Optimize hands to store up to 8 cards inline
pub type Hand = SmallVec<[Card; 8]>;
// Optimize the CrazyEights struct to store 6 players hands inline
pub type PlayerCardCounts = SmallVec<[usize; 6]>;
type Hands = SmallVec<[Hand; 6]>;

// use thiserror::Error;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Action {
    Draw,
    PlayCard(Card),
    PlayWild(Card, Suit),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionRequest {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrazyEights {
    resigned: Vec<Player>,
    hands: Hands,
    draw_pile: Vec<Card>,
    whose_turn: Player,

    // Because that you can use "powers" to affect what needs to be played next
    // (i. e. Eights can change the suit), we need to keep track of the current_suit + top_card
    current_suit: Suit,
    top_card: Card,
}

impl CrazyEights {
    pub fn player_card_counts(&self) -> PlayerCardCounts {
        SmallVec::from_iter(self.hands.iter().map(|hand| hand.len()))
    }

    pub fn discard_pile<'a, 'b>(
        &'a self,
        settings: &'b Settings,
    ) -> impl Iterator<Item = Card> + 'b {
        let num_in_play: usize = self.hands.iter().map(|h| h.len()).sum::<usize>() + 1;

        let mut in_play: Vec<Card> = Vec::with_capacity(num_in_play);

        in_play.push(self.top_card);

        for hand in &self.hands {
            in_play.extend(hand.iter().cloned());
        }

        in_play.sort();

        settings
            .deck()
            .iter()
            .filter(move |card| in_play.binary_search(card).is_err())
            .cloned()
    }
}

impl Play for CrazyEights {
    type Action = Action;
    type Settings = Settings;
    type ActionRequest = ActionRequest;
    type ActionError = ActionError;
    type SpectatorView = SpectatorView;
    type PlayerView = PlayerView;

    fn number_of_players_for_settings(settings: &Self::Settings) -> u8 {
        settings.num_players()
    }

    fn player_views_into(
        &self,
        settings: &<Self as Play>::Settings,
        player_views: &mut HashMap<Player, <Self as Play>::PlayerView>,
    ) {
        for (player_id, hand) in self.hands.iter().enumerate() {
            player_views.insert(
                Player::new(player_id.try_into().unwrap()),
                PlayerView::new(hand.clone()),
            );
        }
    }

    fn spectator_view(&self, settings: &<Self as Play>::Settings) -> <Self as Play>::SpectatorView {
        SpectatorView {
            resigned: self.resigned.clone(),
            settings: settings.clone(),
            player_card_counts: self.player_card_counts(),
            whose_turn: self.whose_turn,
            discard_pile: self.discard_pile(&settings).collect(),
            current_suit: self.current_suit,
            draw_pile_size: self.draw_pile.len(),
            top_card: self.top_card,
        }
    }

    fn initial_state_for_settings(_: &<Self as Play>::Settings, rng: &mut impl rand::Rng) -> Self {
        todo!()
    }

    fn is_valid_for_settings(&self, _: &<Self as Play>::Settings) -> bool {
        todo!()
    }

    fn action_requests_into(
        &self,
        _: &<Self as Play>::Settings,
        _: &mut Vec<(Player, <Self as Play>::ActionRequest)>,
    ) {
        todo!()
    }

    fn advance(
        &mut self,
        _settings: &Self::Settings,
        _actions: impl Iterator<
            Item = (
                (Player, <Self as Play>::ActionRequest),
                ActionResponse<<Self as Play>::Action>,
            ),
        >,
        _rng: &mut impl rand::Rng,
        game_advance: &mut GameAdvance<Self>,
    ) {
    }
}
