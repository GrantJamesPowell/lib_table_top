#![allow(dead_code)]
#![feature(never_type)]

mod player_view;
mod settings;
mod spectator_view;
pub use player_view::PlayerView;
pub use settings::Settings;
pub use spectator_view::SpectatorView;

use lttcore::{
    common::deck::{Card, Color, Rank, Suit},
    play::{ActionResponse, GameAdvance},
    Play, Player,
};
use thiserror::Error;

struct CrazyEights {
    resigned: Vec<Player>,
}
