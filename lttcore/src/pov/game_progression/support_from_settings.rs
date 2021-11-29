use crate::{
    play::Play,
    pov::{GameProgression, GameProgressionBuilder},
    Seed,
};

impl<T: Play> GameProgression<T> {
    pub fn from_settings(settings: impl Into<T::Settings>) -> Self {
        Self::from_settings_and_seed(settings, Seed::random())
    }

    pub fn from_settings_and_seed(settings: impl Into<T::Settings>, seed: impl Into<Seed>) -> Self {
        GameProgressionBuilder::default()
            .settings(settings.into())
            .seed(seed.into())
            .build()
            .unwrap()
    }
}
