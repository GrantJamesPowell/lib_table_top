use crate::{
    raw_storage::{RawCustomSettings, RawGameProgression, RawStorage},
    storage::{MetaData, StorageError},
};
use async_trait::async_trait;
use lttcore::{
    encoder::Encoding,
    id::{GameId, SettingsId},
};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct HashMapDB {
    encoding: Encoding,
    custom_settings: RwLock<HashMap<SettingsId, (RawCustomSettings, MetaData)>>,
    game_progression: RwLock<HashMap<GameId, (RawGameProgression, MetaData)>>,
}

#[async_trait]
impl RawStorage for HashMapDB {
    fn encoding(&self) -> Encoding {
        self.encoding
    }

    async fn read_raw_custom_settings(
        &self,
        id: SettingsId,
    ) -> Result<(SettingsId, RawCustomSettings, MetaData), StorageError> {
        self.custom_settings
            .read()
            .expect("rwlock isn't dead")
            .get(&id)
            .map(|(raw, meta)| (id, raw.clone(), meta.clone()))
            .ok_or(StorageError::NotFound)
    }

    async fn read_raw_game_progression(
        &self,
        id: GameId,
    ) -> Result<(GameId, RawGameProgression, MetaData), StorageError> {
        self.game_progression
            .read()
            .expect("rwlock isn't dead")
            .get(&id)
            .map(|(raw, meta)| (id, raw.clone(), meta.clone()))
            .ok_or(StorageError::NotFound)
    }

    async fn write_raw_custom_settings(
        &self,
        (id, raw, meta): (SettingsId, RawCustomSettings, MetaData),
    ) -> Result<(), StorageError> {
        self.custom_settings
            .write()
            .expect("rwlock isn't dead")
            .insert(id, (raw, meta));

        Ok(())
    }

    async fn write_raw_game_progression(
        &self,
        (id, raw, meta): (GameId, RawGameProgression, MetaData),
    ) -> Result<(), StorageError> {
        self.game_progression
            .write()
            .expect("rwlock isn't dead")
            .insert(id, (raw, meta));

        Ok(())
    }
}
