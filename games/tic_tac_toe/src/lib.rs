#![allow(dead_code)]
#![feature(never_type)]

use lttcore::{
    play::{ActionResponse, GameAdvance},
    Play, Player,
};
use thiserror::Error;

mod board;
mod marker;
mod settings;
mod spectator_view;

pub use board::{Board, Col, Position, Row, POSSIBLE_WINS};
pub use marker::*;
pub use settings::Settings;
pub use spectator_view::{SpectatorView, Status};

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
    /// use lttcore::{Play, player::p};
    /// use tic_tac_toe::{TicTacToe, Status::*, Marker::*, Settings};
    ///
    /// let settings = Settings::new([p(1), p(2)]);
    /// let mut game: TicTacToe = Default::default();
    /// assert_eq!(game.spectator_view(&settings).status(), InProgress{ next_up: X });
    /// game.resign(X);
    /// assert_eq!(game.spectator_view(&settings).status(), WinByResignation { winner: O });
    /// ```
    pub fn resign(&mut self, marker: Marker) {
        self.resigned = Some(marker);
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Action {
    pub position: Position,
}

#[derive(Error, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ActionError {
    /// Returned when trying to claim an already claimed space
    #[error("space ({:?}, {:?}) is taken", attempted.0, attempted.1)]
    SpaceIsTaken { attempted: Position },
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
        if let Status::InProgress { next_up: marker } = self.spectator_view(settings).status() {
            let player = settings.player_for_marker(marker);
            action_requests.push((player, ActionRequest { marker }));
        }
    }

    fn player_view(
        &self,
        _player: Player,
        _settings: &Self::Settings,
    ) -> <Self as Play>::PlayerView {
        Default::default()
    }

    fn spectator_view(&self, _settings: &Self::Settings) -> SpectatorView {
        SpectatorView::from_board_and_resigned(self.board, self.resigned)
    }

    fn initial_state_for_settings(_settings: &<Self as Play>::Settings) -> Self {
        Default::default()
    }

    fn is_valid_for_settings(&self, _settings: &Settings) -> bool {
        true
    }

    fn players(settings: &<Self as Play>::Settings) -> &[Player] {
        settings.players()
    }

    fn advance(
        &mut self,
        _settings: &Self::Settings,
        actions: impl Iterator<
            Item = (
                (Player, <Self as Play>::ActionRequest),
                <Self as Play>::ActionResponse,
            ),
        >,
        _rng: &mut impl rand::Rng,
        game_advance: &mut GameAdvance<Self>,
    ) {
        use ActionResponse::*;

        for ((player, action_request), response) in actions {
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
                            game_advance
                                .action_errors
                                .push((player, (action_request, err)));
                        }
                    }
                }
            }
        }
    }
}
