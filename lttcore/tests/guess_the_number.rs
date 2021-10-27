use lttcore::examples::guess_the_number::{
    Guess, GuessTheNumber, Guesses, Settings, SettingsBuilder, SettingsBuilderError,
    SpectatorUpdate,
};
use lttcore::number_of_players::{EIGHT_PLAYER, ONE_PLAYER};
use lttcore::seed::SEED_42;
use lttcore::{ActionResponse, GameRunner, GameRunnerBuilder, Player};

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

    let spec_view = game_runner.spectator_view();
    let mut turn = game_runner.turn().unwrap();

    for player in turn.pending_action_requests() {
        let guess: Guess = player.as_u64().into();
        turn.add_action(player, guess);
    }

    assert!(turn.is_ready_to_submit());
    let (game_runner, advance) = game_runner.submit_turn(turn).unwrap();
    assert!(advance.debug_msgs.is_empty());
    // let expected_update: Guesses = EIGHT_PLAYER
    //     .players()
    //     .map(|p: Player| -> Guess { p.as_u64().into() })
    //     .map(|g: Guess| -> ActionResponse<Guess> { g.into() })
    //     .collect();

    assert_eq!(advance.spectator_update, game_runner.game().clone().into());
    // The game is now over
    assert!(game_runner.turn().is_none());
}
