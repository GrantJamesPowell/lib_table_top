use crate::NumberOfPlayers;
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::convert::TryFrom;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::sync::Arc;

pub trait LttSettings:
    Clone + Debug + Default + PartialEq + Eq + Sync + Send + Serialize + DeserializeOwned + 'static
{
    fn number_of_players(&self) -> NumberOfPlayers;
    fn game_modes() -> &'static HashMap<&'static str, Arc<Self>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Mode<T: LttSettings> {
    name: &'static str,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<T>,
}

impl<T: LttSettings> Mode<T> {
    pub fn try_new(name: &str) -> Option<Self> {
        T::game_modes()
            .get_key_value(name)
            .map(|(&name, _val)| Mode { name, _phantom: std::marker::PhantomData })
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn settings(&self) -> &'static T {
        T::game_modes().get(self.name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::guess_the_number::Settings;
    use serde_json::json;

    #[test]
    fn test_serde_mode() {
        // Using the try_new constructor
        assert!(Mode::<Settings>::try_new("players-1-range-1-10").is_some());
        assert!(Mode::<Settings>::try_new("players-2-range-1-10").is_some());
        assert!(Mode::<Settings>::try_new("players-3-range-1-10").is_some());
        assert!(Mode::<Settings>::try_new("players-4-range-1-10").is_some());
        assert!(Mode::<Settings>::try_new("foo bar baz").is_none());
        
        // Serde
        let mode: Mode<Settings> = Mode::try_new("players-1-range-1-10").unwrap();
        let serialized = serde_json::to_value(&mode).unwrap();
        assert_eq!(serialized, json!({"name": "players-1-range-1-10"}));
        let deserialized: Mode<Settings> = serde_json::from_value(serialized).unwrap();
        assert_eq!(deserialized, mode);

        // it's an error to try to deserialize an invalid game
        assert!(serde_json::from_value::<Mode<Settings>>(json!({"name": "foo bar baz"})).is_err());
    }
}
