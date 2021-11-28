use super::{Action, Board, Position, TicTacToe};
use crate::bots::Bot;
use crate::pov::PlayerPov;
use crate::{Play, Seed};

pub trait TicTacToeBot {
    fn claim_space(&self, board: &Board, seed: Seed) -> Position;
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
        let board = player_pov.public_info.board();
        let position = self.0.claim_space(board, seed);
        Action { position }
    }
}
