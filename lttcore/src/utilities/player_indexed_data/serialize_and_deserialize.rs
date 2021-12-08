/// Support serializing and deserializing [`PlayerIndexedData`] as a regular ol' map
use super::PlayerIndexedData;
use crate::play::Player;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::fmt;

impl<T: Serialize> Serialize for PlayerIndexedData<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(&k, v)?;
        }
        map.end()
    }
}

struct PlayerIndexedDataVisitor<T> {
    _phantom: std::marker::PhantomData<fn() -> PlayerIndexedData<T>>,
}

impl<T> PlayerIndexedDataVisitor<T> {
    fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for PlayerIndexedDataVisitor<T> {
    type Value = PlayerIndexedData<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("player indexed data")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut pid = PlayerIndexedData::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((player, value)) = access.next_entry::<Player, T>()? {
            pid.insert(player, value);
        }

        Ok(pid)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for PlayerIndexedData<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(PlayerIndexedDataVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::play::Player;

    #[test]
    fn test_serialization() {
        let pid: PlayerIndexedData<u32> = (0..5).map(|i| (Player::new(i), i)).collect();

        assert_eq!(
            serde_json::to_value(&pid).unwrap(),
            serde_json::json!({
                "0": 0,
                "1": 1,
                "2": 2,
                "3": 3,
                "4": 4,
            })
        );
    }

    #[test]
    fn test_deserialization() {
        let pid: PlayerIndexedData<u32> = (0..5).map(|i| (Player::new(i), i)).collect();

        let json = serde_json::json!({
            "0": 0,
            "1": 1,
            "2": 2,
            "3": 3,
            "4": 4,
        });

        let deserialized: PlayerIndexedData<u32> = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, pid);

        let json = serde_json::json!({"foo": "bar"});
        let deserialized: Result<PlayerIndexedData<u32>, _> = serde_json::from_value(json);
        let err = deserialized.unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid type: string \"foo\", expected u32"
        );
    }
}
