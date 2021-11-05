use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::borrow::Cow;

use std::fmt::Debug;

pub trait View: Clone + Debug + PartialEq + Eq + Send + Serialize + DeserializeOwned {
    type Update: Clone + Debug + PartialEq + Eq + Send + Serialize + DeserializeOwned;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInfo;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInfoUpdate;

impl View for NoSecretPlayerInfo {
    type Update = NoSecretPlayerInfoUpdate;
}
