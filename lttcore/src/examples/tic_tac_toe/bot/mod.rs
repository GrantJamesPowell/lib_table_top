//! Working with tic-tac-toe bot players

pub mod prebuilt;
pub mod test_helpers;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{Action, Board, Position, TicTacToe};
use crate::bot::Bot;
use crate::play::{Play, Seed};
use crate::pov::player::PlayerPov;
use std::{fmt::Display, panic::RefUnwindSafe};

/// A simplified [`Bot`](`crate::bot::Bot`) wrapper specialized for playing [`TicTacToe`]
pub trait TicTacToeBot: Serialize + DeserializeOwned {
    /// Method to choose which [`Position`] to claim given a [`Board`] and a [`Seed`]. Your bot will only be
    /// called when it's your turn to make a move, so [`Board::whose_turn`] will be the marker that
    /// represents your bots. For examples checkout the [`prebuilt`] module
    fn claim_space(&self, board: &Board, seed: &Seed) -> Position;
}

/// Wrapper type to implement [`Bot`](`crate::bot::Bot`) for any [`TicTacToeBot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct TicTacToeBotWrapper<T: TicTacToeBot>(pub T);

impl<T: TicTacToeBot + Display> Display for TicTacToeBotWrapper<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Just use the inner attribute's `Display`
        write!(fmt, "{}", self.0)
    }
}

impl<T> Bot for TicTacToeBotWrapper<T>
where
    T: TicTacToeBot + RefUnwindSafe + Clone + Sync + Send + 'static + Serialize + DeserializeOwned,
{
    type Game = TicTacToe;

    fn run(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        seed: &Seed,
    ) -> <Self::Game as Play>::Action {
        let board = player_pov.public_info.board();
        let position = self.0.claim_space(board, seed);
        Action { position }
    }
}
