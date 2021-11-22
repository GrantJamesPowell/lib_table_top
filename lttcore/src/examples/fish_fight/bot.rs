use super::{Action, FishFight, FishPositions, Phase, PlayerBoards, Settings};
use crate::bots::Bot;
use crate::common::cartesian::Point;
use crate::pov::PlayerPov;
use crate::utilities::PlayerIndexedData as PID;
use crate::{Play, Player, TurnNum};

pub trait FishFightBot {
    fn setup_board(settings: &Settings, rng: &mut impl rand::Rng) -> FishPositions;
    fn guess(pov: FishFightGuessPov<'_>, rng: &mut impl rand::Rng) -> Point;
}

#[derive(Debug, PartialEq, Eq)]
pub struct FishFightGuessPov<'a> {
    pub turn_num: TurnNum,
    pub player: Player,
    pub settings: &'a Settings,
    pub boards: &'a PID<PlayerBoards>,
    pub player_fish_positions: &'a FishPositions,
}

#[derive(Debug)]
pub struct FishFightBotWrapper<T: FishFightBot>(T);

impl<T: FishFightBot> Bot for FishFightBotWrapper<T> {
    type Game = FishFight;

    fn run(
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> <Self::Game as Play>::Action {
        match player_pov.public_info.phase() {
            Phase::Setup => {
                let position = T::setup_board(player_pov.settings, rng);
                Action::Setup(position)
            }
            Phase::Playing => {
                let pov = FishFightGuessPov {
                    turn_num: player_pov.turn_num,
                    player: player_pov.player,
                    settings: player_pov.settings,
                    player_fish_positions: &player_pov.secret_info.fish_positions,
                    boards: player_pov.public_info.player_boards(),
                };
                let position = T::guess(pov, rng);
                Action::Guess(position)
            }
        }
    }
}
