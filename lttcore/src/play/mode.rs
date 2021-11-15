use crate::play::LttSettings;
use crate::Play;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mode<T: Play> {
    name: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Play> Mode<T> {
    pub fn try_new(name: &str) -> Option<Self> {
        <T::Settings as LttSettings>::game_modes()
            .get_key_value(name)
            .map(|(&name, _val)| Mode {
                name,
                _phantom: std::marker::PhantomData,
            })
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn settings(&self) -> &'static T::Settings {
        <T::Settings as LttSettings>::game_modes()
            .get(self.name)
            .unwrap()
    }
}

impl<T: Play> std::ops::Deref for Mode<T> {
    type Target = T::Settings;

    fn deref(&self) -> &Self::Target {
        self.settings()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::GuessTheNumber;

    #[test]
    fn test_mode_deref() {
        let mode = Mode::<GuessTheNumber>::try_new("players-2-range-1-10").unwrap();

        assert_eq!(mode.range(), 1..=10);
        assert_eq!(mode.number_of_players(), 2.try_into().unwrap());
    }

    // TODO: Fix Mode Serialization
    // use serde_json::json;

    // #[test]
    // fn test_serde_mode() {
    //     // Using the try_new constructor
    //     assert!(Mode::<GuessTheNumber>::try_new("players-1-range-1-10").is_some());
    //     assert!(Mode::<GuessTheNumber>::try_new("players-2-range-1-10").is_some());
    //     assert!(Mode::<GuessTheNumber>::try_new("players-3-range-1-10").is_some());
    //     assert!(Mode::<GuessTheNumber>::try_new("players-4-range-1-10").is_some());
    //     assert!(Mode::<GuessTheNumber>::try_new("foo bar baz").is_none());

    //     // Serde
    //     let mode: Mode<GuessTheNumber> = Mode::try_new("players-1-range-1-10").unwrap();
    //     let serialized = serde_json::to_value(&mode).unwrap();
    //     assert_eq!(serialized, json!({"name": "players-1-range-1-10"}));
    //     let deserialized: Mode<GuessTheNumber> = serde_json::from_value(serialized).unwrap();
    //     assert_eq!(deserialized, mode);

    //     // it's an error to try to deserialize an invalid game
    //     assert!(
    //         serde_json::from_value::<Mode<GuessTheNumber>>(json!({"name": "foo bar baz"})).is_err()
    //     );
    // }
}
