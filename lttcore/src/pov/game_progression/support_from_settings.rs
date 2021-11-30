use crate::{
    play::{Play, Seed, SettingsPtr},
    pov::game_progression::{GameProgression, GameProgressionBuilder},
};

impl<T: Play> GameProgression<T> {
    pub fn from_settings(settings: impl Into<SettingsPtr<T::Settings>>) -> Self {
        Self::from_settings_and_seed(settings, Seed::random())
    }

    pub fn from_settings_and_seed(
        settings: impl Into<SettingsPtr<T::Settings>>,
        seed: impl Into<Seed>,
    ) -> Self {
        GameProgressionBuilder::default()
            .settings(settings.into())
            .seed(seed.into())
            .build()
            .unwrap()
    }
}
