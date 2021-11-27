use crate::{play::BuiltinGameModes, utilities::number_of_players::TWO_PLAYER};
use crate::{play::LttSettings, NumberOfPlayers};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Settings;

impl LttSettings for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        TWO_PLAYER
    }
}

impl BuiltinGameModes for Settings {
    fn builtin_game_mode_names() -> &'static [&'static str] {
        &["default"]
    }

    fn settings_for_builtin(name: &str) -> Option<&'static Self> {
        if name == "default" {
            Some(&Settings)
        } else {
            None
        }
    }
}
