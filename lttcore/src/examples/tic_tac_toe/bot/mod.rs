//! Working with tic-tac-toe bot players

pub mod prebuilt;
pub mod test_helpers;

use super::{Action, Board, Position, TicTacToe};
use crate::bots::Bot;
use crate::play::{Play, Seed};
use crate::pov::player::PlayerPov;

/// A simplified [`Bot`](`crate::bots::Bot`) wrapper specialized for playing [`TicTacToe`]
pub trait TicTacToeBot {
    /// Method to choose which [`Position`] to claim given a [`Board`] and a [`Seed`]. Your bot will only be
    /// called when it's your turn to make a move, so [`Board::whose_turn`] will be the marker that
    /// represents your bots. For examples checkout the [`prebuilt`] module
    fn claim_space(&self, board: &Board, seed: Seed) -> Position;
}

/// Wrapper type to implement [`Bot`](`crate::bots::Bot`) for any [`TicTacToeBot`]
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
