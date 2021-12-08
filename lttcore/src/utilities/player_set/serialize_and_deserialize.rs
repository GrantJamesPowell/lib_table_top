use super::PlayerSet;
use crate::play::Player;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::fmt;

impl Serialize for PlayerSet {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for player in self.iter() {
            seq.serialize_element(&player)?;
        }
        seq.end()
    }
}

struct PlayerSetVisitor;

impl<'de> Visitor<'de> for PlayerSetVisitor {
    type Value = PlayerSet;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("player set")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut access: S) -> Result<Self::Value, S::Error> {
        let mut set = PlayerSet::empty();

        while let Some(player) = access.next_element::<Player>()? {
            set.insert(player);
        }

        Ok(set)
    }
}

impl<'de> Deserialize<'de> for PlayerSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(PlayerSetVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{player_set, utilities::PlayerSet};

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_value(&player_set![]).unwrap(),
            serde_json::json!([])
        );

        let ps = player_set![1, 2, 3, 4, 1000, 2000, 3000, u32::MAX];

        assert_eq!(
            serde_json::to_value(&ps).unwrap(),
            serde_json::json!([1, 2, 3, 4, 1000, 2000, 3000, u32::MAX])
        );
    }

    #[test]
    fn deserialize() {
        // normal
        assert_eq!(
            player_set![1, 2, 3, 4, 1000, 2000, 3000, 4000],
            serde_json::from_str("[1, 2, 3, 4, 1000, 2000, 3000, 4000]").unwrap()
        );
        // with repeats
        assert_eq!(
            player_set![1, 2, 3],
            serde_json::from_str("[1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]").unwrap()
        );

        // errors
        let deserialized: Result<PlayerSet, _> = serde_json::from_str("{\"foo\": \"bar\"}");
        let err = deserialized.unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid type: map, expected player set at line 1 column 0"
        );
    }
}
