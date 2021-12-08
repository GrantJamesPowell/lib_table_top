#![allow(missing_docs)]
#![allow(dead_code)]

use crate::{
    common::deck::{Card, DrawPile},
    play::{
        settings::NumPlayers, view::NoSecretPlayerInfo, ActionResponse, GameState, GameStateUpdate,
        Play, Score, View,
    },
    utilities::{PlayerIndexedData as PID, PlayerSet},
    LibTableTopIdentifier,
};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod bot;
pub mod settings;
pub use settings::Settings;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlackJack;

impl LibTableTopIdentifier for BlackJack {
    fn lib_table_top_identifier() -> &'static str {
        "BlackJack"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicInfo {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PublicInfoUpdate {}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<u64>> {
        todo!()
    }
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSecretInfo {
    draw_pile: DrawPile<Card>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameSecretInfoUpdate {}

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
        settings: &Self::Settings,
        rng: &mut impl rand::Rng,
    ) -> GameState<Self> {
        let player_secret_info = settings
            .number_of_players()
            .players()
            .map(|player| (player, NoSecretPlayerInfo))
            .collect();

        let mut cards = settings.deck.cards();
        cards.shuffle(rng);
        let game_secret_info = GameSecretInfo {
            draw_pile: DrawPile::from(cards),
        };

        GameState {
            player_secret_info,
            game_secret_info,
            public_info: PublicInfo {},
            action_requests: Some(PlayerSet::empty()),
        }
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
