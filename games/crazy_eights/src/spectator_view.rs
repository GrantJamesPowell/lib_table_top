use std::collections::HashMap;
use std::error::Error;

use lttcore::common::deck::{Card, Rank, Suit};
use lttcore::{Player, View};

use crate::Action;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardInfo {
    current_suit_and_rank: (Suit, Rank),
    draw_pile_num_remaining_cards: usize,
    top_card: Card,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpectatorView {
    whose_turn: Player,
    discard_pile: Vec<Card>,
    player_card_count: HashMap<Player, usize>,
    board_info: BoardInfo,
}

impl SpectatorView {
    fn valid_actions(&self, hand: &[Card]) -> impl Iterator<Item = Action> {
        // todo!()
        [].into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerAction {
    PlayCard {
        card: Card,
        player: Player,
        player_hand_size_after_playing_card: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Update {
    player_action: PlayerAction,
    board_info: BoardInfo,
}

impl View for SpectatorView {
    type Update = Update;

    fn update(&mut self, _update: &Self::Update) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
