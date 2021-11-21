use crate::pov::PlayerPov;
use crate::{GameProgression, Play};

pub trait Bot {
    type Game: Play;

    fn run(
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> <Self::Game as Play>::Action;
}

pub trait OmniscientBot {
    type Game: Play;

    fn run(
        game_progression: &GameProgression<Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> <Self::Game as Play>::Action;
}
