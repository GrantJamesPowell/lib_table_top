pub trait View {
    type Update;

    fn update(&mut self, _update: Self::Update) {}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NoSecretPlayerInformation {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NoSecretPlayerInformationUpdate {}

impl View for NoSecretPlayerInformation {
    type Update = NoSecretPlayerInformationUpdate;
}
