use super::{Action, Board, Marker, Position, TicTacToe};
use crate::bots::Bot;
use crate::pov::PlayerPov;
use crate::{Play, Seed};

pub trait TicTacToeBot {
    fn claim_space(&self, marker: Marker, board: &Board, seed: Seed) -> Position;
}

#[derive(Debug)]
pub struct TicTacToeBotWrapper<T: TicTacToeBot>(pub T);

impl<T: TicTacToeBot> Bot for TicTacToeBotWrapper<T> {
    type Game = TicTacToe;

    fn run(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        seed: Seed,
    ) -> <Self::Game as Play>::Action {
        let marker = Marker::try_from(player_pov.player).expect("is a valid player for TicTacToe");
        let board = player_pov.public_info.board();
        let position = self.0.claim_space(marker, board, seed);

        Action { position }
    }
}
