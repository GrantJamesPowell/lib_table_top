use crate::play::{
    number_of_players::FOUR_PLAYER,
    settings::{Builtin, BuiltinGameModes, NumPlayers},
    NumberOfPlayers,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings;

impl BuiltinGameModes for Settings {
    fn builtins() -> &'static [Builtin<Self>] {
        todo!()
    }
}

impl NumPlayers for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        FOUR_PLAYER
    }
}
