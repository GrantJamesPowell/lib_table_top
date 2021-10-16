use std::error::Error;

use lttcore::common::deck::{Card, Rank, Suit};
use lttcore::{Player, View};

use crate::{Action, PlayerCardCounts};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardInfo {
    pub(crate) current_suit_and_rank: (Suit, Rank),
    pub(crate) draw_pile_size: usize,
    pub(crate) discard_pile_size: usize,
    pub(crate) top_card: Card,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpectatorView {
    pub(crate) whose_turn: Player,
    pub(crate) player_card_counts: PlayerCardCounts,
    pub(crate) board_info: BoardInfo,
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
