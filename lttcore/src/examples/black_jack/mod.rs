#![allow(missing_docs)]

use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use crate::{play::{Play, GameState, view::NoSecretPlayerInfo, settings::{NumPlayers, BuiltinGameModes, Builtin}, Score, View, NumberOfPlayers, ActionResponse, GameStateUpdate}, LibTableTopIdentifier, utilities::PlayerIndexedData as PID};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlackJack;

impl LibTableTopIdentifier for BlackJack {
    fn lib_table_top_identifier() -> &'static str {
        "BlackJack"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionError {

}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Settings {

}

impl NumPlayers for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        todo!()
    }
}

impl BuiltinGameModes for Settings {
    fn builtins() -> &'static [Builtin<Self>] {
        &[]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicInfo {

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PublicInfoUpdate {

}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<u64>> {
        todo!()
    }
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameSecretInfo {

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameSecretInfoUpdate {

}

impl View for GameSecretInfo {
    type Update = GameSecretInfoUpdate;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {
        todo!()
    }
}



impl Play for BlackJack {
    type Action = Action;
    type ActionError = ActionError;
    type Settings = Settings;
    type PublicInfo = PublicInfo;
    type PlayerSecretInfo = NoSecretPlayerInfo;
    type GameSecretInfo = GameSecretInfo;

    fn initial_state_for_settings(
        _settings: &Self::Settings,
        _rng: &mut impl rand::Rng,
    ) -> GameState<Self> {
        todo!()
    }

    fn resolve(
        _game_state: &GameState<Self>,
        _settings: &Self::Settings,
        _actions: Cow<'_, PID<ActionResponse<Self>>>,
        _rng: &mut impl rand::Rng,
    ) -> GameStateUpdate<Self> {
        todo!()
    }
}
