use lttcore::examples::TicTacToe;
use lttcore::pov::game_progression::GameProgressionBuilder;

#[test]
fn test_you_can_build_a_tic_tac_toe_game_progression() {
    let game_progression = GameProgressionBuilder::<TicTacToe>::default().build();
    assert!(game_progression.is_ok())
}
