use lttcore::{number_of_players::FOUR_PLAYER, NumberOfPlayers, Player, PlayerSet};
use serde_json::json;

#[test]
fn test_serialize_player_set() {
    let mut set: PlayerSet = Default::default();

    assert_eq!(json!([0, 0, 0, 0]), serde_json::to_value(&set).unwrap());

    set.add(0);

    assert_eq!(json!([1, 0, 0, 0]), serde_json::to_value(&set).unwrap());

    set.add(1);

    assert_eq!(json!([3, 0, 0, 0]), serde_json::to_value(&set).unwrap());

    set.add(64);

    assert_eq!(json!([3, 1, 0, 0]), serde_json::to_value(&set).unwrap());

    set.add(128);
    set.add(192);

    assert_eq!(json!([3, 1, 1, 1]), serde_json::to_value(&set).unwrap());
}

#[test]
fn test_serialize_player() {
    let player: Player = 1.into();
    assert_eq!(json!(1), serde_json::to_value(&player).unwrap());
}

#[test]
fn test_serialize_number_of_players() {
    let num_players: NumberOfPlayers = serde_json::from_str("3").unwrap();
    assert_eq!(num_players.get(), 3);
    assert_eq!(json!(4), serde_json::to_value(&FOUR_PLAYER).unwrap());
}
