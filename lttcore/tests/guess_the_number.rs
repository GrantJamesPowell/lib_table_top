use lttcore::examples::guess_the_number::{Settings, SettingsBuilder};
use lttcore::utilities::number_of_players::ONE_PLAYER;

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
fn test_it_shows_the_player_the_correct_things() {
    // let settings: Settings = (0..=10).try_into().unwrap();
    // let mut host: GameHost<GuessTheNumber> = GameHost::from_settings(settings);
    // let mut players: Vec<_> = host.game_players().collect();

    // assert_eq!(players.len(), 1);
    // let game_player = players.pop().expect("there is a player");
    // let pov = game_player.player_pov();
    // assert_eq!(pov.turn_num, 0);
    // assert_eq!(pov.action_requests, game_player.player().into());
    // assert_eq!(pov.public_info.as_ref(), &PublicInfo::InProgress);
    // assert_eq!(pov.secret_info, Default::default());

    // assert!(game_player.is_player_input_needed());
    // let guess: Guess = 42.into();
    // match host.submit_action_response(game_player.player(), guess) {
    //     Some(updates) => {
    //         // panic!("foo bar baz")
    //     }
    //     None => {
    //         panic!("Didn't finish the game");
    //     }
    // }
}
