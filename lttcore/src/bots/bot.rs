use crate::play::{Play, Seed};
use crate::pov::player::PlayerPov;

pub trait Bot {
    type Game: Play;

    fn run(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: Seed,
    ) -> <Self::Game as Play>::Action;
}
