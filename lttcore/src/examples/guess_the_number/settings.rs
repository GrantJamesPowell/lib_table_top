use crate::utilities::number_of_players::ONE_PLAYER;
use crate::{
    play::{BuiltinGameModes, LttSettings},
    NumberOfPlayers,
};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

lazy_static! {
    static ref BUILTIN_GAME_MODES: (Vec<&'static str>, Vec<Settings>) = {
        let (names, settings): (Vec<&'static str>, Vec<Settings>) = builtin_game_modes()
            .map(|(name, settings)| (&*Box::leak(name.into_boxed_str()), settings))
            .unzip();

        (names, settings)
    };
}

#[derive(Builder, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[builder(derive(Debug), build_fn(validate = "Self::validate"))]
pub struct Settings {
    #[builder(default = "0..=u64::MAX")]
    range: RangeInclusive<u64>,
    #[builder(default = "ONE_PLAYER")]
    number_of_players: NumberOfPlayers,
}

impl TryFrom<RangeInclusive<u64>> for Settings {
    type Error = SettingsBuilderError;

    fn try_from(range: RangeInclusive<u64>) -> Result<Self, SettingsBuilderError> {
        SettingsBuilder::default().range(range).build()
    }
}

impl SettingsBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(range) = &self.range {
            if range.is_empty() {
                return Err("range must not be empty".into());
            }
        }

        Ok(())
    }
}

impl Settings {
    pub fn range(&self) -> RangeInclusive<u64> {
        self.range.clone()
    }

    pub fn number_of_players(&self) -> NumberOfPlayers {
        self.number_of_players
    }
}

impl Default for Settings {
    fn default() -> Self {
        SettingsBuilder::default().build().unwrap()
    }
}

impl LttSettings for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        self.number_of_players
    }
}

impl BuiltinGameModes for Settings {
    fn builtin_game_mode_names() -> &'static [&'static str] {
        &BUILTIN_GAME_MODES.0
    }

    fn settings_for_builtin(name: &str) -> Option<&'static Self> {
        BUILTIN_GAME_MODES
            .0
            .iter()
            .position(|x| x.eq(&name))
            .and_then(|idx| BUILTIN_GAME_MODES.1.get(idx))
    }
}

fn builtin_game_modes() -> impl Iterator<Item = (String, Settings)> {
    (1..=4)
        .flat_map(|number_of_players| {
            [("1-10", 1..=10), ("u64", 0..=u64::MAX)]
                .into_iter()
                .map(move |range_config| (number_of_players, range_config))
        })
        .map(|(number_of_players, (range_name, range))| {
            let name = format!("players-{}-range-{}", number_of_players, range_name);
            let settings = SettingsBuilder::default()
                .number_of_players(number_of_players.try_into().unwrap())
                .range(range)
                .build()
                .unwrap();

            (name, settings)
        })
        .chain(std::iter::once((
            String::from("default"),
            Settings::default(),
        )))
}
