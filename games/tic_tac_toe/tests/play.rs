use lttcore::{player::p, GameRunner, GameRunnerBuilder, Player};
use tic_tac_toe::{Settings, TicTacToe};

#[test]
fn test_you_can_build_one() {
    let settings = Settings::new([p(1), p(2)]);

    let game_runner = GameRunnerBuilder::<TicTacToe>::default()
        .settings(settings)
        .build()
        .unwrap();
}
