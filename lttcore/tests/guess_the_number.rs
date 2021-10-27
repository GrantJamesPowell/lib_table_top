use lttcore::examples::guess_the_number::{
    GuessTheNumber, Settings, SettingsBuilder, SettingsBuilderError,
};
use lttcore::number_of_players::ONE_PLAYER;

#[test]
fn test_building_default_settings() {
    let settings: Settings = SettingsBuilder::default().build().unwrap();
    assert_eq!(settings.range(), 0..=u64::MAX);
    assert_eq!(settings.num_players(), ONE_PLAYER);
}

#[test]
fn test_it_rejects_settings_where_range_is_empty() {
    let err = SettingsBuilder::default()
        .range(3..=2)
        .build()
        .map_err(|err| err.to_string());

    assert_eq!(err, Err("range must not be empty".to_string()));
}
