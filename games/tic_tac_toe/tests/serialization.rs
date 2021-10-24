use lttcore::{rng::SEED_42, ActionResponse, GameRunnerBuilder};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value::Null};
use std::fmt::Debug;
use tic_tac_toe::{ttt, Action, ActionError, Board, Col, Marker::*, Row, TicTacToe};

#[test]
fn test_serde_row_col() {
    let row = Row::new(0);
    let col = Col::new(1);

    test_simple_serialization(&row, 0);
    test_simple_serialization(&col, 1);
}

#[test]
fn test_serde_board() {
    let game = ttt!([
        X O X
        - - -
        X O X
    ]);

    let serialized = serde_json::to_value(game.board()).unwrap();

    assert_eq!(
        serialized,
        json!([[0, 1, 0], [Null, Null, Null], [0, 1, 0]])
    );

    let deserialized: Board = serde_json::from_value(serialized).unwrap();
    assert_eq!(game.board(), &deserialized);
}

#[test]
fn test_serde_action_and_error() {
    let position = (Col::new(1), Row::new(2));
    let action = Action { position };
    test_simple_serialization(&action, json!({"position": [1, 2] }));

    let action_error = ActionError::SpaceIsTaken {
        attempted: position,
    };
    test_simple_serialization(
        &action_error,
        json!({"SpaceIsTaken": {"attempted": [1, 2] }}),
    );
}

#[test]
fn test_serde_ttt() {
    let game = ttt!([
      - - -
      O X O
      X X X
    ]);

    test_simple_serialization(
        &game,
        json!({
            "resigned": [0, 0, 0, 0],
            "board": [
                [0, 0, 0],
                [1, 0, 1],
                [Null, Null, Null]
            ]
        }),
    );
}

#[test]
fn test_serde_ttt_game_runner() {
    let game_runner = GameRunnerBuilder::<TicTacToe>::default()
        .seed(SEED_42)
        .build()
        .unwrap();

    test_simple_serialization(
        &game_runner,
        json!({
            "seed": "2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a",
            "settings": Null,
            "turn_num": 0,
            "state": {
                "resigned": [0, 0, 0, 0],
                "board": [
                    [Null, Null, Null],
                    [Null, Null, Null],
                    [Null, Null, Null]
                ]
            }
        }),
    );

    let mut turn = game_runner.turn().unwrap();
    turn.add_action(
        X,
        Action {
            position: (Col::new(1), Row::new(2)),
        },
    )
    .unwrap();
    let (game_runner, game_advance) = game_runner.submit_turn(turn).unwrap();

    test_simple_serialization(
        &game_advance.spectator_update,
        json!({"Claim": [0, (1, 2)]}),
    );

    test_simple_serialization(
        &game_runner,
        json!({
            "seed": "2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a",
            "settings": Null,
            "turn_num": 1,
            "state": {
                "resigned": [0, 0, 0, 0],
                "board": [
                    [Null, Null, Null],
                    [Null, Null, 0],
                    [Null, Null, Null]
                ]
            }
        }),
    );
    let mut turn = game_runner.turn().unwrap();
    turn.add_action(O, ActionResponse::Resign).unwrap();

    let (game_runner, game_advance) = game_runner.submit_turn(turn).unwrap();

    test_simple_serialization(&game_advance.spectator_update, json!({"Resign": 1}));

    test_simple_serialization(
        &game_runner,
        json!({
            "seed": "2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a",
            "settings": Null,
            "turn_num": 2,
            "state": {
                "resigned": [2, 0, 0, 0],
                "board": [
                    [Null, Null, Null],
                    [Null, Null, 0],
                    [Null, Null, Null]
                ]
            }
        }),
    );
}

fn test_simple_serialization<'a, T, U>(data: &T, expected: U)
where
    T: Serialize + Debug + PartialEq + DeserializeOwned,
    U: Serialize,
{
    let serialized = serde_json::to_value(data).unwrap();
    assert_eq!(serialized, serde_json::to_value(&expected).unwrap());
    let deserialized: T = serde_json::from_value(serialized).unwrap();
    assert_eq!(&deserialized, data);
}
