mod board;
mod bot;
mod public_info;
mod settings;

pub use board::{Area, Board, Dimensions, Fish, Position, PositionedFish};
pub use bot::{FishFightBot, FishFightBotWrapper, FishFightGuessPov};
pub use public_info::{PublicInfo, PublicInfoUpdate};
pub use settings::Settings;

use crate::play::{ActionResponse, GameAdvance, LttSettings, View};
use crate::utilities::PlayerIndexedData as PID;
use crate::{Play, Player, PlayerSet};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;

pub type FishPositions = SmallVec<[PositionedFish; 4]>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FishFight {
    player_positions: PID<PlayerSecretInfo>,
    public_info: PublicInfo,
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
        self.fish_positions = fish_positions;
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
        Cow::Borrowed(&self.player_positions[player])
    }

    fn public_info(&self, _settings: &Self::Settings) -> Cow<'_, Self::PublicInfo> {
        Cow::Borrowed(&self.public_info)
    }

    fn initial_state_for_settings(settings: &Self::Settings, _rng: &mut impl rand::Rng) -> Self {
        Self {
            player_positions: settings
                .number_of_players()
                .player_indexed_data(|_| Default::default()),
            public_info: PublicInfo::Setup,
        }
    }

    fn which_players_input_needed(&self, settings: &Self::Settings) -> PlayerSet {
        settings.number_of_players().player_set()
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
