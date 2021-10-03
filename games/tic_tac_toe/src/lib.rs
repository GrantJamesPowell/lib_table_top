#![allow(dead_code)]

use lib_table_top_core::{Play, Player, View};
use thiserror::Error;

mod board;
mod settings;

pub use board::{Board, Col, Position, Row, Status};
pub use settings::{Settings, SettingsError};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TicTacToe {
    board: Board,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Action(pub Position);

#[derive(Error, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ActionError {
    /// Returned when trying to claim an already claimed space
    #[error("space ({:?}, {:?}) is taken", attempted.0, attempted.1)]
    SpaceIsTaken { attempted: Position },

    /// Returned when the wrong player tries to take a turn
    #[error("not {:?}'s turn", attempted)]
    OtherPlayerTurn { attempted: Marker },
}

pub struct SpectatorView(Board);

impl View for SpectatorView {
    type Update = Action;

    fn update(&mut self, _action: Self::Update) {}
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;

    type Settings = Settings;
    type SettingsError = SettingsError;

    type SpectatorView = SpectatorView;

    fn action_requests(&self, _settings: &Self::Settings) -> Vec<Player> {
        todo!()
    }

    fn player_view(&self) -> <Self as Play>::PlayerView {
        Default::default()
    }

    fn spectator_view(&self) -> <Self as Play>::SpectatorView {
        SpectatorView(self.board)
    }

    fn initial_state_for_settings(_settings: &<Self as Play>::Settings) -> Self {
        TicTacToe {
            board: Board([[None; 3]; 3]),
        }
    }

    fn advance(
        &mut self,
        _settings: &Self::Settings,
        _actions: &[(Player, <Self as Play>::Action)],
        _rng: &mut impl rand::Rng,
    ) -> Result<<<Self as Play>::SpectatorView as View>::Update, <Self as Play>::ActionError> {
        todo!()
    }
}
