use crate::board::POSSIBLE_WINS;
use crate::{helpers::opponent, Board, Position};
use lttcore::{Player, PlayerSet, View};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// There are still available positions to be claimed on the board
    InProgress { next_up: Player },
    /// All positions have been claimed and there is no winner
    Draw,
    /// Win by resignation
    WinByResignation { winner: Player },
    /// There *is* a winner via connecting three spaces
    Win {
        winner: Player,
        positions: [Position; 3],
    },
}

use Status::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpectatorView {
    board: Board,
    resigned: PlayerSet,
}

impl SpectatorView {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn from_board(board: Board) -> Self {
        Self {
            board,
            resigned: Default::default(),
        }
    }

    pub fn from_board_and_resigned(board: Board, resigned: PlayerSet) -> Self {
        Self { board, resigned }
    }

    /// Returns the status of the current game
    /// ```
    /// use lttcore::{Play, Player};
    /// use tic_tac_toe::{ttt, TicTacToe, Board, Row, Col, Status::*, Marker::*};
    /// let settings = Default::default();
    ///
    /// // In progress
    /// let game: TicTacToe = Default::default();
    /// assert_eq!(game.spectator_view(&settings).status(), InProgress{ next_up: X.into() });
    ///
    /// // A draw
    /// let game: TicTacToe = ttt!([
    ///   O X O
    ///   X X O
    ///   X O X
    /// ]);
    /// assert_eq!(game.spectator_view(&settings).status(), Draw);
    ///
    /// // If someone resigns
    /// let mut game: TicTacToe = Default::default();
    /// game.resign(X);
    /// assert_eq!(game.spectator_view(&settings).status(), WinByResignation { winner: O.into() });
    ///
    /// // With a winning position
    /// let game: TicTacToe = ttt!([
    ///   - - -
    ///   - - -
    ///   X X X
    /// ]).into();
    ///
    /// assert_eq!(
    ///   game.spectator_view(&settings).status(),
    ///   Win {
    ///     winner: X.into(),
    ///     positions: [
    ///       (Col::new(0), Row::new(0)),
    ///       (Col::new(0), Row::new(1)),
    ///       (Col::new(0), Row::new(2))
    ///     ]
    ///   }
    /// );
    /// ```
    pub fn status(&self) -> Status {
        if let Some(loser) = self.resigned.players().next() {
            return WinByResignation {
                winner: opponent(loser),
            };
        }

        POSSIBLE_WINS
            .iter()
            .filter_map(|&positions| {
                let [a, b, c] = positions.map(|pos| self.board.at_position(pos));

                if a == b && b == c {
                    a.map(|winner| Win { winner, positions })
                } else {
                    None
                }
            })
            .next()
            .unwrap_or_else(|| {
                if !self.board.has_open_spaces() {
                    Draw
                } else {
                    InProgress {
                        next_up: self.board.whose_turn(),
                    }
                }
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpectatorViewUpdate {
    Resign(Player),
    Claim(Player, Position),
}

impl View for SpectatorView {
    type Update = SpectatorViewUpdate;

    fn update(&mut self, update: &Self::Update) -> Result<(), Box<dyn Error>> {
        match update {
            SpectatorViewUpdate::Resign(player) => self.resigned.add(*player),
            SpectatorViewUpdate::Claim(player, position) => {
                self.board.claim_space(*player, *position)?
            }
        }
        Ok(())
    }
}
