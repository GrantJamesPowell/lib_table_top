use lttcore::{number_of_players::FOUR_PLAYER, NumberOfPlayers, Player, PlayerSet, Seed};
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

#[test]
fn test_serialize_rng() {
    let cases = [
        (
            [0u8; 32],
            "0000000000000000000000000000000000000000000000000000000000000000",
        ),
        (
            [1u8; 32],
            "0101010101010101010101010101010101010101010101010101010101010101",
        ),
        (
            [2u8; 32],
            "0202020202020202020202020202020202020202020202020202020202020202",
        ),
        (
            [3u8; 32],
            "0303030303030303030303030303030303030303030303030303030303030303",
        ),
        (
            [4u8; 32],
            "0404040404040404040404040404040404040404040404040404040404040404",
        ),
        (
            [16u8; 32],
            "1010101010101010101010101010101010101010101010101010101010101010",
        ),
        (
            [32u8; 32],
            "2020202020202020202020202020202020202020202020202020202020202020",
        ),
        (
            [64u8; 32],
            "4040404040404040404040404040404040404040404040404040404040404040",
        ),
        (
            [128u8; 32],
            "8080808080808080808080808080808080808080808080808080808080808080",
        ),
        (
            [255u8; 32],
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ),
    ];

    for &(bytes, hex) in cases.iter() {
        let seed: Seed = bytes.into();
        let serialized = serde_json::to_value(&seed).unwrap();
        assert_eq!(serialized, json!(hex));
        let deserialized: Seed = serde_json::from_value(serialized).unwrap();
        assert_eq!(&seed, &deserialized);
    }
}
