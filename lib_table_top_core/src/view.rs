use std::error::Error;

pub trait View {
    type Update;

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
