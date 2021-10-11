use crate::{Board, Marker, Position};
use lttcore::View;
use std::error::Error;
use crate::board::POSSIBLE_WINS;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// There are still available positions to be claimed on the board
    InProgress { next_up: Marker },
    /// All positions have been claimed and there is no winner
    Draw,
    /// Win by resignation
    WinByResignation { winner: Marker },
    /// There *is* a winner via connecting three spaces
    Win {
        winner: Marker,
        positions: [Position; 3],
    },
}

use Status::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct SpectatorView {
    board: Board,
    resigned: Option<Marker>
}

impl SpectatorView {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn from_board(board: Board) -> Self {
        Self {
            board,
            resigned: None,
        }
    }

    pub fn from_board_and_resigned(board: Board, resigned: Option<Marker>) -> Self {
        Self { board, resigned }
    }

    /// Returns the status of the current game
    /// ```
    /// use lttcore::{Play, player::p};
    /// use tic_tac_toe::{TicTacToe, Board, Row, Col, Status::*, Marker::*, Settings};
    /// let settings = Settings::new([p(1), p(2)]);
    ///
    /// // In progress
    /// let game: TicTacToe = Default::default();
    /// assert_eq!(game.spectator_view(&settings).status(), InProgress{ next_up: X });
    ///
    /// // A draw
    /// let game: TicTacToe = Board::from_ints([
    ///   [1, 2, 1],
    ///   [1, 1, 2],
    ///   [2, 1, 2]
    /// ]).into();
    /// assert_eq!(game.spectator_view(&settings).status(), Draw);
    ///
    /// // If someone resigns
    /// let mut game: TicTacToe = Default::default();
    /// game.resign(X);
    /// assert_eq!(game.spectator_view(&settings).status(), WinByResignation { winner: O });
    ///
    /// // With a winning position
    /// let game: TicTacToe = Board::from_ints([
    ///   [1, 1, 1],
    ///   [0, 0, 0],
    ///   [0, 0, 0]
    /// ]).into();
    ///
    /// assert_eq!(
    ///   game.spectator_view(&settings).status(),
    ///   Win {
    ///     winner: X,
    ///     positions: [
    ///       (Col::new(0), Row::new(0)),
    ///       (Col::new(0), Row::new(1)),
    ///       (Col::new(0), Row::new(2))
    ///     ]
    ///   }
    /// );
    /// ```
    pub fn status(&self) -> Status {
        if let Some(loser) = self.resigned {
            return WinByResignation {
                winner: loser.opponent(),
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
                if self.board.is_full() {
                    Draw
                } else {
                    InProgress {
                        next_up: self.board.whose_turn(),
                    }
                }
            })
    }
}

impl View for SpectatorView {
    type Update = (Marker, Position);

    fn update(&mut self, &(marker, position): &Self::Update) -> Result<(), Box<dyn Error>> {
        self.board.claim_space(marker, position)?;
        Ok(())
    }
}
