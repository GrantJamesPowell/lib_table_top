use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use std::panic::RefUnwindSafe;

pub trait View:
    RefUnwindSafe
    + Clone
    + Debug
    + PartialEq
    + Eq
    + Sync
    + Send
    + Serialize
    + DeserializeOwned
    + 'static
{
    type Update: Clone + Debug + PartialEq + Eq + Sync + Send + Serialize + DeserializeOwned;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInfo;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInfoUpdate;

impl View for NoSecretPlayerInfo {
    type Update = NoSecretPlayerInfoUpdate;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretGameInfo;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretGameInfoUpdate;

impl View for NoSecretGameInfo {
    type Update = NoSecretGameInfoUpdate;
}
