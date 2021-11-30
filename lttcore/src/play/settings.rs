use super::NumberOfPlayers;
use semver::Version;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

/// Custom settings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Custom<T: BuiltinGameModes> {
    pub name: Option<Cow<'static, str>>,
    pub settings: Arc<T>,
}

/// Builtin settings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Builtin<T> {
    pub name: Cow<'static, str>,
    pub settings: T,
    pub since_version: Version,
}

/// Trait describing to the runtime how many players are needed for this configuration of the
/// settings. Must always return the same value for the same input
pub trait NumPlayers {
    fn number_of_players(&self) -> NumberOfPlayers;
}

/// Trait describing to the runtime what the "builtin" game modes (settings) for your game are. The
/// runtime optimizes the builtin modes to be represented by only their name on disk and on the
/// wire. For backwards compatibility reasons it's important to never remove or edit game modes
/// once they have been released. There exists a special game mode called "default" that
/// `SettingsPtr::default` will attempt to use if it exists. It is recommend you include "default"
/// in your game modes and have its settings be exactly equal to `YourSettings::Default`
pub trait BuiltinGameModes: Sized + 'static {
    /// Return the list of builtins game modes (settings) for your game. Entries should never be
    /// removed from this list or modified once released. For examples on how to efficently build
    /// `'static [Builtin<Self>]` slices dynamically at start up, see
    /// `examples/guess_the_number/settings.rs` or if you have few custom modes (or none) see
    /// `examples/tic_tac_toe/settings.rs` as an example
    fn builtins() -> &'static [Builtin<Self>];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerifiedBuiltin<T: BuiltinGameModes + 'static>(&'static Builtin<T>);

impl<T: BuiltinGameModes> VerifiedBuiltin<T> {
    pub fn settings(&self) -> &T {
        &self.0.settings
    }
}

impl<T: BuiltinGameModes> Serialize for VerifiedBuiltin<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.name)
    }
}

#[derive(Debug)]
struct VerifiedBuiltinVistor<T>(std::marker::PhantomData<fn() -> T>);

impl<'de, T: BuiltinGameModes + 'static> Visitor<'de> for VerifiedBuiltinVistor<T> {
    type Value = VerifiedBuiltin<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "expecting a built in game mode")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        VerifiedBuiltin::from_str(v)
            .map_err(|_| E::custom(format!("'{}' is not a builtin game mode", v)))
    }
}

impl<'de, T: BuiltinGameModes> Deserialize<'de> for VerifiedBuiltin<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<VerifiedBuiltin<T>, D::Error> {
        let visitor: VerifiedBuiltinVistor<T> = VerifiedBuiltinVistor(Default::default());
        deserializer.deserialize_str(visitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidBuiltin;

impl<T: BuiltinGameModes> From<&'static Builtin<T>> for VerifiedBuiltin<T> {
    fn from(builtin: &'static Builtin<T>) -> Self {
        Self(builtin)
    }
}

impl<T: BuiltinGameModes> FromStr for VerifiedBuiltin<T> {
    type Err = InvalidBuiltin;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        T::builtins()
            .iter()
            .find(|builtin| builtin.name.eq(&name))
            .map(VerifiedBuiltin)
            .ok_or(InvalidBuiltin)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettingsPtr<T: BuiltinGameModes + 'static> {
    Builtin(VerifiedBuiltin<T>),
    Custom(Custom<T>),
}

impl<T: BuiltinGameModes + Default> Default for SettingsPtr<T> {
    fn default() -> Self {
        VerifiedBuiltin::<T>::from_str("default")
            .map(SettingsPtr::from)
            .unwrap_or_else(|_| SettingsPtr::from(T::default()))
    }
}

impl<T: BuiltinGameModes> SettingsPtr<T> {
    pub fn is_builtin(ptr: &SettingsPtr<T>) -> bool {
        matches!(ptr, SettingsPtr::Builtin(_))
    }

    pub fn is_custom(ptr: &SettingsPtr<T>) -> bool {
        matches!(ptr, SettingsPtr::Custom(_))
    }

    pub fn name(ptr: &SettingsPtr<T>) -> Option<&str> {
        match ptr {
            SettingsPtr::Builtin(VerifiedBuiltin(builtin)) => Some(&builtin.name),
            SettingsPtr::Custom(Custom { name, .. }) => name.as_ref().map(|cow| cow.as_ref()),
        }
    }

    pub fn settings(ptr: &SettingsPtr<T>) -> &T {
        match ptr {
            SettingsPtr::Builtin(VerifiedBuiltin(builtin)) => &builtin.settings,
            SettingsPtr::Custom(Custom { settings, .. }) => settings,
        }
    }
}

impl<T: BuiltinGameModes> std::ops::Deref for SettingsPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        SettingsPtr::settings(self)
    }
}

impl<T: BuiltinGameModes> From<VerifiedBuiltin<T>> for SettingsPtr<T> {
    fn from(verified: VerifiedBuiltin<T>) -> Self {
        Self::Builtin(verified)
    }
}

impl<T: BuiltinGameModes> From<&'static Builtin<T>> for SettingsPtr<T> {
    fn from(builtin: &'static Builtin<T>) -> Self {
        Self::Builtin(VerifiedBuiltin(builtin))
    }
}

impl<T: BuiltinGameModes> From<T> for SettingsPtr<T> {
    fn from(settings: T) -> Self {
        Self::from(Arc::new(settings))
    }
}

impl<T: BuiltinGameModes> From<Arc<T>> for SettingsPtr<T> {
    fn from(settings: Arc<T>) -> Self {
        Self::Custom(Custom {
            settings,
            name: None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::examples::guess_the_number::Settings;

    #[test]
    fn serde_builtin_game_mode() {
        let game_modes: Vec<VerifiedBuiltin<Settings>> =
            Settings::builtins().iter().map(VerifiedBuiltin).collect();
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

        let valid: VerifiedBuiltin<Settings> =
            serde_json::from_str("\"players-1-range-1-10\"").unwrap();
        assert_eq!(
            Some(valid.settings()),
            Some(
                VerifiedBuiltin::from_str("players-1-range-1-10")
                    .unwrap()
                    .settings()
            )
        );

        let invalid: Result<VerifiedBuiltin<Settings>, _> =
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
                "name": serde_json::Value::Null,
                "settings": {
                    "number_of_players": 1,
                    "range": {"start": 0, "end": u64::MAX }
                }
            }})
        );

        let deserialized = serde_json::from_value(serialized).unwrap();
        assert_eq!(settings_ptr, deserialized);

        let deserialized: SettingsPtr<Settings> =
            serde_json::from_value(serde_json::json!({"Builtin": "default"})).unwrap();
        assert_eq!(SettingsPtr::settings(&deserialized), &Settings::default());

        let settings_ptr: SettingsPtr<Settings> = VerifiedBuiltin::from_str("players-1-range-1-10")
            .unwrap()
            .into();
        assert_eq!(
            serde_json::to_value(&settings_ptr).unwrap(),
            serde_json::json!({"Builtin": "players-1-range-1-10"})
        );
    }

    #[test]
    fn settings_ptr_default() {
        let settings: SettingsPtr<Settings> = SettingsPtr::default();
        assert!(SettingsPtr::is_builtin(&settings))
    }
}
