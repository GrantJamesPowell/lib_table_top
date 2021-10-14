#![allow(dead_code)]
#![feature(never_type, derive_default_enum)]

#[macro_use]
extern crate derive_builder;

mod player_view;
mod settings;
mod spectator_view;
pub use player_view::PlayerView;
pub use settings::Settings;
pub use spectator_view::SpectatorView;

use lttcore::{
    common::deck::{Card, Suit},
    Player,
};
// use thiserror::Error;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Action {
    /// Draw a card from the draw pile. Reshuffles the deck if there are no cards remaining in the
    /// draw pile. If there are no cards in the draw pile or discard pile, this is a no-op.
    Draw,
    /// Play a card from your hand
    Play(Card),
    /// Play and eight, and select the next suit
    PlayEight(Card, Suit),
}

pub struct CrazyEights {
    resigned: Vec<Player>,
}
