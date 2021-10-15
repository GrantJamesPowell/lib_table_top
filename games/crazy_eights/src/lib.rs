#![allow(dead_code)]
#![feature(never_type, derive_default_enum)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;

mod player_view;
mod settings;
mod spectator_view;
pub use player_view::PlayerView;
pub use settings::Settings;
pub use spectator_view::{BoardInfo, SpectatorView};

use lttcore::{
    common::deck::{Card, Rank, Suit},
    play::{ActionResponse, GameAdvance},
    Play, Player,
};
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
// Optimize hands to store up to 8 cards inline
pub type Hand = SmallVec<[Card; 8]>;
// Optimize the CrazyEights struct to store 6 players hands inline
pub type PlayerCardCounts = SmallVec<[usize; 6]>;
type Hands = SmallVec<[Hand; 6]>;

// use thiserror::Error;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Action {
    /// Draw a card from the draw pile. Reshuffles the deck if there are no cards remaining in the
    /// draw pile. If there are no cards in the draw pile or discard pile, this is a no-op.
    Draw,
    /// Play a card from your hand
    Play(Card),
    /// Play and eight, and select the next suit
    PlayEight(Card, Suit),
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
    current_suit_and_rank: (Suit, Rank),
    top_card: Card,
}

impl CrazyEights {
    pub fn discard_pile_size(&self) -> usize {
        todo!()
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
        let player_card_counts: PlayerCardCounts =
            SmallVec::from_iter(self.hands.iter().map(|hand| hand.len()));

        SpectatorView {
            player_card_counts,
            whose_turn: self.whose_turn,
            board_info: BoardInfo {
                top_card: self.top_card,
                current_suit_and_rank: self.current_suit_and_rank,
                draw_pile_size: self.draw_pile.len(),
                discard_pile_size: self.discard_pile_size(),
            },
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
