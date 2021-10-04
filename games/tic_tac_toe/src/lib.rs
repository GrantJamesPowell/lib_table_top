#![allow(dead_code)]

use lib_table_top_core::play::{ActionResponse, GameAdvance};
use lib_table_top_core::{view::NoSecretPlayerInformationUpdate, Play, Player, View};
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
        if self.board.status() == Status::InProgress {
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
        TicTacToe {
            board: Board([[None; 3]; 3]),
        }
    }

    fn advance(
        &mut self,
        settings: &Self::Settings,
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
