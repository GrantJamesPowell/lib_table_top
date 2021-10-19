#![allow(dead_code)]

use lttcore::{
    number_of_players::TWO_PLAYER,
    play::{ActionResponse, GameAdvance},
    NumberOfPlayers, Play, Player, PlayerSet,
};
use thiserror::Error;

mod board;
mod spectator_view;

pub use board::{Board, Col, Position, Row, POSSIBLE_WINS};
pub use spectator_view::{SpectatorView, Status};

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TicTacToe {
    board: Board,
    resigned: PlayerSet,
}

impl From<Board> for TicTacToe {
    fn from(board: Board) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }
}

/// Conveniences for Player 0 and Player 1
///
/// ```
/// use lttcore::Player;
/// use tic_tac_toe::Marker::*;
///
/// let p0: Player = 0.into();
/// let p1: Player = 1.into();
///
/// assert_eq!(p0, X.into());
/// assert_eq!(p1, O.into());
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
}

impl Into<Player> for Marker {
    fn into(self) -> Player {
        match self {
            Marker::X => 0.into(),
            Marker::O => 1.into(),
        }
    }
}

/// Returns the opponent of a player in TicTacToe
///
/// ```
/// use lttcore::Player;
/// use tic_tac_toe::{Marker::*, opponent};
///
/// let p0: Player = 0.into();
/// let p1: Player = 1.into();
///
/// assert_eq!(opponent(p0), p1);
/// assert_eq!(opponent(p1), p0);
/// assert_eq!(opponent(X), p1);
/// assert_eq!(opponent(O), p0);
/// ```
///
/// # Panics
///
/// This panics with a player not in [0, 1]
///
/// ```should_panic
/// use lttcore::Player;
/// use tic_tac_toe::opponent;
///
/// let p3: Player = 3.into();
/// opponent(p3);
/// ```
pub fn opponent(player: impl Into<Player>) -> Player {
    match player.into().as_u8() {
        0 => 1.into(),
        1 => 0.into(),
        _ => panic!("Invalid Player for TicTacToe"),
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
    /// assert_eq!(game.spectator_view(&settings).status(), InProgress{ next_up: X.into() });
    /// game.resign(X); // or game.resign(0.into());
    /// assert_eq!(game.spectator_view(&settings).status(), WinByResignation { winner: O.into() });
    /// ```
    pub fn resign(&mut self, player: impl Into<Player>) {
        self.resigned.add(player.into());
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

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;
    type SpectatorView = SpectatorView;

    fn action_requests_into(
        &self,
        settings: &Self::Settings,
        action_requests: &mut Vec<(Player, Self::ActionRequest)>,
    ) {
        if let Status::InProgress { next_up } = self.spectator_view(settings).status() {
            action_requests.push((next_up, Default::default()));
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
        for player in TWO_PLAYER.players() {
            views.insert(player, Default::default());
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
                self.resign(player);
                GameAdvance::Advance {
                    spectator_update: Update::Resign(player),
                    player_updates: Default::default(),
                }
            }
            Response(action) => match self.board.claim_space(player, action.position) {
                Ok(_) => GameAdvance::Advance {
                    spectator_update: Update::Claim(player, action.position),
                    player_updates: Default::default(),
                },
                Err(error) => GameAdvance::Unadvanceable {
                    error,
                    request: (player, action_request),
                },
            },
        }
    }
}
