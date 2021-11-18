use crate::utilities::number_of_players::ONE_PLAYER;
use crate::{play::LttSettings, NumberOfPlayers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::sync::Arc;

lazy_static! {
    static ref GAME_MODES: HashMap<&'static str, Arc<Settings>> = {
        let mut modes = HashMap::new();
        for number_of_players in 1..=4 {
            for (range_name, range) in [("1-10", 1..=10), ("u64", 0..=u64::MAX)] {
                let name: &'static str = Box::leak(
                    format!("players-{}-range-{}", number_of_players, range_name).into_boxed_str(),
                );

                let settings = SettingsBuilder::default()
                    .number_of_players(number_of_players.try_into().unwrap())
                    .range(range)
                    .build()
                    .unwrap();

                modes.insert(name, Arc::new(settings));
            }
        }

        modes
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

    fn game_modes() -> &'static HashMap<&'static str, Arc<Self>> {
        &GAME_MODES
    }
}
