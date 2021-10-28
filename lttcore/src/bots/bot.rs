use crate::{Play, Player, Spectator};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub struct BotContext<'a, T: Play> {
    pub player: Player,
    pub secret_info: &'a <T as Play>::PlayerSecretInfo,
    pub spectator: &'a Spectator<T>,
}

pub trait Bot {
    type Game: Play;

    fn run<'a>(
        situation: BotContext<'a, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>>;
}
