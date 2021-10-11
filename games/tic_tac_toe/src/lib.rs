#![allow(dead_code)]
#![feature(never_type)]

use lttcore::{
    play::{ActionResponse, GameAdvance},
    Play, Player,
};
use thiserror::Error;

mod board;
mod marker;
mod spectator_view;

pub use board::{Board, Col, Position, Row, POSSIBLE_WINS};
pub use marker::*;
pub use spectator_view::{SpectatorView, Status};

use std::collections::HashMap;
use Marker::*;

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
    /// use lttcore::Play;
    /// use tic_tac_toe::{TicTacToe, Status::*, Marker::*};
    ///
    /// let settings = Default::default();
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
    type SpectatorView = SpectatorView;

    fn action_requests_into(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<(Player, Self::ActionRequest)>,
    ) {
        if let Status::InProgress { next_up: marker } = self.spectator_view(settings).status() {
            action_requests.push((marker.player(), ActionRequest { marker }));
        }
    }

    fn spectator_view(&self, _settings: &Self::Settings) -> SpectatorView {
        SpectatorView::from_board_and_resigned(self.board, self.resigned)
    }

    fn initial_state_for_settings(_settings: &<Self as Play>::Settings) -> Self {
        Default::default()
    }

    fn is_valid_for_settings(&self, _settings: &<Self as Play>::Settings) -> bool {
        true
    }

    fn number_of_players_for_settings(_settings: &<Self as Play>::Settings) -> u8 {
        2
    }

    fn player_views_into(
        &self,
        _settings: &<Self as Play>::Settings,
        views: &mut HashMap<Player, <Self as Play>::PlayerView>,
    ) {
        // This is pretty much a no-op since tic tac toe has no secret info
        for marker in [X, O] {
            views.insert(marker.player(), Default::default());
        }
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
        use crate::spectator_view::Update;
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
                                .push(Update::Claim(action_request.marker, action.position));
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
