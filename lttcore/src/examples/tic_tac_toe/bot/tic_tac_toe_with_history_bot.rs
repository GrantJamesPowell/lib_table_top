#![allow(missing_docs)]

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Display, panic::RefUnwindSafe};

use crate::{
    bot::StatefulBot,
    examples::{
        tic_tac_toe::{Action, Board, Marker, Position, PublicInfo, PublicInfoUpdate},
        TicTacToe,
    },
    play::{
        view::{NoSecretPlayerInfo, NoSecretPlayerInfoUpdate},
        Seed, TurnNum,
    },
    pov::player::PlayerPov,
};

pub trait TicTacToeWithHistoryBot:
    RefUnwindSafe + Clone + Sync + Send + 'static + Serialize + DeserializeOwned
{
    fn claim_space(
        &self,
        board: &Board,
        seed: &Seed,
        history: &[(TurnNum, Marker, Position)],
    ) -> Position;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct TicTacToeWithHistoryBotWrapper<T: TicTacToeWithHistoryBot> {
    bot: T,
    history: Vec<(TurnNum, Marker, Position)>,
}

impl<T: TicTacToeWithHistoryBot + Display> Display for TicTacToeWithHistoryBotWrapper<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Just use the inner attribute's `Display`
        write!(fmt, "{}", self.bot)
    }
}

impl<T: TicTacToeWithHistoryBot> StatefulBot for TicTacToeWithHistoryBotWrapper<T> {
    type Game = TicTacToe;

    fn on_action_request(&mut self, player_pov: &PlayerPov<'_, TicTacToe>, seed: &Seed) -> Action {
        let board = player_pov.public_info.board();
        let position = self.bot.claim_space(board, seed, &self.history);
        Action { position }
    }

    fn on_turn_advance(
        &mut self,
        turn_num: TurnNum,
        _public_info: &PublicInfo,
        _player_secret_info: &NoSecretPlayerInfo,
        public_info_update: &PublicInfoUpdate,
        _player_secret_info_update: Option<&NoSecretPlayerInfoUpdate>,
    ) {
        if let PublicInfoUpdate::Claim(marker, position) = public_info_update {
            self.history.push((turn_num, *marker, *position));
        }
    }
}
