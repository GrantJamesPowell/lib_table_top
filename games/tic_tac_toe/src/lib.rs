#![allow(dead_code)]
#![feature(never_type)]

use lttcore::{
    number_of_players::TWO_PLAYER,
    play::{ActionResponse, GameAdvance},
    NumberOfPlayers, Play, Player,
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

    fn initial_state_for_settings(
        _settings: &<Self as Play>::Settings,
        _rng: &mut impl rand::Rng,
    ) -> Self {
        Default::default()
    }

    fn is_valid_for_settings(&self, _settings: &<Self as Play>::Settings) -> bool {
        true
    }

    fn number_of_players_for_settings(_settings: &<Self as Play>::Settings) -> NumberOfPlayers {
        TWO_PLAYER
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
        mut actions: impl Iterator<
            Item = (
                (Player, <Self as Play>::ActionRequest),
                ActionResponse<<Self as Play>::Action>,
            ),
        >,
        _rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self> {
        use crate::spectator_view::Update;
        use ActionResponse::*;

        let ((player, action_request), response) = actions
            .next()
            .expect("Tic Tac Toe is single player at a time");

        match response {
            Resign => {
                self.resign(action_request.marker);
                GameAdvance::Advance {
                    spectator_update: Update::Resign(action_request.marker),
                    player_updates: Default::default(),
                }
            }
            Response(action) => {
                match self
                    .board
                    .claim_space(action_request.marker, action.position)
                {
                    Ok(_) => GameAdvance::Advance {
                        spectator_update: Update::Claim(action_request.marker, action.position),
                        player_updates: Default::default(),
                    },
                    Err(error) => GameAdvance::Unadvanceable {
                        error,
                        request: (player, action_request),
                    },
                }
            }
        }
    }
}
