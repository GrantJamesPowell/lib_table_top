use std::error::Error;

use lttcore::common::deck::{Card, Suit};
use lttcore::{Player, View};

use crate::{Action, BoardInfo, Direction, PlayerCardCounts, Settings};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpectatorView {
    pub(crate) resigned: Vec<Player>,
    pub(crate) settings: Settings,
    pub(crate) discard_pile: Vec<Card>,
    pub(crate) board_info: BoardInfo,
    pub(crate) player_card_counts: PlayerCardCounts,
    pub(crate) draw_pile_size: usize,
}

impl SpectatorView {
    fn valid_actions(&self, hand: &[Card]) -> impl Iterator<Item = Action> {
        // todo!()
        [].into_iter()
    }
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
