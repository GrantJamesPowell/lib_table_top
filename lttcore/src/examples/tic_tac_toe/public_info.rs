use super::{Position, TicTacToe};
use crate::play::{Score, View};
use crate::Player;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

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

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        match update.as_ref() {
            PublicInfoUpdate::Resign(player) => {
                self.0.resign(*player);
            }
            PublicInfoUpdate::Claim(player, position) => {
                self.0
                    .claim_space(*player, *position)
                    .expect("ttt recieves a valid PublicInfoUpdate");
            }
        }
    }
}
