#![allow(dead_code)]

use lib_table_top_core::play::{ActionResponse, GameAdvance};
use lib_table_top_core::{view::NoSecretPlayerInformationUpdate, Play, Player, View};
use thiserror::Error;

mod board;
mod settings;

pub use board::{Board, Col, Position, Row, POSSIBLE_WINS};
pub use settings::{Settings, SettingsError};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
}

use Marker::*;

impl Marker {
    pub fn opponent(&self) -> Self {
        match self {
            X => O,
            O => X,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// There are still available positions to be claimed on the board
    InProgress,
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
    /// assert_eq!(game.status(), InProgress);
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
    /// assert_eq!(game.status(), InProgress);
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
                    InProgress
                }
            })
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Action(pub Position);

#[derive(Error, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ActionError {
    /// Returned when trying to claim an already claimed space
    #[error("space ({:?}, {:?}) is taken", attempted.0, attempted.1)]
    SpaceIsTaken { attempted: Position },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActionRequest {
    marker: Marker,
    player: Player,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct SpectatorView(Board);

impl View for SpectatorView {
    type Update = Action;

    fn update(&mut self, _action: Self::Update) {}
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;
    type ActionRequest = ActionRequest;

    type Settings = Settings;
    type SettingsError = SettingsError;

    type SpectatorView = SpectatorView;

    fn action_requests(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<Self::ActionRequest>,
    ) {
        if self.status() == Status::InProgress {
            let marker = self.board.whose_turn();
            let player = settings.player_for_marker(marker);
            action_requests.push(ActionRequest { player, marker })
        }
    }

    fn player_view(&self) -> <Self as Play>::PlayerView {
        Default::default()
    }

    fn spectator_view(&self) -> <Self as Play>::SpectatorView {
        SpectatorView(self.board)
    }

    fn initial_state_for_settings(_settings: &<Self as Play>::Settings) -> Self {
        Default::default()
    }

    fn advance(
        &mut self,
        _settings: &Self::Settings,
        actions: &[(
            <Self as Play>::ActionRequest,
            ActionResponse<<Self as Play>::Action>,
        )],
        _rng: &mut impl rand::Rng,
        _game_advance: &mut GameAdvance<
            ActionRequest,
            ActionError,
            NoSecretPlayerInformationUpdate,
            Action,
        >,
    ) {
        for (ActionRequest { marker, .. }, response) in actions {
            match response {
                ActionResponse::Resign => {
                    todo!()
                }
                ActionResponse::Response(Action(position)) => {
                    let _ = self.board.claim_space(*marker, *position);
                }
            }
        }
        todo!()
    }
}
