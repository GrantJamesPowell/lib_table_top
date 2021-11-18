use crate::utilities::number_of_players::TWO_PLAYER;
use crate::{play::LttSettings, NumberOfPlayers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Settings;

lazy_static! {
    static ref GAME_MODES: HashMap<&'static str, Arc<Settings>> =
        [("standard", Arc::new(Settings))].into_iter().collect();
}

impl LttSettings for Settings {
    fn game_modes() -> &'static HashMap<&'static str, Arc<Self>> {
        &GAME_MODES
    }

    fn number_of_players(&self) -> NumberOfPlayers {
        TWO_PLAYER
    }
}
