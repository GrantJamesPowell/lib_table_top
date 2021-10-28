use lttcore::examples::guess_the_number::{
    Guess, GuessTheNumber, Settings, SettingsBuilder,
};
use lttcore::number_of_players::{EIGHT_PLAYER, ONE_PLAYER};
use lttcore::seed::SEED_42;
use lttcore::{GameRunnerBuilder};

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

#[test]
fn test_playing_guess_the_number() {
    let settings = SettingsBuilder::default()
        .num_players(EIGHT_PLAYER)
        .build()
        .unwrap();
    let game_runner = GameRunnerBuilder::<GuessTheNumber>::default()
        .settings(settings)
        .seed(SEED_42)
        .build()
        .unwrap();

    let mut action_requests = game_runner.action_requests();

    for player in action_requests.unaccounted_for_players() {
        let guess: Guess = player.as_u64().into();
        action_requests.add_action(player, guess);
    }

    assert!(action_requests.is_ready_to_submit());
    let (game_runner, advance) = game_runner.submit_actions(action_requests.into_actions());
    assert!(advance.debug_msgs.is_empty());
    assert_eq!(
        advance.spectator_update.public_info_update,
        game_runner.game().clone().into()
    );
}
