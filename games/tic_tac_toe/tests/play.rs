use lttcore::{play::NoCustomSettings, GameRunnerBuilder};
use tic_tac_toe::TicTacToe;

#[test]
fn test_you_can_build_one() {
    let game_runner = GameRunnerBuilder::<TicTacToe>::default()
        .settings(NoCustomSettings {})
        .build();

    assert!(game_runner.is_ok());
}
