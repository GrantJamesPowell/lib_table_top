use lib_table_top_core::{Play, Player, View};
use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Row {
    Row0 = 0,
    Row1 = 1,
    Row2 = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Col {
    Col0 = 0,
    Col1 = 1,
    Col2 = 2,
}

/// A type representing a position on the board, denoted in terms of (x, y)
pub type Position = (Col, Row);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board([[Option<Player>; 3]; 3]);

use Col::*;
use Row::*;

pub struct Action(Row, Col);

pub type Settings = [Player; 2];

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum SettingsError {
    /// Returned when both players are the same
    #[error("Players must be different")]
    PlayersCantBeTheSame,
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ActionError {
    /// Returned when trying to claim an already claimed space
    #[error("space ({:?}, {:?}) is taken", attempted.0, attempted.1)]
    SpaceIsTaken { attempted: Position },
    /// Returned when the wrong player tries to take a turn
    #[error("not {:?}'s turn", attempted)]
    OtherPlayerTurn { attempted: Player },
}

pub struct SpectatorView(Board);

impl View for SpectatorView {
    type Update = Action;

    fn update(&mut self, action: Self::Update) {}
}

#[derive(Clone)]
struct TicTacToe {
    board: Board,
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;

    type Settings = Settings;
    type SettingsError = SettingsError;

    type SpectatorView = SpectatorView;
}
