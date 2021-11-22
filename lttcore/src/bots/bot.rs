use crate::pov::PlayerPov;
use crate::Play;

pub trait Bot {
    type Game: Play;

    fn run(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> <Self::Game as Play>::Action;
}
