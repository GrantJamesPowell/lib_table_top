use lttcore::examples::guess_the_number::{
    GuessTheNumber, Settings, SettingsBuilder, SettingsBuilderError,
};
use lttcore::number_of_players::ONE_PLAYER;

#[test]
fn test_building_default_settings() {
    let settings: Settings = SettingsBuilder::default().build().unwrap();
    assert_eq!(settings.min(), 0);
    assert_eq!(settings.max(), u64::MAX);
    assert_eq!(settings.num_players(), ONE_PLAYER);
}

#[test]
fn test_it_rejects_settings_where_min_greater_than_or_equal_to_max() {
    let err = SettingsBuilder::default()
        .min(42)
        .max(41)
        .build()
        .map_err(|err| err.to_string());

    assert_eq!(err, Err("min must be strictly less than max".to_string()));
}
