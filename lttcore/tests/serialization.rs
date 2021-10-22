use lttcore::PlayerSet;
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
