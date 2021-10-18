use std::error::Error;

use lttcore::common::deck::{Card, Suit};
use lttcore::{Player, View};

use crate::{Action, Direction, PlayerCardCounts, Settings};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpectatorView {
    pub(crate) direction: Direction,
    pub(crate) resigned: Vec<Player>,
    pub(crate) settings: Settings,
    pub(crate) whose_turn: Player,
    pub(crate) player_card_counts: PlayerCardCounts,
    pub(crate) discard_pile: Vec<Card>,
    pub(crate) current_suit: Suit,
    pub(crate) draw_pile_size: usize,
    pub(crate) top_card: Card,
}

impl SpectatorView {
    fn valid_actions(&self, hand: &[Card]) -> impl Iterator<Item = Action> {
        // todo!()
        [].into_iter()
    }
}

pub struct BoardInfo {
    player_card_counts: PlayerCardCounts,
    whose_turn: Player,
    direction: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Update {
    Resignation { player: Player },
    DrawCards { player: Player },
    PlayCard { card: Card, player: Player },
}

impl View for SpectatorView {
    type Update = Update;

    fn update(&mut self, _update: &Self::Update) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
