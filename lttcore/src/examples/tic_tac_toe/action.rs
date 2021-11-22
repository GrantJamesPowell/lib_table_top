use super::Position;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    pub position: Position,
}

impl From<Position> for Action {
    fn from(position: Position) -> Self {
        Self { position }
    }
}

#[derive(Error, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionError {
    /// Returned when trying to claim _any_ square when the board is full
    #[error("all spaces taken")]
    AllSpacesTaken,
    /// Returned when trying to claim an already claimed space
    #[error("space {} is taken", attempted)]
    SpaceIsTaken { attempted: Position },
}
