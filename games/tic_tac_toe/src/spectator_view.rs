use crate::{Board, Marker, Position};
use lttcore::View;
use std::error::Error;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct SpectatorView(Board);

impl SpectatorView {
    pub fn board(&self) -> &Board {
        &self.0
    }
}

impl From<Board> for SpectatorView {
    fn from(board: Board) -> Self {
        Self(board)
    }
}

impl View for SpectatorView {
    type Update = (Marker, Position);

    fn update(&mut self, (marker, position): Self::Update) -> Result<(), Box<dyn Error>> {
        self.0.claim_space(marker, position)?;
        Ok(())
    }
}
