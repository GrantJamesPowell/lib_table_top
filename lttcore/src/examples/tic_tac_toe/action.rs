use super::Position;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Wrapper representing the choosing of a position to play on by a player
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    /// The position to claim
    pub position: Position,
}

impl<T: Into<Position>> From<T> for Action {
    fn from(position: T) -> Self {
        Self {
            position: position.into(),
        }
    }
}

/// The possible failures that can happen when trying to apply a user provided action to the game
#[derive(Error, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionError {
    /// Returned when trying to claim _any_ square when the board is full
    #[error("all spaces taken")]
    AllSpacesTaken,
    /// Returned when trying to claim an already claimed space
    #[error("space {} is taken", attempted)]
    SpaceIsTaken {
        /// The space that was attempted to be claimed
        attempted: Position,
    },
}
