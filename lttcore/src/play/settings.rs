use crate::NumberOfPlayers;
use serde::{
    de::{DeserializeOwned, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

pub trait LttSettings:
    Clone + Debug + Default + PartialEq + Eq + Sync + Send + Serialize + DeserializeOwned + 'static
{
    fn number_of_players(&self) -> NumberOfPlayers;
}

pub trait BuiltinGameModes {
    /// List of names of builtin game modes.
    fn builtin_game_mode_names() -> &'static [&'static str] {
        &[]
    }

    /// Turn a builtin game mode into a `'static` settings reference. This function should return
    /// `Some` for all names listed in `BuiltinGameModes::builtin_game_mode_names`
    /// `BuiltinGameModes::builtin_game_mode_names()`
    fn settings_for_builtin<'a>(name: &'a str) -> Option<&'static Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuiltinGameMode<T: BuiltinGameModes>(&'static str, std::marker::PhantomData<fn() -> T>);

impl<T: BuiltinGameModes> BuiltinGameMode<T> {
    pub fn name(&self) -> &str {
        self.0
    }

    pub fn settings<'a>(&'a self) -> &'static T {
        T::settings_for_builtin(self.name()).expect("already validated that this builtin exists")
    }
}

impl<T: BuiltinGameModes> Serialize for BuiltinGameMode<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0)
    }
}

#[derive(Debug)]
struct BuiltinGameModeVistor<T>(std::marker::PhantomData<fn() -> T>);

impl<'de, T: BuiltinGameModes> Visitor<'de> for BuiltinGameModeVistor<T> {
    type Value = BuiltinGameMode<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "expecting a built in game mode")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        BuiltinGameMode::from_str(v)
            .map_err(|_| E::custom(format!("'{}' is not a builtin game mode", v)))
    }
}

impl<'de, T: BuiltinGameModes> Deserialize<'de> for BuiltinGameMode<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<BuiltinGameMode<T>, D::Error> {
        let visitor: BuiltinGameModeVistor<T> = BuiltinGameModeVistor(Default::default());
        deserializer.deserialize_str(visitor)
    }
}

pub fn builtin_game_modes<T: BuiltinGameModes>() -> impl Iterator<Item = BuiltinGameMode<T>> {
    T::builtin_game_mode_names()
        .iter()
        .map(|name| BuiltinGameMode(name, std::marker::PhantomData))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidBuiltinGameMode;

impl<T: BuiltinGameModes> FromStr for BuiltinGameMode<T> {
    type Err = InvalidBuiltinGameMode;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        println!("herer!");
        T::builtin_game_mode_names()
            .iter()
            .find(|&x| x.eq(&name))
            .map(|&name| BuiltinGameMode(name, std::marker::PhantomData))
            .ok_or(InvalidBuiltinGameMode)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettingsPtr<T: BuiltinGameModes> {
    Builtin(BuiltinGameMode<T>),
    Custom(Arc<T>),
}

impl<T: BuiltinGameModes> SettingsPtr<T> {
    pub fn is_builtin(ptr: SettingsPtr<T>) -> bool {
        match ptr {
            SettingsPtr::Builtin(_) => true,
            _ => false,
        }
    }

    pub fn is_custom(ptr: SettingsPtr<T>) -> bool {
        match ptr {
            SettingsPtr::Custom(_) => true,
            _ => false,
        }
    }
}

impl<T: BuiltinGameModes> From<BuiltinGameMode<T>> for SettingsPtr<T> {
    fn from(builtin: BuiltinGameMode<T>) -> Self {
        Self::Builtin(builtin)
    }
}

impl<T: BuiltinGameModes> From<T> for SettingsPtr<T> {
    fn from(settings: T) -> Self {
        Self::Custom(Arc::new(settings))
    }
}

impl<T: BuiltinGameModes> From<Arc<T>> for SettingsPtr<T> {
    fn from(settings: Arc<T>) -> Self {
        Self::Custom(settings)
    }
}

impl<T: BuiltinGameModes + 'static> std::ops::Deref for SettingsPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            SettingsPtr::Builtin(builtin) => builtin.settings(),
            SettingsPtr::Custom(settings) => &settings,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::examples::guess_the_number::Settings;

    #[test]
    fn serde_builtin_game_mode() {
        let game_modes: Vec<BuiltinGameMode<Settings>> = builtin_game_modes::<Settings>().collect();
        assert_eq!(
            serde_json::to_value(&game_modes).unwrap(),
            serde_json::json!([
                "players-1-range-1-10",
                "players-1-range-u64",
                "players-2-range-1-10",
                "players-2-range-u64",
                "players-3-range-1-10",
                "players-3-range-u64",
                "players-4-range-1-10",
                "players-4-range-u64",
                "default",
            ])
        );

        let valid: BuiltinGameMode<Settings> =
            serde_json::from_str("\"players-1-range-1-10\"").unwrap();
        assert_eq!(
            Some(valid.settings()),
            Settings::settings_for_builtin("players-1-range-1-10")
        );

        let invalid: Result<BuiltinGameMode<Settings>, _> =
            serde_json::from_value(serde_json::json!("foo bar baz"));
        assert_eq!(
            invalid.unwrap_err().to_string(),
            "'foo bar baz' is not a builtin game mode"
        );
    }

    #[test]
    fn serde_settings_ptr() {
        let settings_ptr: SettingsPtr<Settings> = Settings::default().into();
        let serialized = serde_json::to_value(&settings_ptr).unwrap();
        assert_eq!(
            serialized,
            serde_json::json!({"Custom": {
                "number_of_players": 1,
                "range": {"start": 0, "end": u64::MAX }
            }})
        );

        let deserialized = serde_json::from_value(serialized).unwrap();
        assert_eq!(settings_ptr, deserialized);

        let deserialized: SettingsPtr<Settings> =
            serde_json::from_value(serde_json::json!({"Builtin": "default"})).unwrap();
        assert_eq!(std::ops::Deref::deref(&deserialized), &Settings::default());

        let settings_ptr: SettingsPtr<Settings> = BuiltinGameMode::from_str("players-1-range-1-10")
            .unwrap()
            .into();
        assert_eq!(
            serde_json::to_value(&settings_ptr).unwrap(),
            serde_json::json!({"Builtin": "players-1-range-1-10"})
        );
    }
}
