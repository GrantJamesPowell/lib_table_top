#![allow(dead_code)]
#![feature(never_type)]

use lttcore::{play::ActionResponse, Play, Player};
use thiserror::Error;

mod board;
mod marker;
mod settings;
mod spectator_view;

pub use board::{Board, Col, Position, Row, POSSIBLE_WINS};
pub use marker::*;
pub use settings::Settings;
pub use spectator_view::SpectatorView;

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
        marker: Marker,
        positions: [Position; 3],
    },
}

use Status::*;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TicTacToe {
    board: Board,
    resigned: Option<Marker>,
}

impl From<Board> for TicTacToe {
    fn from(board: Board) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }
}

impl TicTacToe {
    /// Resigns a player, ending the game
    ///
    /// ```
    /// use tic_tac_toe::{TicTacToe, Status::*, Marker::*};
    ///
    /// let mut game: TicTacToe = Default::default();
    /// assert_eq!(game.status(), InProgress{ next_up: X });
    /// game.resign(X);
    /// assert_eq!(game.status(), WinByResignation { winner: O });
    /// ```
    pub fn resign(&mut self, marker: Marker) {
        self.resigned = Some(marker);
    }

    /// Returns the status of the current game
    /// ```
    /// use tic_tac_toe::{TicTacToe, Board, Row, Col, Status::*, Marker::*};
    ///
    /// // In progress
    /// let game: TicTacToe = Default::default();
    /// assert_eq!(game.status(), InProgress{ next_up: X });
    ///
    /// // A draw
    /// let game: TicTacToe = Board::from_ints([
    ///   [1, 2, 1],
    ///   [1, 1, 2],
    ///   [2, 1, 2]
    /// ]).into();
    /// assert_eq!(game.status(), Draw);
    ///
    /// // If someone resigns
    /// let mut game: TicTacToe = Default::default();
    /// game.resign(X);
    /// assert_eq!(game.status(), WinByResignation { winner: O });
    ///
    /// // With a winning position
    /// let game: TicTacToe = Board::from_ints([
    ///   [1, 1, 1],
    ///   [0, 0, 0],
    ///   [0, 0, 0]
    /// ]).into();
    ///
    /// assert_eq!(
    ///   game.status(),
    ///   Win {
    ///     marker: X,
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
                    a.map(|marker| Win { marker, positions })
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Action {
    position: Position,
}

#[derive(Error, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ActionError {
    /// Returned when trying to claim an already claimed space
    #[error("space ({:?}, {:?}) is taken", attempted.0, attempted.1)]
    SpaceIsTaken { attempted: Position },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActionRequest {
    marker: Marker,
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;
    type ActionRequest = ActionRequest;

    type Settings = Settings;
    type SettingsError = !;

    type SpectatorView = SpectatorView;

    fn action_requests_into(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<(Player, Self::ActionRequest)>,
    ) {
        if let Status::InProgress { next_up: marker } = self.status() {
            let player = settings.player_for_marker(marker);
            action_requests.push((player, ActionRequest { marker }));
        }
    }

    fn player_view(&self) -> <Self as Play>::PlayerView {
        Default::default()
    }

    fn spectator_view(&self) -> SpectatorView {
        self.board.into()
    }

    fn initial_state_for_settings(_settings: &<Self as Play>::Settings) -> Self {
        Default::default()
    }

    fn is_valid_for_settings(&self, _settings: &Settings) -> bool {
        true
    }

    fn advance(
        &mut self,
        _settings: &Self::Settings,
        actions: &[((Player, ActionRequest), <Self as Play>::ActionResponse)],
        _rng: &mut impl rand::Rng,
        game_advance: &mut <Self as Play>::GameAdvance,
    ) {
        use ActionResponse::*;

        for &((_player, action_request), response) in actions {
            match response {
                Resign => {
                    self.resign(action_request.marker);
                    break;
                }
                Response(action) => {
                    match self
                        .board
                        .claim_space(action_request.marker, action.position)
                    {
                        Ok(_) => {
                            game_advance
                                .spectator_view_updates
                                .push((action_request.marker, action.position));
                        }
                        Err(err) => {
                            game_advance.action_errors.insert(action_request, err);
                        }
                    }
                }
            }
        }
    }
}
