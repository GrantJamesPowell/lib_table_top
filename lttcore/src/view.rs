use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;

pub trait View: Clone + Debug + Serialize + DeserializeOwned {
    type Update: Clone + Debug + Serialize + DeserializeOwned;

    fn update(&mut self, _update: &Self::Update) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInformation;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSecretPlayerInformationUpdate;

impl View for NoSecretPlayerInformation {
    type Update = NoSecretPlayerInformationUpdate;
}
