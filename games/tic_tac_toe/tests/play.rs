use lttcore::GameRunnerBuilder;
use tic_tac_toe::TicTacToe;

#[test]
fn test_you_can_build_a_tic_tac_toe_game_runner() {
    let game_runner = GameRunnerBuilder::<TicTacToe>::default().build();
    assert!(game_runner.is_ok())
}

#[test]
fn test_tic_tac_toe_has_a_first_turn() {
    let game_runner = GameRunnerBuilder::<TicTacToe>::default().build().unwrap();
    assert!(game_runner.turn().is_some());
}
