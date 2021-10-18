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
    common::deck::{Card, DrawPile, Suit},
    play::{
        ActionResponse::{self, *},
        GameAdvance,
    },
    Play, Player,
};
use rand::prelude::*;
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;

// Optimize hands to store up to 8 cards inline
pub type Hand = SmallVec<[Card; 8]>;
// Optimize the CrazyEights struct to store 6 players hands inline
pub type PlayerCardCounts = SmallVec<[usize; 6]>;
type Hands = SmallVec<[Hand; 6]>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Action {
    Draw,
    PlayCard(Card),
    PlayWild(Card, Suit),
}

use Action::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActionRequest;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardInfo {
    pub(crate) top_card: Card,
    pub(crate) current_suit: Suit,
    pub(crate) whose_turn: Player,
    pub(crate) direction: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrazyEights {
    resigned: Vec<Player>,
    hands: Hands,
    draw_pile: DrawPile<Card>,
    board_info: BoardInfo,
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

        in_play.push(self.board_info.top_card);

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
            discard_pile: self.discard_pile(&settings).collect(),
            draw_pile_size: self.draw_pile.len(),
            board_info: self.board_info,
        }
    }

    fn initial_state_for_settings(
        settings: &<Self as Play>::Settings,
        rng: &mut impl rand::Rng,
    ) -> Self {
        let mut draw_pile: Vec<Card> = Vec::from(settings.deck());
        draw_pile.shuffle(rng);
        let mut draw_pile: DrawPile<Card> = draw_pile.into();

        let top_card = draw_pile
            .draw()
            .expect("There must be at least one card in the deck");
        let mut hands: Hands = smallvec![SmallVec::new(); settings.num_players() as usize];

        for _card in 0..settings.starting_num_cards_per_player() {
            for hand in hands.iter_mut() {
                hand.extend(draw_pile.draw());
            }
        }

        Self {
            draw_pile,
            resigned: Vec::new(),
            hands,
            board_info: BoardInfo {
                whose_turn: Player::new(0),
                top_card,
                current_suit: top_card.suit(),
                direction: Default::default(),
            },
        }
    }

    fn is_valid_for_settings(&self, _: &<Self as Play>::Settings) -> bool {
        todo!()
    }

    fn action_requests_into(
        &self,
        settings: &<Self as Play>::Settings,
        action_requests: &mut Vec<(Player, <Self as Play>::ActionRequest)>,
    ) {
        action_requests.push((self.board_info.whose_turn, Default::default()));
    }

    fn advance(
        &mut self,
        settings: &Self::Settings,
        actions: impl Iterator<
            Item = (
                (Player, <Self as Play>::ActionRequest),
                ActionResponse<<Self as Play>::Action>,
            ),
        >,
        rng: &mut impl rand::Rng,
        game_advance: &mut GameAdvance<Self>,
    ) {
        use crate::player_view::Update as PlayerViewUpdate;
        use crate::spectator_view::Update as SpectatorViewUpdate;

        for ((player, _action_request), action_response) in actions {
            match action_response {
                Resign => {
                    self.resigned.push(player);
                    game_advance
                        .spectator_view_updates
                        .push(SpectatorViewUpdate::Resignation { player: player })
                }
                Response(Draw) => {
                    let drawn = self.draw_pile.draw();
                    self.hands[player.as_usize()].extend(drawn);

                    if let Some(card) = drawn {
                        game_advance
                            .player_view_updates
                            .push((player, PlayerViewUpdate::AddCards(vec![card])));
                    };
                }
                Response(PlayCard(card)) => {
                    todo!()
                }
                Response(PlayWild(card, suit)) => {
                    todo!()
                }
            }
        }
    }
}
