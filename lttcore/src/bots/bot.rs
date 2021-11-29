use crate::play::Play;
use crate::pov::PlayerPov;
use crate::Seed;

pub trait Bot {
    type Game: Play;

    fn run(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: Seed,
    ) -> <Self::Game as Play>::Action;
}
