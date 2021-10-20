#![allow(dead_code)]

use lttcore::{
    number_of_players::TWO_PLAYER,
    play::{ActionResponse, DebugMsg, DebugMsgs, GameAdvance},
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
/// Markers implement `Into<Player>` and `PartialEq` with Player,
/// most methods and functions accept an `impl Into<Player>` so markers
/// can be used in their stead
/// ```
/// use lttcore::Player;
/// use tic_tac_toe::Marker::*;
///
/// let p0: Player = 0.into();
/// let p1: Player = 1.into();
///
/// assert_eq!(p0, X);
/// assert_eq!(p1, O);
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
}

impl PartialEq<Marker> for Player {
    fn eq(&self, &other: &Marker) -> bool {
        let p: Player = other.into();
        *self == p
    }
}

impl PartialEq<Player> for Marker {
    fn eq(&self, &other: &Player) -> bool {
        let p: Player = (*self).into();
        other == p
    }
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
    type Status = Status;

    fn action_requests(&self, settings: &Self::Settings) -> PlayerSet {
        match self.spectator_view(settings).status() {
            Status::InProgress { next_up } => next_up.into(),
            _ => Default::default(),
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

    fn number_of_players_for_settings(_settings: &<Self as Play>::Settings) -> NumberOfPlayers {
        TWO_PLAYER
    }

    fn player_views(
        &self,
        _settings: &<Self as Play>::Settings,
    ) -> HashMap<Player, Self::PlayerView> {
        TWO_PLAYER
            .players()
            .map(|player| (player, Default::default()))
            .collect()
    }

    fn advance(
        &self,
        _settings: &Self::Settings,
        mut actions: impl Iterator<Item = (Player, ActionResponse<<Self as Play>::Action>)>,
        _rng: &mut impl rand::Rng,
    ) -> (Self, GameAdvance<Self>) {
        use crate::spectator_view::Update;
        use ActionResponse::*;

        let (player, response) = actions
            .next()
            .expect("Tic Tac Toe is single player at a time");

        let mut new_state = self.clone();
        let mut debug_msgs: DebugMsgs<Self> = Default::default();

        let spectator_update = {
            match response {
                Resign => {
                    new_state.resign(player);
                    Update::Resign(player)
                }
                Response(attempted_action @ Action { position }) => {
                    match new_state.board.claim_space(player, position) {
                        Ok(_) => Update::Claim(player, position),
                        Err(err) => {
                            let replacement = new_state.board.empty_spaces().next().unwrap();

                            new_state.board.claim_space(player, replacement).unwrap();

                            debug_msgs.push((
                                player,
                                DebugMsg {
                                    attempted_action,
                                    replaced_action: Action {
                                        position: replacement,
                                    },
                                    error: err,
                                },
                            ));

                            Update::Claim(player, position)
                        }
                    }
                }
            }
        };

        (
            new_state,
            GameAdvance {
                debug_msgs,
                spectator_update,
                player_updates: Default::default(),
            },
        )
    }
}
