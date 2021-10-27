use crate::{Position, TicTacToe};
use lttcore::{Player, View};
use serde::{Deserialize, Serialize};
use std::{error::Error, ops::Deref};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfo(TicTacToe);

impl Deref for PublicInfo {
    type Target = TicTacToe;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PublicInfo {
    pub fn from_ttt(ttt: TicTacToe) -> Self {
        Self(ttt)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInfoUpdate {
    Resign(Player),
    Claim(Player, Position),
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: &Self::Update) -> Result<(), Box<dyn Error>> {
        match update {
            PublicInfoUpdate::Resign(player) => {
                self.0.resign(*player);
            }
            PublicInfoUpdate::Claim(player, position) => {
                self.0.claim_space(*player, *position)?;
            }
        }
        Ok(())
    }
}
