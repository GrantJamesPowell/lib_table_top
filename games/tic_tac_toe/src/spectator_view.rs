use lib_table_top_core::View;
use crate::{Board, Marker, Position};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct SpectatorView(Board);

impl From<Board> for SpectatorView {
    fn from(board: Board) -> Self {
        Self(board)
    }
}

impl View for SpectatorView {
    type Update = (Marker, Position);

    fn update(&mut self, _action: Self::Update) {}
}
