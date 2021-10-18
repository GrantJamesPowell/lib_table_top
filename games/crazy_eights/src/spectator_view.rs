use std::error::Error;

use lttcore::common::deck::{Card, Suit};
use lttcore::{player::PlayerResignations, Player, View};

use crate::{Action, BoardInfo, PlayerCardCounts, Settings};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpectatorView {
    pub(crate) resigned: PlayerResignations,
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
pub enum PlayerAction {
    Resignation { player: Player },
    DrawCards { player: Player },
    PlayCard { card: Card, player: Player },
}

impl View for SpectatorView {
    type Update = (PlayerAction, BoardInfo);

    fn update(&mut self, _update: &Self::Update) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
