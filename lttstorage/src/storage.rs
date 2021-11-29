use async_trait::async_trait;

use super::raw_storage::{RawCustomSettings, RawStorage};
use chrono::prelude::*;
use lttcore::play::{settings::Custom, Play};
use lttcore::{
    encoder::EncodingError,
    id::{GameId, SettingsId, UserId},
    GameProgression,
};
use std::borrow::Cow;

#[derive(Debug)]
pub enum StorageError {
    NotFound,
    EncodingError(EncodingError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsType {
    Builtin(String),
    Custom(SettingsId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaData {
    pub owner: Option<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait Storage<T: Play>: Send + Sync + 'static {
    async fn read_custom_settings(
        &self,
        id: SettingsId,
    ) -> Result<(SettingsId, Custom<T::Settings>, MetaData), StorageError>;

    async fn write_custom_settings(
        &self,
        settings: (SettingsId, Custom<T::Settings>, MetaData),
    ) -> Result<(), StorageError>;

    async fn read_game_progression(
        &self,
        id: GameId,
    ) -> Result<(GameId, GameProgression<T>, MetaData), StorageError>;
}

#[async_trait]
impl<T: Play, Raw: RawStorage> Storage<T> for Raw {
    async fn read_custom_settings(
        &self,
        id: SettingsId,
    ) -> Result<(SettingsId, Custom<T::Settings>, MetaData), StorageError> {
        self.read_raw_custom_settings(id)
            .await
            .and_then(|(id, raw, meta)| {
                let settings = self
                    .encoding()
                    .deserialize(raw.bytes)
                    .map_err(StorageError::EncodingError)?;

                let custom = Custom {
                    name: raw.name.map(Cow::Owned),
                    settings,
                };
                Ok((id, custom, meta))
            })
    }

    async fn read_game_progression(
        &self,
        _id: GameId,
    ) -> Result<(GameId, GameProgression<T>, MetaData), StorageError> {
        todo!()
    }

    async fn write_custom_settings(
        &self,
        (id, custom, meta): (SettingsId, Custom<T::Settings>, MetaData),
    ) -> Result<(), StorageError> {
        let raw = RawCustomSettings {
            name: custom.name.map(|name| name.into_owned()),
            game_type: String::from(T::lib_table_top_identifier()),
            bytes: self
                .encoding()
                .serialize(&custom.settings)
                .map_err(StorageError::EncodingError)?,
        };
        self.write_raw_custom_settings((id, raw, meta)).await
    }
}
