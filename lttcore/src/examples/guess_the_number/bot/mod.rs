pub mod prebuilt;

use super::GuessTheNumber;
use crate::bot::Bot;
use crate::play::{Play, Seed};
use crate::pov::player::PlayerPov;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Display, ops::RangeInclusive, panic::RefUnwindSafe};

pub trait GuessTheNumberBot:
    RefUnwindSafe + Serialize + DeserializeOwned + Clone + Sync + Send + 'static
{
    fn guess(&self, range: RangeInclusive<u32>, seed: &Seed) -> u32;

    fn into_bot(self) -> GuessTheNumberBotWrapper<Self> {
        GuessTheNumberBotWrapper(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct GuessTheNumberBotWrapper<T: GuessTheNumberBot>(T);

impl<T: GuessTheNumberBot + Display> Display for GuessTheNumberBotWrapper<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Just use the inner attribute's `Display`
        write!(fmt, "{}", self.0)
    }
}

impl<T: GuessTheNumberBot> Bot for GuessTheNumberBotWrapper<T> {
    type Game = GuessTheNumber;

    fn on_action_request(
        &self,
        player_pov: &PlayerPov<'_, Self::Game>,
        seed: &Seed,
    ) -> <Self::Game as Play>::Action {
        let guess = self.0.guess(player_pov.settings.range(), seed);
        guess.into()
    }
}
