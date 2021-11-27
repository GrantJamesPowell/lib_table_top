use crate::{play::LttSettings, NumberOfPlayers};
use crate::{
    play::{Builtin, BuiltinGameModes},
    utilities::number_of_players::TWO_PLAYER,
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

impl LttSettings for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        TWO_PLAYER
    }
}

impl BuiltinGameModes for Settings {
    fn builtins() -> &'static [Builtin<Self>] {
        &BUILTINS
    }
}
