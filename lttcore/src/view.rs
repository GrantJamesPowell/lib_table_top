use std::error::Error;
use std::fmt::Debug;

pub trait View: Clone + Debug {
    type Update: Clone + Debug;

    fn update(&mut self, _update: Self::Update) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NoSecretPlayerInformation {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NoSecretPlayerInformationUpdate {}

impl View for NoSecretPlayerInformation {
    type Update = NoSecretPlayerInformationUpdate;
}
