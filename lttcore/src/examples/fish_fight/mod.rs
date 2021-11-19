mod bot;

use crate::utilities::PlayerIndexedData as PID;
use crate::{
    play::{ActionResponse, GameAdvance, LttSettings},
    utilities::number_of_players::TWO_PLAYER,
};
use crate::{NumberOfPlayers, Play, Player, PlayerSet, View};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitVec;

impl BitVec {
    pub fn set(&mut self, _val: bool) {
        todo!()
    }

    pub fn get(&self, _i: usize) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    pub misses: BitVec,
    pub hits: BitVec,
}

type Position = (u8, u8);
type FishPositions = SmallVec<[Position; 8]>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FishFight {
    Setup,
    Playing {
        player_positions: PID<PlayerSecretInfo>,
        public_info: PublicInfo,
    },
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerSecretInfo {
    fish_positions: FishPositions,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerSecretInfoUpdate(FishPositions);

impl View for PlayerSecretInfo {
    type Update = PlayerSecretInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        let PlayerSecretInfoUpdate(fish_positions) = update.into_owned();
        self.fish_positions = Some(fish_positions);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PublicInfo {
    Setup,
    Playing { boards: PID<Board> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicInfoUpdate {
    pub guesses: PID<Position>,
    pub hits: PID<BitVec>,
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, _update: Cow<'_, Self::Update>) {}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Settings {
    width: u8,
    height: u8,
    fish_dimensions: SmallVec<[(u8, u8); 8]>,
    removed_spaces: BitVec,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 15,
            height: 15,
            fish_dimensions: todo!(),
            removed_spaces: Default::default(),
        }
    }
}

impl LttSettings for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        TWO_PLAYER
    }

    fn game_modes() -> &'static HashMap<&'static str, std::sync::Arc<Self>> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Setup(FishPositions),
    Guess(Position),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionError {}

impl Play for FishFight {
    type Action = Action;
    type ActionError = ActionError;
    type Settings = Settings;
    type PublicInfo = PublicInfo;
    type PlayerSecretInfo = PlayerSecretInfo;

    fn player_secret_info(
        &self,
        _settings: &Self::Settings,
        player: Player,
    ) -> Cow<'_, Self::PlayerSecretInfo> {
        match self {
            Self::Setup => Cow::Owned(PlayerSecretInfo::default()),
            Self::Playing {
                player_positions, ..
            } => Cow::Borrowed(&player_positions[player]),
        }
    }

    fn public_info(&self, _settings: &Self::Settings) -> Cow<'_, Self::PublicInfo> {
        match self {
            Self::Setup => Cow::Owned(PublicInfo::default()),
            Self::Playing { public_info, .. } => Cow::Borrowed(public_info),
        }
    }

    fn initial_state_for_settings(_settings: &Self::Settings, _rng: &mut impl rand::Rng) -> Self {
        Self::Setup
    }

    fn which_players_input_needed(&self, _settings: &Self::Settings) -> PlayerSet {
        TWO_PLAYER.player_set()
    }

    fn advance<'a>(
        &'a mut self,
        _settings: &Self::Settings,
        _actions: impl Iterator<Item = (Player, Cow<'a, ActionResponse<Self>>)>,
        _rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self> {
        todo!()
    }
}
