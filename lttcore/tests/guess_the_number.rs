use lttcore::examples::guess_the_number::{
    ActionError::*, Guess, PublicInfo, PublicInfoUpdate, Settings, SettingsBuilder,
};
use lttcore::examples::GuessTheNumber;
use lttcore::play::{
    ActionResponse, Actions, DebugMsg, DebugMsgs, EnumeratedGameAdvance, GameAdvance,
};
use lttcore::seed::SEED_42;
use lttcore::utilities::number_of_players::{ONE_PLAYER, TWO_PLAYER};
use lttcore::view::NoSecretPlayerInfo;
use lttcore::{GamePlayer, GameProgression, Player};

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
    let settings: Settings = SettingsBuilder::default()
        .range(1..=10)
        .num_players(TWO_PLAYER)
        .build()
        .unwrap();

    let mut game: GameProgression<GuessTheNumber> =
        GameProgression::from_settings_and_seed(settings, SEED_42);

    let players: Vec<GamePlayer<GuessTheNumber>> = game.game_players().collect();

    assert_eq!(players.len(), 2);

    let mut actions: Actions<GuessTheNumber> = Default::default();

    for player in players {
        assert!(player.is_player_input_needed());
        let pov = player.player_pov();
        assert_eq!(pov.turn_num, 0.into());
        assert_eq!(pov.public_info, &PublicInfo::InProgress);
        assert_eq!(pov.secret_info, &NoSecretPlayerInfo);

        // Note: Player(0) produces guess outside of range 1..=10
        let guess: Guess = player.player().as_u8().into();
        actions.push((player.player(), guess.into()))
    }

    let EnumeratedGameAdvance {
        turn_num,
        game_advance:
            GameAdvance {
                next_players_input_needed,
                public_info_update,
                player_secret_info_updates,
                debug_msgs,
            },
    } = game.submit_actions(actions);

    assert_eq!(turn_num, 1.into());
    // The game only lasts one "turn"
    assert!(next_players_input_needed.is_empty());
    assert!(player_secret_info_updates.is_empty());

    let expected_debug_msgs: DebugMsgs<GuessTheNumber> = [(
        Player::new(0),
        DebugMsg {
            attempted: Guess(0),
            error: GuessOutOfRange {
                guess: 0,
                range: 1..=10,
            },
        },
    )]
    .into_iter()
    .collect();

    assert_eq!(debug_msgs, expected_debug_msgs);
    assert_eq!(
        public_info_update,
        PublicInfoUpdate {
            secret_number: 8,
            guesses: [
                ActionResponse::Response(Guess(0)),
                ActionResponse::Response(Guess(1)),
            ]
            .into_iter()
            .collect()
        }
    )
}
