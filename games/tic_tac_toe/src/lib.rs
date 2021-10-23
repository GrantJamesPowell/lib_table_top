#![allow(dead_code)]

mod action;
mod board;
pub mod helpers;
mod marker;
mod spectator_view;
mod tic_tac_toe;

pub use crate::tic_tac_toe::TicTacToe;
pub use action::{Action, ActionError};
pub use board::{Board, Col, Position, Row, POSSIBLE_WINS};
pub use marker::Marker;
pub use spectator_view::{SpectatorView, Status};
