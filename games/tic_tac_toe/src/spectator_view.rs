use crate::{Position, TicTacToe};
use lttcore::{Player, View};
use serde::{Deserialize, Serialize};
use std::{error::Error, ops::Deref};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpectatorView(TicTacToe);

impl Deref for SpectatorView {
    type Target = TicTacToe;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SpectatorView {
    pub fn from_ttt(ttt: TicTacToe) -> Self {
        Self(ttt)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpectatorViewUpdate {
    Resign(Player),
    Claim(Player, Position),
}

impl View for SpectatorView {
    type Update = SpectatorViewUpdate;

    fn update(&mut self, update: &Self::Update) -> Result<(), Box<dyn Error>> {
        match update {
            SpectatorViewUpdate::Resign(player) => {
                self.0.resign(*player);
            }
            SpectatorViewUpdate::Claim(player, position) => {
                self.0.claim_space(*player, *position)?;
            }
        }
        Ok(())
    }
}
