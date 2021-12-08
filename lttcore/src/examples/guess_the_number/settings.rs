use crate::play::{
    number_of_players::ONE_PLAYER,
    settings::{Builtin, BuiltinGameModes, NumPlayers},
    NumberOfPlayers,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::RangeInclusive;

lazy_static! {
    static ref BUILTIN_GAME_MODES: &'static [Builtin<Settings>] = {
        let builtins: Vec<_> = builtin_game_modes().collect();
        &*Box::leak(builtins.into_boxed_slice())
    };
}

#[derive(Builder, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[builder(derive(Debug), build_fn(validate = "Self::validate"))]
pub struct Settings {
    #[builder(default = "0..=u32::MAX")]
    range: RangeInclusive<u32>,
    #[builder(default = "ONE_PLAYER")]
    number_of_players: NumberOfPlayers,
}

impl TryFrom<RangeInclusive<u32>> for Settings {
    type Error = SettingsBuilderError;

    fn try_from(range: RangeInclusive<u32>) -> Result<Self, SettingsBuilderError> {
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
    pub fn range(&self) -> RangeInclusive<u32> {
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

impl NumPlayers for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        self.number_of_players
    }
}

impl BuiltinGameModes for Settings {
    fn builtins() -> &'static [Builtin<Self>] {
        &BUILTIN_GAME_MODES
    }
}

fn builtin_game_modes() -> impl Iterator<Item = Builtin<Settings>> {
    (1..=4)
        .flat_map(|number_of_players| {
            [("1-10", 1..=10), ("u32", 0..=u32::MAX)]
                .into_iter()
                .map(move |range_config| (number_of_players, range_config))
        })
        .map(|(number_of_players, (range_name, range))| {
            let settings = SettingsBuilder::default()
                .number_of_players(number_of_players.try_into().unwrap())
                .range(range)
                .build()
                .unwrap();

            let name = format!("players-{}-range-{}", number_of_players, range_name);
            Builtin {
                settings,
                name: Cow::Owned(name),
                since_version: Version::new(0, 0, 0),
            }
        })
        .chain(std::iter::once(Builtin {
            name: Cow::Borrowed("default"),
            settings: Settings::default(),
            since_version: Version::new(0, 0, 0),
        }))
}
