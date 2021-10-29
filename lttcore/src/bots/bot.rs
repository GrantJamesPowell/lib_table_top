use crate::{ActionRequest, Omniscience, Play};
use std::error::Error;

pub trait Bot {
    type Game: Play;

    fn run(
        action_request: &ActionRequest<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>>;
}

pub trait OmniscientBot {
    type Game: Play;

    fn run(
        omniscience: &Omniscience<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>>;
}
