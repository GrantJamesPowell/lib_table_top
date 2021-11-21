use lttcore::examples::tic_tac_toe::{Action, ActionError, Col, Row};
use lttcore::ttt;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value::Null};
use std::fmt::Debug;

#[test]
fn test_serde_row_col() {
    let row = Row::new(0);
    let col = Col::new(1);

    test_simple_serialization(&row, 0);
    test_simple_serialization(&col, 1);
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
    let board = ttt!([
      - - -
      O X O
      X X X
    ]);

    test_simple_serialization(
        &board,
        json!([["X", "X", "X"], ["O", "X", "O"], [Null, Null, Null]]),
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
