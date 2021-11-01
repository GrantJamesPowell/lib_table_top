use crate::pov::PlayerPov;
use crate::{GameProgression, Play};
use std::error::Error;

pub trait Bot {
    type Game: Play;

    fn run(
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>>;
}

pub trait OmniscientBot {
    type Game: Play;

    fn run(
        game_progression: &GameProgression<Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>>;
}
