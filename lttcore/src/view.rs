use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;

pub trait View: Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned {
    type Update: Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned;

    fn update(&mut self, _update: &Self::Update) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInfo;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInfoUpdate;

impl View for NoSecretPlayerInfo {
    type Update = NoSecretPlayerInfoUpdate;
}
