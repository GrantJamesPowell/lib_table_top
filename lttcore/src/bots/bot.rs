use crate::{Play, Player, Spectator};
use std::error::Error;

pub trait Bot {
    type Game: Play;

    fn run(
        player: Player,
        spectator: &Spectator<Self::Game>,
        secret_info: &<Self::Game as Play>::PlayerSecretInfo,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>>;
}
