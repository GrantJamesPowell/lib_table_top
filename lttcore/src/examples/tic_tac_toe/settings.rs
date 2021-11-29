use crate::play::{
    number_of_players::TWO_PLAYER,
    settings::{Builtin, BuiltinGameModes, NumPlayers},
    NumberOfPlayers,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Settings;

static BUILTINS: [Builtin<Settings>; 1] = [Builtin {
    name: Cow::Borrowed("default"),
    settings: Settings,
    since_version: Version::new(0, 0, 0),
}];

impl NumPlayers for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        TWO_PLAYER
    }
}

impl BuiltinGameModes for Settings {
    fn builtins() -> &'static [Builtin<Self>] {
        &BUILTINS
    }
}
