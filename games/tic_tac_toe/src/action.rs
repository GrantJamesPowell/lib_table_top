use crate::Position;
use thiserror::Error;

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
