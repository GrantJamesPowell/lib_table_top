use super::storage::{MetaData, SettingsType, StorageError};
use async_trait::async_trait;
use bytes::Bytes;
use lttcore::{
    encoder::Encoding,
    id::{GameId, SettingsId},
};
use lttcore::{utilities::PlayerIndexedData, Seed, TurnNum};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawCustomSettings {
    pub name: Option<String>,
    pub bytes: Bytes,
    pub game_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawHistoryEvent {
    turn_num: TurnNum,
    actions: PlayerIndexedData<Bytes>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawGameProgression {
    seed: Seed,
    settings: SettingsType,
    turn_num: TurnNum,
    initial_state: Bytes,
    history_events: Vec<RawHistoryEvent>,
}

#[async_trait]
pub trait RawStorage: Send + Sync + 'static {
    fn encoding(&self) -> Encoding;

    async fn read_raw_custom_settings(
        &self,
        id: SettingsId,
    ) -> Result<(SettingsId, RawCustomSettings, MetaData), StorageError>;

    async fn read_raw_game_progression(
        &self,
        id: GameId,
    ) -> Result<(GameId, RawGameProgression, MetaData), StorageError>;

    async fn write_raw_custom_settings(
        &self,
        insert: (SettingsId, RawCustomSettings, MetaData),
    ) -> Result<(), StorageError>;

    async fn write_raw_game_progression(
        &self,
        insert: (GameId, RawGameProgression, MetaData),
    ) -> Result<(), StorageError>;
}
