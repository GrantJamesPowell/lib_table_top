use crate::{ActionRequest, Play};
use std::error::Error;

pub trait Bot {
    type Game: Play;

    fn run(
        action_request: ActionRequest<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>>;
}
