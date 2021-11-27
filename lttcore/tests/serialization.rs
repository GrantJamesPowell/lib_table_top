use lttcore::common::deck::{Card, Color::*, DrawPile, Rank, Suit::*};
use lttcore::examples::guess_the_number::Settings;
use lttcore::examples::GuessTheNumber;
use lttcore::seed::SEED_42;
use lttcore::utilities::number_of_players::FOUR_PLAYER;
use lttcore::{GamePlayer, GameProgression, NumberOfPlayers, Player, PlayerSet, Seed};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value::Null};
use std::fmt::Debug;

#[test]
fn test_serialize_player_set() {
    let mut set: PlayerSet = Default::default();

    assert_eq!(json!([0, 0, 0, 0]), serde_json::to_value(&set).unwrap());

    set.insert(0);

    assert_eq!(json!([1, 0, 0, 0]), serde_json::to_value(&set).unwrap());

    set.insert(1);

    assert_eq!(json!([3, 0, 0, 0]), serde_json::to_value(&set).unwrap());

    set.insert(64);

    assert_eq!(json!([3, 1, 0, 0]), serde_json::to_value(&set).unwrap());

    set.insert(128);
    set.insert(192);

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
    for (bytes, expected) in [
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
    ] {
        let seed: Seed = bytes.into();
        test_simple_serialization((seed, expected));
    }
}

#[test]
fn test_serialize_directions() {
    use lttcore::common::direction::{ArrowKey, Compass, LeftOrRight};

    for test_case in [
        (ArrowKey::Up, "Up"),
        (ArrowKey::Down, "Down"),
        (ArrowKey::Left, "Left"),
        (ArrowKey::Right, "Right"),
    ] {
        test_simple_serialization(test_case);
    }

    for test_case in [(LeftOrRight::Left, "Left"), (LeftOrRight::Right, "Right")] {
        test_simple_serialization(test_case);
    }

    for test_case in [
        (Compass::North, "North"),
        (Compass::East, "East"),
        (Compass::West, "West"),
        (Compass::South, "South"),
    ] {
        test_simple_serialization(test_case);
    }
}

#[test]
fn test_serialize_suits_and_colors() {
    for test_case in [(Clubs, "c"), (Diamonds, "d"), (Hearts, "h"), (Spades, "s")] {
        test_simple_serialization(test_case);
    }

    for test_case in [(Black, "b"), (Red, "r")] {
        test_simple_serialization(test_case);
    }
}

#[test]
fn test_serialize_ranks() {
    for test_case in [
        (Rank::Ace, 1),
        (Rank::Two, 2),
        (Rank::Three, 3),
        (Rank::Four, 4),
        (Rank::Five, 5),
        (Rank::Six, 6),
        (Rank::Seven, 7),
        (Rank::Eight, 8),
        (Rank::Nine, 9),
        (Rank::Ten, 10),
        (Rank::Jack, 11),
        (Rank::Queen, 12),
        (Rank::King, 13),
    ] {
        test_simple_serialization(test_case);
    }
}

#[test]
fn test_serialize_cards() {
    for (card, expected) in [
        ((Rank::Ace, Clubs), "[1,\"c\"]"),
        ((Rank::Four, Diamonds), "[4,\"d\"]"),
        ((Rank::Jack, Hearts), "[11,\"h\"]"),
        ((Rank::King, Spades), "[13,\"s\"]"),
    ] {
        let card: Card = card.into();
        let serialized = serde_json::to_string(&card).unwrap();
        assert_eq!(serialized, expected);
        let deserialized: Card = serde_json::from_str(&serialized).unwrap();
        assert_eq!(card, deserialized);
    }
}

#[test]
fn test_serialize_draw_pile() {
    let empty_draw_pile: DrawPile<Card> = vec![].into();
    let serialized = serde_json::to_value(&empty_draw_pile).unwrap();
    assert_eq!(serialized, json!({"offset": 0, "pile": []}));

    let mut draw_pile: DrawPile<usize> = vec![1, 2, 3].into();
    assert_eq!(draw_pile.draw(), Some(1));
    let serialized = serde_json::to_value(&draw_pile).unwrap();
    assert_eq!(serialized, json!({"offset": 1, "pile": [1,2,3]}));
    let deserialized: DrawPile<usize> = serde_json::from_value(serialized).unwrap();
    assert_eq!(draw_pile, deserialized);
}

#[test]
fn test_serializing_game_player_and_observer_and_updates() {
    let settings: Settings = (1..=10).try_into().unwrap();
    let game: GameProgression<GuessTheNumber> =
        GameProgression::from_settings_and_seed(settings, SEED_42);

    let serialized = serde_json::to_value(&game).unwrap();
    assert_eq!(
        serialized,
        json!({
            "seed":"2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a",
            "settings":{
                "Custom": {
                    "name": serde_json::Value::Null,
                    "settings": {
                        "range": {
                            "start": 1,
                            "end":10
                            },
                        "number_of_players": 1
                    }
                }
            }
            ,
            "initial_state": Null,
            "turn_num": 0,
            "state":{
                "secret_number": 8,
                "guesses": Null},
                "history":[]
        })
    );

    let game_player = game.game_player(0);
    let serialized = serde_json::to_value(&game_player).unwrap();
    assert_eq!(
        serialized,
        json!({
          "game_observer": {
            "turn_num": 0,
            "action_requests": [1, 0, 0, 0],
            "settings": {
                "Custom": {
                    "name": serde_json::Value::Null,
                    "settings": {
                        "range": {
                            "start": 1,
                            "end": 10
                        },
                        "number_of_players": 1
                    }
                }
            },
            "public_info": "InProgress"
          },
          "player": 0,
          "secret_info": Null
        })
    );
    let deserialized: GamePlayer<GuessTheNumber> = serde_json::from_value(serialized).unwrap();
    assert_eq!(deserialized, game_player);
}

fn test_simple_serialization<T, U>((data, expected): (T, U))
where
    T: Serialize + Debug + PartialEq + DeserializeOwned,
    U: Serialize,
{
    let serialized = serde_json::to_value(&data).unwrap();
    assert_eq!(serialized, serde_json::to_value(&expected).unwrap());
    let deserialized: T = serde_json::from_value(serialized).unwrap();
    assert_eq!(deserialized, data);
}
